<script lang="ts">
  import * as A from "$lib/api";
  import Card from "$lib/components/Card.svelte";
  import Cover from "$lib/components/Cover.svelte";
  import Icon from "$lib/components/Icon.svelte";
  import TrackList from "$lib/components/TrackList.svelte";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";

  let q = $state(searchMemory.q);
  let results = $state<Awaited<ReturnType<typeof A.search>> | null>(searchMemory.results);
  let busy = $state(false);
  let input: HTMLInputElement;
  let timer: ReturnType<typeof setTimeout>;
  let seq = 0;

  $effect(() => {
    input?.focus();
  });

  function onInput() {
    clearTimeout(timer);
    const term = q.trim();
    if (!term) {
      results = null;
      searchMemory.q = "";
      searchMemory.results = null;
      return;
    }
    // Espera que l'usuari pari d'escriure per no fer una crida per lletra.
    timer = setTimeout(async () => {
      const my = ++seq;
      busy = true;
      try {
        const r = await A.search(term);
        if (my === seq) {
          results = r;
          searchMemory.q = term;
          searchMemory.results = r;
        }
      } catch (e) {
        ui.say(String(e).startsWith("rate_limited") ? "Spotify diu que anem massa ràpid, espera uns segons" : "No s'ha pogut cercar");
      } finally {
        if (my === seq) busy = false;
      }
    }, 280);
  }

  const best = $derived(results?.tracks[0]);
</script>

<script lang="ts" module>
  // Es conserva la cerca en tornar enrere.
  export const searchMemory: { q: string; results: Awaited<ReturnType<typeof A.search>> | null } = { q: "", results: null };
</script>

<section class="page">
  <label class="box" class:busy>
    <Icon name="search" size={22} />
    <input
      bind:this={input}
      bind:value={q}
      oninput={onInput}
      placeholder="Què vols escoltar?"
      spellcheck="false"
      onkeydown={(e) => e.key === "Escape" && ((q = ""), onInput())}
    />
    <span class="wave"></span>
  </label>

  {#if results && best}
    <div class="top">
      <div>
        <h2>Millor resultat</h2>
        <button class="best" onclick={() => player.playList(results!.tracks, 0, `Cerca: ${q}`)}>
          <Cover src={best.album.image} size={104} radius={10} />
          <span class="bname">{best.name}</span>
          <span class="bsub">Cançó · {best.artists.map((a) => a.name).join(", ")}</span>
          <span class="bplay"><Icon name="play" size={24} /></span>
        </button>
      </div>
      <div class="songs">
        <h2>Cançons</h2>
        <TrackList tracks={results.tracks.slice(0, 5)} context={`Cerca: ${q}`} showAlbum={false} />
      </div>
    </div>

    {#if results.artists.length}
      <h2>Artistes</h2>
      <div class="grid">{#each results.artists.slice(0, 6) as a, i (a.id)}<Card item={a} index={i} />{/each}</div>
    {/if}
    {#if results.albums.length}
      <h2>Àlbums</h2>
      <div class="grid">{#each results.albums.slice(0, 6) as a, i (a.id)}<Card item={a} index={i} />{/each}</div>
    {/if}
    {#if results.playlists.length}
      <h2>Playlists</h2>
      <div class="grid">{#each results.playlists.slice(0, 6) as p, i (p.id)}<Card item={p} index={i} />{/each}</div>
    {/if}
  {:else if !q}
    <p class="hint">Busca cançons, artistes, àlbums o playlists. Prem <kbd>Ctrl</kbd> + <kbd>K</kbd> des de qualsevol lloc.</p>
  {/if}
</section>

<style>
  .box {
    position: sticky;
    top: 0;
    z-index: 5;
    display: flex;
    align-items: center;
    gap: 14px;
    height: 60px;
    padding: 0 22px;
    border-radius: 30px;
    background: #15141f;
    border: 1px solid rgb(255 255 255 / 0.08);
    color: var(--text-2);
    overflow: hidden;
    margin-bottom: 28px;
    transition:
      border-color 0.2s,
      box-shadow 0.3s;
  }
  .box:focus-within {
    border-color: color-mix(in oklab, var(--pa) 60%, transparent);
    box-shadow: 0 0 0 4px color-mix(in oklab, var(--pa) 18%, transparent);
    color: var(--text);
  }
  input {
    flex: 1;
    background: none;
    border: 0;
    outline: 0;
    font: inherit;
    font-size: 18px;
    color: var(--text);
  }
  input::placeholder {
    color: var(--text-3);
  }
  .wave {
    position: absolute;
    left: 0;
    bottom: 0;
    height: 2px;
    width: 40%;
    background: linear-gradient(90deg, transparent, var(--pa), var(--pb), transparent);
    opacity: 0;
    transform: translateX(-100%);
  }
  .busy .wave {
    opacity: 1;
    animation: sweep 1s linear infinite;
  }
  @keyframes sweep {
    to {
      transform: translateX(250%);
    }
  }
  .top {
    display: grid;
    grid-template-columns: minmax(280px, 0.8fr) 1.6fr;
    gap: 24px;
  }
  .top h2 {
    margin-top: 0;
  }
  .best {
    position: relative;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 10px;
    padding: 22px;
    border-radius: 18px;
    background: rgb(255 255 255 / 0.06);
    text-align: left;
    transition: background 0.2s;
    animation: rise 0.45s cubic-bezier(0.2, 0.8, 0.2, 1) both;
  }
  .best:hover {
    background: rgb(255 255 255 / 0.1);
  }
  .bname {
    font-size: 28px;
    font-weight: 700;
    letter-spacing: -0.02em;
    color: var(--text);
    margin-top: 8px;
  }
  .bsub {
    color: var(--text-2);
  }
  .bplay {
    position: absolute;
    right: 20px;
    bottom: 20px;
    width: 52px;
    height: 52px;
    border-radius: 50%;
    display: grid;
    place-items: center;
    background: var(--accent);
    color: #0b0b12;
    box-shadow: 0 10px 24px -8px var(--accent);
    opacity: 0;
    transform: translateY(8px);
    transition:
      opacity 0.2s,
      transform 0.3s cubic-bezier(0.2, 0.8, 0.2, 1);
  }
  .best:hover .bplay {
    opacity: 1;
    transform: none;
  }
  .hint {
    color: var(--text-2);
  }
  kbd {
    font: inherit;
    font-size: 12px;
    padding: 2px 7px;
    border-radius: 6px;
    background: rgb(255 255 255 / 0.1);
    color: var(--text);
  }
  @media (max-width: 760px) {
    .top {
      grid-template-columns: 1fr;
    }
  }
</style>
