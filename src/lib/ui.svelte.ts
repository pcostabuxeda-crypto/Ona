import { flushSync } from "svelte";
import type { Collection } from "./api";

export type MenuItem = { label: string; icon: string; run: () => void };

export type View =
  | { name: "home" }
  | { name: "search" }
  | { name: "library" }
  | { name: "settings" }
  | { name: "collection"; kind: Collection["kind"]; id: string };

class Ui {
  stack = $state<View[]>([{ name: "home" }]);
  view = $derived(this.stack[this.stack.length - 1]);
  nowPlaying = $state(false);
  /** La pantalla de lletra hi registra la seva animació de sortida. */
  npCloser: (() => void) | null = null;
  queueOpen = $state(false);
  toast = $state<string | null>(null);
  /** Menú contextual obert (clic dret). */
  menu = $state<{ x: number; y: number; items: MenuItem[] } | null>(null);
  user = $state<string | null>(null);
  private toastTimer: ReturnType<typeof setTimeout> | undefined;

  go(v: View) {
    const top = this.view;
    if (JSON.stringify(top) === JSON.stringify(v)) return;
    transition(() => {
      // Les seccions principals no s'apilen: comencen un historial nou.
      this.stack = v.name === "collection" ? [...this.stack, v] : [v];
      this.nowPlaying = false;
    });
  }

  back() {
    if (this.stack.length < 2) return;
    transition(() => (this.stack = this.stack.slice(0, -1)));
  }

  setNowPlaying(open: boolean) {
    // Té les seves pròpies animacions: en obrir la portada vola i fa dues voltes; en tancar, a l'inrevés.
    if (!open && this.nowPlaying && this.npCloser) return this.npCloser();
    this.nowPlaying = open;
    if (open) this.queueOpen = false;
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

/** Canvi de vista amb View Transitions (la portada "vola" d'un lloc a l'altre). */
export function transition(fn: () => void) {
  const doc = document as Document & { startViewTransition?: (cb: () => void) => unknown };
  if (doc.startViewTransition && !matchMedia("(prefers-reduced-motion: reduce)").matches) {
    doc.startViewTransition(() => flushSync(fn));
  } else {
    fn();
  }
}

export const ui = new Ui();
