<script lang="ts">
  /** Targeta com les del client: portada, títol, subtítol i botó verd en passar-hi per sobre. */
  import * as A from "$lib/api";
  import type { Collection } from "$lib/api";
  import { covers } from "$lib/covers.svelte";
  import { TILES } from "$lib/tiles";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";
  import Cover from "./Cover.svelte";
  import Icon from "./Icon.svelte";

  let { item }: { item: Collection } = $props();

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

<div class="card" role="button" tabindex="0" onclick={open} onkeydown={(e) => e.key === "Enter" && open()}>
  <div class="art">
    {#if TILES[item.kind]}
      <div class="liked" style:background={TILES[item.kind]?.bg}><Icon name={TILES[item.kind]?.icon ?? ""} size={48} /></div>
    {:else}
      <Cover src={item.image ?? covers[item.id]} size="100%" radius={6} round={item.kind === "artist"} />
    {/if}
    <button class="play-fab" onclick={play} title="Reprodueix"><Icon name="play" size={24} /></button>
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
  }
  .card:hover {
    background: var(--highlight);
  }
  .art {
    position: relative;
    aspect-ratio: 1;
    margin-bottom: 4px;
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
  .play-fab {
    position: absolute;
    right: 8px;
    bottom: 8px;
    opacity: 0;
    transform: translateY(8px);
    transition: opacity 0.3s, transform 0.3s;
  }
  .card:hover .play-fab,
  .play-fab:focus-visible {
    opacity: 1;
    transform: none;
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
