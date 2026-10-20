//! Ona: client de Spotify lleuger. Aquí viu tot el que no és interfície:
//! sessió de Spotify (librespot), reproductor, endpoints interns de Spotify i lletres.
//! Les dades venen dels endpoints spclient que fa servir el client d'escriptori (metadades
//! protobuf, playlist4, col·lecció, context-resolve) amb la sessió de librespot: no cal cap Client ID propi.

mod auth;
mod connect;
mod local;
mod lyrics;
mod media;
mod meta;
mod pb;
mod solo;
mod taskbar;

use std::{path::PathBuf, sync::Arc};

use librespot::{
    core::{
        authentication::Credentials, cache::Cache, config::SessionConfig, session::Session,
        spotify_id::SpotifyId, SpotifyUri,
    },
    playback::{
        audio_backend,
        config::{AudioFormat, Bitrate, PlayerConfig},
        mixer::{softmixer::SoftMixer, Mixer, MixerConfig},
        player::{Player, PlayerEvent},
    },
};
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_opener::OpenerExt;
use tokio::sync::Mutex;

const OAUTH_SCOPES: &[&str] = &[
    "streaming",
    "user-read-private",
    "user-read-email",
    "playlist-read-private",
    "playlist-read-collaborative",
    "playlist-modify-private",
    "playlist-modify-public",
    "user-library-read",
    "user-library-modify",
    "user-read-recently-played",
    "user-top-read",
    "user-follow-read",
    "user-read-playback-state",
    "user-modify-playback-state",
];
const AUDIO_CACHE_BYTES: u64 = 512 * 1024 * 1024;

type Res<T> = Result<T, String>;

/// Error que la interfície reconeix: credencials desades que Spotify ja no accepta.
const SESSION_EXPIRED: &str = "session_expired";

fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}

#[derive(Default)]
struct Inner {
    session: Option<Session>,
    player: Option<Arc<Player>>,
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

struct Ona {
    inner: Mutex<Inner>,
    mixer: Arc<SoftMixer>,
    data_dir: PathBuf,
    http: reqwest::Client,
    /// Inici de sessió en curs al navegador; un de nou cancel·la l'anterior (i allibera el port).
    pending_login: std::sync::Mutex<Option<tokio::task::AbortHandle>>,
    now: SharedNow,
    connect: connect::Connect,
}

impl Ona {
    fn forget_credentials(&self) {
        let _ = std::fs::remove_file(self.data_dir.join("auth").join("credentials.json"));
    }

    fn cache(&self) -> Res<Cache> {
        Cache::new(
            Some(self.data_dir.join("auth")),
            None,
            Some(self.data_dir.join("audio")),
            Some(AUDIO_CACHE_BYTES),
        )
        .map_err(err)
    }

    /// Obre una sessió nova i la connecta al reproductor (que es crea el primer cop).
    async fn start(&self, app: &AppHandle, creds: Credentials) -> Res<String> {
        let session = Session::new(SessionConfig::default(), Some(self.cache()?));
        // Credencials rebutjades (caducades o revocades): la interfície ho diu i torna a demanar l'inici de sessió.
        session.connect(creds, true).await.map_err(|e| {
            if e.kind == librespot::core::error::ErrorKind::PermissionDenied && e.to_string().contains("credentials") {
                SESSION_EXPIRED.to_string()
            } else {
                err(e)
            }
        })?;
        let user = session.username();

        let mut inner = self.inner.lock().await;
        let player = match &inner.player {
            Some(player) => {
                player.set_session(session.clone());
                player.clone()
            }
            None => {
                let player = self.make_player(session.clone())?;
                forward_events(app.clone(), &player, self.now.clone());
                inner.player = Some(player.clone());
                player
            }
        };
        inner.session = Some(session);
        self.connect.start(app.clone(), self.cache()?, player);
        Ok(user)
    }

    fn make_player(&self, session: Session) -> Res<Arc<Player>> {
        let config = PlayerConfig {
            bitrate: Bitrate::Bitrate320,
            normalisation: true,
            ..PlayerConfig::default()
        };
        let backend = audio_backend::find(None).ok_or("No hi ha cap sortida d'àudio")?;
        Ok(Player::new(config, session, self.mixer.get_soft_volume(), move || {
            backend(None, AudioFormat::default())
        }))
    }

