<script lang="ts">
  /**
   * Contingut de la finestra de lletra flotant: la línia actual i la següent. Canvia de línia amb una
   * molla (la següent puja i creix). Sense WebGL ni braccato: dos elements al DOM i un rellotge que
   * només desperta quan toca la línia següent (o cada 150 ms amb síl·labes).
   */
  import { onMount } from "svelte";
  import { emitTo, listen } from "@tauri-apps/api/event";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { detectParser } from "@braccato/parsers";
  import * as A from "$lib/api";
  import type { MiniState } from "$lib/mini.svelte";
  import Icon from "./Icon.svelte";
  import { estimateWords, type Word } from "$lib/karaoke";

  type Line = { t: number; text: string; words?: Word[] };

  const win = getCurrentWindow();
  let st = $state<MiniState | null>(null);
  let lines = $state<Line[] | null>(null); // null: carregant
  let synced = $state(true);
  let now = $state(0); // posició actual (ms) segons el rellotge local
  let scale = $state(1);
  try {
    scale = Number(localStorage.getItem("ona.mini.scale")) || 1;
  } catch {}

  const pos = () => (st ? (st.playing ? st.positionMs + (Date.now() - st.at) : st.positionMs) : 0);

  // Lletra de la pista actual (la mateixa ordre que la vista gran).
  let lastId = "";
  $effect(() => {
    const t = st?.track;
    if (!t || t.id === lastId) return;
    lastId = t.id;
    lines = null;
    A.getLyrics({ id: t.id, uri: "", name: t.name, artists: [{ id: "", name: t.artists.split(", ")[0] ?? "" }], album: { id: "", name: t.album }, duration_ms: t.duration_ms, explicit: false }, !st?.light)
      .then((l) => {
        if (t.id !== lastId) return;
        synced = !!l?.synced;
        if (l?.ttml) {
          try {
            // TTML «per línia» (itunes:timing="Line") porta les paraules buides: abans la línia actual sortia
            // en blanc i només es veia la següent. Sense paraules, s'estimen com a les altres lletres.
            const ps = detectParser(l.ttml)
              .parse(l.ttml, t.duration_ms)
              .filter((x) => x.words.trim());
            lines = ps.map((x, i) => ({
              t: x.startTimeMs,
              text: x.words,
              words: x.parts?.length
                ? x.parts.map((p) => ({ t: p.startTimeMs, text: p.words }))
                : estimateWords(x.words, x.startTimeMs, ps[i + 1]?.startTimeMs),
            }));
            return;
          } catch {}
        }
        const ls = (l?.lines ?? []).filter((x) => x.text.trim());
        // Sense temps per paraula: s'estimen, perquè la lletra també reaccioni paraula a paraula.
        lines = ls.map((x, i) => ({ t: x.t, text: x.text, words: synced ? estimateWords(x.text, x.t, ls[i + 1]?.t) : undefined }));
      })
      .catch(() => t.id === lastId && (lines = []));
  });

  const active = $derived.by(() => {
    if (!lines?.length || !synced) return 0;
    let ans = -1;
    for (let i = 0; i < lines.length && lines[i].t <= now + 250; i++) ans = i;
    return Math.max(0, ans);
  });

  const look = $derived(st?.look ?? "spotify");
  /** Última paraula que ja s'ha començat a cantar (la que brilla). */
  const singing = $derived.by(() => {
    const ws = lines?.[active]?.words;
    if (!ws) return -1;
    let k = -1;
    for (let i = 0; i < ws.length && ws[i].t <= now + 250; i++) k = i;
    return k;
  });
  /** Durada de la línia i quant fa que ha començat (per a la barreta que s'omple). */
  const lineMs = $derived(lines?.[active] ? Math.max(500, (lines[active + 1]?.t ?? lines[active].t + 4000) - lines[active].t) : 0);

  // Rellotge: dorm fins al pròxim canvi (línia o paraula); res mentre està en pausa.
  let timer: ReturnType<typeof setTimeout> | undefined;
  $effect(() => {
    void st;
    void lines;
    clearTimeout(timer);
    const tick = () => {
      now = pos();
      if (!st?.playing || !lines?.length || !synced) return;
      const cur = lines[active];
      const nextT = cur?.words ? Math.min(...cur.words.map((w) => w.t).filter((x) => x > now), lines[active + 1]?.t ?? Infinity) : (lines[active + 1]?.t ?? Infinity);
      const wait = Math.min(isFinite(nextT) ? nextT - now - 250 : 2000, 2000);
      timer = setTimeout(tick, Math.max(cur?.words ? 60 : 120, wait));
    };
    tick();
    return () => clearTimeout(timer);
  });

  onMount(() => {
    const un = listen<MiniState>("mini:state", ({ payload }) => (st = payload));
    emitTo("main", "mini:ready", null).catch(() => {});
    // Recorda la posició i la mida.
    const save = async () => {
      const p = await win.outerPosition();
      const s = await win.innerSize();
      const f = await win.scaleFactor();
      try {
        localStorage.setItem("ona.mini", JSON.stringify({ x: p.x / f, y: p.y / f, w: s.width / f, h: s.height / f }));
      } catch {}
    };
    const m = win.onMoved(save);
    const r = win.onResized(save);
    return () => {
      un.then((f) => f());
      m.then((f) => f());
      r.then((f) => f());
    };
  });

  const cmd = (a: string) => emitTo("main", "mini:cmd", a).catch(() => {});
  function zoom(d: number) {
    scale = Math.min(1.6, Math.max(0.7, scale + d));
    try {
      localStorage.setItem("ona.mini.scale", String(scale));
    } catch {}
  }
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && win.close()} />

