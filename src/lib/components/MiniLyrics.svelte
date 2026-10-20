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

  type Word = { t: number; text: string };
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
            lines = detectParser(l.ttml)
              .parse(l.ttml, t.duration_ms)
              .map((x) => ({ t: x.startTimeMs, text: x.words, words: x.parts?.map((p) => ({ t: p.startTimeMs, text: p.words })) }));
            return;
          } catch {}
        }
        lines = (l?.lines ?? []).filter((x) => x.text.trim()).map((x) => ({ t: x.t, text: x.text }));
      })
      .catch(() => t.id === lastId && (lines = []));
  });

  const active = $derived.by(() => {
    if (!lines?.length || !synced) return 0;
    let ans = -1;
    for (let i = 0; i < lines.length && lines[i].t <= now + 250; i++) ans = i;
    return Math.max(0, ans);
  });

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

<div class="mini" data-tauri-drag-region style:--pa={st?.color ?? "#333"} style:--k={scale}>
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
          {#if lines[active].words}
            {#each lines[active].words ?? [] as w, i (i)}<span class:sung={w.t <= now + 250}>{w.text}</span>{/each}
          {:else}
            {lines[active].text}
          {/if}
        </p>
        {#if synced && lines[active + 1]}<p class="next" data-tauri-drag-region>{lines[active + 1].text}</p>{/if}
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
</style>
