<script lang="ts" module>
  // Estrelles fixes: un sol element amb moltes `box-shadow` (es pinta un cop, no costa res).
  function starField(count: number, size: number) {
    const out: string[] = [];
    for (let i = 0; i < count; i++) {
      const x = (Math.random() * 100).toFixed(2);
      const y = (Math.random() * 100).toFixed(2);
      const a = (0.35 + Math.random() * 0.65).toFixed(2);
      out.push(`${x}vw ${y}vh 0 ${size / 2}px rgb(255 255 255 / ${a})`);
    }
    return out.join(",");
  }
  const SMALL = starField(140, 1);
  const BIG = starField(45, 2);
  const SHOOTERS = Array.from({ length: 5 }, (_, i) => ({
    top: 5 + Math.random() * 45,
    left: 30 + Math.random() * 70,
    delay: i * 3.1 + Math.random() * 2,
    dur: 9 + Math.random() * 6,
  }));
  // Viatge: estrelles al voltant del centre (offsets en vmax) que s'acosten escalant la capa sencera.
  function starsAround(count: number, size: number) {
    const out: string[] = [];
    for (let i = 0; i < count; i++) {
      const x = ((Math.random() - 0.5) * 120).toFixed(2);
      const y = ((Math.random() - 0.5) * 120).toFixed(2);
      const a = (0.4 + Math.random() * 0.6).toFixed(2);
      out.push(`${x}vmax ${y}vmax 0 ${size / 2}px rgb(255 255 255 / ${a})`);
    }
    return out.join(",");
  }
  // Tres profunditats; cada una amb dues fases desfasades mitja volta perquè el flux no tingui forats.
  const DEPTHS = [
    { count: 90, size: 1, dur: 60 },
    { count: 45, size: 1.5, dur: 40 },
    { count: 18, size: 2.5, dur: 26 },
  ];
  const WARP = DEPTHS.flatMap((d) =>
    [0, 0.5].map((phase) => ({ shadow: starsAround(d.count, d.size), size: d.size, dur: d.dur, delay: -d.dur * (phase + Math.random() * 0.2) })),
  );
  const BUBBLES = Array.from({ length: 14 }, () => ({
    left: Math.random() * 100,
    size: 10 + Math.random() * 46,
    delay: -Math.random() * 30,
    dur: 18 + Math.random() * 22,
  }));
</script>

<script lang="ts">
  /**
   * Fons de tota l'app a l'estètica Ona (de l'Ona 0.1), amb efectes animats només amb CSS
   * (`transform`/`opacity`: els mou la GPU sense JavaScript). En mode lleuger no hi ha efectes.
   */
  import { player } from "$lib/player.svelte";
  import { settings } from "$lib/settings.svelte";
  import { themeById } from "$lib/themes";
  import { ui } from "$lib/ui.svelte";
  import { onFrame, useSpectrum } from "$lib/viz.svelte";

  const theme = $derived(themeById(settings.theme));
  const image = $derived(theme.id === "cover" ? (player.current?.album.thumb ?? null) : null);
  const fx = $derived(settings.light ? "none" : settings.effect);

  // Salt per l'hiperespai en canviar de cançó o de vista: un sol cop, amb WAAPI (compositor).
  let warp = $state<HTMLElement>();
  let rays = $state<HTMLElement>();
  let first = true;
  $effect(() => {
    void player.current?.uri;
    void ui.view;
    if (first) return void (first = false);
    if (!warp || !rays || matchMedia("(prefers-reduced-motion: reduce)").matches) return;
    warp.animate([{ scale: 1 }, { scale: 1.45, offset: 0.3 }, { scale: 1 }], { duration: 1100, easing: "cubic-bezier(.2,.7,.2,1)" });
    rays.animate(
      [
        { opacity: 0, scale: 0.6 },
        { opacity: 0.55, offset: 0.25 },
        { opacity: 0, scale: 1.5 },
      ],
      { duration: 900, easing: "ease-out" },
    );
  });

  // El viatge segueix la música: amb els greus, les estrelles acceleren; a cada cop fort, una empenta.
  // Només mentre sona i amb el visualitzador activat (fa servir el mateix espectre del Rust).
  $effect(() => {
    const w = warp;
    if (fx !== "warp" || !settings.viz || !player.playing || !w) return;
    if (matchMedia("(prefers-reduced-motion: reduce)").matches) return;
    const anims = [...w.querySelectorAll(".layer")].flatMap((l) => l.getAnimations());
    const leave = useSpectrum();
    let rate = 1;
    let avg = 0.2;
    let lastKick = 0;
    const off = onFrame((bass) => {
      const next = rate + (1 + bass * 5 - rate) * 0.2;
      // Només es toca l'animació si el canvi es nota: cada canvi es torna a enviar al compositor.
      if (Math.abs(next - rate) > 0.03) {
        rate = next;
        for (const a of anims) a.playbackRate = rate;
      }
      avg += (bass - avg) * 0.05;
      const now = performance.now();
      if (bass > 0.25 && bass > avg * 1.35 && now - lastKick > 380) {
        lastKick = now;
        w.animate([{ scale: 1 }, { scale: 1.045 }, { scale: 1 }], { duration: 360, easing: "ease-out" });
      }
    });
    return () => {
      off();
      leave();
      for (const a of anims) a.playbackRate = 1;
    };
  });
</script>

