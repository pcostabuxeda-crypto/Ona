import { listen } from "@tauri-apps/api/event";
import * as A from "./api";
import type { Track } from "./api";
import { DEFAULT_PALETTE, paletteFrom, type Palette } from "./color";
import { nextStep, shuffleRest, unshuffleRest } from "./queue";
import { ui } from "./ui.svelte";

type Repeat = "off" | "all" | "one";

const SESSION_KEY = "ona.session";
/** La posició va a part: es desa sovint i és petita (la cua només quan canvia). */
const POS_KEY = "ona.pos";
const MAX_SAVED = 3000;
const BAK_KEY = "ona.session.bak";

function read(key: string) {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}
function write(key: string, value: string) {
  try {
    localStorage.setItem(key, value);
  } catch {}
}
/** Una sessió desada només val si es pot llegir i és coherent. */
function valid(raw: string | null): Saved | null {
  try {
    const s = JSON.parse(raw ?? "null") as Saved | null;
    return s?.queue?.length && s.order?.length && s.pos >= 0 && s.pos < s.order.length ? s : null;
  } catch {
    return null;
  }
}

type Saved = {
  queue: Track[];
  order: number[];
  pos: number;
  context: string;
  shuffle: boolean;
  repeat: Repeat;
  upNext?: Track[];
  fromUser?: Track | null;
  /** Sessions antigues (abans de POS_KEY). */
  positionMs?: number;
};

class PlayerState {
  /** Context: la playlist, àlbum... que sona. */
  queue = $state<Track[]>([]);
  /** Ordre de reproducció (índexs de `queue`); canvia amb l'aleatori. */
  order = $state<number[]>([]);
  pos = $state(-1);
  context = $state("");
  /** Cua de l'usuari: sona abans que el context, en ordre d'arribada (com Spotify). */
  upNext = $state<Track[]>([]);
  /** La pista que sona si ve de la cua de l'usuari (el context es queda on era). */
  fromUser = $state<Track | null>(null);
  /** El que sona perquè ho ha posat el mòbil (Spotify Connect); té prioritat sobre la cua d'Ona. */
  remote = $state<Track | null>(null);
  private localMs = 0;

  playing = $state(false);
  loading = $state(false);
  /** Darrer error de reproducció (es mostra a la barra fins que sona alguna cosa). */
  error = $state<string | null>(null);
  shuffle = $state(false);
  repeat = $state<Repeat>("off");
  volume = $state(0.7);
  palette = $state<Palette>(DEFAULT_PALETTE);

