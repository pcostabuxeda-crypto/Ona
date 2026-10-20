//! Pantalla de bloqueig i Centre de control d'iOS (MPNowPlayingInfoCenter i MPRemoteCommandCenter;
//! souvlaki no hi arriba). La sessió d'àudio va en mode reproducció perquè la música continuï amb
//! la pantalla bloquejada. Els botons tornen com el mateix event `media` que a l'escriptori.

use std::{ptr::NonNull, sync::Mutex};

use block2::RcBlock;
use objc2::{rc::Retained, runtime::AnyObject};
use objc2_avf_audio::{AVAudioSession, AVAudioSessionCategoryPlayback};
use objc2_foundation::{NSDictionary, NSNumber, NSString};
use objc2_media_player::{
    MPChangePlaybackPositionCommandEvent, MPMediaItemPropertyAlbumTitle, MPMediaItemPropertyArtist,
    MPMediaItemPropertyPlaybackDuration, MPMediaItemPropertyTitle, MPNowPlayingInfoCenter,
    MPNowPlayingInfoPropertyElapsedPlaybackTime, MPNowPlayingInfoPropertyPlaybackRate, MPRemoteCommand,
    MPRemoteCommandCenter, MPRemoteCommandEvent, MPRemoteCommandHandlerStatus,
};
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter, State};

struct Track {
    title: String,
    artist: String,
    album: String,
    duration_ms: Option<u64>,
}

/// La cançó actual, per tornar-la a publicar quan canvia l'estat o la posició.
#[derive(Default)]
pub struct Media(Mutex<Option<Track>>);

/// Activa la sessió d'àudio i registra els botons del sistema.
pub fn init(app: &AppHandle) -> Media {
    unsafe {
        let session = AVAudioSession::sharedInstance();
        if let Some(category) = AVAudioSessionCategoryPlayback {
            if let Err(e) = session.setCategory_error(category) {
                log::warn!("AVAudioSession: {e:?}");
            }
        }
        if let Err(e) = session.setActive_error(true) {
            log::warn!("AVAudioSession actiu: {e:?}");
        }

        let center = MPRemoteCommandCenter::sharedCommandCenter();
        on(&center.playCommand(), app, |_| Some(json!({ "action": "play" })));
        on(&center.pauseCommand(), app, |_| Some(json!({ "action": "pause" })));
        on(&center.togglePlayPauseCommand(), app, |_| Some(json!({ "action": "toggle" })));
        on(&center.nextTrackCommand(), app, |_| Some(json!({ "action": "next" })));
        on(&center.previousTrackCommand(), app, |_| Some(json!({ "action": "prev" })));
        on(&center.changePlaybackPositionCommand(), app, |ev| {
            let s = ev.cast::<MPChangePlaybackPositionCommandEvent>().as_ref().positionTime();
            Some(json!({ "action": "seek", "ms": (s.max(0.0) * 1000.0) as u64 }))
        });
    }
    Media::default()
}

unsafe fn on(
    command: &MPRemoteCommand,
    app: &AppHandle,
    payload: impl Fn(NonNull<MPRemoteCommandEvent>) -> Option<Value> + 'static,
) {
    let app = app.clone();
    let block = RcBlock::new(move |ev: NonNull<MPRemoteCommandEvent>| match payload(ev) {
        Some(p) => {
            let _ = app.emit("media", p);
            MPRemoteCommandHandlerStatus::Success
        }
        None => MPRemoteCommandHandlerStatus::CommandFailed,
    });
    command.setEnabled(true);
    // El centre de comandes reté els blocs: no cal guardar-los.
    let _ = command.addTargetWithHandler(&block);
}

// shortcut: sense caràtula (MPMediaItemArtwork vol una UIImage descarregada); afegir-la si l'usuari la troba a faltar.
fn publish(t: &Track, playing: bool, position_ms: u64) {
    unsafe {
        let title = NSString::from_str(&t.title);
        let artist = NSString::from_str(&t.artist);
        let album = NSString::from_str(&t.album);
        let elapsed = NSNumber::new_f64(position_ms as f64 / 1000.0);
        let rate = NSNumber::new_f64(if playing { 1.0 } else { 0.0 });
        let mut keys: Vec<&NSString> = vec![
            MPMediaItemPropertyTitle,
            MPMediaItemPropertyArtist,
            MPMediaItemPropertyAlbumTitle,
            MPNowPlayingInfoPropertyElapsedPlaybackTime,
            MPNowPlayingInfoPropertyPlaybackRate,
        ];
        let mut objects: Vec<&AnyObject> = vec![&title, &artist, &album, &elapsed, &rate];
        let duration: Option<Retained<NSNumber>> = t.duration_ms.map(|d| NSNumber::new_f64(d as f64 / 1000.0));
        if let Some(d) = &duration {
            keys.push(MPMediaItemPropertyPlaybackDuration);
            objects.push(d);
        }
        let info = NSDictionary::from_slices(&keys, &objects);
        MPNowPlayingInfoCenter::defaultCenter().setNowPlayingInfo(Some(&info));
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
    let _ = cover;
    let mut guard = media.0.lock().unwrap();
    *guard = title.map(|title| Track {
        title,
        artist: artist.unwrap_or_default(),
        album: album.unwrap_or_default(),
        duration_ms,
    });
    match guard.as_ref() {
        Some(t) => publish(t, false, 0),
        None => unsafe { MPNowPlayingInfoCenter::defaultCenter().setNowPlayingInfo(None) },
    }
}

/// Estat (sona / en pausa) i posició, perquè la barra de temps de la pantalla de bloqueig vagi bé.
#[tauri::command]
pub fn media_state(media: State<'_, Media>, playing: bool, position_ms: u64) {
    if let Some(t) = media.0.lock().unwrap().as_ref() {
        publish(t, playing, position_ms);
    }
}
