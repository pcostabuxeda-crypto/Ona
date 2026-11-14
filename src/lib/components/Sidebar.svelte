<script lang="ts">
  /** "La teva biblioteca": playlists (rootlist), artistes seguits i àlbums desats. */
  import * as A from "$lib/api";
  import type { Collection } from "$lib/api";
  import { covers } from "$lib/covers.svelte";
  import { TILES } from "$lib/tiles";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";
  import Cover from "./Cover.svelte";
  import Icon from "./Icon.svelte";
  import Skeleton from "./Skeleton.svelte";
  import { settings } from "$lib/settings.svelte";
  import { onMount, tick } from "svelte";
  import { matches } from "$lib/fuzzy";

  let { collapsed = false }: { collapsed?: boolean } = $props();

  const LIKED: Collection = { kind: "liked", id: "liked", name: "Cançons que m'agraden", subtitle: "Playlist" };

  let playlists = $state<Collection[]>([]);
  let artists = $state<Collection[]>([]);
  let albums = $state<Collection[]>([]);
  let filter = $state<"" | "playlist" | "artist" | "album">("");
  let wide = $state(false);
  let query = $state("");
  let search = $state<HTMLInputElement>();
  /** Esperant les playlists (només amb compte de Spotify). */
  let loading = $state(false);
  /** Primera càrrega: les files entren esglaonades una sola vegada. */
  let fresh = $state(true);
  onMount(() => {
    const t = setTimeout(() => (fresh = false), 2500);
    return () => clearTimeout(t);
  });

  $effect(() => {
    if (ui.free) return;
    loading = true;
    A.myPlaylists()
      .then((r) => {
        playlists = r;
        A.fillCovers(r);
      })
      .catch(() => ui.say("No s'han pogut carregar les playlists"))
      .finally(() => (loading = false));
    A.followedArtists().then((r) => (artists = r)).catch(() => {});
    A.savedAlbums().then((r) => (albums = r)).catch(() => {});
  });

  // Fitxers locals i Audius hi són sempre (no depenen del compte de Spotify).
  const own = $derived(ui.free ? [A.LOCAL, A.AUDIUS] : [LIKED, A.LOCAL, A.AUDIUS]);
  const all = $derived(
    filter === "playlist"
      ? [...own, ...playlists]
      : filter === "artist"
        ? artists
        : filter === "album"
          ? albums
          : [...own, ...playlists, ...artists, ...albums],
  );
  // Tolerant a accents i a un error per paraula (queixa d'Apple Music: cal l'ortografia exacta).
  const items = $derived(query.trim() ? all.filter((c) => matches(query, `${c.name} ${c.subtitle}`)) : all);

  /** Ctrl+F: cerca a la biblioteca (desplega la barra si està plegada). */
  async function find(e: KeyboardEvent) {
    if (!(e.ctrlKey && !e.shiftKey && !e.altKey && e.key.toLowerCase() === "f")) return;
    e.preventDefault();
    wide = true;
    await tick();
    search?.focus();
    search?.select();
  }
  const small = $derived(collapsed && !wide);

  const isOpen = (c: Collection) => ui.view.name === "collection" && ui.view.kind === c.kind && ui.view.id === c.id;
  const isPlaying = (c: Collection) => player.playing && player.context === c.name;
  const open = (c: Collection) => ui.go({ name: "collection", kind: c.kind, id: c.id });
</script>

<svelte:window onkeydown={find} />

