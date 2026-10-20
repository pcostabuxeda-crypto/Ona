//! Controls multimèdia de Windows (SMTC): tecles multimèdia, la caràtula del volum de Windows
//! i apps com FluentFlyout. La interfície hi envia què sona; els botons tornen com a event `media`.

use std::{ffi::c_void, sync::Mutex, time::Duration};

use serde_json::json;
use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, MediaPosition, PlatformConfig, SeekDirection};
use tauri::{AppHandle, State};

#[derive(Default)]
pub struct Media(Mutex<Option<MediaControls>>);

/// Finestra nativa invisible només per als controls multimèdia: així funcionen encara que la finestra
/// principal es tanqui (mode «només so»). S'ha de crear al fil principal, que en processa els missatges.
#[cfg(windows)]
fn hidden_window() -> Option<*mut c_void> {
    use windows::{
        core::w,
        Win32::{
            Foundation::{HWND, LPARAM, LRESULT, WPARAM},
            System::LibraryLoader::GetModuleHandleW,
            UI::WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, RegisterClassW, WINDOW_EX_STYLE, WNDCLASSW, WS_OVERLAPPED},
        },
    };
    unsafe extern "system" fn proc(h: HWND, m: u32, w: WPARAM, l: LPARAM) -> LRESULT {
        DefWindowProcW(h, m, w, l)
    }
    unsafe {
        let instance = GetModuleHandleW(None).ok()?.into();
        let class = WNDCLASSW { lpfnWndProc: Some(proc), hInstance: instance, lpszClassName: w!("OnaMedia"), ..Default::default() };
        RegisterClassW(&class);
        let hwnd = CreateWindowExW(WINDOW_EX_STYLE(0), w!("OnaMedia"), w!("Ona"), WS_OVERLAPPED, 0, 0, 0, 0, None, None, Some(instance), None).ok()?;
        Some(hwnd.0)
    }
}

/// Registra Ona als controls multimèdia del sistema.
pub fn init(app: &AppHandle) -> Option<MediaControls> {
    let hwnd = hidden_window()?;
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
            crate::control(&handle, payload);
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
    app: AppHandle,
    media: State<'_, Media>,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    cover: Option<String>,
    duration_ms: Option<u64>,
) {
    update(&app, &media, title.as_deref(), artist.as_deref(), album.as_deref(), cover.as_deref(), duration_ms);
}

pub fn update(
    app: &AppHandle,
    media: &Media,
    title: Option<&str>,
    artist: Option<&str>,
    album: Option<&str>,
    cover: Option<&str>,
    duration_ms: Option<u64>,
) {
    crate::taskbar::set_track(app, title, artist);
    let mut guard = media.0.lock().unwrap();
    let Some(c) = guard.as_mut() else { return };
    if title.is_none() {
        let _ = c.set_playback(MediaPlayback::Stopped);
        return;
    }
    let _ = c.set_metadata(MediaMetadata { title, artist, album, cover_url: cover, duration: duration_ms.map(Duration::from_millis) });
}

/// Estat (sona / en pausa) i posició, perquè la barra de temps del sistema vagi bé.
#[tauri::command]
pub fn media_state(app: AppHandle, media: State<'_, Media>, playing: bool, position_ms: u64) {
    set_state(&app, &media, playing, position_ms);
}

pub fn set_state(app: &AppHandle, media: &Media, playing: bool, position_ms: u64) {
    crate::taskbar::set_playing(app, playing);
    let mut guard = media.0.lock().unwrap();
    let Some(c) = guard.as_mut() else { return };
    let progress = Some(MediaPosition(Duration::from_millis(position_ms)));
    let _ = c.set_playback(if playing {
        MediaPlayback::Playing { progress }
    } else {
        MediaPlayback::Paused { progress }
    });
}