    /// Retorna una sessió vàlida, reconnectant amb les credencials desades si s'ha caigut.
    async fn session(&self, app: &AppHandle) -> Res<Session> {
        let current = self.inner.lock().await.session.clone();
        if let Some(s) = current.filter(|s| !s.is_invalid()) {
            return Ok(s);
        }
        let creds = self.cache()?.credentials().ok_or("not_logged_in")?;
        self.start(app, creds).await?;
        self.inner
            .lock()
            .await
            .session
            .clone()
            .ok_or_else(|| "not_logged_in".into())
    }

    async fn player(&self, app: &AppHandle) -> Res<Arc<Player>> {
        self.session(app).await?;
        self.inner
            .lock()
            .await
            .player
            .clone()
            .ok_or_else(|| "not_logged_in".into())
    }
}

/// Reenvia els esdeveniments del reproductor a la interfície com a `player`.
/// Què sona ara mateix, per si la interfície es recarrega mentre el reproductor continua.
#[derive(Clone)]
struct NowState {
    uri: String,
    position_ms: u32,
    playing: bool,
    at: std::time::Instant,
}

type SharedNow = Arc<std::sync::Mutex<Option<NowState>>>;

fn forward_events(app: AppHandle, player: &Arc<Player>, now: SharedNow) {
    let mut rx = player.get_player_event_channel();
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            remember(&now, &event);
            solo::on_event(&app, &event);
            let payload = match event {
                PlayerEvent::Loading { track_id, position_ms, .. } => {
                    json!({ "kind": "loading", "uri": track_id.to_string(), "position_ms": position_ms })
                }
                PlayerEvent::Playing { track_id, position_ms, .. }
                | PlayerEvent::PositionCorrection { track_id, position_ms, .. }
                | PlayerEvent::Seeked { track_id, position_ms, .. } => {
                    json!({ "kind": "playing", "uri": track_id.to_string(), "position_ms": position_ms })
                }
                PlayerEvent::Paused { track_id, position_ms, .. } => {
                    json!({ "kind": "paused", "uri": track_id.to_string(), "position_ms": position_ms })
                }
                PlayerEvent::TimeToPreloadNextTrack { track_id, .. } => {
                    json!({ "kind": "preload", "uri": track_id.to_string() })
                }
                PlayerEvent::EndOfTrack { track_id, .. } => {
                    json!({ "kind": "ended", "uri": track_id.to_string() })
                }
                PlayerEvent::Unavailable { track_id, .. } => {
                    json!({ "kind": "unavailable", "uri": track_id.to_string() })
                }
                PlayerEvent::Stopped { track_id, .. } => {
                    json!({ "kind": "stopped", "uri": track_id.to_string() })
                }
                _ => continue,
            };
            let _ = app.emit("player", payload);
        }
    });
}

fn remember(now: &SharedNow, event: &PlayerEvent) {
    let set = |uri: &SpotifyUri, position_ms: u32, playing: bool| {
        *now.lock().unwrap() = Some(NowState {
            uri: uri.to_string(),
            position_ms,
            playing,
            at: std::time::Instant::now(),
        });
    };
    match event {
        PlayerEvent::Loading { track_id, position_ms, .. } => set(track_id, *position_ms, false),
        PlayerEvent::Playing { track_id, position_ms, .. }
        | PlayerEvent::PositionCorrection { track_id, position_ms, .. }
        | PlayerEvent::Seeked { track_id, position_ms, .. } => set(track_id, *position_ms, true),
        PlayerEvent::Paused { track_id, position_ms, .. } => set(track_id, *position_ms, false),
        PlayerEvent::Stopped { .. } => *now.lock().unwrap() = None,
        _ => {}
    }
}

/// Estat actual del reproductor (o null si no sona res): la interfície el demana en arrencar.
#[tauri::command]
fn player_state(ona: State<'_, Ona>) -> Option<Value> {
    let s = ona.now.lock().unwrap().clone()?;
    let pos = if s.playing {
        s.position_ms as u128 + s.at.elapsed().as_millis()
    } else {
        s.position_ms as u128
    };
    Some(json!({ "uri": s.uri, "position_ms": pos, "playing": s.playing }))
}

// ---------- Compte ----------

