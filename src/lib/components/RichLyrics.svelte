<script lang="ts">
  /**
   * Lletra amb síl·labes que s'il·luminen (@braccato/core) sobre un fons fluid fet amb la portada
   * (@kawarp/core, WebGL). Tot s'atura quan la música para o la finestra no es veu.
   */
  import { onMount } from "svelte";
  import "@braccato/core/element";
  import "@braccato/core/styles/variables.css";
  import "@braccato/core/styles/lyrics.css";
  import "@braccato/core/styles/instrumental.css";
  import { detectParser } from "@braccato/parsers";
  import type { Lyric } from "@braccato/types";
  import { Kawarp } from "@kawarp/core";
  import type { Lyrics } from "$lib/api";
  import { player } from "$lib/player.svelte";

  /** `inset`: marge esquerre per a la columna de la pantalla completa; `dark`: vel més fosc (estètica Liner). */
  /** `space`: mode espai — sense fons fluid (es veu el Cosmos) i cada línia arriba des del fons. */
  let {
    lyrics,
    inset = "40px",
    dark = false,
    space = false,
  }: { lyrics: NonNullable<Lyrics>; inset?: string; dark?: boolean; space?: boolean } = $props();

  // Línies en profunditat: les que vindran esperen petites i lluny, l'activa arriba amb molla i
  // les passades s'allunyen per darrere. Només transicions de `transform`/`opacity` en canviar de línia.
  const DEPTH = `
      .blyrics-container > div {
        transition: transform 700ms var(--spring-soft), opacity 500ms ease;
      }
      .blyrics-container > div.blyrics--active {
        --blyrics-scale: 1;
      }
      .blyrics-container > div.blyrics--active ~ div {
        --blyrics-scale: 0.9;
        opacity: 0.8;
      }
      .blyrics-container > div:has(~ div.blyrics--active) {
        --blyrics-scale: 1.04;
        opacity: 0.3;
      }`;

  let canvas: HTMLCanvasElement;
  let view: HTMLElement & { lyrics: Lyric[] | null; currentTime: number; playing: boolean; theme: string; renderer?: { noteUserScroll(): void } | null };
  let kawarp = $state.raw<Kawarp | null>(null);

  /** TTML (síl·labes) o línies amb temps → format de braccato. */
  const lines = $derived.by((): Lyric[] => {
    const dur = player.current?.duration_ms ?? 0;
    if (lyrics.ttml) {
      try {
        return detectParser(lyrics.ttml).parse(lyrics.ttml, dur);
      } catch {}
    }
    return lyrics.lines.map((l, i) => {
      const end = lyrics.lines[i + 1]?.t ?? dur;
      return { startTimeMs: l.t, durationMs: Math.max(0, end - l.t), words: l.text || "♪" };
    });
  });

  $effect(() => {
    if (view) view.lyrics = lines;
  });

  // Fons: la portada actual.
  $effect(() => {
    const src = player.current?.album.image;
    if (kawarp && src) kawarp.loadImage(src).catch((e) => console.warn("kawarp", e));
  });

  // Només s'anima mentre sona i es veu.
  let visible = $state(!document.hidden);
  $effect(() => {
    if (!kawarp) return;
    if (player.playing && visible) kawarp.start();
    else kawarp.stop();
  });

  onMount(() => {
    // Tema eficient (mesurat en release: 5,1 % de CPU amb els efectes per defecte → 2,0 %): la lletra
    // s'il·lumina paraula per paraula, però sense ona per lletra, rebot, brillantor, escala, desenfocament
    // ni l'escombrat amb mask-image, que són el que obliga a repintar a cada fotograma.
    view.theme = `
      /* blyrics-letter-wave = false; */
      .blyrics-container {
        --blyrics-font-size: clamp(26px, 3.2vw, 40px);
        --blyrics-lyric-active-color: #fff;
        --blyrics-lyric-inactive-color: rgb(255 255 255 / 0.35);
        --blyrics-animate-word-wobble: 0;
        --blyrics-animate-highlight-glow: 0;
        --blyrics-animate-highlight-swipe: 0;
        --blyrics-animate-line-scale: 0;
        --blyrics-blur-amount: 0px;
        --blyrics-scale: 1;
        --blyrics-active-scale: 1;
        font-family: var(--font-title);
        font-weight: 700;
      }${space ? DEPTH : ""}`;
    try {
      // Mode espai: el fons és el Cosmos, no cal el fons fluid (ni la GPU que gasta).
      kawarp = space ? null : new Kawarp(canvas, { blurPasses: 6, animationSpeed: 0.6, saturation: 1.3 });
    } catch {
      kawarp = null; // sense WebGL: es queda el color de fons
    }
    const onVis = () => (visible = !document.hidden);
    document.addEventListener("visibilitychange", onVis);
    // Resolució interna petita: el desenfocament no deixa veure la diferència i la GPU treballa molt menys.
    const fit = () => {
      const r = canvas.getBoundingClientRect();
      canvas.width = 256;
      canvas.height = Math.max(1, Math.round((256 * r.height) / Math.max(1, r.width)));
      kawarp?.resize();
    };
    fit();
    const ro = new ResizeObserver(fit);
    ro.observe(canvas);

    // El rellotge de braccato: només mentre sona (en pausa, n'hi ha prou amb un fotograma).
    let raf = 0;
    const tick = () => {
      view.currentTime = player.liveMs() / 1000;
      view.playing = player.playing;
      raf = player.playing && visible ? requestAnimationFrame(tick) : 0;
    };
    const stopWatch = $effect.root(() => {
      $effect(() => {
        void player.playing;
        void visible;
        void player.positionMs; // salts de posició en pausa
        if (!raf) tick();
      });
    });

    const seek = (e: Event) => player.seekTo((e as CustomEvent<{ timeS: number }>).detail.timeS * 1000);
    view.addEventListener("braccato:line-click", seek);

    return () => {
      cancelAnimationFrame(raf);
      stopWatch();
      ro.disconnect();
      document.removeEventListener("visibilitychange", onVis);
      view.removeEventListener("braccato:line-click", seek);
      kawarp?.dispose();
    };
  });