  /** Posició coneguda i quan la vam saber; la posició actual s'interpola. */
  private baseMs = 0;
  private baseAt = 0;
  /** Rellotge que avança 4 cops per segon només mentre sona. */
  now = $state(0);
  /** Canvia quan canvia la base del temps (play, pausa, salt): la lletra flotant s'hi sincronitza. */
  epoch = $state(0);
  private ticker: ReturnType<typeof setInterval> | null = null;
  private preloaded = "";
  /** Cançó que el reproductor de Rust té carregada ("" si cap: per exemple, després de reiniciar l'app). */
  private inBackend = "";
  /** Motor per a les pistes amb `src` (Audius, fitxers locals); Spotify va pel reproductor de Rust. */
  private audioEl: HTMLAudioElement | null = null;
  /** La cua ha canviat i s'ha de tornar a desar. */
  private dirty = false;
  private inited = false;
  private warnedSave = false;

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
      this.error = null;
      this.setBase(a.currentTime * 1000, true);
    });
    a.addEventListener("pause", () => mine() && !a.ended && this.setBase(a.currentTime * 1000, false));
    a.addEventListener("seeked", () => mine() && this.setBase(a.currentTime * 1000, !a.paused));
    a.addEventListener("ended", () => mine() && this.advance(true));
    a.addEventListener("error", () => mine() && this.skipUnavailable());
    this.audioEl = a;
    return a;
  }

  /** Carrega i reprodueix una pista web a `ms`. */
  private playWeb(src: string, ms: number) {
    if (this.inBackend) {
      this.call(A.pause());
      this.inBackend = "";
    }
    const a = this.audio;
    if (a.src !== src) a.src = src;
    a.currentTime = ms / 1000;
    a.play().catch((e) => this.playFailed(e));
  }

  /** play() també falla amb AbortError quan un altre src l'interromp (saltar ràpid): això no és un error. */
  private playFailed(e: unknown) {
    if ((e as Error)?.name !== "AbortError") this.skipUnavailable();
  }

  /** Les ordres al reproductor de Rust: si fallen, l'usuari ho ha de veure (abans es perdien). */
  private call(p: Promise<unknown>) {
    p.catch((e) => {
      this.loading = false;
      this.setBase(this.positionMs, false);
      this.error = String(e).includes("not_logged_in")
        ? "Cal tornar a iniciar sessió amb Spotify."
        : "No s'ha pogut reproduir.";
    });
  }

  current = $derived(this.remote ?? this.fromUser ?? (this.pos >= 0 ? this.queue[this.order[this.pos]] : undefined));
  positionMs = $derived.by(() => {
    void this.now;
    return this.liveMs();
  });
  progress = $derived(this.current?.duration_ms ? this.positionMs / this.current.duration_ms : 0);

  /** Posició exacta ara mateix, sense esperar el rellotge de 4 Hz (per a animacions per fotograma). */
  liveMs() {
    if (!this.current) return 0;
    const p = this.playing ? this.baseMs + (performance.now() - this.baseAt) : this.baseMs;
    return this.current.duration_ms ? Math.min(p, this.current.duration_ms) : p;
  }

  async init() {
    // Un sol cop: si es tornés a cridar, els esdeveniments es gestionarien dues vegades (salts dobles).
    if (this.inited) return;
    this.inited = true;
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
          this.error = null;
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
          this.skipUnavailable();
          break;
        case "stopped":
          this.setBase(0, false);
          break;
      }
    });

    // Spotify Connect: el mòbil fa sonar Ona. Rust ja ha posat en pausa el reproductor local.
    await listen<{ kind: string; uri?: string; position_ms?: number }>("connect", async ({ payload: e }) => {
      if (e.kind === "inactive") return this.leaveRemote();
      if (!e.uri) return;
      if (this.remote?.uri !== e.uri) {
        const t = await A.trackByUri(e.uri).catch(() => null);
        if (!t) return;
        if (!this.remote) {
          this.localMs = this.positionMs;
          this.audioEl?.pause();
        }
        this.remote = t;
        this.refreshPalette();
      }
      this.loading = e.kind === "loading";
      if (e.kind !== "loading") this.setBase(e.position_ms ?? 0, e.kind === "playing");
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
    // Mini memòria: la posició cada pocs segons mentre sona; la cua, només si ha canviat.
    setInterval(() => this.playing && this.save(), 5000);
    addEventListener("beforeunload", () => this.save());
  }

  /** Marca la cua com a canviada i la desa (la còpia gran només es fa quan cal). */
  private touch() {
    this.dirty = true;
    this.preloaded = "";
    this.save();
  }

  /** Desa la posició (petita) sempre, i la cua (gran) només si ha canviat des de l'últim cop. */
  save() {
    if (!this.current || this.remote) return;
    try {
      localStorage.setItem(POS_KEY, JSON.stringify({ uri: this.current.uri, positionMs: Math.round(this.positionMs) }));
    } catch {}
    if (!this.dirty) return;
    this.dirty = false;
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
      upNext: $state.snapshot(this.upNext) as Track[],
      fromUser: $state.snapshot(this.fromUser) as Track | null,
    };
    try {
      localStorage.setItem(SESSION_KEY, JSON.stringify(data));
    } catch {
      // Abans es perdia en silenci; ara l'usuari ho sap (un cop).
      if (!this.warnedSave) ui.say("No s'ha pogut desar la cua: és massa gran per a la memòria de l'app.");
      this.warnedSave = true;
    }
  }

  /** En arrencar: recupera la cua desada i, si Rust encara està sonant, la posició real. */
  private async restoreSession() {
    let savedPos: { uri: string; positionMs: number } | null = null;
    try {
      savedPos = JSON.parse(localStorage.getItem(POS_KEY) ?? "null");
    } catch {}
    // La cua és de l'usuari: si la sessió està malmesa es recupera de la còpia (queixa d'Apple Music:
    // playlists i baixades que desapareixen). Una sessió bona refà la còpia.
    const raw = read(SESSION_KEY);
    let saved = valid(raw);
    if (saved) write(BAK_KEY, raw!);
    else if ((saved = valid(read(BAK_KEY)))) this.dirty = true; // reescriu la malmesa al proper desat
    if (saved) {
      this.queue = saved.queue;
      this.order = saved.order;
      this.pos = saved.pos;
      this.context = saved.context ?? "";
      this.shuffle = !!saved.shuffle;
      this.repeat = saved.repeat ?? "off";
      this.upNext = saved.upNext ?? [];
      this.fromUser = saved.fromUser ?? null;
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
        this.fromUser = null;
      }
    }
    if (live && live.uri === this.current?.uri) {
      this.inBackend = live.uri;
      this.setBase(live.position_ms, live.playing);
    } else if (this.current) {
      // L'app s'ha reiniciat: la cançó queda en pausa on era, i el play la torna a carregar.
      const ms = savedPos?.uri === this.current.uri ? savedPos.positionMs : (saved?.positionMs ?? 0);
      this.setBase(ms, false);
    }
    if (this.current) this.refreshPalette();
  }

  /** En tancar la sessió de Spotify: la cua de Spotify ja no es pot reproduir. */
  forget() {
    if (this.playing) this.toggle();
    this.queue = [];
    this.order = [];
    this.pos = -1;
    this.upNext = [];
    this.fromUser = null;
    this.context = "";
    this.error = null;
    this.inBackend = "";
    try {
      localStorage.removeItem(SESSION_KEY);
      localStorage.removeItem(BAK_KEY);
      localStorage.removeItem(POS_KEY);
    } catch {}
  }

  private setBase(ms: number, playing: boolean) {
    if (this.current) A.mediaState(playing, ms);
    this.epoch++;
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

  /** Reprodueix `tracks` començant per `start`. La cua de l'usuari es conserva (com Spotify). */
  playList(tracks: Track[], start = 0, context = "") {
    this.queue = tracks;
    this.context = context;
    this.order = tracks.map((_, i) => i);
    this.fromUser = null;
    if (this.shuffle) {
      this.order = shuffled(this.order, start);
      this.pos = 0;
    } else {
      this.pos = start;
    }
    this.dirty = true;
    this.load();
  }

  /** Al davant de la cua de l'usuari: sona just després de l'actual. */
  playNext(t: Track) {
    if (!this.current) return this.playList([t]);
    this.upNext = [{ ...t }, ...this.upNext];
    this.touch();
  }

  /** Al final de la cua de l'usuari (però abans que la resta del context). */
  addToQueue(t: Track) {
    if (!this.current) return this.playList([t]);
    this.upNext = [...this.upNext, { ...t }];
    this.touch();
  }

  /** El que ve del context (fins a 100), amb la seva posició dins de `order`. */
  upcoming = $derived(
    this.order.slice(this.pos + 1, this.pos + 101).map((qi, k) => ({ track: this.queue[qi], key: qi, at: this.pos + 1 + k })),
  );
  upcomingTotal = $derived(Math.max(0, this.order.length - this.pos - 1));

  /** Treu del context la cançó que és a la posició `at` de l'ordre. */
  removeAt(at: number) {
    if (at <= this.pos || at >= this.order.length) return;
    const order = [...this.order];
    order.splice(at, 1);
    this.order = order;
    this.touch();
  }

  /** Mou una cançó del context d'una posició a una altra (totes dues per davant de l'actual). */
  move(from: number, to: number) {
    if (from <= this.pos || to <= this.pos || from === to) return;
    const order = [...this.order];
    const [x] = order.splice(from, 1);
    order.splice(Math.min(to, order.length), 0, x);
    this.order = order;
    this.touch();
  }

  /** Mou un element de la cua de l'usuari. */
  moveUser(from: number, to: number) {
    if (from === to || from < 0 || from >= this.upNext.length) return;
    const list = [...this.upNext];
    const [x] = list.splice(from, 1);
    list.splice(Math.max(0, Math.min(to, list.length)), 0, x);
    this.upNext = list;
    this.touch();
  }

  removeUser(i: number) {
    this.upNext = this.upNext.filter((_, k) => k !== i);
    this.touch();
  }

  /** Buida la cua de l'usuari; retorna el que hi havia per poder-ho desfer. */
  clearUser(): Track[] {
    const was = this.upNext;
    this.upNext = [];
    this.touch();
    return was;
  }

  restoreUser(list: Track[]) {
    this.upNext = [...list, ...this.upNext];
    this.touch();
  }

  clearUpcoming() {
    this.order = this.order.slice(0, this.pos + 1);
    this.touch();
  }

  /** Connect ja no fa sonar res aquí: es torna a la cançó d'Ona, en pausa on s'havia quedat. */
  private async leaveRemote() {
    if (!this.remote) return;
    this.remote = null;
    this.loading = false;
    // El reproductor local diu la veritat (pot ser que ja soni el que s'ha triat a Ona).
    const live = await A.playerState().catch(() => null);
    if (live && live.uri === this.current?.uri) this.setBase(live.position_ms, live.playing);
    else this.setBase(this.localMs, false);
    this.refreshPalette();
  }

  private load() {
    this.remote = null; // reproduir des d'Ona allibera Connect (ho fa Rust a `play`)
    const t = this.current;
    if (!t) return;
    this.loading = true;
    this.error = null;
    this.preloaded = "";
    this.setBase(0, false);
    if (t.src) {
      this.playWeb(t.src, 0);
    } else {
      this.audioEl?.pause();
      this.inBackend = t.uri;
      this.call(A.playUri(t.uri));
    }
    this.refreshPalette();
    this.dirty = true;
    this.save();
  }

  /** Torna a provar la pista que ha fallat. */
  retry() {
    this.load();
  }

  private async refreshPalette() {
    const t = this.current;
    const p = await paletteFrom(t?.album.thumb ?? t?.album.image);
    if (t === this.current) this.palette = p;
  }

  /** La pista que sonarà després de l'actual (per precarregar-la). */
  private nextTrack(): Track | undefined {
    if (this.repeat === "one") return this.current;
    const n = nextStep(this.upNext.length, this.pos, this.order.length, this.repeat === "all");
    return n === "user" ? this.upNext[0] : n >= 0 ? this.queue[this.order[n]] : undefined;
  }

  private preloadNext() {
    const t = this.nextTrack();
    if (t && !t.src && t.uri !== this.preloaded) {
      this.preloaded = t.uri;
      A.preloadUri(t.uri).catch(() => {});
    }
  }

  /** Passa a la següent: primer la cua de l'usuari, després el context. Fals si no n'hi ha cap. */
  private step(wrap: boolean): boolean {
    const n = nextStep(this.upNext.length, this.pos, this.order.length, wrap);
    if (n === "user") {
      this.fromUser = this.upNext[0];
      this.upNext = this.upNext.slice(1);
    } else if (n >= 0) {
      this.fromUser = null;
      this.pos = n;
    } else return false;
    return true;
  }

  /** Final de cançó (natural o per error). */
  private advance(natural: boolean) {
    if (natural && this.repeat === "one") return this.load();
    if (this.step(natural && this.repeat === "all")) return this.load();
    this.setBase(0, false);
  }

  /** La pista no es pot reproduir: s'avisa i se salta. */
  private skipUnavailable() {
    const t = this.current;
    if (t) ui.say(`S'ha saltat «${t.name}»: no està disponible.`);
    this.advance(false);
  }

  toggle() {
    if (this.remote) return void A.connectCmd("toggle");
    if (!this.current) return;
    if (this.current.src) {
      if (this.playing) this.audio.pause();
      else if (this.audio.src === this.current.src) this.audio.play().catch((e) => this.playFailed(e));
      else this.playWeb(this.current.src, this.positionMs);
    } else if (this.playing) {
      this.setBase(this.positionMs, false);
      this.call(A.pause());
    } else if (this.inBackend !== this.current.uri) {
      // Després de reiniciar: es carrega la cançó on s'havia quedat.
      this.loading = true;
      this.error = null;
      this.inBackend = this.current.uri;
      this.call(A.playUri(this.current.uri, Math.round(this.positionMs)));
    } else {
      this.call(A.resume());
    }
    this.save();
  }

  next() {
    if (this.remote) return void A.connectCmd("next");
    if (this.step(this.repeat !== "off")) this.load();
  }

  prev() {
    if (this.remote) return void A.connectCmd("prev");
    if (this.positionMs > 3000) return this.seekTo(0);
    // Des d'una pista de la cua de l'usuari es torna a la del context.
    if (this.fromUser) {
      this.fromUser = null;
      return this.load();
    }
    if (this.pos <= 0) return this.seekTo(0);
    this.pos--;
    this.load();
  }

  /** Salta a la posició `at` de l'ordre del context. */
  jumpTo(at: number) {
    if (at <= this.pos || at >= this.order.length) return;
    this.fromUser = null;
    this.pos = at;
    this.load();
  }

  /** Reprodueix ara l'element `i` de la cua de l'usuari. */
  jumpToUser(i: number) {
    const t = this.upNext[i];
    if (!t) return;
    this.fromUser = t;
    this.upNext = this.upNext.filter((_, k) => k !== i);
    this.load();
  }

  seekTo(ms: number) {
    if (!this.current) return;
    this.setBase(ms, this.playing);
    if (this.remote) return void A.connectCmd("seek", ms);
    if (this.current.src) {
      if (this.audioEl?.src === this.current.src) this.audioEl.currentTime = ms / 1000;
    } else if (this.inBackend === this.current.uri) this.call(A.seek(ms));
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

  /** L'aleatori només barreja el que encara ha de sonar del context; la cua de l'usuari no es toca. */
  toggleShuffle() {
    this.shuffle = !this.shuffle;
    if (this.pos >= 0) this.order = this.shuffle ? shuffleRest(this.order, this.pos) : unshuffleRest(this.order, this.pos);
    this.touch();
  }

  cycleRepeat() {
    this.repeat = this.repeat === "off" ? "all" : this.repeat === "all" ? "one" : "off";
    this.touch();
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
