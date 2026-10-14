<script lang="ts">
  import * as A from "$lib/api";
  import type { Collection, Track } from "$lib/api";
  import Card from "$lib/components/Card.svelte";
  import Cover from "$lib/components/Cover.svelte";
  import TrackList from "$lib/components/TrackList.svelte";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";

  let { scroller }: { scroller?: HTMLElement } = $props();

  let recent = $state<Track[]>([]);
  let top = $state<Track[]>([]);
  let playlists = $state<Collection[]>([]);
  let albums = $state<Collection[]>([]);

  const hour = new Date().getHours();
  const greeting = hour < 6 ? "Bona nit" : hour < 14 ? "Bon dia" : hour < 21 ? "Bona tarda" : "Bona nit";

  $effect(() => {
    A.recentTracks().then((r) => (recent = r)).catch(() => {});
    A.topTracks().then((r) => (top = r)).catch(() => {});
    A.myPlaylists().then((r) => (playlists = r)).catch(() => {});
    A.savedAlbums().then((r) => (albums = r)).catch(() => {});
  });

  const quick = $derived(recent.slice(0, 8));
</script>

<section class="page">
  <h1 class="hello">{greeting}{ui.user ? `, ${ui.user}` : ""}</h1>

  {#if quick.length}
    <div class="quick">
      {#each quick as t, i (t.id)}
        <button class="q" style:--d="{i * 40}ms" onclick={() => player.playList(recent, i, "Escoltat fa poc")}>
          <Cover src={t.album.thumb} size={56} radius={0} />
          <span>{t.name}</span>
        </button>
      {/each}
    </div>
  {/if}

  {#if playlists.length}
    <h2>Les teves playlists</h2>
    <div class="grid">
      <Card item={{ kind: "liked", id: "liked", name: "M'agrada", subtitle: "Cançons desades" }} />
      {#each playlists.slice(0, 11) as p, i (p.id)}<Card item={p} index={i + 1} />{/each}
    </div>
  {/if}

  {#if top.length}
    <h2>El teu mes, en bucle</h2>
    <TrackList tracks={top.slice(0, 10)} context="Les més escoltades" />
  {/if}

  {#if albums.length}
    <h2>Àlbums desats</h2>
    <div class="grid">
      {#each albums as a, i (a.id)}<Card item={a} index={i} />{/each}
    </div>
  {/if}
</section>

<style>
  .hello {
    font-size: 40px;
    margin: 8px 0 24px;
  }
  .quick {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 10px;
    margin-bottom: 36px;
  }
  .q {
    display: flex;
    align-items: center;
    gap: 14px;
    border-radius: 10px;
    overflow: hidden;
    background: rgb(255 255 255 / 0.07);
    text-align: left;
    font-weight: 600;
    color: var(--text);
    transition: background 0.2s;
    animation: rise 0.5s cubic-bezier(0.2, 0.8, 0.2, 1) both;
    animation-delay: var(--d);
  }
  .q span {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    padding-right: 12px;
  }
  .q:hover {
    background: rgb(255 255 255 / 0.14);
  }
</style>
