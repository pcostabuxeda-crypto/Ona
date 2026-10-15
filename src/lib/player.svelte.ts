import { listen } from "@tauri-apps/api/event";
import * as A from "./api";
import type { Track } from "./api";
import { DEFAULT_PALETTE, paletteFrom, type Palette } from "./color";

type Repeat = "off" | "all" | "one";

const SESSION_KEY = "ona.session";
const MAX_SAVED = 3000;

type Saved = {
  queue: Track[];
  order: number[];
  pos: number;
  context: string;
  shuffle: boolean;
  repeat: Repeat;
  positionMs: number;
};

class PlayerState {
  queue = $state<Track[]>([]);
  /** Ordre de reproducció (índexs de `queue`); canvia amb l'aleatori. */
  order = $state<number[]>([]);
  pos = $state(-1);
  context = $state("");

  playing = $state(false);
  loading = $state(false);
  shuffle = $state(false);
  repeat = $state<Repeat>("off");
  volume = $state(0.7);
  palette = $state<Palette>(DEFAULT_PALETTE);

  /** Posició coneguda i quan la vam saber; la posició actual s'interpola. */
  private baseMs = 0;
  private baseAt = 0;
  /** Rellotge que avança 4 cops per segon només mentre sona. */
  now = $state(0);
  private ticker: ReturnType<typeof setInterval> | null = null;
  private preloaded = "";
  /** Cançó que el reproductor de Rust té carregada ("" si cap: per exemple, després de reiniciar l'app). */
  private inBackend = "";
  /** Motor per a les pistes amb `src` (Audius, fitxers locals); Spotify va pel reproductor de Rust. */
  private audioEl: HTMLAudioElement | null = null;

  private get audio() {
    if (this.audioEl) return this.audioEl;
    const a = new Audio();
    a.preload = "auto";
    a.volume = this.volume;
    const mine = () => !!this.current?.src && a.src === this.current.src;
    a.addEventListener("waiting", () => mine() && (this.loading = true));
    // Els fitxers locals no saben la durada fins que es carreguen.
    a.addEventListener("loadedmetadata", () => {
      if (mine() && this.current && !this.current.duration_ms && isFinite(a.duration)) this.current.duration_ms = Math.round(a.duration * 1000);
    });
    a.addEventListener("playing", () => {
      if (!mine()) return;
      this.loading = false;
      this.setBase(a.currentTime * 1000, true);
    });
    a.addEventListener("pause", () => mine() && !a.ended && this.setBase(a.currentTime * 1000, false));
    a.addEventListener("seeked", () => mine() && this.setBase(a.currentTime * 1000, !a.paused));
    a.addEventListener("ended", () => mine() && this.advance(true));
    a.addEventListener("error", () => mine() && this.advance(false));
    this.audioEl = a;
    return a;
  }

  /** Carrega i reprodueix una pista web a `ms`. */
  private playWeb(src: string, ms: number) {
    if (this.inBackend) {
      A.pause();
      this.inBackend = "";
    }
    const a = this.audio;
    if (a.src !== src) a.src = src;
    a.currentTime = ms / 1000;
    a.play().catch(() => this.advance(false));
  }

  current = $derived(this.pos >= 0 ? this.queue[this.order[this.pos]] : undefined);
  positionMs = $derived.by(() => {
    void this.now;
    if (!this.current) return 0;
    const p = this.playing ? this.baseMs + (performance.now() - this.baseAt) : this.baseMs;
    return Math.min(p, this.current.duration_ms);
  });
  progress = $derived(this.current ? this.positionMs / this.current.duration_ms : 0);

  /** Posició exacta ara mateix, sense esperar el rellotge de 4 Hz (per a animacions per fotograma). */
  liveMs() {
    if (!this.current) return 0;
    const p = this.playing ? this.baseMs + (performance.now() - this.baseAt) : this.baseMs;
    return Math.min(p, this.current.duration_ms);
  }

  async init() {
    try {
      const v = Number(localStorage.getItem("ona.volume"));
      if (v >= 0 && v <= 1 && localStorage.getItem("ona.volume") !== null) this.volume = v;
    } catch {}
    A.setVolume(this.volume);

    await listen<{ kind: string; uri: string; position_ms?: number }>("player", ({ payload: e }) => {
      if (this.current && e.uri !== this.current.uri && e.kind !== "preload") return;
      switch (e.kind) {
        case "loading":
          this.loading = true;
          break;
        case "playing":
          this.loading = false;
          this.setBase(e.position_ms ?? 0, true);
          break;
        case "paused":
          this.loading = false;
          this.setBase(e.position_ms ?? 0, false);
          break;
        case "preload":
          this.preloadNext();
          break;
        case "ended":
          this.advance(true);
          break;
        case "unavailable":
          this.advance(false);
          break;
        case "stopped":
          this.setBase(0, false);
          break;
      }
    });

    // Botons del sistema: tecles multimèdia, caràtula de Windows, FluentFlyout...
    await listen<{ action: string; ms?: number }>("media", ({ payload: m }) => {
      if (m.action === "play" && !this.playing) this.toggle();
      else if (m.action === "pause" && this.playing) this.toggle();
      else if (m.action === "toggle") this.toggle();
      else if (m.action === "next") this.next();
      else if (m.action === "prev") this.prev();
      else if (m.action === "seek" && m.ms != null) this.seekTo(m.ms);
      else if (m.action === "seekBy" && m.ms != null) this.seekTo(Math.max(0, this.positionMs + m.ms));
    });
    // La cançó que es mostra al sistema segueix la que hi ha a la interfície.
    $effect.root(() => {
      $effect(() => {
        A.mediaUpdate(this.current);
      });
    });

    await this.restoreSession();
    // Mini memòria: es desa cada pocs segons mentre sona, i en tancar la finestra.
    setInterval(() => this.playing && this.save(), 5000);
    addEventListener("beforeunload", () => this.save());
  }

