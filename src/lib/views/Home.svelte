<script lang="ts">
  /** Inici amb el format del client: dreceres i prestatges (fets amb la teva biblioteca). */
  import * as A from "$lib/api";
  import type { Collection, HomeSection } from "$lib/api";
  import { paletteFrom } from "$lib/color";
  import Card from "$lib/components/Card.svelte";
  import Cover from "$lib/components/Cover.svelte";
  import Icon from "$lib/components/Icon.svelte";
  import { covers } from "$lib/covers.svelte";
  import { TILES } from "$lib/tiles";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";

  let shortcuts = $state<Collection[]>([]);
  let sections = $state<HomeSection[]>([]);
  let failed = $state(false);
  let tint = $state("#333");

  $effect(() => {
    A.home(ui.free)
      .then((h) => {
        shortcuts = h.shortcuts;
        sections = h.sections;
        paletteFrom(h.shortcuts.find((c) => c.image)?.image).then((p) => (tint = p.a));
        if (!ui.free) A.fillCovers(h.sections[0]?.items ?? []);
      })
      .catch(() => (failed = true));
  });

  const open = (c: Collection) => ui.go({ name: "collection", kind: c.kind, id: c.id });
  async function play(e: MouseEvent, c: Collection) {
    e.stopPropagation();
    const d = await A.loadCollection(c.kind, c.id).catch(() => null);
    if (d) player.playList(d.tracks, 0, d.info.name);
  }
</script>

<section class="page home" style:--tint={tint}>
  {#if failed}
    <p class="muted">No s'ha pogut carregar l'inici.</p>
  {/if}

  {#if shortcuts.length}
    <div class="shortcuts">
      {#each shortcuts as c (c.kind + c.id)}
        <div class="sc" role="button" tabindex="0" onclick={() => open(c)} onkeydown={(e) => (e.key === "Enter" || e.key === " ") && (e.preventDefault(), open(c))}>
          {#if TILES[c.kind]}
            <span class="liked" style:background={TILES[c.kind]?.bg}><Icon name={TILES[c.kind]?.icon ?? ""} size={20} /></span>
          {:else}
            <Cover src={c.image ?? covers[c.id]} size={56} radius={0} round={c.kind === "artist"} />
          {/if}
          <span>{c.name}</span>
          <button class="play-fab" onclick={(e) => play(e, c)} title="Reprodueix"><Icon name="play" size={20} /></button>
        </div>
      {/each}
    </div>
  {/if}

  {#each sections as s (s.title)}
    <div class="shelf-title"><h2>{s.title}</h2></div>
    <div class="shelf">
      {#each s.items as item (item.kind + item.id)}<Card {item} />{/each}
    </div>
  {/each}
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
  }
  .sc:hover {
    background: var(--tinted-hi);
  }
  .liked {
    width: 56px;
    height: 56px;
    flex: none;
    display: grid;
    place-items: center;
  }
  .sc > span:not(.liked) {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    display: -webkit-box;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2;
    padding-right: 48px;
  }
  .sc .play-fab {
    position: absolute;
    right: 8px;
    width: 32px;
    height: 32px;
    opacity: 0;
  }
  .sc:hover .play-fab {
    opacity: 1;
  }
</style>
