//! Mode «només so»: si es tanca la finestra mentre sona música, es destrueix la WebView (la part
//! que més memòria gasta) i Rust continua la cua que li ha passat la interfície. Les tecles
//! multimèdia, la safata i el mòbil (Connect) continuen funcionant. En tornar a obrir Ona, la
//! interfície demana quantes cançons ha avançat Rust i continua des d'allà.

use std::sync::{atomic::AtomicBool, Arc, Mutex};

/// La interfície ha respost a «close-to-tray» (si no, es tanca l'app com abans).
pub static HANDED: AtomicBool = AtomicBool::new(false);

use librespot::{
    core::SpotifyUri,
    playback::player::{Player, PlayerEvent},
};
use serde::Deserialize;
use serde_json::Value;
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, State, WebviewWindowBuilder};

use crate::{media::Media, Ona, Res};

/// Una cançó de la cua, amb el que cal per als controls del sistema.
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    uri: String,
    title: String,
    artist: String,
    album: Option<String>,
    cover: Option<String>,
    duration_ms: Option<u64>,
}

struct Queue {
    items: Vec<Item>,
    cursor: usize,
    repeat_one: bool,
    player: Arc<Player>,
}

struct Geometry {
    pos: PhysicalPosition<i32>,
    size: PhysicalSize<u32>,
    maximized: bool,
}

#[derive(Default)]
pub struct Solo {
    queue: Mutex<Option<Queue>>,
    geometry: Mutex<Option<Geometry>>,
}

impl Solo {
    pub fn active(&self) -> bool {
        self.queue.lock().unwrap().is_some()
    }
}

fn load(app: &AppHandle, q: &Queue) {
    let Some(it) = q.items.get(q.cursor) else { return };
    if let Ok(uri) = SpotifyUri::from_uri(&it.uri) {
        q.player.load(uri, true, 0);
    }
    let media = app.state::<Media>();
    crate::media::update(app, &media, Some(&it.title), Some(&it.artist), it.album.as_deref(), it.cover.as_deref(), it.duration_ms);
}

/// Següent de la cua; fals si no n'hi ha més.
fn advance(app: &AppHandle, q: &mut Queue) -> bool {
    if !q.repeat_one {
        if q.cursor + 1 >= q.items.len() {
            return false;
        }
        q.cursor += 1;
    }
    load(app, q);
    true
}

/// La interfície tanca la finestra amb música: es queda la cua i es destrueixen les WebViews.
#[tauri::command]
pub async fn sound_only(app: AppHandle, ona: State<'_, Ona>, items: Vec<Item>, repeat_one: bool) -> Res<()> {
    let player = ona.inner.lock().await.player.clone().ok_or("not_logged_in")?;
    if let Some(w) = app.get_webview_window("main") {
        if let (Ok(pos), Ok(size)) = (w.outer_position(), w.inner_size()) {
            let maximized = w.is_maximized().unwrap_or(false);
            *app.state::<Solo>().geometry.lock().unwrap() = Some(Geometry { pos, size, maximized });
        }
    }
    *app.state::<Solo>().queue.lock().unwrap() = Some(Queue { items, cursor: 0, repeat_one, player });
    HANDED.store(true, std::sync::atomic::Ordering::Release);
    for w in app.webview_windows().into_values() {
        let _ = w.destroy();
    }
    log::info!("mode només so");
    Ok(())
}

/// En tornar a obrir Ona: quantes cançons ha avançat Rust (i Rust deixa de portar la cua).
#[tauri::command]
pub fn sound_only_take(solo: State<'_, Solo>) -> usize {
    solo.queue.lock().unwrap().take().map_or(0, |q| q.cursor)
}

/// Torna a crear la finestra principal (amb la mida i el lloc d'abans).
pub fn reopen(app: &AppHandle) {
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || {
        if handle.get_webview_window("main").is_some() {
            return;
        }
        let Some(conf) = handle.config().app.windows.iter().find(|w| w.label == "main").cloned() else { return };
        let w = match WebviewWindowBuilder::from_config(&handle, &conf).and_then(|b| b.build()) {
            Ok(w) => w,
            Err(e) => return log::warn!("no s'ha pogut tornar a obrir la finestra: {e}"),
        };
        if let Some(g) = handle.state::<Solo>().geometry.lock().unwrap().take() {
            let _ = w.set_position(g.pos);
            let _ = w.set_size(g.size);
            if g.maximized {
                let _ = w.maximize();
            }
        }
        let _ = w.set_focus();
        crate::taskbar::reinit_thumb(&handle);
    });
}

/// Esdeveniments del reproductor mentre no hi ha interfície.
pub fn on_event(app: &AppHandle, event: &PlayerEvent) {
    let solo = app.state::<Solo>();
    let mut guard = solo.queue.lock().unwrap();
    let Some(q) = guard.as_mut() else { return };
    match event {
        PlayerEvent::EndOfTrack { .. } | PlayerEvent::Unavailable { .. } => {
            if !advance(app, q) {
                crate::media::set_state(app, &app.state::<Media>(), false, 0);
            }
        }
        PlayerEvent::TimeToPreloadNextTrack { .. } if !q.repeat_one => {
            if let Some(uri) = q.items.get(q.cursor + 1).and_then(|it| SpotifyUri::from_uri(&it.uri).ok()) {
                q.player.preload(uri);
            }
        }
        PlayerEvent::Playing { position_ms, .. } | PlayerEvent::Seeked { position_ms, .. } => {
            crate::media::set_state(app, &app.state::<Media>(), true, *position_ms as u64);
        }
        PlayerEvent::Paused { position_ms, .. } => {
            crate::media::set_state(app, &app.state::<Media>(), false, *position_ms as u64);
        }
        _ => {}
    }
}

/// Tecles multimèdia, safata i miniatura sense interfície. Fals si hi ha interfície (ho fa ella).
pub fn control(app: &AppHandle, payload: &Value) -> bool {
    let solo = app.state::<Solo>();
    let mut guard = solo.queue.lock().unwrap();
    let Some(q) = guard.as_mut() else { return false };
    let action = payload["action"].as_str().unwrap_or("");
    let ms = payload["ms"].as_i64();
    let ona = app.state::<Ona>();
    // Si el mòbil fa sonar Ona, els botons són per a Connect.
    if ona.connect.is_active() {
        let _ = match action {
            "play" | "pause" | "toggle" => ona.connect.command("toggle", None),
            "next" | "prev" => ona.connect.command(action, None),
            "seek" => ona.connect.command("seek", ms.map(|m| m as u32)),
            _ => Ok(()),
        };
        return true;
    }
    let now = ona.now.lock().unwrap().clone();
    let playing = now.as_ref().is_some_and(|n| n.playing);
    let position = now.as_ref().map_or(0, |n| n.position_ms as i64 + if n.playing { n.at.elapsed().as_millis() as i64 } else { 0 });
    match action {
        "play" => q.player.play(),
        "pause" => q.player.pause(),
        "toggle" if playing => q.player.pause(),
        "toggle" => q.player.play(),
        "next" => {
            if !q.repeat_one && q.cursor + 1 < q.items.len() {
                q.cursor += 1;
                load(app, q);
            }
        }
        "prev" if position > 3000 || q.cursor == 0 => q.player.seek(0),
        "prev" => {
            q.cursor -= 1;
            load(app, q);
        }
        "seek" => q.player.seek(ms.unwrap_or(0).max(0) as u32),
        "seekBy" => q.player.seek((position + ms.unwrap_or(0)).max(0) as u32),
        _ => {}
    }
    true
}