<div class="themebg" aria-hidden="true" style:background={theme.background}>
  {#if image}
    {#key image}
      <img class="photo" src={image} alt="" />
    {/key}
  {/if}

  {#if fx === "stars"}
    <div class="stars small" style:box-shadow={SMALL}></div>
    <div class="stars big" style:box-shadow={BIG}></div>
    {#each SHOOTERS as s, i (i)}
      <span class="shooter" style:top="{s.top}%" style:left="{s.left}%" style:animation-delay="{s.delay}s" style:animation-duration="{s.dur}s"></span>
    {/each}
  {:else if fx === "warp"}
    <div class="nebula"></div>
    <div class="warp" bind:this={warp}>
      {#each WARP as l, i (i)}
        <div class="layer" style:animation-duration="{l.dur}s" style:animation-delay="{l.delay}s">
          <span class="dots" style:width="{l.size}px" style:height="{l.size}px" style:box-shadow={l.shadow}></span>
        </div>
      {/each}
    </div>
    <div class="rays" bind:this={rays}></div>
  {:else if fx === "bubbles"}
    {#each BUBBLES as b, i (i)}
      <span
        class="bubble"
        style:left="{b.left}%"
        style:width="{b.size}px"
        style:height="{b.size}px"
        style:animation-delay="{b.delay}s"
        style:animation-duration="{b.dur}s"
      ></span>
    {/each}
  {/if}
  <!-- Vel: els temes de colors són vius; el contingut ha de continuar llegint-se bé. -->
  <div class="dim" class:off={theme.id === "ona"}></div>
</div>

<style>
  .themebg {
    position: fixed;
    inset: 0;
    /* .app fa `isolation: isolate`: amb -1 queda darrere de tot el contingut de l'app. */
    z-index: -1;
    overflow: hidden;
    pointer-events: none;
    transition: background 0.8s;
  }
  /* "Portada": la miniatura de 64 px escalada ja surt borrosa; el blur acaba de suavitzar (es pinta un cop). */
  .photo {
    position: absolute;
    inset: -10%;
    width: 120%;
    height: 120%;
    object-fit: cover;
    filter: blur(40px) saturate(1.6) brightness(0.6);
    animation: show 0.9s ease both;
  }
  @keyframes show {
    from {
      opacity: 0;
    }
  }
  .dim {
    position: absolute;
    inset: 0;
    background: #05040a;
    opacity: 0.45;
    transition: opacity 0.3s;
  }
  .dim.off {
    opacity: 0;
  }
  .stars {
    position: absolute;
    left: 0;
    top: 0;
    border-radius: 50%;
  }
  .stars.small {
    width: 1px;
    height: 1px;
  }
  .stars.big {
    width: 2px;
    height: 2px;
    animation: twinkle 4s ease-in-out infinite alternate;
  }
  @keyframes twinkle {
    from {
      opacity: 0.35;
    }
  }
  /* Estrella fugaç: una ratlla amb cua que travessa en diagonal i després espera. */
  .shooter {
    position: absolute;
    width: 160px;
    height: 2px;
    border-radius: 2px;
    background: linear-gradient(90deg, rgb(255 255 255 / 0.95), rgb(255 255 255 / 0));
    rotate: -32deg;
    opacity: 0;
    animation: shoot linear infinite;
  }
  @keyframes shoot {
    0% {
      transform: translateX(0);
      opacity: 0;
    }
    2% {
      opacity: 1;
    }
    9% {
      transform: translateX(-520px);
      opacity: 0;
    }
    100% {
      transform: translateX(-520px);
      opacity: 0;
    }
  }
  .bubble {
    position: absolute;
    bottom: -80px;
    border-radius: 50%;
    background: radial-gradient(circle at 30% 30%, rgb(255 255 255 / 0.35), rgb(255 255 255 / 0.05) 60%, transparent);
    border: 1px solid rgb(255 255 255 / 0.18);
    animation: rise-bubble linear infinite;
  }
  @keyframes rise-bubble {
    to {
      transform: translateY(-115vh) translateX(30px);
    }
  }
  /* Nebulosa tenyida per la portada que gira molt a poc a poc (un sol element, compositor). */
  .nebula {
    position: absolute;
    inset: -30%;
    background:
      radial-gradient(30% 22% at 35% 40%, color-mix(in oklab, var(--pb, #8b7bff) 22%, transparent), transparent 70%),
      radial-gradient(26% 20% at 68% 62%, #7a2bd133, transparent 70%);
    animation: nebula 140s linear infinite;
  }
  @keyframes nebula {
    to {
      rotate: 360deg;
    }
  }
  .warp {
    position: absolute;
    inset: 0;
  }
  .layer {
    position: absolute;
    inset: 0;
    opacity: 0;
    animation: approach linear infinite;
  }
  .dots {
    position: absolute;
    left: 50%;
    top: 50%;
    border-radius: 50%;
  }
  /* Neix petita i transparent al fons, creix cap a nosaltres i s'esvaeix abans de passar de llarg. */
  @keyframes approach {
    from {
      scale: 0.35;
      opacity: 0;
    }
    30% {
      opacity: 1;
    }
    85% {
      opacity: 1;
    }
    to {
      scale: 1.9;
      opacity: 0;
    }
  }
  /* Raigs de l'hiperespai: línies fines que surten del centre, només visibles durant el salt. */
  .rays {
    position: absolute;
    inset: -20%;
    opacity: 0;
    background: repeating-conic-gradient(from 0deg, rgb(255 255 255 / 0.5) 0deg 0.35deg, transparent 0.35deg 7deg);
    mask: radial-gradient(circle, transparent 8%, #000 30%, transparent 70%);
  }
  @media (prefers-reduced-motion: reduce) {
    .nebula,
    .layer,
    .stars.big,
    .shooter,
    .bubble {
      animation: none;
    }
    .layer {
      opacity: 0.8;
    }
  }
</style>
