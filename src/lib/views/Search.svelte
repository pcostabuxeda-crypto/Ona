<script lang="ts" module>
  import type { Collection, Track } from "$lib/api";
  type Results = { tracks: Track[]; artists: Collection[]; albums: Collection[]; playlists: Collection[]; audius: Track[] };
  // Es conserva la cerca en tornar enrere.
  export const searchMemory: { q: string; results: Results | null } = { q: "", results: null };
</script>

<script lang="ts">
  /** Cerca a Spotify (si hi ha compte) i a Audius; el text ve del cercador de la barra de dalt (ui.query). */
  import * as A from "$lib/api";
  import Card from "$lib/components/Card.svelte";
  import Cover from "$lib/components/Cover.svelte";
  import Empty from "$lib/components/Empty.svelte";
  import Icon from "$lib/components/Icon.svelte";
  import Skeleton from "$lib/components/Skeleton.svelte";
  import TrackList from "$lib/components/TrackList.svelte";
  import { audiusSearch } from "$lib/sources";
  import { ui } from "$lib/ui.svelte";

  let results = $state(searchMemory.results);
  /** Hi ha una cerca en marxa (o esperant que l'usuari pari d'escriure). */
  let pending = $state(false);
  let failed = $state(false);
  let seq = 0;

  $effect(() => {
    const term = ui.query.trim();
    failed = false;
    if (!term) {
      results = searchMemory.results = null;
      searchMemory.q = "";
      pending = false;
      return;
    }
    if (term === searchMemory.q) {
      pending = false;
      return;
    }
    pending = true;
    // Espera que l'usuari pari d'escriure per no fer una crida per lletra.
    const timer = setTimeout(() => run(term), 300);
    return () => clearTimeout(timer);
  });

  async function run(term: string) {
    const my = ++seq;
    try {
      const none = { tracks: [], artists: [], albums: [], playlists: [] };
      const [sp, audius] = await Promise.all([
        ui.free ? none : A.search(term).catch(() => null),
        audiusSearch(term).catch(() => null),
      ]);
      // Si una font falla es mostra l'altra; si fallen totes dues, és un error (no «cap resultat»).
      if ((ui.free || !sp) && !audius) throw new Error("search");
      const r: Results = { ...(sp ?? none), audius: audius ?? [] };
      if (my === seq) {
        results = searchMemory.results = r;
        searchMemory.q = term;
        pending = false;
      }
    } catch {
      if (my === seq) (failed = true), (pending = false);
    }
  }

  const top = $derived(results?.artists[0] ?? results?.albums[0] ?? results?.playlists[0]);
  const open = () => top && ui.go({ name: "collection", kind: top.kind, id: top.id });
  const nothing = $derived(
    !!results && !results.tracks.length && !results.artists.length && !results.albums.length && !results.playlists.length && !results.audius.length,
  );
  function retry() {
    failed = false;
    pending = true;
    run(ui.query.trim());
  }
</script>

