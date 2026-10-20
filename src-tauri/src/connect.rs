//! Spotify Connect: Ona surt com a dispositiu a l'app de Spotify del mòbil.
//! Té la seva pròpia sessió i el seu propi reproductor perquè Spirc s'apropia del reproductor
//! que fa servir (adopta qualsevol càrrega i en salta la següent en acabar), i la cua d'Ona viu a la
//! interfície. Quan el mòbil fa sonar Ona, el reproductor local es posa en pausa; quan l'usuari
//! reprodueix des d'Ona, Connect s'allibera. El volum també és a part: Spirc imposa el seu a cada
//! reproducció, i el controlen el mòbil o el control de volum d'Ona mentre mana Connect.

use std::sync::{
    atomic::{AtomicBool, AtomicU32, Ordering},
    Arc, Mutex,
};

use librespot::{
    connect::{ConnectConfig, Spirc},
    core::{cache::Cache, config::DeviceType, config::SessionConfig, session::Session},
    playback::{
        audio_backend,
        config::{AudioFormat, Bitrate, PlayerConfig},
        mixer::{softmixer::SoftMixer, Mixer, MixerConfig},
        player::{Player, PlayerEvent},
    },
};
use serde_json::json;
use tauri::{AppHandle, Emitter};

pub struct Connect {
    spirc: Mutex<Option<Spirc>>,
    /// Connect fa sonar (o té carregada) una cançó en aquest PC.
    active: Arc<AtomicBool>,
    running: Arc<AtomicBool>,
    /// Volum que ha triat l'usuari a Ona (u32::MAX = encara cap), per començar amb el mateix.
    volume: AtomicU32,
}

impl Default for Connect {
    fn default() -> Self {
        Self {
            spirc: Default::default(),
            active: Default::default(),
            running: Default::default(),
            volume: AtomicU32::new(u32::MAX),
        }
    }
}

impl Connect {
    /// Engega el dispositiu Connect (si no corre ja). Es fa en segon pla: no endarrereix l'inici.
    pub fn start(&self, app: AppHandle, cache: Cache, local: Arc<Player>) {
        if self.running.swap(true, Ordering::AcqRel) {
            return;
        }
        let Some(creds) = cache.credentials() else {
            self.running.store(false, Ordering::Release);
            return;
        };
        let (running, active) = (self.running.clone(), self.active.clone());
        let initial_volume = u16::try_from(self.volume.load(Ordering::Acquire)).unwrap_or(u16::MAX / 2);
        let slot = app.clone();
        tauri::async_runtime::spawn(async move {
            let session = Session::new(SessionConfig::default(), Some(cache));
            let Ok(mixer) = SoftMixer::open(MixerConfig::default()) else { return running.store(false, Ordering::Release) };
            let mixer = Arc::new(mixer);
            let Some(backend) = audio_backend::find(None) else { return running.store(false, Ordering::Release) };
            let config = PlayerConfig { bitrate: Bitrate::Bitrate320, normalisation: true, ..PlayerConfig::default() };
            let player = Player::new(config, session.clone(), mixer.get_soft_volume(), move || {
                backend(None, AudioFormat::default())
            });
            forward(app, &player, local, active.clone());
            let connect = ConnectConfig {
                name: "Ona".into(),
                device_type: DeviceType::Computer,
                initial_volume,
                ..ConnectConfig::default()
            };
            match Spirc::new(connect, session, creds, player, mixer as Arc<dyn Mixer>).await {
                Ok((spirc, task)) => {
                    use tauri::Manager;
                    *slot.state::<crate::Ona>().connect.spirc.lock().unwrap() = Some(spirc);
                    task.await;
                    log::info!("Spotify Connect aturat");
                }
                Err(e) => log::warn!("Spotify Connect: {e}"),
            }
            active.store(false, Ordering::Release);
            running.store(false, Ordering::Release);
        });
    }

    /// Volum d'Ona; si mana Connect, també el del dispositiu Connect (el mòbil el veu canviar).
    pub fn set_volume(&self, v: u16) {
        self.volume.store(v as u32, Ordering::Release);
        if self.active.load(Ordering::Acquire) {
            if let Some(s) = self.spirc.lock().unwrap().as_ref() {
                let _ = s.set_volume(v);
            }
        }
    }

    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Acquire)
    }

    /// L'usuari reprodueix des d'Ona: Connect para i deixa de ser el dispositiu actiu.
    pub fn release(&self) {
        if self.active.swap(false, Ordering::AcqRel) {
            if let Some(s) = self.spirc.lock().unwrap().as_ref() {
                let _ = s.disconnect(true);
            }
        }
    }

    pub fn shutdown(&self) {
        if let Some(s) = self.spirc.lock().unwrap().take() {
            let _ = s.shutdown();
        }
    }

    pub fn command(&self, action: &str, ms: Option<u32>) -> Result<(), String> {
        let guard = self.spirc.lock().unwrap();
        let s = guard.as_ref().ok_or("Spotify Connect no està engegat")?;
        match action {
            "toggle" => s.play_pause(),
            "next" => s.next(),
            "prev" => s.prev(),
            "seek" => s.set_position_ms(ms.unwrap_or(0)),
            _ => return Err(format!("acció desconeguda: {action}")),
        }
        .map_err(|e| e.to_string())
    }
}

/// Esdeveniments del reproductor de Connect cap a la interfície com a `connect`.
fn forward(app: AppHandle, player: &Arc<Player>, local: Arc<Player>, active: Arc<AtomicBool>) {
    let mut rx = player.get_player_event_channel();
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            // Després d'alliberar Connect encara arriben la pausa i la desconnexió: no són del mòbil.
            let mine = active.load(Ordering::Acquire);
            let payload = match event {
                PlayerEvent::Loading { .. } | PlayerEvent::Paused { .. } if !mine => continue,
                PlayerEvent::Loading { track_id, position_ms, .. } => {
                    json!({ "kind": "loading", "uri": track_id.to_string(), "position_ms": position_ms })
                }
                PlayerEvent::Playing { track_id, position_ms, .. }
                | PlayerEvent::PositionCorrection { track_id, position_ms, .. }
                | PlayerEvent::Seeked { track_id, position_ms, .. } => {
                    // El mòbil mana: el que sonava a Ona es posa en pausa.
                    if !active.swap(true, Ordering::AcqRel) {
                        local.pause();
                    }
                    json!({ "kind": "playing", "uri": track_id.to_string(), "position_ms": position_ms })
                }
                PlayerEvent::Paused { track_id, position_ms, .. } => {
                    json!({ "kind": "paused", "uri": track_id.to_string(), "position_ms": position_ms })
                }
                PlayerEvent::SessionDisconnected { .. } | PlayerEvent::Stopped { .. } => {
                    if !active.swap(false, Ordering::AcqRel) {
                        continue;
                    }
                    json!({ "kind": "inactive" })
                }
                _ => continue,
            };
            let _ = app.emit("connect", payload);
        }
    });
}