<div
  class="mini {look}"
  class:paused={!st?.playing}
  data-tauri-drag-region
  style:--pa={st?.color ?? "#333"}
  style:--pb={st?.color2 ?? "#222"}
  style:--k={scale}
>
  <!-- Les taques giren un tros a cada línia nova (amb molla): reaccionen a la cançó i no gasten entre línies. -->
  {#if look === "ona"}<div class="blobs" aria-hidden="true" style:rotate="{active * 47}deg"></div>{/if}
  {#if !st?.track}
    <p class="msg" data-tauri-drag-region>No sona res.</p>
  {:else if lines === null}
    <p class="msg" data-tauri-drag-region>Carregant la lletra…</p>
  {:else if !lines.length}
    <p class="msg" data-tauri-drag-region>Aquesta cançó no té lletra.</p>
  {:else}
    <div class="lines" data-tauri-drag-region aria-live="polite">
      {#key active}
        <p class="cur" data-tauri-drag-region>
          {#if lines[active].words?.length}
            {#each lines[active].words ?? [] as w, i (i)}<span class:sung={i <= singing} class:now={i === singing}>{w.text}</span>{/each}
          {:else}
            {lines[active].text}
          {/if}
        </p>
        {#if synced && lines[active + 1]}<p class="next" data-tauri-drag-region>{lines[active + 1].text}</p>{/if}
        {#if synced && look !== "spotify"}
          <!-- S'omple al llarg de la línia (compositor: scaleX); comença on toca si s'obre a mitja línia. -->
          <i class="lbar" aria-hidden="true" style:animation-duration="{lineMs}ms" style:animation-delay="{-Math.max(0, now - lines[active].t)}ms"></i>
        {/if}
      {/key}
    </div>
  {/if}

  <div class="tools">
    <button onclick={() => cmd("prev")} title="Anterior" aria-label="Anterior"><Icon name="prev" size={16} /></button>
    <button onclick={() => cmd("toggle")} title={st?.playing ? "Pausa" : "Reprodueix"} aria-label={st?.playing ? "Pausa" : "Reprodueix"}>
      <Icon name={st?.playing ? "pause" : "play"} size={16} />
    </button>
    <button onclick={() => cmd("next")} title="Següent" aria-label="Següent"><Icon name="next" size={16} /></button>
    <span class="sp"></span>
    <button onclick={() => zoom(-0.1)} title="Lletra més petita" aria-label="Lletra més petita">A−</button>
    <button onclick={() => zoom(0.1)} title="Lletra més gran" aria-label="Lletra més gran">A+</button>
    <button onclick={() => win.close()} title="Tanca (Esc)" aria-label="Tanca la lletra flotant"><Icon name="close" size={16} /></button>
  </div>
</div>

<style>
  .mini {
    position: fixed;
    inset: 0;
    overflow: hidden;
    padding: 14px 18px;
    box-sizing: border-box;
    border-radius: 12px;
    background: linear-gradient(135deg, color-mix(in srgb, var(--pa) 55%, #121212), #121212 80%);
    transition: background 0.6s;
    display: flex;
    flex-direction: column;
    justify-content: center;
    user-select: none;
  }
  .lines {
    position: relative;
  }
  .cur,
  .next,
  .msg {
    margin: 0;
    font-family: var(--font-title);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .cur {
    font-size: calc(20px * var(--k));
    font-weight: 700;
    color: #fff;
    transform-origin: left center;
    animation: rise var(--t-spring) var(--spring) both;
  }
  .cur span {
    color: rgb(255 255 255 / 0.45);
    transition: color var(--t-quick);
  }
  .cur span.sung {
    color: #fff;
  }
  .next {
    margin-top: 4px;
    font-size: calc(15px * var(--k));
    color: var(--text-2);
    animation: enter var(--t-spring) var(--spring-soft) both;
  }
  /* La següent puja i creix fins a ser l'actual. */
  @keyframes rise {
    from {
      transform: translateY(70%) scale(0.75);
      opacity: 0.6;
    }
  }
  @keyframes enter {
    from {
      transform: translateY(60%);
      opacity: 0;
    }
  }
  .msg {
    font-size: 15px;
    color: var(--text-2);
  }
  .tools {
    position: absolute;
    left: 8px;
    right: 8px;
    bottom: 6px;
    display: flex;
    align-items: center;
    gap: 2px;
    opacity: 0;
    transition: opacity var(--t-quick);
  }
  .mini:hover .tools,
  .tools:focus-within {
    opacity: 1;
  }
  .tools button {
    height: 26px;
    min-width: 26px;
    padding: 0 6px;
    border-radius: 13px;
    display: grid;
    place-items: center;
    font-size: 12px;
    font-weight: 700;
    color: var(--text-2);
    background: rgb(0 0 0 / 0.45);
  }
  .tools button:hover {
    color: #fff;
  }
  .sp {
    flex: 1;
  }

  /* ---------- Ona i Liner: cada paraula s'aixeca i s'encén quan toca ---------- */
  .ona .cur span,
  .liner .cur span {
    display: inline-block;
    white-space: pre;
    opacity: 0.38;
    color: #fff;
    transition:
      opacity 0.35s ease-out,
      transform 0.6s var(--spring);
  }
  .ona .cur span.sung,
  .liner .cur span.sung {
    opacity: 1;
  }
  .ona .cur span.now,
  .liner .cur span.now {
    transform: translateY(-0.08em) scale(1.06);
  }
  .ona .cur span.now {
    color: color-mix(in oklab, var(--pa) 45%, white);
    text-shadow: 0 0 18px color-mix(in srgb, var(--pa) 70%, transparent);
  }
  .lbar {
    position: absolute;
    left: 0;
    right: 0;
    bottom: -8px;
    height: 2px;
    border-radius: 2px;
    background: currentColor;
    color: #fff;
    opacity: 0.5;
    transform-origin: left;
    animation: fillbar linear both;
  }
  .ona .lbar {
    color: color-mix(in oklab, var(--pa) 50%, white);
    opacity: 0.8;
  }
  @keyframes fillbar {
    from {
      transform: scaleX(0);
    }
  }
  .paused .lbar {
    animation-play-state: paused;
  }
  /* Ona: fons de vidre fosc amb dues taques de color de la portada que giren a poc a poc. */
  .mini.ona {
    border-radius: 18px;
    background: #0d0b14;
    box-shadow: inset 0 0 0 1px rgb(255 255 255 / 0.08);
  }
  .blobs {
    position: absolute;
    inset: -60%;
    background:
      radial-gradient(closest-side at 30% 40%, color-mix(in oklab, var(--pa) 80%, white 8%), transparent),
      radial-gradient(closest-side at 70% 60%, color-mix(in oklab, var(--pb) 75%, white 6%), transparent);
    opacity: 0.85;
    transition: rotate 1.6s var(--spring-soft);
    pointer-events: none;
  }
  .ona .lines,
  .ona .msg {
    position: relative;
    z-index: 1;
  }
  .ona .tools {
    z-index: 1;
  }
  .liner {
    border-radius: 16px;
    background: #050506;
    box-shadow: inset 0 0 0 1px rgb(255 255 255 / 0.07);
  }
  .liner .next {
    color: #a1a1a8;
  }
  @media (prefers-reduced-motion: reduce) {
    .lbar {
      animation: none;
    }
    .blobs {
      transition: none;
    }
    .cur span {
      transition: none !important;
    }
  }
</style>
