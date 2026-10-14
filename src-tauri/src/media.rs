//! Controls multimèdia de Windows (SMTC): tecles multimèdia, la caràtula del volum de Windows
//! i apps com FluentFlyout. La interfície hi envia què sona; els botons tornen com a event `media`.

use std::{ffi::c_void, sync::Mutex, time::Duration};

use serde_json::json;
use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, MediaPosition, PlatformConfig, SeekDirection};
use tauri::{AppHandle, Emitter, Manager, State};

#[derive(Default)]
pub struct Media(Mutex<Option<MediaControls>>);

/// Registra Ona als controls multimèdia del sistema. Cal la finestra (HWND) ja creada.
pub fn init(app: &AppHandle) -> Option<MediaControls> {
    let window = app.get_webview_window("main")?;
    let hwnd = window.hwnd().ok()?.0 as *mut c_void;
    let mut controls = MediaControls::new(PlatformConfig {
        display_name: "Ona",
        dbus_name: "cat.ona.app",
        hwnd: Some(hwnd),
    })
    .map_err(|e| log::warn!("SMTC: {e}"))
    .ok()?;

    let handle = app.clone();
    controls
        .attach(move |event| {
            let payload = match event {
                MediaControlEvent::Play => json!({ "action": "play" }),
                MediaControlEvent::Pause => json!({ "action": "pause" }),
                MediaControlEvent::Toggle => json!({ "action": "toggle" }),
                MediaControlEvent::Next => json!({ "action": "next" }),
                MediaControlEvent::Previous => json!({ "action": "prev" }),
                MediaControlEvent::Stop => json!({ "action": "pause" }),
                MediaControlEvent::SetPosition(MediaPosition(d)) => {
                    json!({ "action": "seek", "ms": d.as_millis() as u64 })
                }
                MediaControlEvent::Seek(dir) | MediaControlEvent::SeekBy(dir, _) => json!({
                    "action": "seekBy",
                    "ms": if dir == SeekDirection::Forward { 10_000 } else { -10_000 }
                }),
                _ => return,
            };
            let _ = handle.emit("media", payload);
        })
        .map_err(|e| log::warn!("SMTC attach: {e}"))
        .ok()?;
    Some(controls)
}

impl Media {
    pub fn set(&self, controls: Option<MediaControls>) {
        *self.0.lock().unwrap() = controls;
    }
}

/// Cançó actual (null si no sona res).
#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn media_update(
    media: State<'_, Media>,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    cover: Option<String>,
    duration_ms: Option<u64>,
) {
    let mut guard = media.0.lock().unwrap();
    let Some(c) = guard.as_mut() else { return };
    if title.is_none() {
        let _ = c.set_playback(MediaPlayback::Stopped);
        return;
    }
    let _ = c.set_metadata(MediaMetadata {
        title: title.as_deref(),
        artist: artist.as_deref(),
        album: album.as_deref(),
        cover_url: cover.as_deref(),
        duration: duration_ms.map(Duration::from_millis),
    });
}

/// Estat (sona / en pausa) i posició, perquè la barra de temps del sistema vagi bé.
#[tauri::command]
pub fn media_state(media: State<'_, Media>, playing: bool, position_ms: u64) {
    let mut guard = media.0.lock().unwrap();
    let Some(c) = guard.as_mut() else { return };
    let progress = Some(MediaPosition(Duration::from_millis(position_ms)));
    let _ = c.set_playback(if playing {
        MediaPlayback::Playing { progress }
    } else {
        MediaPlayback::Paused { progress }
    });
}
