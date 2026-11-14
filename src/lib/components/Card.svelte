<script lang="ts">
  /** Targeta com les del client: portada, títol, subtítol i botó verd en passar-hi per sobre. */
  import * as A from "$lib/api";
  import type { Collection } from "$lib/api";
  import { covers } from "$lib/covers.svelte";
  import { TILES } from "$lib/tiles";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";
  import { tilt } from "$lib/motion";
  import { settings } from "$lib/settings.svelte";
  import Cover from "./Cover.svelte";
  import Icon from "./Icon.svelte";

  /** `index`: posició al prestatge, per a l'entrada esglaonada. */
  let { item, index = 0 }: { item: Collection; index?: number } = $props();

  const open = () => ui.go({ name: "collection", kind: item.kind, id: item.id });

  async function play(e: MouseEvent) {
    e.stopPropagation();
    try {
      const d = await A.loadCollection(item.kind, item.id);
      player.playList(d.tracks, 0, d.info.name);
    } catch {
      ui.say("No s'ha pogut reproduir");
    }
  }
</script>

<div class="card" class:still={settings.light} style:--i={index} use:tilt role="button" tabindex="0" onclick={open} onkeydown={(e) => (e.key === "Enter" || e.key === " ") && (e.preventDefault(), open())}>
  <div class="art">
    {#if TILES[item.kind]}
      <div class="liked" style:background={TILES[item.kind]?.bg}><Icon name={TILES[item.kind]?.icon ?? ""} size={48} /></div>
    {:else}
      <Cover src={item.image ?? covers[item.id]} size="100%" radius={6} round={item.kind === "artist"} tilt="inherit" />
    {/if}
    <!-- L'embolcall fa l'aparició; el botó conserva el seu hover i el «clic que s'enfonsa». -->
    <span class="fab"><button class="play-fab" onclick={play} title="Reprodueix" aria-label="Reprodueix {item.name}"><Icon name="play" size={24} /></button></span>
  </div>
  <span class="name">{item.name}</span>
  <span class="sub">{item.subtitle}</span>
</div>

<style>
  .card {
    min-width: 0;
    padding: 12px;
    border-radius: 6px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    cursor: pointer;
    transition: background 0.2s;
    /* Entrada esglaonada: cada targeta arriba una mica després de l'anterior (màx. 12). */
    animation: rise var(--t-spring) var(--spring) both;
    animation-delay: calc(min(var(--i), 12) * 35ms);
  }
  .card.still {
    animation: none;
  }
  .card:hover,
  .card:focus-visible {
    background: var(--highlight);
  }
  .art {
    position: relative;
    aspect-ratio: 1;
    margin-bottom: 4px;
    transition: translate var(--t-spring) var(--spring);
  }
  /* La portada s'aixeca amb molla en passar-hi per sobre o amb el focus del teclat. */
  .card:hover .art,
  .card:focus-visible .art {
    translate: 0 -4px;
  }
  .art :global(.cover) {
    aspect-ratio: 1;
    height: auto !important;
  }
  .liked {
    width: 100%;
    height: 100%;
    border-radius: 6px;
    display: grid;
    place-items: center;
  }
  .fab {
    position: absolute;
    right: 8px;
    bottom: 8px;
    opacity: 0;
    translate: 0 10px;
    scale: 0.85;
    /* Sortir de pressa; entrar amb molla (regla de sota). */
    transition:
      opacity var(--t-quick) ease-out,
      translate var(--t-quick) ease-out,
      scale var(--t-quick) ease-out;
  }
  .card:hover .fab,
  .card:focus-visible .fab,
  .fab:focus-within {
    opacity: 1;
    translate: none;
    scale: 1;
    transition:
      opacity var(--t-quick) ease-out,
      translate var(--t-spring) var(--spring),
      scale var(--t-spring) var(--spring);
  }
  @keyframes rise {
    from {
      opacity: 0;
      translate: 0 16px;
      scale: 0.96;
    }
  }
  .name,
  .sub {
    overflow: hidden;
    display: -webkit-box;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2;
  }
  .name {
    font-size: 16px;
    -webkit-line-clamp: 1;
  }
  .sub {
    font-size: 14px;
    color: var(--text-2);
  }
</style>
