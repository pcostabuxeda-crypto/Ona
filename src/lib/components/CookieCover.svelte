<script lang="ts" module>
  /** Forma "galeta" (Material 3 Expressive): cercle amb 9 ondulacions suaus. */
  const COOKIE = (() => {
    const pts: string[] = [];
    const n = 180;
    for (let i = 0; i < n; i++) {
      const a = (i / n) * Math.PI * 2;
      const r = 0.455 + 0.045 * Math.cos(a * 9);
      pts.push(`${(50 + Math.cos(a) * r * 100).toFixed(2)}% ${(50 + Math.sin(a) * r * 100).toFixed(2)}%`);
    }
    return `polygon(${pts.join(",")})`;
  })();
</script>

<script lang="ts">
  /**
   * Portada amb forma de galeta que gira mentre sona. La velocitat s'accelera i es frena
   * suaument (Web Animations + playbackRate); la rotació la fa el compositor.
   */
  import Cover from "./Cover.svelte";

  let { src, playing, size = 240 }: { src?: string; playing: boolean; size?: number } = $props();

  let spinner: HTMLDivElement;
  let anim: Animation | undefined;
  let ramp = 0;

  $effect(() => {
    anim = spinner.animate([{ rotate: "0turn" }, { rotate: "1turn" }], { duration: 20000, iterations: Infinity });
    anim.playbackRate = 0;
    return () => anim?.cancel();
  });

  $effect(() => {
    const target = playing ? 1 : 0;
    const a = anim;
    if (!a) return;
    cancelAnimationFrame(ramp);
    const from = a.playbackRate;
    const start = performance.now();
    const dur = playing ? 900 : 1400;
    const step = (now: number) => {
      const k = Math.min(1, (now - start) / dur);
      a.playbackRate = from + (target - from) * (1 - Math.pow(1 - k, 3));
      if (k < 1) ramp = requestAnimationFrame(step);
    };
    ramp = requestAnimationFrame(step);
    return () => cancelAnimationFrame(ramp);
  });
</script>

<div class="cookie" class:playing style:--s="{size}px">
  <div class="glow"></div>
  <div class="spin" bind:this={spinner} style:clip-path={COOKIE}>
    <Cover {src} size="100%" radius={0} />
  </div>
</div>

<style>
  .cookie {
    position: relative;
    width: var(--s);
    height: var(--s);
    transform: scale(0.94);
    transition: transform 0.9s var(--spring);
  }
  .cookie.playing {
    transform: none;
  }
  /* Halo de color estàtic darrere (no gira, així no es repinta). */
  .glow {
    position: absolute;
    inset: -14%;
    border-radius: 50%;
    background: radial-gradient(closest-side, color-mix(in oklab, var(--accent) 45%, transparent), transparent);
    opacity: 0.7;
  }
  .spin {
    position: absolute;
    inset: 0;
    will-change: rotate;
  }
</style>
