import type { Collection } from "./api";

export type MenuItem = { label: string; icon: string; run: () => void };

export type View =
  | { name: "home" }
  | { name: "search" }
  | { name: "lyrics" }
  | { name: "collection"; kind: Collection["kind"]; id: string };

/** Panell de la dreta, com al client: vista "Ara sona" o la cua. */
export type Panel = "npv" | "queue" | null;

class Ui {
  stack = $state<View[]>([{ name: "home" }]);
  forward = $state<View[]>([]);
  view = $derived(this.stack[this.stack.length - 1]);
  panel = $state<Panel>("npv");
  /** Text del cercador de la barra de dalt. */
  query = $state("");
  toast = $state<string | null>(null);
  /** Menú contextual obert (clic dret). */
  menu = $state<{ x: number; y: number; items: MenuItem[] } | null>(null);
  user = $state<{ name: string; image?: string } | null>(null);
  /** Sense compte de Spotify: només Audius i fitxers locals. */
  free = $state(false);
  private toastTimer: ReturnType<typeof setTimeout> | undefined;

  go(v: View) {
    if (JSON.stringify(this.view) === JSON.stringify(v)) return;
    this.stack = [...this.stack, v].slice(-50);
    this.forward = [];
  }

  back() {
    if (this.stack.length < 2) return;
    this.forward = [this.view, ...this.forward];
    this.stack = this.stack.slice(0, -1);
  }

  fwd() {
    const [next, ...rest] = this.forward;
    if (!next) return;
    this.stack = [...this.stack, next];
    this.forward = rest;
  }

  togglePanel(p: Exclude<Panel, null>) {
    this.panel = this.panel === p ? null : p;
  }

  toggleLyrics() {
    if (this.view.name === "lyrics") this.back();
    else this.go({ name: "lyrics" });
  }

  openMenu(e: MouseEvent, items: MenuItem[]) {
    e.preventDefault();
    e.stopPropagation();
    this.menu = { x: e.clientX, y: e.clientY, items };
  }

  say(msg: string) {
    this.toast = msg;
    clearTimeout(this.toastTimer);
    this.toastTimer = setTimeout(() => (this.toast = null), 3200);
  }
}

export const ui = new Ui();
