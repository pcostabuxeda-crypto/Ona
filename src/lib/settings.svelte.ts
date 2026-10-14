/** Preferències visuals, desades al navegador de l'app (localStorage). */
export type VizStyle = "normal" | "circular";
export type VizShape = "bars" | "wave";
import type { Effect, ThemeId } from "./themes";

const KEY = "ona.settings";

const DEFAULTS = {
  vizEnabled: true,
  vizStyle: "circular" as VizStyle,
  vizShape: "bars" as VizShape,
  /** 0.5 – 2 */
  vizSensitivity: 1.2,
  liveBg: true,
  /** 0 – 1 (desenfocament de la portada de fons) */
  bgBlur: 0.7,
  /** 0 – 1 (quant reacciona el fons al ritme) */
  bgReactivity: 0.6,
  theme: "ona" as ThemeId,
  effect: "none" as Effect,
  /** 0.15 – 0.9: opacitat dels panells quan hi ha un tema amb fons. */
  panelAlpha: 0.45,
  /** 0 – 0.8: enfosquiment del fons del tema. */
  bgDim: 0.2,
  /** Mode estalvi: sense visualitzador, fons viu, efectes ni ona a la barra. */
  eco: false,
};

type Settings = typeof DEFAULTS;

function load(): Settings {
  try {
    return { ...DEFAULTS, ...JSON.parse(localStorage.getItem(KEY) ?? "{}") };
  } catch {
    return { ...DEFAULTS };
  }
}

export const settings = $state<Settings>(load());

/** Desa automàticament quan canvia qualsevol preferència. */
export function persistSettings() {
  $effect.root(() => {
    $effect(() => {
      const snapshot = JSON.stringify(settings);
      try {
        localStorage.setItem(KEY, snapshot);
      } catch {}
    });
  });
}
