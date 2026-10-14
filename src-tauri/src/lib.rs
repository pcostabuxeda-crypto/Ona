//! Ona: client de Spotify lleuger. Aquí viu tot el que no és interfície:
//! sessió de Spotify (librespot), reproductor, proxy de la Web API i lletres.

mod auth;
mod lyrics;
mod media;
mod viz;

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

fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}

#[derive(Default)]
struct Inner {
    session: Option<Session>,
    player: Option<Arc<Player>>,
    web: Option<WebToken>,
}

/// Token OAuth per a la Web API. Va amb el client ID propi de l'usuari:
/// el de librespot (keymaster) el comparteix tothom i Spotify el té sempre limitat (429).
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct WebToken {
    client_id: String,
    access: String,
    refresh: String,
    /// Segons UNIX en què caduca.
    expires: u64,
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
    viz: Arc<viz::VizShared>,
    now: SharedNow,
}

impl Ona {
    fn cache(&self) -> Res<Cache> {
        Cache::new(
            Some(self.data_dir.join("auth")),
            None,
            Some(self.data_dir.join("audio")),
            Some(AUDIO_CACHE_BYTES),
        )
        .map_err(err)
    }

    fn client_id_path(&self) -> PathBuf {
        self.data_dir.join("client_id.txt")
    }

    fn client_id(&self) -> Option<String> {
        let id = std::fs::read_to_string(self.client_id_path()).ok()?;
        let id = id.trim().to_string();
        (!id.is_empty()).then_some(id)
    }

    fn token_path(&self) -> PathBuf {
        self.data_dir.join("auth").join("web.json")
    }

    fn save_web(&self, t: &WebToken) {
        if let Ok(json) = serde_json::to_string(t) {
            let _ = std::fs::write(self.token_path(), json);
        }
    }

    fn load_web(&self) -> Option<WebToken> {
        serde_json::from_str(&std::fs::read_to_string(self.token_path()).ok()?).ok()
    }

    /// Token vàlid per a la Web API; el renova si queda menys d'un minut.
    async fn web_token(&self) -> Res<String> {
        let cached = self.inner.lock().await.web.clone().or_else(|| self.load_web());
        let mut t = cached.ok_or("not_logged_in")?;
        if t.expires > unix_now() + 60 {
            self.inner.lock().await.web = Some(t.clone());
            return Ok(t.access);
        }
        let fresh = auth::refresh(&self.http, &t.client_id, &t.refresh)
            .await
            .map_err(|e| {
                log::warn!("refresh: {e}");
                "not_logged_in".to_string()
            })?;
        t.access = fresh.access;
        t.refresh = fresh.refresh;
        t.expires = unix_now() + fresh.expires_in;
        self.save_web(&t);
        self.inner.lock().await.web = Some(t.clone());
        Ok(t.access)
    }

    /// Obre una sessió nova i la connecta al reproductor (que es crea el primer cop).
    async fn start(&self, app: &AppHandle, creds: Credentials) -> Res<String> {
        let session = Session::new(SessionConfig::default(), Some(self.cache()?));
        session.connect(creds, true).await.map_err(err)?;
        let user = session.username();

        let mut inner = self.inner.lock().await;
        match &inner.player {
            Some(player) => player.set_session(session.clone()),
            None => {
                let player = self.make_player(session.clone())?;
                forward_events(app.clone(), &player, self.now.clone());
                inner.player = Some(player);
            }
        }
        inner.session = Some(session);
        Ok(user)
    }