/// En arrencar: si hi ha credencials desades, entra sol. Retorna el nom d'usuari o null.
#[tauri::command]
async fn restore(app: AppHandle, ona: State<'_, Ona>) -> Res<Option<String>> {
    // Si la interfície es recarrega, la sessió continua viva: no cal tornar a connectar.
    if let Some(s) = ona.inner.lock().await.session.clone().filter(|s| !s.is_invalid()) {
        return Ok(Some(s.username()));
    }
    let Some(creds) = ona.cache()?.credentials() else {
        return Ok(None);
    };
    let r = ona.start(&app, creds).await;
    if matches!(&r, Err(e) if e == SESSION_EXPIRED) {
        ona.forget_credentials();
    }
    r.map(Some)
}

/// OAuth al navegador. Cancel·la qualsevol intent anterior que encara esperi.
async fn browser_oauth(app: &AppHandle, ona: &Ona, client_id: String) -> Res<auth::Tokens> {
    let http = ona.http.clone();
    let opener = app.clone();
    let task = tauri::async_runtime::spawn(async move {
        auth::authorize(&http, &client_id, OAUTH_SCOPES, |url| {
            opener.opener().open_url(url, None::<&str>).map_err(err)
        })
        .await
    });
    if let Some(prev) = ona
        .pending_login
        .lock()
        .unwrap()
        .replace(task.inner().abort_handle())
    {
        prev.abort();
    }
    match task.await {
        Ok(r) => r,
        Err(_) => Err("cancelled".into()),
    }
}

/// Inicia sessió amb el compte de Spotify (librespot, client oficial) al navegador.
#[tauri::command]
async fn login(app: AppHandle, ona: State<'_, Ona>) -> Res<String> {
    if let Some(creds) = ona.cache()?.credentials() {
        match ona.start(&app, creds).await {
            // Si Spotify les rebutja, s'esborren i es torna al navegador (abans l'usuari quedava encallat).
            Err(e) if e == SESSION_EXPIRED => ona.forget_credentials(),
            r => return r,
        }
    }
    let token = browser_oauth(&app, &ona, SessionConfig::default().client_id).await?;
    ona.start(&app, Credentials::with_access_token(token.access)).await
}

#[tauri::command]
async fn logout(ona: State<'_, Ona>) -> Res<()> {
    ona.connect.shutdown();
    let mut inner = ona.inner.lock().await;
    if let Some(player) = &inner.player {
        player.stop();
    }
    if let Some(session) = inner.session.take() {
        session.shutdown();
    }
    ona.forget_credentials();
    Ok(())
}

// ---------- Reproductor ----------

/// Botons de la safata, tecles multimèdia i miniatura: els porta Rust si no hi ha interfície.
pub fn control(app: &AppHandle, payload: Value) {
    if !solo::control(app, &payload) {
        let _ = app.emit("media", payload);
    }
}

#[tauri::command]
fn quit(app: AppHandle) {
    solo::HANDED.store(true, std::sync::atomic::Ordering::Release);
    app.exit(0);
}

/// Botons d'Ona mentre sona el que ha posat el mòbil (Spotify Connect).
#[tauri::command]
fn connect_cmd(ona: State<'_, Ona>, action: String, ms: Option<u32>) -> Res<()> {
    ona.connect.command(&action, ms)
}

#[tauri::command]
async fn play(app: AppHandle, ona: State<'_, Ona>, uri: String, position_ms: Option<u32>) -> Res<()> {
    let uri = SpotifyUri::from_uri(&uri).map_err(err)?;
    ona.connect.release();
    ona.player(&app).await?.load(uri, true, position_ms.unwrap_or(0));
    Ok(())
}

#[tauri::command]
async fn preload(app: AppHandle, ona: State<'_, Ona>, uri: String) -> Res<()> {
    let uri = SpotifyUri::from_uri(&uri).map_err(err)?;
    ona.player(&app).await?.preload(uri);
    Ok(())
}

#[tauri::command]
async fn resume(app: AppHandle, ona: State<'_, Ona>) -> Res<()> {
    ona.connect.release();
    ona.player(&app).await?.play();
    Ok(())
}

#[tauri::command]
async fn pause(app: AppHandle, ona: State<'_, Ona>) -> Res<()> {
    ona.player(&app).await?.pause();
    Ok(())
}

#[tauri::command]
async fn seek(app: AppHandle, ona: State<'_, Ona>, position_ms: u32) -> Res<()> {
    ona.player(&app).await?.seek(position_ms);
    Ok(())
}

