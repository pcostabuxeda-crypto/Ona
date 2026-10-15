<script lang="ts">
  import * as A from "$lib/api";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";
  import Cover from "./Cover.svelte";
  import Icon from "./Icon.svelte";
  import Slider from "./Slider.svelte";

  let liked = $state(false);
  let lastVol = 0.7;

  $effect(() => {
    const t = player.current;
    liked = false;
    if (t) A.isLiked([t.id]).then(([v]) => t === player.current && (liked = v)).catch(() => {});
  });

  async function toggleLike() {
    const t = player.current;
    if (!t) return;
    liked = !liked;
    try {
      await A.setLiked(t.id, liked);
      ui.say(liked ? "S'ha afegit a Cançons que m'agraden" : "S'ha tret de Cançons que m'agraden");
    } catch {
      liked = !liked;
      ui.say("No s'ha pogut desar");
    }
  }

  function toggleMute() {
    if (player.volume > 0) {
      lastVol = player.volume;
      player.setVolume(0);
    } else player.setVolume(lastVol || 0.7);
  }

  const t = $derived(player.current);
</script>

<footer class="bar">
  <div class="now">
    {#if t}
      <button class="cover" onclick={() => ui.togglePanel("npv")} title="Ara sona"><Cover src={t.album.thumb} size={56} radius={4} /></button>
      <div class="txt">
        <a class="name" href="#album" onclick={(e) => (e.preventDefault(), ui.go({ name: "collection", kind: "album", id: t.album.id }))}>{t.name}</a>
        <span class="artists">
          {#each t.artists as a, j (a.id + j)}
            {#if j > 0},&nbsp;{/if}<a href="#artist" onclick={(e) => (e.preventDefault(), ui.go({ name: "collection", kind: "artist", id: a.id }))}>{a.name}</a>
          {/each}
        </span>
      </div>
      <button class="ic like" class:on={liked} onclick={toggleLike} title={liked ? "Treu de Cançons que m'agraden" : "Desa a Cançons que m'agraden"}>
        <Icon name={liked ? "check" : "addCircle"} size={16} />
      </button>
    {/if}
  </div>

  <div class="mid">
    <div class="controls">
      <button class="ic" class:on={player.shuffle} onclick={() => player.toggleShuffle()} title="Ordre aleatori"><Icon name="shuffle" size={16} /></button>
      <button class="ic" onclick={() => player.prev()} title="Anterior"><Icon name="prev" size={16} /></button>
      <button class="play" onclick={() => player.toggle()} title={player.playing ? "Pausa" : "Reprodueix"} disabled={!t}>
        <Icon name={player.playing ? "pause" : "play"} size={16} />
      </button>
      <button class="ic" onclick={() => player.next()} title="Següent"><Icon name="next" size={16} /></button>
      <button class="ic" class:on={player.repeat !== "off"} onclick={() => player.cycleRepeat()} title="Repeteix">
        <Icon name="repeat" size={16} />{#if player.repeat === "one"}<i class="one">1</i>{/if}
      </button>
    </div>
    <div class="time">
      <span>{A.fmtTime(player.positionMs)}</span>
      <Slider value={player.progress} onchange={(v) => t && player.seekTo(v * t.duration_ms)} label="Progrés" />
      <span>{A.fmtTime(t?.duration_ms ?? 0)}</span>
    </div>
  </div>

  <div class="right">
    <button class="ic" class:on={ui.view.name === "lyrics"} onclick={() => ui.toggleLyrics()} title="Lletra" disabled={!t}><Icon name="lyrics" size={16} /></button>
    <button class="ic" class:on={ui.panel === "queue"} onclick={() => ui.togglePanel("queue")} title="Cua"><Icon name="queue" size={16} /></button>
    <button class="ic" class:on={ui.panel === "npv"} onclick={() => ui.togglePanel("npv")} title="Vista Ara sona"><Icon name="npv" size={16} /></button>
    <button class="ic" onclick={toggleMute} title={player.volume ? "Silencia" : "Activa el so"}><Icon name={player.volume ? "volume" : "mute"} size={16} /></button>
    <div class="vol"><Slider value={player.volume} onchange={(v) => player.setVolume(v)} label="Volum" /></div>
  </div>
</footer>

<style>
  .bar {
    height: 72px;
    flex: none;
    display: grid;
    grid-template-columns: minmax(180px, 3fr) minmax(0, 4fr) minmax(180px, 3fr);
    align-items: center;
    gap: 8px;
    padding: 0 16px;
  }
  .now {
    display: flex;
    align-items: center;
    gap: 12px;
    min-width: 0;
  }
  .cover {
    flex: none;
  }
  .txt {
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .name,
  .artists {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .name {
    font-size: 14px;
  }
  .artists {
    font-size: 12px;
    color: var(--text-2);
  }
  a:hover {
    text-decoration: underline;
    color: var(--text);
  }
  .ic {
    position: relative;
    width: 32px;
    height: 32px;
    flex: none;
    display: grid;
    place-items: center;
    color: var(--text-2);
  }
  .ic:hover:not(:disabled) {
    color: var(--text);
  }
  .ic.on {
    color: var(--accent);
  }
  .ic.on::after {
    content: "";
    position: absolute;
    bottom: 0;
    width: 4px;
    height: 4px;
    border-radius: 50%;
    background: var(--accent);
  }
  .like.on::after {
    display: none;
  }
  .ic:disabled {
    opacity: 0.4;
    cursor: default;
  }
  .one {
    position: absolute;
    top: 4px;
    right: 3px;
    font-size: 8px;
    font-style: normal;
    font-weight: 700;
  }
  .mid {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }
  .controls {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .play {
    width: 32px;
    height: 32px;
    margin: 0 8px;
    border-radius: 50%;
    display: grid;
    place-items: center;
    background: #fff;
    color: #000;
    transition: transform 0.1s;
  }
  .play:hover:not(:disabled) {
    transform: scale(1.06);
  }
  .time {
    width: 100%;
    max-width: 722px;
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: var(--text-2);
    font-variant-numeric: tabular-nums;
  }
  .time span {
    min-width: 40px;
    text-align: right;
  }
  .time span:last-child {
    text-align: left;
  }
  .right {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 4px;
  }
  .vol {
    width: 93px;
    flex: 0 1 93px;
  }
</style>