    fn make_player(&self, session: Session) -> Res<Arc<Player>> {
        let config = PlayerConfig {
            bitrate: Bitrate::Bitrate320,
            normalisation: true,
            ..PlayerConfig::default()
        };
        let backend = audio_backend::find(None).ok_or("No hi ha cap sortida d'àudio")?;
        let shared = self.viz.clone();
        Ok(Player::new(
            config,
            session,
            self.mixer.get_soft_volume(),
            move || {
                Box::new(viz::VizSink {
                    inner: backend(None, AudioFormat::default()),
                    shared,
                })
            },
        ))
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
    let web_id = ona.client_id();
    if web_id.is_none() || ona.load_web().map(|t| Some(t.client_id)) != Some(web_id) {
        return Ok(None);
    }
    ona.start(&app, creds).await.map(Some)
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

/// Inicia sessió en dos passos (cadascun només si cal):
/// 1. reproducció: librespot amb el client de Spotify;
/// 2. Web API: el client ID propi de l'usuari.
#[tauri::command]
async fn login(app: AppHandle, ona: State<'_, Ona>) -> Res<String> {
    let web_id = ona.client_id().ok_or("no_client_id")?;

    let user = match ona.cache()?.credentials() {
        Some(creds) => ona.start(&app, creds).await?,
        None => {
            let token = browser_oauth(&app, &ona, SessionConfig::default().client_id).await?;
            ona.start(&app, Credentials::with_access_token(token.access))
                .await?
        }
    };

    if ona.load_web().filter(|t| t.client_id == web_id).is_none() {
        let token = browser_oauth(&app, &ona, web_id.clone()).await?;
        let web = WebToken {
            client_id: web_id,
            access: token.access,
            refresh: token.refresh,
            expires: unix_now() + token.expires_in,
        };
        ona.save_web(&web);
        ona.inner.lock().await.web = Some(web);
    }
    Ok(user)
}

#[tauri::command]
fn get_client_id(ona: State<'_, Ona>) -> Option<String> {
    ona.client_id()
}

#[tauri::command]
fn set_client_id(ona: State<'_, Ona>, client_id: String) -> Res<()> {
    let id = client_id.trim();
    if id.len() != 32 || !id.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("El Client ID són 32 caràcters (lletres a-f i números).".into());
    }
    std::fs::create_dir_all(&ona.data_dir).map_err(err)?;
    std::fs::write(ona.client_id_path(), id).map_err(err)
}

#[tauri::command]
async fn logout(ona: State<'_, Ona>) -> Res<()> {
    let mut inner = ona.inner.lock().await;
    if let Some(player) = &inner.player {
        player.stop();
    }
    if let Some(session) = inner.session.take() {
        session.shutdown();
    }
    inner.web = None;
    let _ = std::fs::remove_file(ona.data_dir.join("auth").join("credentials.json"));
    let _ = std::fs::remove_file(ona.token_path());
    Ok(())
}

// ---------- Reproductor ----------

#[tauri::command]
async fn play(app: AppHandle, ona: State<'_, Ona>, uri: String, position_ms: Option<u32>) -> Res<()> {
    let uri = SpotifyUri::from_uri(&uri).map_err(err)?;
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
    ona.mixer
        .set_volume((volume.clamp(0.0, 1.0) * u16::MAX as f64) as u16);
}

/// Activa o atura l'anàlisi d'espectre (només es fa mentre la interfície la mostra).
#[tauri::command]
fn set_viz(ona: State<'_, Ona>, enabled: bool) {
    ona.viz
        .enabled
        .store(enabled, std::sync::atomic::Ordering::Relaxed);
}

// ---------- Web API i lletres ----------

/// Proxy de la Web API de Spotify (cerca, playlists, biblioteca...). `path` comença per `/v1/`.
#[tauri::command]
async fn api(
    ona: State<'_, Ona>,
    path: String,
    method: Option<String>,
    body: Option<Value>,
) -> Res<Value> {
    if !path.starts_with("/v1/") {
        return Err("Ruta no permesa".into());
    }
    let token = ona.web_token().await?;
    let method = reqwest::Method::from_bytes(method.as_deref().unwrap_or("GET").as_bytes())
        .map_err(err)?;
    let mut req = ona
        .http
        .request(method, format!("https://api.spotify.com{path}"))
        .bearer_auth(&token);
    if let Some(body) = body {
        req = req.json(&body);
    }
    let resp = req.send().await.map_err(err)?;
    let status = resp.status();
    if status.as_u16() == 429 {
        let wait = resp
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("?")
            .to_string();
        return Err(format!("rate_limited:{wait}"));
    }
    let text = resp.text().await.map_err(err)?;
    if !status.is_success() {
        log::warn!("API {path} -> {status}: {text}");
        return Err(format!("HTTP {status}: {text}"));
    }
    if text.trim().is_empty() {
        return Ok(Value::Null);
    }
    serde_json::from_str(&text).map_err(err)
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
) -> Res<Value> {
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
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.unminimize();
                let _ = w.show();
                let _ = w.set_focus();
            }
        }))
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
            let viz = Arc::new(viz::VizShared::default());
            viz::spawn_analyser(app.handle().clone(), viz.clone());
            app.manage(Ona {
                inner: Mutex::new(Inner::default()),
                mixer: Arc::new(mixer),
                data_dir,
                http,
                pending_login: Default::default(),
                viz,
                now: Default::default(),
            });
            Ok(())
        })
        .on_window_event(|window, event| {
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
            restore, login, logout, get_client_id, set_client_id, play, preload, resume, pause, seek, set_volume, set_viz, player_state, media::media_update, media::media_state, api,
            get_lyrics
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