/// `volume` de 0.0 a 1.0.
#[tauri::command]
fn set_volume(ona: State<'_, Ona>, volume: f64) {
    let v = (volume.clamp(0.0, 1.0) * u16::MAX as f64) as u16;
    ona.mixer.set_volume(v);
    ona.connect.set_volume(v);
}

// ---------- Endpoints interns i lletres ----------

/// Pistes (nom, artistes, àlbum, portada, durada) per a una llista d'URIs, per lots.
#[tauri::command]
async fn tracks(app: AppHandle, ona: State<'_, Ona>, uris: Vec<String>) -> Res<Value> {
    meta::tracks(&ona.session(&app).await?, &uris).await
}

/// Targetes (nom, artistes, portada) d'àlbums.
#[tauri::command]
async fn albums(app: AppHandle, ona: State<'_, Ona>, uris: Vec<String>) -> Res<Value> {
    meta::albums(&ona.session(&app).await?, &uris).await
}

/// Targetes (nom, retrat) d'artistes.
#[tauri::command]
async fn artists(app: AppHandle, ona: State<'_, Ona>, uris: Vec<String>) -> Res<Value> {
    meta::artists(&ona.session(&app).await?, &uris).await
}

#[tauri::command]
async fn album(app: AppHandle, ona: State<'_, Ona>, id: String) -> Res<Value> {
    meta::album(&ona.session(&app).await?, &id).await
}

#[tauri::command]
async fn artist(app: AppHandle, ona: State<'_, Ona>, id: String) -> Res<Value> {
    meta::artist(&ona.session(&app).await?, &id).await
}

/// Fitxers d'àudio de la carpeta Música (es reprodueixen amb l'<audio> de la interfície).
#[tauri::command]
async fn local_tracks(app: AppHandle) -> Res<Value> {
    let dir = app.path().audio_dir().map_err(err)?;
    tauri::async_runtime::spawn_blocking(move || local::tracks(&dir)).await.map_err(err)
}

/// Cerca de cançons: el context `spotify:search:<text>` que resol el reproductor.
#[tauri::command]
async fn search(app: AppHandle, ona: State<'_, Ona>, q: String) -> Res<Value> {
    let term: String = q
        .trim()
        .bytes()
        .map(|b| match b {
            b' ' => "+".to_string(),
            b if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) => (b as char).to_string(),
            b => format!("%{b:02X}"),
        })
        .collect();
    let session = ona.session(&app).await?;
    let ctx = session.spclient().get_context(&format!("spotify:search:{term}")).await.map_err(err)?;
    let uris: Vec<String> = ctx.pages.iter().flat_map(|p| &p.tracks).map(|t| t.uri().to_string()).take(50).collect();
    meta::tracks(&session, &uris).await
}

/// Pistes d'un context (playlist, àlbum, artista, M'agrada...) com les resol el reproductor.
#[tauri::command]
async fn context(app: AppHandle, ona: State<'_, Ona>, uri: String) -> Res<Value> {
    if !uri.starts_with("spotify:") || uri.contains(['/', '?', '#']) {
        return Err("URI no vàlida".into());
    }
    let session = ona.session(&app).await?;
    let bytes = session
        .spclient()
        .request_as_json(&reqwest::Method::GET, &format!("/context-resolve/v1/{uri}"), None, None)
        .await
        .map_err(err)?;
    serde_json::from_slice(&bytes).map_err(err)
}

fn list_header(a: &librespot::protocol::playlist4_external::ListAttributes) -> Value {
    let image = a
        .picture
        .as_ref()
        .filter(|p| !p.is_empty())
        .map(|p| format!("https://i.scdn.co/image/{}", p.iter().map(|b| format!("{b:02x}")).collect::<String>()))
        // Les playlists de Spotify (DJ, mixos...) porten la imatge com a URLs per mida.
        .or_else(|| a.picture_size.iter().find(|p| p.target_name() == "default").or(a.picture_size.first()).map(|p| p.url().to_string()));
    json!({ "name": a.name(), "description": a.description(), "image": image })
}

