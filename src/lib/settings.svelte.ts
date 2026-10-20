/** Preferències desades al navegador de l'app (localStorage). */
const KEY = "ona.settings";

type Settings = {
  /** Mode lleuger: lletra simple per línies i fons de color pla (sense WebGL ni animació per síl·laba). */
  light: boolean;
  /** Estètica tipus Liner: interfície fosca i neutra, barra d'icones i lletra a pantalla completa. */
  liner: boolean;
};

function load(): Settings {
  try {
    // Només els camps d'ara: la clau la compartia l'Ona 0.1 (temes, visualitzador...).
    const s = JSON.parse(localStorage.getItem(KEY) ?? "{}");
    return { light: s.light === true, liner: s.liner === true };
  } catch {
    return { light: false, liner: false };
  }
}

export const settings = $state<Settings>(load());

export function setSetting<K extends keyof Settings>(key: K, value: Settings[K]) {
  settings[key] = value;
  try {
    localStorage.setItem(KEY, JSON.stringify(settings));
  } catch {}
}
