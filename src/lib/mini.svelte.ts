/**
 * Lletra flotant: una segona finestra petita, sempre a sobre (P1-7). La finestra principal hi envia
 * l'estat (pista, posició i quan, si sona) només quan canvia; la finestra flotant interpola sola.
 */
import { emitTo, listen } from "@tauri-apps/api/event";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { player } from "./player.svelte";
import { settings, type Look } from "./settings.svelte";
import { ui } from "./ui.svelte";

export const MINI = "lyrics-mini";
const GEOM_KEY = "ona.mini";

export const mini = $state({ open: false });
let win: WebviewWindow | null = null;
let wired = false;

export type MiniState = {
  track: { id: string; name: string; artists: string; album: string; duration_ms: number; image?: string } | null;
  positionMs: number;
  at: number;
  playing: boolean;
  color: string;
  /** Segon color de la portada (fons animat de l'estètica Ona). */
  color2: string;
  light: boolean;
  look: Look;
};

function state(): MiniState {
  const t = player.current;
  return {
    track: t
      ? { id: t.id, name: t.name, artists: t.artists.map((a) => a.name).join(", "), album: t.album.name, duration_ms: t.duration_ms, image: t.album.thumb }
      : null,
    positionMs: player.liveMs(),
    at: Date.now(),
    playing: player.playing,
    color: player.palette.a,
    color2: player.palette.b,
    light: settings.light,
    look: settings.look,
  };
}

/** Un cop: envia l'estat quan canvia, i atén els botons de la finestra flotant. */
function wire() {
  if (wired) return;
  wired = true;
  $effect.root(() => {
    $effect(() => {
      void player.current;
      void player.epoch;
      void player.palette;
      void settings.light;
      void settings.look;
      if (mini.open) emitTo(MINI, "mini:state", state()).catch(() => {});
    });
  });
  listen("mini:ready", () => emitTo(MINI, "mini:state", state()).catch(() => {}));
  listen<string>("mini:cmd", ({ payload }) => {
    if (payload === "prev") player.prev();
    else if (payload === "toggle") player.toggle();
    else if (payload === "next") player.next();
  });
}

export async function toggleMini() {
  wire();
  if (win) {
    await win.close().catch(() => {});
    return;
  }
  let g: { x?: number; y?: number; w?: number; h?: number } = {};
  try {
    g = JSON.parse(localStorage.getItem(GEOM_KEY) ?? "{}");
  } catch {}
  win = new WebviewWindow(MINI, {
    url: "/?mini=1",
    title: "Ona · lletra",
    width: g.w ?? 380,
    height: g.h ?? 120,
    minWidth: 280,
    minHeight: 90,
    maxWidth: 640,
    maxHeight: 240,
    ...(g.x != null && g.y != null ? { x: g.x, y: g.y } : {}),
    decorations: false,
    alwaysOnTop: true,
    skipTaskbar: true,
    resizable: true,
    shadow: true,
    focus: false,
    backgroundColor: "#121212",
  });
  win.once("tauri://created", () => (mini.open = true));
  win.once("tauri://error", () => {
    win = null;
    mini.open = false;
    ui.say("No s'ha pogut obrir la lletra flotant.");
  });
  win.once("tauri://destroyed", () => {
    win = null;
    mini.open = false;
  });
}
