//! Visualitzador: s'interposa entre el reproductor i la sortida d'àudio, guarda les últimes
//! mostres i, ~30 cops per segon (només si la interfície ho demana), envia l'espectre en bandes.

use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

use librespot::playback::{
    audio_backend::{Sink, SinkResult},
    convert::Converter,
    decoder::AudioPacket,
};
use rustfft::{num_complex::Complex, FftPlanner};
use tauri::{AppHandle, Emitter, Manager};

const FFT_SIZE: usize = 2048;
const BANDS: usize = 64;
const SAMPLE_RATE: f32 = 44_100.0;
const FRAME: Duration = Duration::from_millis(33);

#[derive(Default)]
pub struct VizShared {
    pub enabled: AtomicBool,
    ring: Mutex<Ring>,
}

#[derive(Default)]
struct Ring {
    samples: VecDeque<f32>,
    last_write: Option<Instant>,
}

/// Sortida d'àudio que copia les mostres (en mono) abans de passar-les a la de debò.
pub struct VizSink {
    pub inner: Box<dyn Sink>,
    pub shared: Arc<VizShared>,
}

impl Sink for VizSink {
    fn start(&mut self) -> SinkResult<()> {
        self.inner.start()
    }

    fn stop(&mut self) -> SinkResult<()> {
        self.inner.stop()
    }

    fn write(&mut self, packet: AudioPacket, converter: &mut Converter) -> SinkResult<()> {
        if self.shared.enabled.load(Ordering::Relaxed) {
            if let Ok(samples) = packet.samples() {
                let mut ring = self.shared.ring.lock().unwrap();
                for pair in samples.as_chunks::<2>().0 {
                    ring.samples.push_back(((pair[0] + pair[1]) * 0.5) as f32);
                }
                let excess = ring.samples.len().saturating_sub(FFT_SIZE * 2);
                ring.samples.drain(..excess);
                ring.last_write = Some(Instant::now());
            }
        }
        self.inner.write(packet, converter)
    }
}

/// Bucle que calcula l'espectre i l'emet com a `viz` (64 valors 0-255).
pub fn spawn_analyser(app: AppHandle, shared: Arc<VizShared>) {
    std::thread::spawn(move || {
        let fft = FftPlanner::<f32>::new().plan_fft_forward(FFT_SIZE);
        let window: Vec<f32> = (0..FFT_SIZE)
            .map(|i| 0.5 - 0.5 * (std::f32::consts::TAU * i as f32 / FFT_SIZE as f32).cos())
            .collect();
        let edges = band_edges();
        let mut buf = vec![Complex::new(0.0f32, 0.0); FFT_SIZE];

        loop {
            std::thread::sleep(FRAME);
            // Sense finestra (mode «només so») o minimitzada: ningú no el veu, no es calcula.
            let hidden = app.get_webview_window("main").is_none() || crate::MINIMIZED.load(Ordering::Relaxed);
            if !shared.enabled.load(Ordering::Relaxed) || hidden {
                std::thread::sleep(Duration::from_millis(200));
                continue;
            }
            {
                let ring = shared.ring.lock().unwrap();
                let fresh = ring
                    .last_write
                    .is_some_and(|t| t.elapsed() < Duration::from_millis(250));
                if !fresh || ring.samples.len() < FFT_SIZE {
                    continue;
                }
                // Les mostres més noves són les del final, però la sortida en té encara
                // unes quantes a la cua: agafant la finestra una mica enrere queda més sincronitzat.
                let end = ring.samples.len().saturating_sub(FFT_SIZE / 2).max(FFT_SIZE);
                let start = end - FFT_SIZE;
                for (i, s) in ring.samples.range(start..end).enumerate() {
                    buf[i] = Complex::new(s * window[i], 0.0);
                }
            }
            fft.process(&mut buf);

            let bands: Vec<u8> = edges
                .windows(2)
                .map(|w| {
                    let (a, b) = (w[0], w[1].max(w[0] + 1));
                    let peak = buf[a..b].iter().map(|c| c.norm()).fold(0.0f32, f32::max);
                    // dB normalitzat: -60 dB → 0, 0 dB → 1.
                    let db = 20.0 * (peak / (FFT_SIZE as f32 / 4.0)).max(1e-6).log10();
                    (((db + 60.0) / 60.0).clamp(0.0, 1.0) * 255.0) as u8
                })
                .collect();
            let _ = app.emit("viz", bands);
        }
    });
}

/// Límits de les bandes en índexs de l'FFT, en escala logarítmica de 30 Hz a 16 kHz.
fn band_edges() -> Vec<usize> {
    let (lo, hi) = (30.0f32, 16_000.0f32);
    let hz_per_bin = SAMPLE_RATE / FFT_SIZE as f32;
    (0..=BANDS)
        .map(|i| {
            let f = lo * (hi / lo).powf(i as f32 / BANDS as f32);
            ((f / hz_per_bin) as usize).clamp(1, FFT_SIZE / 2)
        })
        .collect()
}
