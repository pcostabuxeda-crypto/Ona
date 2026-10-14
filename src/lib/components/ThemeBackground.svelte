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
  const BUBBLES = Array.from({ length: 14 }, () => ({
    left: Math.random() * 100,
    size: 10 + Math.random() * 46,
    delay: -Math.random() * 30,
    dur: 18 + Math.random() * 22,
  }));
</script>

<script lang="ts">
  /**
   * Fons de tota l'app segons el tema, amb efectes animats només amb CSS
   * (`transform`/`opacity`: els mou la GPU sense JavaScript).
   */
  import { player } from "$lib/player.svelte";
  import { settings } from "$lib/settings.svelte";
  import { themeById } from "$lib/themes";
  import { perf } from "$lib/perf.svelte";

  let { customImage }: { customImage: string | null } = $props();

  const theme = $derived(themeById(settings.theme));
  const image = $derived(
    theme.id === "custom" ? customImage : theme.id === "cover" ? (player.current?.album.thumb ?? null) : null,
  );
</script>

<div class="themebg" aria-hidden="true" style:background={theme.background}>
  {#if image}
    {#key image}
      <img class="photo" class:cover={theme.id === "cover"} src={image} alt="" />
    {/key}
  {/if}

  {#if perf.effect === "stars"}
    <div class="stars small" style:box-shadow={SMALL}></div>
    <div class="stars big" style:box-shadow={BIG}></div>
    {#each SHOOTERS as s, i (i)}
      <span
        class="shooter"
        style:top="{s.top}%"
        style:left="{s.left}%"
        style:animation-delay="{s.delay}s"
        style:animation-duration="{s.dur}s"
      ></span>
    {/each}
  {:else if perf.effect === "bubbles"}
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

  <div class="dim" style:opacity={theme.id === "ona" ? 0 : settings.bgDim}></div>
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
  .photo {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
    animation: show 0.9s ease both;
  }
  /* "Portada": la miniatura de 64 px escalada ja surt borrosa; el blur petit acaba de suavitzar. */
  .photo.cover {
    inset: -10%;
    width: 120%;
    height: 120%;
    filter: blur(40px) saturate(1.6) brightness(0.6);
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
    transition: opacity 0.3s;
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
  @media (prefers-reduced-motion: reduce) {
    .stars.big,
    .shooter,
    .bubble {
      animation: none;
    }
  }
</style>