/// Les playlists de la biblioteca (el "rootlist"), amb carpetes aplanades.
#[tauri::command]
async fn rootlist(app: AppHandle, ona: State<'_, Ona>) -> Res<Value> {
    use librespot::protocol::playlist4_external::SelectedListContent;
    use protobuf::Message;
    let session = ona.session(&app).await?;
    let bytes = session.spclient().get_rootlist(0, Some(1000)).await.map_err(err)?;
    let list = SelectedListContent::parse_from_bytes(&bytes).map_err(err)?;
    Ok(list
        .contents
        .items
        .iter()
        .zip(&list.contents.meta_items)
        .filter(|(i, _)| i.uri().starts_with("spotify:playlist:"))
        .map(|(i, m)| {
            let mut v = list_header(&m.attributes);
            v["uri"] = json!(i.uri());
            v["owner"] = json!(m.owner_username());
            v["total"] = json!(m.length());
            v
        })
        .collect())
}

/// Capçalera i URIs d'una playlist (protobuf playlist4, com el client natiu).
#[tauri::command]
async fn playlist(app: AppHandle, ona: State<'_, Ona>, id: String) -> Res<Value> {
    use librespot::protocol::playlist4_external::SelectedListContent;
    use protobuf::Message;
    let id = SpotifyId::from_base62(&id).map_err(err)?;
    let session = ona.session(&app).await?;
    let bytes = session.spclient().get_playlist(&id).await.map_err(err)?;
    let list = SelectedListContent::parse_from_bytes(&bytes).map_err(err)?;
    let mut v = list_header(&list.attributes);
    v["owner"] = json!(list.owner_username());
    v["uris"] = list.contents.items.iter().map(|i| i.uri()).collect();
    Ok(v)
}

const COLLECTION_PROTO: &str = "application/vnd.collection-v2.spotify.proto";

/// Un "set" de la col·lecció: "collection" (M'agrada i àlbums), "artist", "ylpin" (fixats).
/// Retorna [{uri, added_at}] del més nou al més antic.
#[tauri::command]
async fn collection(app: AppHandle, ona: State<'_, Ona>, set: String) -> Res<Value> {
    let session = ona.session(&app).await?;
    let user = session.username();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("content-type", reqwest::header::HeaderValue::from_static(COLLECTION_PROTO));
    let mut out = Vec::new();
    let mut token = String::new();
    // shortcut: tall a 50 pàgines (15.000 elements), prou per a una biblioteca personal.
    for _ in 0..50 {
        let body = pb::page_request(&user, &set, &token, 300);
        let bytes = session
            .spclient()
            .request(&reqwest::Method::POST, "/collection/v2/paging", Some(headers.clone()), Some(&body))
            .await
            .map_err(err)?;
        let (items, next) = pb::page_response(&bytes).ok_or("resposta de col·lecció il·legible")?;
        out.extend(items.into_iter().filter(|i| !i.removed).map(|i| json!({ "uri": i.uri, "added_at": i.added_at })));
        if next.is_empty() {
            break;
        }
        token = next;
    }
    Ok(Value::Array(out))
}

/// Afegeix o treu URIs d'un set de la col·lecció (M'agrada, desar àlbum, seguir artista).
#[tauri::command]
async fn collection_write(app: AppHandle, ona: State<'_, Ona>, set: String, uris: Vec<String>, remove: bool) -> Res<()> {
    let session = ona.session(&app).await?;
    let body = pb::write_request(&session.username(), &set, &uris, remove, unix_now());
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("content-type", reqwest::header::HeaderValue::from_static(COLLECTION_PROTO));
    session
        .spclient()
        .request(&reqwest::Method::POST, "/collection/v2/write", Some(headers), Some(&body))
        .await
        .map_err(err)?;
    Ok(())
}

/// Perfil de l'usuari (nom, imatge...).
#[tauri::command]
async fn me(app: AppHandle, ona: State<'_, Ona>) -> Res<Value> {
    let session = ona.session(&app).await?;
    let bytes = session
        .spclient()
        .get_user_profile(&session.username(), Some(0), Some(0))
        .await
        .map_err(err)?;
    serde_json::from_slice(&bytes).map_err(err)
}