<nav class="lib panelbox" class:small>
  <header>
    <button class="title" onclick={() => (wide = !wide)} title={small ? "Obre la biblioteca" : "Plega la biblioteca"}>
      <Icon name="library" size={24} />
      {#if !small}<span>La teva biblioteca</span>{/if}
    </button>
  </header>

  {#if !small}
    <div class="chips">
      {#each [["playlist", "Playlists"], ["artist", "Artistes"], ["album", "Àlbums"]] as const as [k, label] (k)}
        <button class="chip" class:on={filter === k} onclick={() => (filter = filter === k ? "" : k)}>{label}</button>
      {/each}
    </div>
    <label class="find">
      <Icon name="search" size={16} />
      <input
        bind:this={search}
        bind:value={query}
        type="search"
        placeholder="Cerca a la biblioteca (Ctrl F)"
        aria-label="Cerca a la biblioteca"
        autocomplete="off"
        spellcheck="false"
        onkeydown={(e) => e.key === "Escape" && ((query = ""), search?.blur())}
      />
    </label>
    {#if query.trim() && !items.length}<p class="none">Cap resultat per «{query.trim()}». Prova amb menys lletres.</p>
    {:else if !query.trim() && !items.length && filter === "artist"}<p class="none">Encara no segueixes cap artista.</p>
    {:else if !query.trim() && !items.length && filter === "album"}<p class="none">Encara no has desat cap àlbum.</p>{/if}
  {/if}

  <ul class:still={settings.light}>
    {#each items as c, i (c.kind + c.id)}
      <li class:rise={fresh && i < 20} style:--i={i}>
        <button class="row" class:on={isOpen(c)} onclick={() => open(c)} title={c.name}>
          {#if TILES[c.kind]}
            <span class="liked" style:background={TILES[c.kind]?.bg}><Icon name={TILES[c.kind]?.icon ?? ""} size={small ? 18 : 20} /></span>
          {:else}
            <Cover src={c.image ?? covers[c.id]} size={48} radius={4} round={c.kind === "artist"} />
          {/if}
          {#if !small}
            <span class="txt">
              <span class="name" class:playing={isPlaying(c)}>{c.name}</span>
              <span class="sub">{c.kind === "artist" ? "Artista" : c.subtitle}</span>
            </span>
            {#if isPlaying(c)}<span class="vol"><Icon name="volume" size={16} /></span>{/if}
          {/if}
        </button>
      </li>
    {/each}
    {#if loading && !playlists.length}
      {#each { length: 6 }, i (i)}
        <li class="sk-row" aria-hidden="true">
          <Skeleton w="48px" h="48px" r="4px" />
          {#if !small}<span><Skeleton w="70%" h="14px" /><Skeleton w="40%" h="12px" /></span>{/if}
        </li>
      {/each}
    {/if}
  </ul>
</nav>

<style>
  .lib {
    width: 300px;
    flex: none;
    display: flex;
    flex-direction: column;
    border-radius: var(--radius);
    background: var(--base);
    overflow: hidden;
  }
  .lib.small {
    width: 72px;
  }
  header {
    padding: 12px 16px 8px;
  }
  .small header {
    padding: 12px 0 8px;
    display: flex;
    justify-content: center;
  }
  .title {
    display: flex;
    align-items: center;
    gap: 12px;
    height: 40px;
    color: var(--text-2);
    font-weight: 700;
    font-size: 16px;
  }
  .title:hover {
    color: var(--text);
  }
  .chips {
    display: flex;
    gap: 8px;
    padding: 0 16px 8px;
    overflow: hidden;
  }
  .find {
    display: flex;
    align-items: center;
    gap: 8px;
    margin: 0 16px 8px;
    padding: 0 10px;
    height: 32px;
    border-radius: 4px;
    background: var(--highlight);
    color: var(--text-2);
  }
  .find:focus-within {
    box-shadow: inset 0 0 0 1px var(--text-2);
    color: var(--text);
  }
  .find input {
    flex: 1;
    min-width: 0;
    background: none;
    border: 0;
    outline: 0;
    color: var(--text);
    font: inherit;
  }
  .none {
    margin: 4px 16px 8px;
    font-size: 13px;
    color: var(--text-2);
  }
  ul {
    list-style: none;
    margin: 0;
    padding: 0 8px 8px;
    overflow-y: auto;
    flex: 1;
  }
  .row {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px;
    border-radius: 6px;
    text-align: left;
  }
  .small .row {
    justify-content: center;
  }
  .row:hover,
  .row:focus-visible {
    background: var(--highlight);
  }
  li.rise {
    animation: rise var(--t-spring) var(--spring) both;
    animation-delay: calc(var(--i) * 22ms);
  }
  .still li.rise {
    animation: none;
  }
  @keyframes rise {
    from {
      opacity: 0;
      translate: -10px 0;
    }
  }
  .sk-row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px;
  }
  .small .sk-row {
    justify-content: center;
  }
  .sk-row span {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .row.on {
    background: var(--elevated-hi);
  }
  .liked {
    width: 48px;
    height: 48px;
    flex: none;
    border-radius: 4px;
    display: grid;
    place-items: center;
  }
  .txt {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .name,
  .sub {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .name {
    font-size: 16px;
  }
  .name.playing,
  .vol {
    color: var(--accent);
  }
  .sub {
    font-size: 14px;
    color: var(--text-2);
  }
</style>