</script>

<div class="rich" class:dark class:space style:--bg={player.palette.a} style:--inset={inset}>
  <canvas bind:this={canvas}></canvas>
  <braccato-lyrics bind:this={view} onwheel={() => view.renderer?.noteUserScroll()}></braccato-lyrics>
  <p class="src">Lletra: {lyrics.source}{lyrics.ttml ? " · betterlyrics.org" : ""}</p>
</div>

<style>
  .rich {
    position: relative;
    height: 100%;
    overflow: hidden;
    background: var(--bg);
  }
  canvas {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
  }
  /* Vel fosc perquè el text blanc es llegeixi sobre portades clares. */
  .rich::after {
    content: "";
    position: absolute;
    inset: 0;
    background: rgb(0 0 0 / 0.3);
    pointer-events: none;
  }
  .dark::after {
    background: radial-gradient(circle at 30% 40%, rgb(0 0 0 / 0.35), rgb(0 0 0 / 0.8));
  }
  /* Mode espai: el Cosmos es veu a través; només un vel suau perquè el text es llegeixi. */
  .rich.space {
    background: transparent;
  }
  .space::after {
    background: radial-gradient(ellipse at 70% 45%, rgb(3 2 8 / 0.45), transparent 70%);
  }
  braccato-lyrics {
    position: relative;
    z-index: 1;
    display: block;
    height: 100%;
    overflow-y: auto;
    padding: 0 40px 0 var(--inset);
    scrollbar-width: none;
  }
  .src {
    position: absolute;
    z-index: 1;
    left: var(--inset);
    bottom: 8px;
    margin: 0;
    font-size: 12px;
    color: rgb(255 255 255 / 0.6);
  }
</style>
