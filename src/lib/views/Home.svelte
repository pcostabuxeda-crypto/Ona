<script lang="ts">
  /** Inici amb el format del client: dreceres i prestatges (fets amb la teva biblioteca). */
  import * as A from "$lib/api";
  import type { Collection, HomeSection } from "$lib/api";
  import { paletteFrom } from "$lib/color";
  import Card from "$lib/components/Card.svelte";
  import Cover from "$lib/components/Cover.svelte";
  import Empty from "$lib/components/Empty.svelte";
  import Icon from "$lib/components/Icon.svelte";
  import Skeleton from "$lib/components/Skeleton.svelte";
  import { settings } from "$lib/settings.svelte";
  import { covers } from "$lib/covers.svelte";
  import { TILES } from "$lib/tiles";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";

  let shortcuts = $state<Collection[]>([]);
  let sections = $state<HomeSection[]>([]);
  let status = $state<"loading" | "ok" | "failed">("loading");
  let tint = $state("#333");

  function load() {
    status = "loading";
    A.home(ui.free)
      .then((h) => {
        shortcuts = h.shortcuts;
        sections = h.sections;
        status = "ok";
        paletteFrom(h.shortcuts.find((c) => c.image)?.image).then((p) => (tint = p.a));
        if (!ui.free) A.fillCovers(h.sections[0]?.items ?? []);
      })
      .catch(() => (status = "failed"));
  }
  $effect(load);

  const open = (c: Collection) => ui.go({ name: "collection", kind: c.kind, id: c.id });
  async function play(e: MouseEvent, c: Collection) {
    e.stopPropagation();
    const d = await A.loadCollection(c.kind, c.id).catch(() => null);
    if (d) player.playList(d.tracks, 0, d.info.name);
  }
</script>

<section class="page home" class:still={settings.light} style:--tint={tint}>
  {#if status === "loading"}
    <div role="status" aria-busy="true" aria-label="Carregant l'inici">
      <div class="shortcuts">
        {#each { length: 6 }, i (i)}
          <div class="sc sk"><Skeleton w="56px" h="56px" r="0" /><Skeleton w="45%" h="14px" /></div>
        {/each}
      </div>
      <div class="shelf-title"><Skeleton w="200px" h="24px" /></div>
      <div class="shelf">
        {#each { length: 6 }, i (i)}
          <div class="card-sk"><Skeleton w="100%" h="auto" r="6px" /><Skeleton w="70%" h="14px" /><Skeleton w="45%" h="12px" /></div>
        {/each}
      </div>
    </div>
  {:else if status === "failed"}
    <Empty icon="home" title="No s'ha pogut carregar l'inici" action="Torna-ho a provar" onaction={load}>
      Comprova la connexió. La música que ja sona no s'atura.
    </Empty>
  {:else if !shortcuts.length && !sections.length}
    <Empty icon="search" title="Encara no tens res a la biblioteca" action="Cerca música" onaction={() => document.querySelector<HTMLInputElement>("#search")?.focus()}>
      Quan desis cançons, àlbums o playlists, sortiran aquí. Prova el cercador (Ctrl K).
    </Empty>
  {/if}

  {#if status === "ok" && shortcuts.length}
    <div class="shortcuts">
      {#each shortcuts as c, i (c.kind + c.id)}
        <div class="sc" style:--i={i} role="button" tabindex="0" onclick={() => open(c)} onkeydown={(e) => (e.key === "Enter" || e.key === " ") && (e.preventDefault(), open(c))}>
          {#if TILES[c.kind]}
            <span class="liked" style:background={TILES[c.kind]?.bg}><Icon name={TILES[c.kind]?.icon ?? ""} size={20} /></span>
          {:else}
            <Cover src={c.image ?? covers[c.id]} size={56} radius={0} round={c.kind === "artist"} />
          {/if}
          <span>{c.name}</span>
          <span class="fab"><button class="play-fab" onclick={(e) => play(e, c)} title="Reprodueix" aria-label="Reprodueix {c.name}"><Icon name="play" size={20} /></button></span>
        </div>
      {/each}
    </div>
  {/if}

  {#if status === "ok"}
    {#each sections as s (s.title)}
      <div class="shelf-title"><h2>{s.title}</h2></div>
      <div class="shelf">
        {#each s.items as item, i (item.kind + item.id)}<Card {item} index={i} />{/each}
      </div>
    {/each}
  {/if}
</section>

<style>
  .home {
    background: linear-gradient(color-mix(in srgb, var(--tint) 45%, var(--base)), var(--base) 320px);
    min-height: 100%;
    box-sizing: border-box;
  }
  .shortcuts {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(min(100%, 220px), 1fr));
    gap: 8px;
    margin: 8px 0 16px;
  }
  .sc {
    position: relative;
    display: flex;
    align-items: center;
    gap: 8px;
    height: 56px;
    border-radius: 4px;
    overflow: hidden;
    background: var(--tinted);
    font-weight: 700;
    cursor: pointer;
    transition: background 0.2s;
    animation: rise var(--t-spring) var(--spring) both;
    animation-delay: calc(min(var(--i, 0), 12) * 30ms);
  }
  .still .sc,
  .sk {
    animation: none;
  }
  .sc:hover,
  .sc:focus-visible {
    background: var(--tinted-hi);
  }
  .card-sk {
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .card-sk > :global(:first-child) {
    aspect-ratio: 1;
    margin-bottom: 4px;
  }
  .liked {
    width: 56px;
    height: 56px;
    flex: none;
    display: grid;
    place-items: center;
  }
  .sc > span:not(.liked, .fab) {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    display: -webkit-box;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2;
    padding-right: 48px;
  }
  .fab {
    position: absolute;
    right: 8px;
    opacity: 0;
    scale: 0.7;
    transition:
      opacity var(--t-quick) ease-out,
      scale var(--t-quick) ease-out;
  }
  .sc:hover .fab,
  .sc:focus-visible .fab,
  .fab:focus-within {
    opacity: 1;
    scale: 1;
    transition:
      opacity var(--t-quick) ease-out,
      scale var(--t-spring) var(--spring);
  }
  .sc .play-fab {
    width: 32px;
    height: 32px;
  }
  @keyframes rise {
    from {
      opacity: 0;
      translate: 0 12px;
    }
  }
</style>
