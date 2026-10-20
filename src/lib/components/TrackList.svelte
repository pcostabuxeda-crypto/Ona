<script lang="ts">
  /** Llista de cançons virtualitzada: només es pinten les files que es veuen. */
  import { onMount } from "svelte";
  import type { Track } from "$lib/api";
  import { fmtTime } from "$lib/api";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";
  import Cover from "./Cover.svelte";
  import Icon from "./Icon.svelte";

  let {
    tracks,
    context = "",
    scroller,
    showAlbum = true,
    showCover = true,
    header = true,
  }: { tracks: Track[]; context?: string; scroller?: HTMLElement; showAlbum?: boolean; showCover?: boolean; header?: boolean } = $props();

  const ROW = 56;
  const OVERSCAN = 8;
  let list: HTMLElement;
  let start = $state(0);
  let end = $state(40);

  function measure() {
    if (!list || !scroller) return;
    const top = list.getBoundingClientRect().top - scroller.getBoundingClientRect().top;
    const first = Math.floor(-top / ROW);
    const count = Math.ceil(scroller.clientHeight / ROW);
    start = Math.max(0, first - OVERSCAN);
    end = Math.min(tracks.length, first + count + OVERSCAN);
  }

  onMount(() => {
    const target = scroller;
    if (!target) return;
    let raf = 0;
    const onScroll = () => {
      cancelAnimationFrame(raf);
      raf = requestAnimationFrame(measure);
    };
    target.addEventListener("scroll", onScroll, { passive: true });
    const ro = new ResizeObserver(onScroll);
    ro.observe(target);
    measure();
    return () => {
      target.removeEventListener("scroll", onScroll);
      ro.disconnect();
    };
  });

  $effect(() => {
    void tracks.length;
    measure();
  });

  const visible = $derived(scroller ? tracks.slice(start, end) : tracks);
  const offset = $derived(scroller ? start : 0);

  function openArtist(e: MouseEvent, id: string) {
    e.preventDefault();
    e.stopPropagation();
    if (id) ui.go({ name: "collection", kind: "artist", id });
  }
  function openAlbum(e: MouseEvent, id: string) {
    e.preventDefault();
    e.stopPropagation();
    if (id) ui.go({ name: "collection", kind: "album", id });
  }
  function addToQueue(e: MouseEvent, t: Track) {
    e.stopPropagation();
    player.addToQueue(t);
    ui.say(`«${t.name}» afegida a la cua`);
  }

  function menu(e: MouseEvent, t: Track, n: number) {
    ui.openMenu(e, [
      { label: "Reprodueix", icon: "play", run: () => player.playList(tracks, n, context) },
      {
        label: "Reprodueix a continuació",
        icon: "playNextIcon",
        run: () => {
          player.playNext(t);
          ui.say(`«${t.name}» sonarà a continuació`);
        },
      },
      {
        label: "Afegeix a la cua",
        icon: "queueAdd",
        run: () => {
          player.addToQueue(t);
          ui.say(`«${t.name}» afegida a la cua`);
        },
      },
      ...(t.artists[0]?.id
        ? [{ label: "Ves a l'artista", icon: "artist", run: () => ui.go({ name: "collection", kind: "artist", id: t.artists[0].id }) }]
        : []),
      ...(t.album.id ? [{ label: "Ves a l'àlbum", icon: "album", run: () => ui.go({ name: "collection", kind: "album", id: t.album.id }) }] : []),
    ]);
  }
</script>