/// Lletra sincronitzada: primer la de Spotify, si no n'hi ha, LRCLIB.
#[tauri::command]
async fn get_lyrics(
    app: AppHandle,
    ona: State<'_, Ona>,
    id: String,
    title: String,
    artist: String,
    album: String,
    duration_ms: u32,
    syllables: Option<bool>,
) -> Res<Value> {
    // Primer la de síl·labes (si no és el mode lleuger), després la de Spotify (per línies) i LRCLIB.
    if syllables.unwrap_or(true) {
        if let Some(found) = lyrics::from_betterlyrics(&ona.http, &title, &artist, &album, duration_ms).await {
            return Ok(found);
        }
    }
    if let Ok(session) = ona.session(&app).await {
        if let Ok(track) = SpotifyId::from_base62(&id) {
            if let Ok(bytes) = session.spclient().get_lyrics(&track).await {
                if let Some(found) = lyrics::from_spotify(&bytes) {
                    return Ok(found);
                }
            }
        }
    }
    Ok(lyrics::from_lrclib(&ona.http, &title, &artist, &album, duration_ms)
        .await
        .unwrap_or(Value::Null))
}

/// Quan la finestra es minimitza, WebView2 allibera tota la memòria que pot (i la recupera en tornar).
#[cfg(windows)]
fn set_low_memory(window: &tauri::WebviewWindow, low: bool) {
    use webview2_com::Microsoft::Web::WebView2::Win32::{
        ICoreWebView2_19, COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_LOW,
        COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_NORMAL,
    };
    use windows_core::Interface;
    let _ = window.with_webview(move |wv| unsafe {
        let Ok(core) = wv.controller().CoreWebView2() else { return };
        if let Ok(core19) = core.cast::<ICoreWebView2_19>() {
            let level = if low {
                COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_LOW
            } else {
                COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_NORMAL
            };
            let _ = core19.SetMemoryUsageTargetLevel(level);
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn,ona=info")).try_init();
    tauri::Builder::default()
        // Només una Ona oberta: si se n'obre una altra, es tanca i porta la primera al davant.
        // (Ha d'anar primer de tot.)
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| taskbar::show_main(app)))
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            let mixer = SoftMixer::open(MixerConfig::default())?;
            let http = reqwest::Client::builder()
                .user_agent(concat!("Ona/", env!("CARGO_PKG_VERSION"), " (https://github.com/)"))
                .build()?;
            let media = media::Media::default();
            media.set(media::init(app.handle()));
            app.manage(media);
            app.manage(solo::Solo::default());
            taskbar::init(app.handle())?;
            app.manage(Ona {
                inner: Mutex::new(Inner::default()),
                mixer: Arc::new(mixer),
                data_dir,
                http,
                pending_login: Default::default(),
                now: Default::default(),
                connect: Default::default(),
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            // Tancar la finestra principal: si sona música, la interfície passa la cua a Rust i
            // Ona continua sense finestra (mode «només so»); si no, tanca l'app (també la lletra flotant).
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    let app = window.app_handle().clone();
                    let ona = app.state::<Ona>();
                    let playing = ona.now.lock().unwrap().as_ref().is_some_and(|n| n.playing) || ona.connect.is_active();
                    if !playing {
                        return app.exit(0);
                    }
                    api.prevent_close();
                    solo::HANDED.store(false, std::sync::atomic::Ordering::Release);
                    let _ = window.emit("close-to-tray", ());
                    // Si la interfície no respon, es tanca com abans.
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_secs(3));
                        if !solo::HANDED.load(std::sync::atomic::Ordering::Acquire) {
                            app.exit(0);
                        }
                    });
                }
            }
            if let tauri::WindowEvent::Resized(_) = event {
                #[cfg(windows)]
                {
                    static LOW: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
                    let minimized = window.is_minimized().unwrap_or(false);
                    if LOW.swap(minimized, std::sync::atomic::Ordering::Relaxed) != minimized {
                        if let Some(w) = window.app_handle().get_webview_window(window.label()) {
                            set_low_memory(&w, minimized);
                        }
                    }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            restore, login, logout, connect_cmd, play, preload, resume, pause, seek, set_volume, player_state, media::media_update, media::media_state, tracks, albums, artists, album, artist, search, local_tracks, context, rootlist, playlist, collection, collection_write, me,
            get_lyrics, quit, solo::sound_only, solo::sound_only_take
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            // En mode «només so» no queda cap finestra, però l'app ha de continuar sonant.
            if let tauri::RunEvent::ExitRequested { api, code: None, .. } = event {
                if app.state::<solo::Solo>().active() {
                    api.prevent_exit();
                }
            }
        });
}
