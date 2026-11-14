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
  import { settings } from "$lib/settings.svelte";
  import { onFrame, useSpectrum } from "$lib/viz.svelte";

  /** `inset`: marge esquerre per a la columna de la pantalla completa; `dark`: vel més fosc (estètica Liner). */
  /**
   * `space`: mode espai — sense fons fluid (es veu el Cosmos) i cada línia arriba des del fons.
   * `aura`: la línia que es canta té al darrere una llum del color de la portada que respira i bat amb
   * els greus, i espurnes que en pugen.
   */
  let {
    lyrics,
    inset = "40px",
    dark = false,
    space = false,
    aura = false,
  }: { lyrics: NonNullable<Lyrics>; inset?: string; dark?: boolean; space?: boolean; aura?: boolean } = $props();

  // Aura: dos pseudoelements de la línia activa, pintats un cop; només s'animen `scale`/`opacity`/`translate`.
  // `--beat` (0–1) el posa el bucle de l'espectre mentre sona.
  const AURA = `
      .blyrics-container > div.blyrics--active::before,
      .blyrics-container > div.blyrics--active::after {
        content: "";
        position: absolute;
        z-index: -1;
        pointer-events: none;
      }
      .blyrics-container > div.blyrics--active::before {
        left: -10%;
        right: 2%;
        top: -70%;
        bottom: -70%;
        border-radius: 50%;
        /* Aclarit cap al blanc: amb portades fosques el color sol gairebé no es veia. */
        --aura: color-mix(in oklab, var(--pb, #8b7bff) 70%, white);
        background: radial-gradient(
          closest-side,
          color-mix(in oklab, var(--aura) 55%, transparent),
          color-mix(in oklab, var(--aura) 22%, transparent) 50%,
          transparent
        );
        opacity: calc(0.7 + var(--beat, 0) * 0.3);
        scale: calc(1 + var(--beat, 0) * 0.18);
        transition: opacity 120ms linear, scale 120ms linear;
        animation: aura-in 700ms var(--spring-soft) backwards, aura-drift 3.4s 700ms ease-in-out infinite alternate;
      }
      .blyrics-container > div.blyrics--active::after {
        inset: -60% 4% 10% 0;
        background-image:
          radial-gradient(3px 3px at 6% 90%, #fff, transparent),
          radial-gradient(2px 2px at 15% 65%, #fffc, transparent),
          radial-gradient(4px 4px at 27% 95%, color-mix(in oklab, var(--pb, #8b7bff) 40%, #fff), transparent),
          radial-gradient(2px 2px at 38% 72%, #fffb, transparent),
          radial-gradient(3px 3px at 49% 88%, #fff, transparent),
          radial-gradient(2px 2px at 60% 60%, #fffc, transparent),
          radial-gradient(4px 4px at 71% 92%, color-mix(in oklab, var(--pb, #8b7bff) 40%, #fff), transparent),
          radial-gradient(2px 2px at 82% 70%, #fffb, transparent),
          radial-gradient(3px 3px at 94% 85%, #fff, transparent);
        animation: sparks 2.8s linear infinite;
      }
      @keyframes aura-in {
        from { opacity: 0; scale: 0.5; }
      }
      @keyframes aura-drift {
        to { translate: 4% 0; }
      }
      @keyframes sparks {
        from { translate: 0 25%; opacity: 0; }
        25% { opacity: 0.9; }
        to { translate: 0 -35%; opacity: 0; }
      }
      @media (prefers-reduced-motion: reduce) {
        .blyrics-container > div.blyrics--active::before { animation: none; transition: none; scale: 1; }
        .blyrics-container > div.blyrics--active::after { display: none; }
      }`;

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

  // L'aura bat amb els greus: mentre sona, es veu i el visualitzador està activat.
  $effect(() => {
    if (!aura || !view || !player.playing || !visible || !settings.viz) return;
    if (matchMedia("(prefers-reduced-motion: reduce)").matches) return;
    const el = view;
    const leave = useSpectrum();
    let last = 0;
    const off = onFrame((bass) => {
      const b = Math.min(1, bass * 3.5);
      // Només s'escriu si es nota: cada canvi recalcula l'estil de la línia activa.
      if (Math.abs(b - last) > 0.03) el.style.setProperty("--beat", (last = b).toFixed(2));
    });
    return () => {
      off();
      leave();
      el.style.setProperty("--beat", "0");
    };
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
      }${space ? DEPTH : ""}${aura ? AURA : ""}`;
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