<section class="page">
  {#if failed}
    <Empty icon="search" title="No s'ha pogut cercar" action="Torna-ho a provar" onaction={retry}>Comprova la connexió.</Empty>
  {:else if pending && !results}
    <div class="top" role="status" aria-busy="true" aria-label="Cercant">
      <div>
        <Skeleton w="220px" h="24px" />
        <div class="best sk"><Skeleton w="92px" h="92px" /><Skeleton w="60%" h="28px" /><Skeleton w="30%" h="14px" /></div>
      </div>
      <div class="sk-songs">
        <Skeleton w="140px" h="24px" />
        {#each { length: 4 }, i (i)}
          <div class="sk-row"><Skeleton w="40px" h="40px" r="4px" /><span><Skeleton w="55%" h="14px" /><Skeleton w="35%" h="12px" /></span></div>
        {/each}
      </div>
    </div>
  {:else if !results}
    <Empty icon="search" title="Què vols escoltar?">
      Escriu al cercador de dalt (Ctrl K): cançons, artistes, àlbums o playlists.{ui.free ? " Sense compte, cerquem a Audius." : ""}
    </Empty>
  {:else if nothing}
    <Empty icon="search" title="Cap resultat per «{searchMemory.q}»">Revisa l'ortografia o prova amb menys paraules.</Empty>
  {:else}
    {#key results}
    <div class="results" class:stale={pending}>
    <div class="top">
      {#if top}
        <div class="best-wrap">
          <h2>Resultat principal</h2>
          <div class="best" role="button" tabindex="0" onclick={open} onkeydown={(e) => (e.key === "Enter" || e.key === " ") && (e.preventDefault(), open())}>
            <Cover src={top.image} size={92} radius={6} round={top.kind === "artist"} />
            <h3>{top.name}</h3>
            <span class="muted"><span class="kind">{top.kind === "artist" ? "Artista" : top.kind === "album" ? "Àlbum" : "Playlist"}</span></span>
            <span class="play-fab"><Icon name="play" size={24} /></span>
          </div>
        </div>
      {/if}
      {#if results.tracks.length || (ui.free && results.audius.length)}
        <div class="songs">
          <h2>Cançons</h2>
          <TrackList tracks={(ui.free ? results.audius : results.tracks).slice(0, 4)} context={`Cerca: ${ui.query}`} showAlbum={false} header={false} />
        </div>
      {/if}
    </div>

    {#each [["Artistes", results.artists], ["Àlbums", results.albums], ["Playlists", results.playlists]] as const as [title, list] (title)}
      {#if list.length}
        <div class="shelf-title"><h2>{title}</h2></div>
        <div class="shelf">
          {#each list as item, i (item.kind + item.id)}<Card {item} index={i} />{/each}
        </div>
      {/if}
    {/each}

    {#if !ui.free && results.audius.length}
      <div class="shelf-title"><h2>A Audius</h2></div>
      <TrackList tracks={results.audius.slice(0, 10)} context="Audius" showAlbum={false} header={false} />
    {:else if ui.free && results.audius.length > 4}
      <div class="shelf-title"><h2>Més a Audius</h2></div>
      <TrackList tracks={results.audius.slice(4)} context="Audius" showAlbum={false} header={false} />
    {/if}
    </div>
    {/key}
  {/if}
</section>

<style>
  /* Resultats vells mentre arriben els nous: s'atenuen en lloc de desaparèixer. */
  .results {
    transition: opacity var(--t-quick) ease-out;
  }
  .stale {
    opacity: 0.55;
  }
  .best.sk {
    cursor: default;
    gap: 16px;
    margin-top: 16px;
  }
  .sk-songs {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
  .sk-row {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  .sk-row span {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .top {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(min(100%, 340px), 1fr));
    gap: 24px;
  }
  h2 {
    font-size: 24px;
    margin: 8px 0 16px;
  }
  .best {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 20px;
    padding: 20px;
    border-radius: var(--radius);
    background: var(--highlight);
    cursor: pointer;
    transition: background 0.2s;
  }
  .best:hover {
    background: var(--elevated-hi);
  }
  .best h3 {
    font-size: 32px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .best .muted {
    margin-top: -14px;
  }
  .kind {
    padding: 4px 12px;
    border-radius: 999px;
    background: rgb(0 0 0 / 0.4);
    color: #fff;
    font-weight: 700;
  }
  .best .play-fab {
    position: absolute;
    right: 20px;
    bottom: 20px;
    opacity: 0;
    translate: 0 10px;
    transition:
      opacity var(--t-quick) ease-out,
      translate var(--t-quick) ease-out;
  }
  .best:hover .play-fab,
  .best:focus-visible .play-fab {
    opacity: 1;
    translate: none;
    transition:
      opacity var(--t-quick) ease-out,
      translate var(--t-spring) var(--spring);
  }
  .best:focus-visible {
    background: var(--elevated-hi);
  }
</style>
