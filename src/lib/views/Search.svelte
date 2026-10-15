<script lang="ts" module>
  // Es conserva la cerca en tornar enrere.
  export const searchMemory: { q: string; results: Awaited<ReturnType<typeof A.search>> | null } = { q: "", results: null };
</script>

<script lang="ts">
  /** Resultats de la consulta "searchDesktop"; el text ve del cercador de la barra de dalt (ui.query). */
  import * as A from "$lib/api";
  import Card from "$lib/components/Card.svelte";
  import Cover from "$lib/components/Cover.svelte";
  import Icon from "$lib/components/Icon.svelte";
  import TrackList from "$lib/components/TrackList.svelte";
  import { ui } from "$lib/ui.svelte";

  let results = $state(searchMemory.results);
  let seq = 0;

  $effect(() => {
    const term = ui.query.trim();
    if (!term) {
      results = searchMemory.results = null;
      searchMemory.q = "";
      return;
    }
    if (term === searchMemory.q) return;
    // Espera que l'usuari pari d'escriure per no fer una crida per lletra.
    const timer = setTimeout(async () => {
      const my = ++seq;
      try {
        const r = await A.search(term);
        if (my === seq) {
          results = searchMemory.results = r;
          searchMemory.q = term;
        }
      } catch {
        ui.say("No s'ha pogut cercar");
      }
    }, 300);
    return () => clearTimeout(timer);
  });

  const top = $derived(results?.artists[0] ?? results?.albums[0] ?? results?.playlists[0]);
  const open = () => top && ui.go({ name: "collection", kind: top.kind, id: top.id });
</script>

<section class="page">
  {#if !results}
    <p class="hint">Escriu al cercador de dalt què vols escoltar.</p>
  {:else}
    <div class="top">
      {#if top}
        <div class="best-wrap">
          <h2>Resultat principal</h2>
          <div class="best" role="button" tabindex="0" onclick={open} onkeydown={(e) => e.key === "Enter" && open()}>
            <Cover src={top.image} size={92} radius={6} round={top.kind === "artist"} />
            <h3>{top.name}</h3>
            <span class="muted"><span class="kind">{{ artist: "Artista", album: "Àlbum", playlist: "Playlist", liked: "" }[top.kind]}</span></span>
            <span class="play-fab"><Icon name="play" size={24} /></span>
          </div>
        </div>
      {/if}
      {#if results.tracks.length}
        <div class="songs">
          <h2>Cançons</h2>
          <TrackList tracks={results.tracks.slice(0, 4)} context={`Cerca: ${ui.query}`} showAlbum={false} header={false} />
        </div>
      {/if}
    </div>

    {#each [["Artistes", results.artists], ["Àlbums", results.albums], ["Playlists", results.playlists]] as const as [title, list] (title)}
      {#if list.length}
        <div class="shelf-title"><h2>{title}</h2></div>
        <div class="shelf">
          {#each list as item (item.kind + item.id)}<Card {item} />{/each}
        </div>
      {/if}
    {/each}
  {/if}
</section>

<style>
  .hint {
    font-size: 16px;
    color: var(--text-2);
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
    transform: translateY(8px);
    transition: opacity 0.3s, transform 0.3s;
  }
  .best:hover .play-fab {
    opacity: 1;
    transform: none;
  }
</style>