  /** Desa la cua i la posició per poder-les recuperar si es recarrega o es reinicia l'app. */
  save() {
    if (!this.current) return;
    let { queue, order, pos } = this;
    if (queue.length > MAX_SAVED) {
      // Es guarda un tros de la cua al voltant de la cançó actual.
      const from = Math.max(0, pos - 50);
      const idx = order.slice(from, from + MAX_SAVED);
      queue = idx.map((i) => this.queue[i]);
      order = idx.map((_, i) => i);
      pos = pos - from;
    }
    const data: Saved = {
      queue: $state.snapshot(queue) as Track[],
      order: [...order],
      pos,
      context: this.context,
      shuffle: this.shuffle,
      repeat: this.repeat,
      positionMs: Math.round(this.positionMs),
    };
    try {
      localStorage.setItem(SESSION_KEY, JSON.stringify(data));
    } catch {}
  }

  /** En arrencar: recupera la cua desada i, si Rust encara està sonant, la posició real. */
  private async restoreSession() {
    let saved: Saved | null = null;
    try {
      saved = JSON.parse(localStorage.getItem(SESSION_KEY) ?? "null");
    } catch {}
    if (saved?.queue?.length && saved.order?.length && saved.pos >= 0 && saved.pos < saved.order.length) {
      this.queue = saved.queue;
      this.order = saved.order;
      this.pos = saved.pos;
      this.context = saved.context ?? "";
      this.shuffle = !!saved.shuffle;
      this.repeat = saved.repeat ?? "off";
    }

    const live = await A.playerState().catch(() => null);
    if (live && live.uri !== this.current?.uri) {
      // Sona una cançó que no és a la cua desada: se'n busquen les dades.
      const t = await A.trackByUri(live.uri).catch(() => null);
      if (t) {
        this.queue = [t];
        this.order = [0];
        this.pos = 0;
        this.context = "";
      }
    }
    if (live && live.uri === this.current?.uri) {
      this.inBackend = live.uri;
      this.setBase(live.position_ms, live.playing);
    } else if (this.current) {
      // L'app s'ha reiniciat: la cançó queda en pausa on era, i el play la torna a carregar.
      this.setBase(saved?.positionMs ?? 0, false);
    }
    if (this.current) this.refreshPalette();
  }

  private setBase(ms: number, playing: boolean) {
    if (this.current) A.mediaState(playing, ms);
    this.baseMs = ms;
    this.baseAt = performance.now();
    this.playing = playing;
    this.now = this.baseAt;
    if (playing && !this.ticker) {
      // Amb la finestra amagada no cal moure la barra de temps.
      this.ticker = setInterval(() => !document.hidden && (this.now = performance.now()), 250);
    } else if (!playing && this.ticker) {
      clearInterval(this.ticker);
      this.ticker = null;
    }
  }

  /** Reprodueix `tracks` començant per `start`. */
  playList(tracks: Track[], start = 0, context = "") {
    this.queue = tracks;
    this.context = context;
    this.order = tracks.map((_, i) => i);
    if (this.shuffle) {
      this.order = shuffled(this.order, start);
      this.pos = 0;
    } else {
      this.pos = start;
    }
    this.load();
  }

  /** Afegeix una cançó just després de l'actual. */
  playNext(t: Track) {
    if (this.pos < 0) return this.playList([t]);
    this.queue = [...this.queue, t];
    const order = [...this.order];
    order.splice(this.pos + 1, 0, this.queue.length - 1);
    this.order = order;
    this.preloaded = "";
    this.save();
  }

  /** Afegeix al final de la cua. */
  addToQueue(t: Track) {
    if (this.pos < 0) return this.playList([t]);
    this.queue = [...this.queue, t];
    this.order = [...this.order, this.queue.length - 1];
    this.preloaded = "";
    this.save();
  }

  /** Les cançons que vénen (fins a 100), amb la seva posició dins de `order`. */
  upcoming = $derived(
    this.order.slice(this.pos + 1, this.pos + 101).map((qi, k) => ({ track: this.queue[qi], key: qi, at: this.pos + 1 + k })),
  );
  upcomingTotal = $derived(Math.max(0, this.order.length - this.pos - 1));

