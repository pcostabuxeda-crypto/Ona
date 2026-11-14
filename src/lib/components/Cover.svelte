<script lang="ts">
  /**
   * Portada amb aparició suau quan carrega, i forma rodona per als artistes.
   * `tilt` (estètica Ona): relleu 3D que segueix el punter amb un reflex de llum; `"inherit"` fa servir
   * les variables d'un pare que ja porta l'acció (p. ex. la targeta sencera).
   */
  import { tilt as tiltAction } from "$lib/motion";

  let {
    src,
    size = 48,
    round = false,
    radius = 8,
    name = "",
    tilt = false,
  }: { src?: string; size?: number | string; round?: boolean; radius?: number; name?: string; tilt?: boolean | "inherit" } = $props();
  let loaded = $state(false);
  const dim = $derived(typeof size === "number" ? `${size}px` : size);
  const self = (node: HTMLElement, on: boolean) => (on ? tiltAction(node) : undefined);
</script>

<div
  class="cover"
  class:relief={!!tilt}
  use:self={tilt === true}
  style:width={dim}
  style:height={dim}
  style:border-radius={round ? "50%" : `${radius}px`}
  style:view-transition-name={name || null}
>
  {#if src}
    <img {src} alt="" loading="lazy" decoding="async" class:loaded onload={() => (loaded = true)} />
  {/if}
</div>

<style>
  .cover {
    flex: none;
    overflow: hidden;
    background: linear-gradient(135deg, rgb(255 255 255 / 0.08), rgb(255 255 255 / 0.02));
    position: relative;
  }
  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
    opacity: 0;
    transform: scale(1.04);
    /* Tokens de moviment: amb moviment reduït, apareix sense transició. */
    transition:
      opacity var(--t-spring) ease-out,
      transform var(--t-spring) var(--spring-soft);
  }
  img.loaded {
    opacity: 1;
    transform: none;
  }

  /* Relleu: torna amb molla; mentre segueix el punter, gairebé immediat. */
  .relief {
    transform: perspective(700px) rotateX(var(--rx, 0deg)) rotateY(var(--ry, 0deg));
    transition: transform var(--t-spring) var(--spring);
  }
  :global(.tilting) .relief,
  .relief:global(.tilting) {
    transition: transform 90ms linear;
  }
  /* Reflex: una taca de llum el doble de gran que es desplaça (transform), no un degradat que es repinta. */
  .relief::after {
    content: "";
    position: absolute;
    inset: -50%;
    background: radial-gradient(circle, rgb(255 255 255 / 0.26), transparent 42%);
    transform: translate(var(--gx, 0%), var(--gy, 0%));
    opacity: 0;
    transition: opacity var(--t-quick) ease-out;
    pointer-events: none;
  }
  :global(.tilting) .relief::after,
  .relief:global(.tilting)::after {
    opacity: 1;
  }
</style>
