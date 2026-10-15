<script lang="ts">
  import * as A from "$lib/api";
  import type { Collection, Loaded } from "$lib/api";
  import { paletteFrom } from "$lib/color";
  import Card from "$lib/components/Card.svelte";
  import Cover from "$lib/components/Cover.svelte";
  import Icon from "$lib/components/Icon.svelte";
  import TrackList from "$lib/components/TrackList.svelte";
  import { player } from "$lib/player.svelte";
  import { TILES } from "$lib/tiles";

  let { kind, id, scroller }: { kind: Collection["kind"]; id: string; scroller?: HTMLElement } = $props();

  let data = $state<Loaded | null>(null);
  let failed = $state(false);
  let albums = $state<Collection[]>([]);
  let tint = $state("#535353");

  $effect(() => {
    A.loadCollection(kind, id)
      .then(async (d) => {
        data = d;
        tint = TILES[kind]?.tint ?? (await paletteFrom(d.info.image)).a;
      })
      .catch(() => (failed = true));
    if (kind === "artist") A.artistAlbums(id).then((r) => (albums = r)).catch(() => {});
  });

  const totalMs = $derived(data?.tracks.reduce((s, t) => s + t.duration_ms, 0) ?? 0);
  const label = $derived(({ playlist: "Playlist", album: "Àlbum", liked: "Playlist", artist: "Artista verificat", audius: "Audius", local: "Al teu ordinador" } as const)[kind]);
  const ctx = $derived(data?.info.name ?? "");
  const playingHere = $derived(player.playing && player.context === ctx && !!ctx);

  function duration(ms: number) {
    const m = Math.round(ms / 60000);
    return m >= 60 ? `${Math.floor(m / 60)} h ${m % 60} min` : `${m} min`;
  }

  function playAll() {
    if (!data?.tracks.length) return;
    if (player.context === ctx && player.current) return player.toggle();
    player.playList(data.tracks, 0, ctx);
  }
  function shuffleAll() {
    if (!data?.tracks.length) return;
    if (!player.shuffle) player.toggleShuffle();
    player.playList(data.tracks, Math.floor(Math.random() * data.tracks.length), ctx);
  }
</script>

<section class="coll" style:--tint={tint}>
  {#if failed}
    <p class="page muted">No s'ha pogut carregar.</p>
  {:else if data}
    {#if kind === "artist"}
      <header class="banner" style:background-image={data.info.image ? `url(${data.info.image})` : null}>
        <div class="banner-txt">
          <span class="verified"><Icon name="check" size={20} /> {label}</span>
          <h1>{data.info.name}</h1>
          <span>{data.info.subtitle}</span>
        </div>
      </header>
    {:else}
      <header class="head">
        {#if TILES[kind]}
          <div class="liked" style:background={TILES[kind]?.bg}><Icon name={TILES[kind]?.icon ?? ""} size={80} /></div>
        {:else}
          <Cover src={data.info.image} size={232} radius={4} />
        {/if}
        <div class="meta">
          <span class="label">{label}</span>
          <h1 class:long={data.info.name.length > 24}>{data.info.name}</h1>
          <span class="by">
            <b>{data.info.subtitle.replace(/^(Playlist|Àlbum) · /, "")}</b>
            · {data.tracks.length} cançons, {duration(totalMs)}
          </span>
        </div>
      </header>
    {/if}

    <div class="body">
      <div class="actions">
        <button class="play-fab big" onclick={playAll} title={playingHere ? "Pausa" : "Reprodueix"}>
          <Icon name={playingHere ? "pause" : "play"} size={28} />
        </button>
        <button class="act" class:on={player.shuffle} onclick={shuffleAll} title="Reprodueix en ordre aleatori"><Icon name="shuffle" size={32} /></button>
      </div>

      {#if kind === "artist"}
        <h2 class="sub">Populars</h2>
        <TrackList tracks={data.tracks.slice(0, 10)} context={ctx} showAlbum={false} header={false} />
        {#if albums.length}
          <div class="shelf-title"><h2>Discografia</h2></div>
          <div class="shelf">
            {#each albums as a (a.id)}<Card item={a} />{/each}
          </div>
        {/if}
      {:else}
        <TrackList tracks={data.tracks} context={ctx} {scroller} showAlbum={kind !== "album"} showCover={kind !== "album"} />
      {/if}
    </div>
  {/if}
</section>

<style>
  .coll {
    min-height: 100%;
    background: linear-gradient(var(--tint), color-mix(in srgb, var(--tint) 30%, var(--base)) 340px, var(--base) 560px);
  }
  .head {
    display: flex;
    align-items: flex-end;
    gap: 24px;
    padding: 64px 24px 24px;
  }
  .head :global(.cover) {
    box-shadow: 0 4px 60px rgb(0 0 0 / 0.5);
  }
  .liked {
    width: 232px;
    height: 232px;
    flex: none;
    border-radius: 4px;
    display: grid;
    place-items: center;
    box-shadow: 0 4px 60px rgb(0 0 0 / 0.5);
  }
  .meta {
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .label {
    font-size: 14px;
  }
  h1 {
    font-size: clamp(32px, 6vw, 96px);
    line-height: 1;
    letter-spacing: -0.04em;
    overflow-wrap: anywhere;
  }
  h1.long {
    font-size: clamp(28px, 4vw, 56px);
  }
  .by {
    font-size: 14px;
    color: rgb(255 255 255 / 0.7);
  }
  .by b {
    color: #fff;
  }
  .banner {
    height: 40vh;
    min-height: 240px;
    max-height: 400px;
    background: var(--tint) center 20% / cover;
    display: flex;
    align-items: flex-end;
  }
  .banner-txt {
    width: 100%;
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    background: linear-gradient(transparent, rgb(0 0 0 / 0.5));
  }
  .verified {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .verified :global(svg) {
    color: #4cb3ff;
  }
  .body {
    background: linear-gradient(rgb(0 0 0 / 0.2), var(--base) 240px);
    padding: 0 24px 32px;
  }
  .actions {
    display: flex;
    align-items: center;
    gap: 32px;
    padding: 24px 0;
  }
  .play-fab.big {
    width: 56px;
    height: 56px;
  }
  .act {
    color: var(--text-2);
    transition: transform 0.1s;
  }
  .act:hover {
    color: var(--text);
    transform: scale(1.04);
  }
  .act.on {
    color: var(--accent);
  }
  .sub {
    font-size: 24px;
    margin-bottom: 16px;
  }
</style>
