<script lang="ts">
  /**
   * Portada en forma de disc que gira mentre sona. La velocitat s'accelera i es frena
   * suaument (Web Animations, playbackRate); la rotació la fa el compositor.
   */
  import Cover from "./Cover.svelte";

  /** `planet`: mode espai — halo del color de la portada, anell inclinat i una lluna que hi orbita. */
  let {
    src,
    playing,
    size = "min(40vh, 30vw)",
    hole = true,
    planet = false,
  }: { src?: string; playing: boolean; size?: string; hole?: boolean; planet?: boolean } = $props();

  let spinner: HTMLDivElement;
  let anim: Animation | undefined;
  let ramp = 0;

  $effect(() => {
    anim = spinner.animate([{ rotate: "0turn" }, { rotate: "1turn" }], {
      duration: 16000,
      iterations: Infinity,
    });
    anim.playbackRate = 0;
    return () => anim?.cancel();
  });

  $effect(() => {
    // Amb «menys moviment» al sistema, el disc no gira.
    const target = playing && !matchMedia("(prefers-reduced-motion: reduce)").matches ? 1 : 0;
    const a = anim;
    if (!a) return;
    cancelAnimationFrame(ramp);
    const from = a.playbackRate;
    const start = performance.now();
    const dur = playing ? 900 : 1300;
    const step = (now: number) => {
      const k = Math.min(1, (now - start) / dur);
      const e = 1 - Math.pow(1 - k, 3);
      a.playbackRate = from + (target - from) * e;
      if (k < 1) ramp = requestAnimationFrame(step);
    };
    ramp = requestAnimationFrame(step);
    return () => cancelAnimationFrame(ramp);
  });
</script>

<div class="disc" style:--s={size} class:playing class:planet>
  {#if planet}
    <div class="glow"></div>
    <!-- L'anell i el disc comparteixen l'espai 3D: la meitat de l'anell passa per darrere. -->
    <div class="orbit">
      <div class="ring"></div>
      <div class="moonpath"><span class="moon"></span></div>
    </div>
  {/if}
  <div class="spin" bind:this={spinner}>
    <Cover {src} size="100%" round />
    <div class="grooves"></div>
  </div>
  <div class="sheen"></div>
  {#if hole}<div class="hole"></div>{/if}
</div>

<style>
  .disc {
    position: relative;
    width: var(--s);
    height: var(--s);
    border-radius: 50%;
    box-shadow:
      0 40px 90px -30px rgb(0 0 0 / 0.95),
      0 0 0 1px rgb(255 255 255 / 0.06);
    transform: scale(0.92);
    transition: transform 0.9s var(--spring);
  }
  .disc.playing {
    transform: none;
  }
  .spin {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    overflow: hidden;
    will-change: rotate;
  }
  /* Solcs de vinil: anells molt subtils damunt la portada. */
  .grooves {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    background:
      radial-gradient(circle, transparent 17%, rgb(0 0 0 / 0.25) 17.5%, transparent 19%),
      repeating-radial-gradient(circle, rgb(0 0 0 / 0) 0 3px, rgb(0 0 0 / 0.08) 3px 4px);
    mix-blend-mode: multiply;
  }
  /* El reflex no gira: és el que fa que el gir es noti. */
  .sheen {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    background: conic-gradient(
      from 30deg,
      transparent 0deg,
      rgb(255 255 255 / 0.13) 30deg,
      transparent 70deg,
      transparent 180deg,
      rgb(255 255 255 / 0.08) 215deg,
      transparent 255deg
    );
    pointer-events: none;
  }
  @media (prefers-reduced-motion: reduce) {
    .disc {
      transition: none;
    }
    .moonpath,
    .moon {
      animation: none;
    }
  }
  /* ---------- Planeta (mode espai) ---------- */
  .planet {
    transform-style: preserve-3d;
    box-shadow:
      0 0 0 1px rgb(255 255 255 / 0.08),
      0 0 60px -10px color-mix(in oklab, var(--pb, #8b7bff) 70%, transparent);
  }
  /* Atmosfera: llum del color de la portada per darrere (es pinta un cop). */
  .glow {
    position: absolute;
    inset: -35%;
    border-radius: 50%;
    background: radial-gradient(circle, color-mix(in oklab, var(--pb, #8b7bff) 45%, transparent) 28%, transparent 62%);
    transform: translateZ(-1px);
    pointer-events: none;
  }
  .orbit {
    position: absolute;
    inset: -24%;
    transform-style: preserve-3d;
    transform: rotateZ(-16deg) rotateX(74deg);
    pointer-events: none;
  }
  .ring {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    border: 2px solid rgb(255 255 255 / 0.28);
    box-shadow:
      0 0 0 6px rgb(255 255 255 / 0.05),
      0 0 24px color-mix(in oklab, var(--pb, #8b7bff) 60%, transparent);
  }
  .moonpath {
    position: absolute;
    inset: 0;
    transform-style: preserve-3d;
    animation: orbit 14s linear infinite;
  }
  .moon {
    position: absolute;
    left: 50%;
    top: -5px;
    width: 10px;
    height: 10px;
    margin-left: -5px;
    border-radius: 50%;
    background: radial-gradient(circle at 35% 35%, #fff, color-mix(in oklab, var(--pb, #8b7bff) 60%, #fff) 60%);
    box-shadow: 0 0 12px rgb(255 255 255 / 0.7);
    /* Desfà el gir i la inclinació de l'òrbita perquè la lluna sempre es vegi rodona. */
    transform: rotateX(-74deg);
    animation: face 14s linear infinite;
  }
  @keyframes orbit {
    to {
      rotate: z 1turn;
    }
  }
  @keyframes face {
    from {
      transform: rotateZ(0deg) rotateX(-74deg);
    }
    to {
      transform: rotateZ(-360deg) rotateX(-74deg);
    }
  }
  .hole {
    position: absolute;
    left: 50%;
    top: 50%;
    width: 7%;
    height: 7%;
    transform: translate(-50%, -50%);
    border-radius: 50%;
    background: #0b0a12;
    box-shadow:
      0 0 0 3px rgb(255 255 255 / 0.12),
      inset 0 2px 4px rgb(0 0 0 / 0.8);
  }
</style>
