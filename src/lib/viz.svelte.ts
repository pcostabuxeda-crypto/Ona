/**
 * Dades de l'espectre que envia el Rust (`viz`). No són $state: es llegeixen dins dels bucles
 * de dibuix (canvas) i del fons, així no es dispara cap actualització de Svelte 30 cops per segon.
 */
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export const BANDS = 64;
/** Valors suavitzats 0–1 (pugen ràpid, baixen lent). */
export const bands = new Float32Array(BANDS);
const target = new Float32Array(BANDS);
let lastFrame = 0;

let users = 0;
let unlisten: (() => void) | undefined;

/** Demana l'espectre mentre hi hagi algú que el faci servir. Retorna la funció per deixar-lo. */
export function useSpectrum(): () => void {
  users++;
  if (users === 1) {
    invoke("set_viz", { enabled: true });
    listen<number[]>("viz", ({ payload }) => {
      for (let i = 0; i < BANDS; i++) target[i] = (payload[i] ?? 0) / 255;
      lastFrame = performance.now();
    }).then((u) => {
      if (users === 0) u();
      else unlisten = u;
    });
  }
  return () => {
    users--;
    if (users === 0) {
      invoke("set_viz", { enabled: false });
      unlisten?.();
      unlisten = undefined;
    }
  };
}

/** Avança el suavitzat; cridar un cop per fotograma. Retorna l'energia dels greus (0–1). */
export function stepSpectrum(sensitivity = 1): number {
  // Si fa estona que no arriben dades (pausa), tot baixa a zero.
  const stale = performance.now() - lastFrame > 300;
  for (let i = 0; i < BANDS; i++) {
    const t = stale ? 0 : Math.min(1, Math.pow(target[i], 1.6) * sensitivity);
    bands[i] += (t - bands[i]) * (t > bands[i] ? 0.55 : 0.12);
  }
  let bass = 0;
  for (let i = 2; i < 10; i++) bass += bands[i];
  return bass / 8;
}

// ---------- Bucle de fotogrames compartit ----------
// Un sol requestAnimationFrame per a tothom (visualitzador + fons), i el suavitzat un cop per fotograma.

const subs = new Set<(bass: number) => void>();
let raf = 0;
let sensitivity = 1;

export function setSensitivity(v: number) {
  sensitivity = v;
}

export function onFrame(cb: (bass: number) => void): () => void {
  subs.add(cb);
  if (subs.size === 1) raf = requestAnimationFrame(loop);
  return () => {
    subs.delete(cb);
    if (subs.size === 0) cancelAnimationFrame(raf);
  };
}

/** Limitat a ~30 fotogrames per segon: es veu igual de fluid i gasta la meitat. */
let last = 0;
function loop(now: number) {
  raf = requestAnimationFrame(loop);
  if (now - last < 31) return;
  last = now;
  const bass = stepSpectrum(sensitivity);
  for (const cb of subs) cb(bass);
}