<div class="tl">
{#if header}
  <div class="row head" class:noalbum={!showAlbum}>
    <span class="idx">#</span>
    <span>Títol</span>
    {#if showAlbum}<span class="album">Àlbum</span>{/if}
    <span></span>
    <span class="dur"><Icon name="clock" size={16} /></span>
  </div>
{/if}
<div class="list" bind:this={list} style:height={scroller ? `${tracks.length * ROW}px` : null} class:virtual={!!scroller}>
  {#each visible as t, i (t.id + ":" + (offset + i))}
    {@const n = offset + i}
    {@const active = player.current?.id === t.id}
    <div
      class="row"
      class:active
      class:noalbum={!showAlbum}
      style:transform={scroller ? `translateY(${n * ROW}px)` : null}
      role="button"
      tabindex="0"
      onclick={() => player.playList(tracks, n, context)}
      onkeydown={(e) => e.key === "Enter" && player.playList(tracks, n, context)}
      oncontextmenu={(e) => menu(e, t, n)}
    >
      <span class="idx">
        {#if active && player.playing}
          <span class="eq"><i></i><i></i><i></i></span>
        {:else}
          <span class="num">{n + 1}</span>
          <span class="playicon"><Icon name="play" size={16} /></span>
        {/if}
      </span>
      <span class="main">
        {#if showCover}<Cover src={t.album.thumb} size={40} radius={4} />{/if}
        <span class="txt">
          <span class="name">{t.name}</span>
          <span class="artists">
            {#if t.explicit}<span class="e">E</span>{/if}
            {#each t.artists as a, j (a.id + j)}
              {#if j > 0},&nbsp;{/if}<a href="#artist" onclick={(e) => openArtist(e, a.id)}>{a.name}</a>
            {/each}
          </span>
        </span>
      </span>
      {#if showAlbum}
        <span class="album"><a href="#album" onclick={(e) => openAlbum(e, t.album.id)}>{t.album.name}</a></span>
      {/if}
      <span class="actions">
        <button title="Afegeix a la cua" onclick={(e) => addToQueue(e, t)}><Icon name="queueAdd" size={18} /></button>
      </span>
      <span class="dur">{fmtTime(t.duration_ms)}</span>
    </div>
  {/each}
</div>
</div>

<style>
  .tl {
    container-type: inline-size;
  }
  .list {
    position: relative;
  }
  .virtual .row {
    position: absolute;
    left: 0;
    right: 0;
    top: 0;
  }
  .row {
    height: 56px;
    display: grid;
    grid-template-columns: 16px minmax(0, 4fr) minmax(0, 3fr) 32px 56px;
    align-items: center;
    gap: 16px;
    padding: 0 16px;
    border-radius: 4px;
    cursor: default;
    transition: background 0.15s;
    contain: layout paint;
  }
  .row.noalbum {
    grid-template-columns: 16px minmax(0, 1fr) 32px 56px;
  }
  .row:not(.head):hover {
    background: rgb(255 255 255 / 0.1);
  }
  .head {
    height: 36px;
    margin-bottom: 16px;
    border-bottom: 1px solid rgb(255 255 255 / 0.1);
    border-radius: 0;
    font-size: 14px;
    color: var(--text-2);
  }
  .head .dur {
    display: flex;
    justify-content: flex-end;
  }
  .row.active .name {
    color: var(--accent);
  }
  .idx {
    display: grid;
    place-items: center;
    color: var(--text-2);
    font-variant-numeric: tabular-nums;
    font-size: 16px;
  }
  .playicon {
    display: none;
    color: var(--text);
  }
  .row:hover .num {
    display: none;
  }
  .row:hover .playicon {
    display: grid;
  }
  .main {
    display: flex;
    align-items: center;
    gap: 12px;
    min-width: 0;
  }
  .txt {
    display: flex;
    flex-direction: column;
    min-width: 0;
    gap: 2px;
  }
  .name,
  .artists,
  .album {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .name {
    font-size: 16px;
    color: var(--text);
  }
  .artists,
  .album {
    font-size: 14px;
    color: var(--text-2);
  }
  a {
    color: inherit;
    text-decoration: none;
  }
  a:hover {
    color: var(--text);
    text-decoration: underline;
  }
  .e {
    display: inline-grid;
    place-items: center;
    width: 15px;
    height: 15px;
    margin-right: 6px;
    border-radius: 2px;
    background: rgb(255 255 255 / 0.6);
    color: #000;
    font-size: 9px;
    font-weight: 700;
    vertical-align: 1px;
  }
  .actions button {
    color: var(--text-2);
    opacity: 0;
    transition: opacity 0.15s;
    display: grid;
    place-items: center;
  }
  .row:hover .actions button {
    opacity: 1;
  }
  .actions button:hover {
    color: var(--text);
  }
  .dur {
    text-align: right;
    font-size: 14px;
    color: var(--text-2);
    font-variant-numeric: tabular-nums;
  }
  .eq {
    display: flex;
    align-items: flex-end;
    gap: 2px;
    height: 14px;
  }
  .eq i {
    width: 3px;
    height: 100%;
    background: var(--accent);
    border-radius: 1px;
    transform-origin: bottom;
    animation: eq 0.9s ease-in-out infinite;
  }
  .eq i:nth-child(2) {
    animation-delay: -0.3s;
  }
  .eq i:nth-child(3) {
    animation-delay: -0.6s;
  }
  @keyframes eq {
    0%,
    100% {
      transform: scaleY(0.3);
    }
    50% {
      transform: scaleY(1);
    }
  }

  /* Quan la llista és estreta, fora la columna d'àlbum. */
  @container (max-width: 560px) {
    .row,
    .row.noalbum {
      grid-template-columns: 16px minmax(0, 1fr) 32px 44px;
      gap: 12px;
      padding: 0 8px;
    }
    .album {
      display: none;
    }
  }
</style>
