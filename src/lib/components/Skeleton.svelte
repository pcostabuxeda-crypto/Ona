<script lang="ts">
  /** Bloc gris de càrrega amb una brillantor que passa (només transform; s'atura en carregar). */
  import { settings } from "$lib/settings.svelte";

  let { w = "100%", h = "14px", r = "6px", round = false }: { w?: string; h?: string; r?: string; round?: boolean } = $props();
</script>

<span class="sk" class:still={settings.light} style:width={w} style:height={h} style:border-radius={round ? "50%" : r} aria-hidden="true"></span>

<style>
  .sk {
    display: block;
    flex: none;
    position: relative;
    overflow: hidden;
    background: var(--tinted);
  }
  .sk::after {
    content: "";
    position: absolute;
    inset: 0;
    background: linear-gradient(90deg, transparent, rgb(255 255 255 / 0.08), transparent);
    transform: translateX(-100%);
    animation: shimmer 1.4s ease-in-out infinite;
  }
  .still::after {
    display: none;
  }
  @media (prefers-reduced-motion: reduce) {
    .sk::after {
      display: none;
    }
  }
  @keyframes shimmer {
    to {
      transform: translateX(100%);
    }
  }
</style>
