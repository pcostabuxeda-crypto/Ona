<script lang="ts">
  import * as A from "$lib/api";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";
  import Cover from "./Cover.svelte";
  import Icon from "./Icon.svelte";
  import PlayButton from "./PlayButton.svelte";
  import { perf } from "$lib/perf.svelte";
  import Progress from "./Progress.svelte";

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
      ui.say(liked ? "Afegida a M'agrada" : "Treta de M'agrada");
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

  function wheelVolume(e: WheelEvent) {
    e.preventDefault();
    player.setVolume(player.volume - Math.sign(e.deltaY) * 0.05);
  }
</script>

<footer class="player" class:empty={!player.current}>
  <div class="now">
    {#if player.current}
      {@const t = player.current}
      <button class="art" onclick={() => ui.setNowPlaying(!ui.nowPlaying)} title="Pantalla de reproducció">
        <span data-mini-cover><Cover src={t.album.thumb} size={56} radius={10} /></span>
        <span class="expand"><Icon name={ui.nowPlaying ? "down" : "lyrics"} size={18} /></span>
      </button>
      <div class="txt">
        {#key t.id}
          <span class="name">{t.name}</span>
          <span class="artist">
            {#each t.artists as a, j (a.id + j)}
              {#if j > 0},&nbsp;{/if}<a href="#artist" onclick={() => ui.go({ name: "collection", kind: "artist", id: a.id })}>{a.name}</a>
            {/each}
          </span>
        {/key}
      </div>
      <button class="icon like" class:on={liked} onclick={toggleLike} title="M'agrada">
        <Icon name={liked ? "heart" : "heartOutline"} size={20} />
      </button>
    {/if}
  </div>

  <div class="center">
    <div class="controls">
      <button class="icon small" class:on={player.shuffle} onclick={() => player.toggleShuffle()} title="Aleatori">
        <Icon name="shuffle" size={18} />
      </button>
      <button class="icon" onclick={() => player.prev()} title="Anterior"><Icon name="prev" size={22} /></button>
      <PlayButton size={42} />
      <button class="icon" onclick={() => player.next()} title="Següent"><Icon name="next" size={22} /></button>
      <button class="icon small rep" class:on={player.repeat !== "off"} onclick={() => player.cycleRepeat()} title="Repeteix">
        <Icon name="repeat" size={18} />
        {#if player.repeat === "one"}<b>1</b>{/if}
      </button>
    </div>
    <div class="time">
      <span>{A.fmtTime(player.positionMs)}</span>
      <Progress
        value={player.progress}
        smooth={player.playing}
        wavy={perf.wave}
        playing={player.playing}
        onseek={(v) => player.current && player.seekTo(v * player.current.duration_ms)}
        label={(v) => (player.current ? A.fmtTime(v * player.current.duration_ms) : "")}
      />
      <span>{A.fmtTime(player.current?.duration_ms ?? 0)}</span>
    </div>
  </div>

  <div class="right">
    <button class="icon" class:on={ui.nowPlaying} onclick={() => ui.setNowPlaying(!ui.nowPlaying)} title="Lletra">
      <Icon name="lyrics" size={20} />
    </button>
    <button class="icon" class:on={ui.queueOpen} onclick={() => (ui.queueOpen = !ui.queueOpen)} title="Cua">
      <Icon name="queue" size={20} />
    </button>
    <div class="vol" onwheel={wheelVolume}>
      <button class="icon" onclick={toggleMute} title="Volum"><Icon name={player.volume === 0 ? "mute" : "volume"} size={20} /></button>
      <Progress value={player.volume} onseek={(v) => player.setVolume(v)} label={(v) => `${Math.round(v * 100)}%`} />
    </div>
  </div>
</footer>

<style>
  .player {
    position: relative;
    z-index: 30;
    margin: 0 10px 10px;
    height: 84px;
    display: grid;
    grid-template-columns: minmax(200px, 1fr) minmax(360px, 1.6fr) minmax(200px, 1fr);
    align-items: center;
    gap: 20px;
    padding: 0 18px 0 14px;
    border-radius: 22px;
    background: var(--panel-solid);
    border: 1px solid rgb(255 255 255 / 0.07);
    box-shadow: 0 20px 50px -20px rgb(0 0 0 / 0.8);
  }
  .player.empty .center,
  .player.empty .right {
    opacity: 0.4;
    pointer-events: none;
  }
  .now {
    display: flex;
    align-items: center;
    gap: 14px;
    min-width: 0;
  }
  .art {
    position: relative;
    border-radius: 10px;
    box-shadow: 0 8px 20px -8px rgb(0 0 0 / 0.8);
  }
  .expand {
    position: absolute;
    inset: 0;
    display: grid;
    place-items: center;
    background: rgb(0 0 0 / 0.5);
    border-radius: 10px;
    color: white;
    opacity: 0;
    transition: opacity 0.2s;
  }
  .art:hover .expand {
    opacity: 1;
  }
  .txt {
    display: flex;
    flex-direction: column;
    min-width: 0;
    gap: 3px;
  }
  .name {
    font-weight: 600;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    animation: rise 0.4s cubic-bezier(0.2, 0.8, 0.2, 1) both;
  }
  .artist {
    font-size: 13px;
    color: var(--text-2);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    animation: rise 0.4s 0.05s cubic-bezier(0.2, 0.8, 0.2, 1) both;
  }
  .artist a {
    color: inherit;
    text-decoration: none;
  }
  .artist a:hover {
    color: var(--text);
    text-decoration: underline;
  }
  .icon {
    position: relative;
    display: grid;
    place-items: center;
    width: 36px;
    height: 36px;
    border-radius: 50%;
    color: var(--text-2);
    transition:
      color 0.15s,
      transform 0.15s;
  }
  .icon:hover {
    color: var(--text);
  }
  .icon:active {
    transform: scale(0.9);
  }
  .icon.on {
    color: var(--accent);
  }
  .icon.on::after {
    content: "";
    position: absolute;
    bottom: 2px;
    width: 4px;
    height: 4px;
    border-radius: 50%;
    background: currentColor;
  }
  .like.on::after {
    display: none;
  }
  .like.on {
    animation: beat 0.4s;
  }
  @keyframes beat {
    40% {
      transform: scale(1.3);
    }
  }
  .rep b {
    position: absolute;
    top: 5px;
    right: 5px;
    font-size: 9px;
  }
  .center {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .controls {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
  }
  .time {
    display: grid;
    grid-template-columns: 42px 1fr 42px;
    align-items: center;
    gap: 10px;
    font-size: 11px;
    color: var(--text-3);
    font-variant-numeric: tabular-nums;
  }
  .time span:first-child {
    text-align: right;
  }
  .right {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 4px;
  }
  .vol {
    display: grid;
    grid-template-columns: 36px 110px;
    align-items: center;
  }

  /* Finestra estreta: dues files (cançó + botons a dalt, controls i temps a baix). */
  @media (max-width: 1000px) {
    .player {
      height: auto;
      grid-template-columns: minmax(0, 1fr) auto;
      grid-template-areas:
        "now right"
        "center center";
      row-gap: 6px;
      column-gap: 10px;
      padding: 10px 14px 8px;
    }
    .now {
      grid-area: now;
    }
    .right {
      grid-area: right;
    }
    .center {
      grid-area: center;
    }
    .vol {
      grid-template-columns: 36px 80px;
    }
  }
  @media (max-width: 560px) {
    .vol {
      grid-template-columns: 36px;
    }
    .vol :global(.bar) {
      display: none;
    }
  }
</style>
