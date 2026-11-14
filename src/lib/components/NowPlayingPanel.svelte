<script lang="ts">
  /** Panell "Ara sona" de la dreta, com al client: portada, títol, artista i el següent de la cua. */
  import * as A from "$lib/api";
  import type { Collection } from "$lib/api";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";
  import Cover from "./Cover.svelte";
  import Icon from "./Icon.svelte";
  import Disc from "./Disc.svelte";
  import Visualizer from "./Visualizer.svelte";
  import { settings } from "$lib/settings.svelte";

  // Estètica Ona: el disc gira al mig i el visualitzador fa un anell de llum al voltant.
  const ona = $derived(settings.look === "ona");
  /** Estètica Ona sense mode lleuger: la portada sura, els títols arriben des del fons. */
  const space = $derived(ona && !settings.light);

  let artist = $state<Collection | null>(null);

  $effect(() => {
    const id = player.current?.artists[0]?.id;
    artist = null;
    if (id)
      A.loadCollection("artist", id)
        .then((d) => id === player.current?.artists[0]?.id && (artist = d.info))
        .catch(() => {});
  });

  const t = $derived(player.current!);
  const next = $derived(player.upcoming[0]);
  const goArtist = (id: string) => id && ui.go({ name: "collection", kind: "artist", id });
</script>

<aside class="panel" class:space>
  <header>
    <span class="ctx">{player.context || t.album.name}</span>
    <button class="x" onclick={() => (ui.panel = null)} title="Tanca" aria-label="Tanca el panell Ara sona"><Icon name="close" size={16} /></button>
  </header>

  <div class="scroll">
    <!-- Estètica Ona: la portada (o el disc) sura sobre una llum del color de la portada mentre sona. -->
    <div class="float" class:floating={space && player.playing}>
      {#if ona && settings.disc}
        <div class="orbit">
          {#if settings.viz && !settings.light}
            <div class="viz"><Visualizer style="circular" shape="bars" colorA={player.palette.a} colorB={player.palette.b} inner={0.36} /></div>
          {/if}
          <Disc src={t.album.image} playing={player.playing} size="72cqi" />
        </div>
      {:else}
        <Cover src={t.album.image} size="100%" radius={ona ? 18 : 8} tilt={ona} />
      {/if}
    </div>
    {#key t.id}
      <div class="title">
        <div class="txt">
          <h2>{t.name}</h2>
          <span class="muted">
            {#each t.artists as a, j (a.id + j)}
              {#if j > 0},&nbsp;{/if}<a href="#artist" onclick={(e) => (e.preventDefault(), goArtist(a.id))}>{a.name}</a>
            {/each}
          </span>
        </div>
      </div>
    {/key}

    {#if artist}
      <button class="card artist" onclick={() => goArtist(artist!.id)}>
        <div class="banner" style:background-image={artist.image ? `url(${artist.image})` : null}>
          <span>Informació sobre l'artista</span>
        </div>
        <div class="info">
          <b>{artist.name}</b>
          <span class="muted">{artist.subtitle}</span>
        </div>
      </button>
    {/if}

    {#if next}
      <div class="card">
        <div class="card-head">
          <b>Següent a la cua</b>
          <button class="link" onclick={() => (ui.panel = "queue")}>Obre la cua</button>
        </div>
        <button class="next" onclick={() => player.jumpTo(next.at)}>
          <Cover src={next.track.album.thumb} size={48} radius={4} />
          <span class="txt">
            <span class="n">{next.track.name}</span>
            <span class="muted">{next.track.artists.map((a) => a.name).join(", ")}</span>
          </span>
        </button>
      </div>
    {/if}
  </div>
</aside>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    border-radius: var(--radius);
    background: var(--base);
    overflow: hidden;
  }
  header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 16px;
  }
  .ctx {
    flex: 1;
    min-width: 0;
    font-weight: 700;
    font-size: 16px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .x {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    display: grid;
    place-items: center;
    color: var(--text-2);
  }
  .x:hover {
    background: var(--highlight);
    color: var(--text);
  }
  .scroll {
    flex: 1;
    overflow-y: auto;
    container-type: inline-size;
    padding: 0 16px 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
  .float {
    position: relative;
    flex: none;
  }
  .float > :global(.cover) {
    aspect-ratio: 1;
    height: auto !important;
  }

  /* ---------- Estètica Ona: espai ---------- */
  /* Llum del color de la portada sota la portada: degradat quiet (sense blur), només en surt l'opacitat. */
  .space .float::before {
    content: "";
    position: absolute;
    inset: 4% 8% -10%;
    border-radius: 50%;
    background: radial-gradient(closest-side, color-mix(in oklab, var(--pb, var(--accent)) 70%, transparent), transparent);
    opacity: 0.55;
    pointer-events: none;
    transition: opacity 1s ease-out;
  }
  .space .floating::before {
    opacity: 0.9;
  }
  /* Sura només mentre sona (en pausa, quieta): una sola animació, d'un sol element. */
  .floating > :global(*) {
    animation: float 7s ease-in-out infinite;
  }
  @keyframes float {
    50% {
      translate: 0 -7px;
    }
  }
  /* Títols que arriben des del fons en canviar de cançó. */
  .space .title h2,
  .space .title .muted {
    animation: from-depth var(--t-spring) var(--spring) both;
  }
  .space .title .muted {
    animation-delay: 70ms;
  }
  @keyframes from-depth {
    from {
      opacity: 0;
      scale: 0.82;
      translate: 0 -6px;
    }
  }
  .space .title h2,
  .space .title .muted {
    transform-origin: left center;
  }
  /* Vidre fi: vora d'una línia i un fil de llum a dalt, sense backdrop-filter. */
  :global(.ona) .card {
    background: rgb(255 255 255 / 0.035);
    box-shadow:
      inset 0 0 0 1px rgb(255 255 255 / 0.09),
      inset 0 1px 0 rgb(255 255 255 / 0.12);
  }
  :global(.ona) .next:hover {
    background: rgb(255 255 255 / 0.06);
  }
  @media (prefers-reduced-motion: reduce) {
    .floating > :global(*),
    .space .title h2,
    .space .title .muted {
      animation: none;
    }
  }
  .orbit {
    position: relative;
    width: 100cqi;
    height: 100cqi;
    display: grid;
    place-items: center;
    flex: none;
  }
  .viz {
    position: absolute;
    inset: 0;
  }
  .title h2 {
    font-size: 24px;
    line-height: 1.2;
  }
  .txt {
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  a:hover {
    text-decoration: underline;
    color: var(--text);
  }
  .card {
    border-radius: var(--radius);
    background: var(--highlight);
    overflow: hidden;
    text-align: left;
  }
  .artist {
    display: block;
    width: 100%;
  }
  .banner {
    aspect-ratio: 4 / 3;
    background: var(--elevated-hi) center / cover;
    padding: 16px;
    box-sizing: border-box;
  }
  .banner span {
    font-weight: 700;
    text-shadow: 0 1px 4px rgb(0 0 0 / 0.6);
  }
  .info {
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .info b {
    font-size: 16px;
  }
  .card-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 16px 8px;
  }
  .link {
    font-size: 14px;
    font-weight: 700;
    color: var(--text-2);
  }
  .link:hover {
    color: var(--text);
    text-decoration: underline;
  }
  .next {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 16px 16px;
    text-align: left;
  }
  .next:hover {
    background: var(--elevated-hi);
  }
  .n {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .next .muted {
    font-size: 14px;
  }
</style>
