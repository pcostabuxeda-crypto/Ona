<script lang="ts">
  import * as A from "$lib/api";
  import type { Collection, Loaded } from "$lib/api";
  import { paletteFrom, DEFAULT_PALETTE, type Palette } from "$lib/color";
  import Card from "$lib/components/Card.svelte";
  import Cover from "$lib/components/Cover.svelte";
  import Icon from "$lib/components/Icon.svelte";
  import TrackList from "$lib/components/TrackList.svelte";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";

  let { kind, id, scroller }: { kind: Collection["kind"]; id: string; scroller?: HTMLElement } = $props();

  let data = $state<Loaded | null>(null);
  let failed = $state(false);
  let albums = $state<Collection[]>([]);
  let pal = $state<Palette>(DEFAULT_PALETTE);

  $effect(() => {
    const k = kind,
      i = id;
    data = null;
    failed = false;
    albums = [];
    A.loadCollection(k, i)
      .then(async (d) => {
        data = d;
        pal = await paletteFrom(d.info.image);
      })
      .catch(() => (failed = true));
    if (k === "artist") A.artistAlbums(i).then((r) => (albums = r)).catch(() => {});
  });

  const totalMs = $derived(data?.tracks.reduce((s, t) => s + t.duration_ms, 0) ?? 0);
  const label = $derived({ playlist: "Playlist", album: "Àlbum", liked: "Playlist", artist: "Artista" }[kind]);
  const ctx = $derived(data?.info.name ?? "");

  function playAll(shuffle = false) {
    if (!data?.tracks.length) return;
    if (shuffle !== player.shuffle) player.toggleShuffle();
    const start = shuffle ? Math.floor(Math.random() * data.tracks.length) : 0;
    player.playList(data.tracks, start, ctx);
  }
</script>

<section class="col" style:--ca={pal.a} style:--cb={pal.b}>
  <div class="glow"></div>
  <header class="head">
    {#if kind === "liked"}
      <div class="liked"><Icon name="heart" size={72} /></div>
    {:else}
      <div class="art">
        <Cover src={data?.info.image} size={220} round={kind === "artist"} radius={14} />
      </div>
    {/if}
    <div class="meta">
      <span class="kind">{label}</span>
      <h1 class:long={(data?.info.name.length ?? 0) > 22}>{data?.info.name ?? " "}</h1>
      {#if data}
        <span class="sub">
          {data.info.subtitle}
          {#if data.tracks.length && kind !== "artist"}
            · {data.tracks.length} cançons · {Math.round(totalMs / 60000)} min
          {/if}
        </span>
      {/if}
    </div>
  </header>

  <div class="tools">
    <button class="play" onclick={() => playAll(false)} disabled={!data?.tracks.length} title="Reprodueix">
      <Icon name="play" size={28} />
    </button>
    <button class="ghost" class:on={player.shuffle} onclick={() => playAll(true)} title="Aleatori">
      <Icon name="shuffle" size={24} />
    </button>
  </div>

  <div class="page body">
    {#if failed}
      <p class="muted">No s'ha pogut carregar. Algunes playlists creades per Spotify no estan disponibles per l'API.</p>
    {:else if !data}
      {#each Array(8) as _, i (i)}<div class="skeleton" style:--d="{i * 60}ms"></div>{/each}
    {:else}
      {#if kind === "artist"}<h2>Populars</h2>{/if}
      <TrackList tracks={data.tracks} context={ctx} {scroller} showAlbum={kind !== "album"} showCover={kind !== "album"} />
      {#if albums.length}
        <h2>Discografia</h2>
        <div class="grid">{#each albums as a, i (a.id)}<Card item={a} index={i} />{/each}</div>
      {/if}
    {/if}
  </div>
</section>

<style>
  .col {
    position: relative;
    min-height: 100%;
  }
  .glow {
    position: absolute;
    inset: 0 0 auto 0;
    height: 420px;
    background:
      radial-gradient(60% 90% at 15% 0%, color-mix(in oklab, var(--ca) 55%, transparent), transparent 70%),
      radial-gradient(50% 80% at 70% 0%, color-mix(in oklab, var(--cb) 35%, transparent), transparent 70%);
    opacity: 0.9;
    pointer-events: none;
    transition: background 0.8s;
    mask-image: linear-gradient(to bottom, black 40%, transparent);
  }
  .head {
    position: relative;
    display: flex;
    align-items: flex-end;
    gap: 28px;
    padding: 36px 32px 24px;
  }
  .art {
    box-shadow: 0 24px 60px -18px rgb(0 0 0 / 0.8);
    border-radius: 14px;
    animation: pop 0.6s cubic-bezier(0.2, 0.8, 0.2, 1) both;
  }
  .liked {
    width: 220px;
    height: 220px;
    flex: none;
    border-radius: 14px;
    display: grid;
    place-items: center;
    color: white;
    background: linear-gradient(135deg, #4b2bd6, #c03aa6 60%, #f08a5d);
    box-shadow: 0 24px 60px -18px rgb(0 0 0 / 0.8);
    animation: pop 0.6s cubic-bezier(0.2, 0.8, 0.2, 1) both;
  }
  .meta {
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-width: 0;
    animation: rise 0.5s 0.08s cubic-bezier(0.2, 0.8, 0.2, 1) both;
  }
  .kind {
    font-size: 12px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: var(--text-2);
  }
  h1 {
    font-size: clamp(36px, 6vw, 76px);
    line-height: 1;
    margin: 0;
    letter-spacing: -0.04em;
    overflow-wrap: anywhere;
  }
  h1.long {
    font-size: clamp(30px, 4vw, 48px);
  }
  .sub {
    color: var(--text-2);
  }
  .tools {
    position: relative;
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 4px 32px 12px;
  }
  .play {
    width: 58px;
    height: 58px;
    border-radius: 50%;
    display: grid;
    place-items: center;
    background: var(--accent);
    color: #0b0b12;
    box-shadow: 0 12px 28px -10px var(--accent);
    transition: transform 0.2s cubic-bezier(0.2, 0.8, 0.2, 1);
  }
  .play:hover {
    transform: scale(1.06);
  }
  .play:active {
    transform: scale(0.96);
  }
  .ghost {
    color: var(--text-2);
  }
  .ghost:hover,
  .ghost.on {
    color: var(--text);
  }
  .ghost.on {
    color: var(--accent);
  }
  .body {
    position: relative;
    padding-top: 8px;
  }
  .muted {
    color: var(--text-2);
  }
  .skeleton {
    height: 46px;
    margin: 12px 12px 0;
    border-radius: 10px;
    background: linear-gradient(90deg, rgb(255 255 255 / 0.04), rgb(255 255 255 / 0.09), rgb(255 255 255 / 0.04));
    background-size: 200% 100%;
    animation:
      shimmer 1.4s linear infinite,
      rise 0.4s both;
    animation-delay: 0s, var(--d);
  }
  @keyframes shimmer {
    to {
      background-position: -200% 0;
    }
  }

  @media (max-width: 700px) {
    .head {
      gap: 18px;
      padding: 24px 18px 16px;
    }
    .art :global(.cover),
    .liked {
      width: 130px !important;
      height: 130px !important;
    }
    .tools {
      padding: 4px 18px 8px;
    }
  }
</style>
