<script lang="ts">
  import * as A from "$lib/api";
  import type { Collection } from "$lib/api";
  import Card from "$lib/components/Card.svelte";

  let playlists = $state<Collection[]>([]);
  let albums = $state<Collection[]>([]);
  let tab = $state<"playlists" | "albums">("playlists");

  $effect(() => {
    A.myPlaylists().then((r) => (playlists = r)).catch(() => {});
    A.savedAlbums().then((r) => (albums = r)).catch(() => {});
  });
</script>

<section class="page">
  <h1>Biblioteca</h1>
  <div class="tabs">
    <button class:on={tab === "playlists"} onclick={() => (tab = "playlists")}>Playlists</button>
    <button class:on={tab === "albums"} onclick={() => (tab = "albums")}>Àlbums</button>
  </div>
  <div class="grid">
    {#if tab === "playlists"}
      <Card item={{ kind: "liked", id: "liked", name: "M'agrada", subtitle: "Cançons desades" }} />
      {#each playlists as p, i (p.id)}<Card item={p} index={i + 1} />{/each}
    {:else}
      {#each albums as a, i (a.id)}<Card item={a} index={i} />{/each}
    {/if}
  </div>
</section>

<style>
  .tabs {
    display: flex;
    gap: 8px;
    margin: 4px 0 20px;
  }
  .tabs button {
    padding: 8px 16px;
    border-radius: 999px;
    background: rgb(255 255 255 / 0.07);
    color: var(--text-2);
    font-weight: 550;
    transition:
      background 0.2s,
      color 0.2s;
  }
  .tabs button.on {
    background: var(--text);
    color: #0b0b12;
  }
</style>