  /** Treu de la cua la cançó que és a la posició `at` de l'ordre. */
  removeAt(at: number) {
    if (at <= this.pos || at >= this.order.length) return;
    const order = [...this.order];
    order.splice(at, 1);
    this.order = order;
    this.preloaded = "";
    this.save();
  }

  /** Mou una cançó de la cua d'una posició a una altra (totes dues per davant de l'actual). */
  move(from: number, to: number) {
    if (from <= this.pos || to <= this.pos || from === to) return;
    const order = [...this.order];
    const [x] = order.splice(from, 1);
    order.splice(Math.min(to, order.length), 0, x);
    this.order = order;
    this.preloaded = "";
    this.save();
  }

  clearUpcoming() {
    this.order = this.order.slice(0, this.pos + 1);
    this.preloaded = "";
    this.save();
  }

  private load() {
    const t = this.current;
    if (!t) return;
    this.loading = true;
    this.preloaded = "";
    this.setBase(0, false);
    if (t.src) {
      this.playWeb(t.src, 0);
    } else {
      this.audioEl?.pause();
      this.inBackend = t.uri;
      A.playUri(t.uri);
    }
    this.refreshPalette();
    this.save();
  }

  private async refreshPalette() {
    const t = this.current;
    const p = await paletteFrom(t?.album.thumb ?? t?.album.image);
    if (t === this.current) this.palette = p;
  }

  private nextPos(): number {
    if (this.repeat === "one") return this.pos;
    if (this.pos + 1 < this.order.length) return this.pos + 1;
    return this.repeat === "all" ? 0 : -1;
  }

  private preloadNext() {
    const n = this.nextPos();
    const t = n >= 0 ? this.queue[this.order[n]] : undefined;
    if (t && !t.src && t.uri !== this.preloaded) {
      this.preloaded = t.uri;
      A.preloadUri(t.uri);
    }
  }

  /** Final de cançó (natural o per error). */
  private advance(natural: boolean) {
    const n = natural ? this.nextPos() : this.pos + 1 < this.order.length ? this.pos + 1 : -1;
    if (n < 0) {
      this.setBase(0, false);
      return;
    }
    this.pos = n;
    this.load();
  }

  toggle() {
    if (!this.current) return;
    if (this.current.src) {
      if (this.playing) this.audio.pause();
      else if (this.audio.src === this.current.src) this.audio.play().catch(() => {});
      else this.playWeb(this.current.src, this.positionMs);
    } else if (this.playing) {
      this.setBase(this.positionMs, false);
      A.pause();
    } else if (this.inBackend !== this.current.uri) {
      // Després de reiniciar: es carrega la cançó on s'havia quedat.
      this.loading = true;
      this.inBackend = this.current.uri;
      A.playUri(this.current.uri, Math.round(this.positionMs));
    } else {
      A.resume();
    }
    this.save();
  }

  next() {
    if (this.pos + 1 < this.order.length) this.pos++;
    else if (this.repeat !== "off") this.pos = 0;
    else return;
    this.load();
  }

  prev() {
    if (this.positionMs > 3000 || this.pos <= 0) return this.seekTo(0);
    this.pos--;
    this.load();
  }

  /** Salta a la posició `at` de l'ordre. */
  jumpTo(at: number) {
    if (at <= this.pos || at >= this.order.length) return;
    this.pos = at;
    this.load();
  }

  seekTo(ms: number) {
    if (!this.current) return;
    this.setBase(ms, this.playing);
    if (this.current.src) {
      if (this.audioEl?.src === this.current.src) this.audioEl.currentTime = ms / 1000;
    } else if (this.inBackend === this.current.uri) A.seek(ms);
    this.save();
  }

  setVolume(v: number) {
    this.volume = Math.min(1, Math.max(0, v));
    A.setVolume(this.volume);
    if (this.audioEl) this.audioEl.volume = this.volume;
    try {
      localStorage.setItem("ona.volume", String(this.volume));
    } catch {}
  }

  toggleShuffle() {
    this.shuffle = !this.shuffle;
    if (this.pos < 0) return;
    const cur = this.order[this.pos];
    if (this.shuffle) {
      this.order = shuffled(this.queue.map((_, i) => i), cur);
      this.pos = 0;
    } else {
      this.order = this.queue.map((_, i) => i);
      this.pos = cur;
    }
    this.preloaded = "";
  }

  cycleRepeat() {
    this.repeat = this.repeat === "off" ? "all" : this.repeat === "all" ? "one" : "off";
    this.preloaded = "";
  }
}

/** Barreja deixant `first` al davant. */
function shuffled(list: number[], first: number) {
  const rest = list.filter((i) => i !== first);
  for (let i = rest.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [rest[i], rest[j]] = [rest[j], rest[i]];
  }
  return [first, ...rest];
}

export const player = new PlayerState();
