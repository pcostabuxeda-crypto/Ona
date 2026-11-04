/** Preferències desades al navegador de l'app (localStorage). */
const KEY = "ona.settings";

export type Look = "spotify" | "liner" | "ona";
export type ThemeId = "ona" | "galaxy" | "aurora" | "sunset" | "ocean" | "cover";
export type Effect = "none" | "stars" | "bubbles";

type Settings = {
  /** Mode lleuger: lletra simple per línies i fons de color pla (sense WebGL ni animació per síl·laba). */
  light: boolean;
  /** Estètica: la del client de Spotify, la tipus Liner o la pròpia d'Ona (temes, disc, ona). */
  look: Look;
  /** Estètica Ona: fons, efecte animat del fons, disc de vinil i visualitzador. */
  theme: ThemeId;
  effect: Effect;
  disc: boolean;
  viz: boolean;
};

const THEMES: ThemeId[] = ["ona", "galaxy", "aurora", "sunset", "ocean", "cover"];
const EFFECTS: Effect[] = ["none", "stars", "bubbles"];

function load(): Settings {
  let s: Record<string, unknown> = {};
  try {
    s = JSON.parse(localStorage.getItem(KEY) ?? "{}");
  } catch {}
  // `liner: true` és de la versió 0.4–0.6, abans que hi hagués tres estètiques.
  const look = (["spotify", "liner", "ona"] as Look[]).find((l) => l === s.look) ?? (s.liner === true ? "liner" : "spotify");
  return {
    light: s.light === true,
    look,
    theme: THEMES.find((t) => t === s.theme) ?? "ona",
    effect: EFFECTS.find((e) => e === s.effect) ?? "stars",
    disc: s.disc !== false,
    viz: s.viz !== false,
  };
}

export const settings = $state<Settings>(load());

export function setSetting<K extends keyof Settings>(key: K, value: Settings[K]) {
  settings[key] = value;
  try {
    localStorage.setItem(KEY, JSON.stringify(settings));
  } catch {}
}
