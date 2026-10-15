/** Preferències desades al navegador de l'app (localStorage). */
const KEY = "ona.settings";

type Settings = {
  /** Mode lleuger: lletra simple per línies i fons de color pla (sense WebGL ni animació per síl·laba). */
  light: boolean;
};

function load(): Settings {
  try {
    // Només els camps d'ara: la clau la compartia l'Ona 0.1 (temes, visualitzador...).
    return { light: JSON.parse(localStorage.getItem(KEY) ?? "{}").light === true };
  } catch {
    return { light: false };
  }
}

export const settings = $state<Settings>(load());

export function setLight(light: boolean) {
  settings.light = light;
  try {
    localStorage.setItem(KEY, JSON.stringify(settings));
  } catch {}
}
