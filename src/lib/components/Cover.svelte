<script lang="ts">
  /** Portada amb aparició suau quan carrega, i forma rodona per als artistes. */
  let {
    src,
    size = 48,
    round = false,
    radius = 8,
    name = "",
  }: { src?: string; size?: number | string; round?: boolean; radius?: number; name?: string } = $props();
  let loaded = $state(false);
  const dim = $derived(typeof size === "number" ? `${size}px` : size);
</script>

<div
  class="cover"
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
</style>
