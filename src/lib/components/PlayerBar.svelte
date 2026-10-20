<script lang="ts">
  import * as A from "$lib/api";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";
  import Cover from "./Cover.svelte";
  import Icon from "./Icon.svelte";
  import Slider from "./Slider.svelte";
  import { mini, toggleMini } from "$lib/mini.svelte";

  let liked = $state(false);
  let lastVol = 0.7;

  $effect(() => {
    const t = player.current;
    liked = false;
    if (t && !t.src) A.isLiked([t.id]).then(([v]) => t === player.current && (liked = v)).catch(() => {});
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
  const repeatLabel = $derived({ off: "Repeteix: no", all: "Repeteix: tot", one: "Repeteix: aquesta cançó" }[player.repeat]);

  /** Pantalla estreta: les accions secundàries van al menú ⋯. */
  function more(e: MouseEvent) {
    ui.openMenu(e, [
      { label: player.shuffle ? "Desactiva l'ordre aleatori" : "Ordre aleatori", icon: "shuffle", run: () => player.toggleShuffle() },
      { label: repeatLabel, icon: "repeat", run: () => player.cycleRepeat() },
      ...(t ? [{ label: "Lletra", icon: "lyrics", run: () => ui.toggleLyrics() }] : []),
      ...(t && !A.MOBILE ? [{ label: "Lletra flotant", icon: "note", run: () => toggleMini() }] : []),
      { label: player.upNext.length ? `Cua (${player.upNext.length})` : "Cua", icon: "queue", run: () => ui.togglePanel("queue") },
      { label: "Ara sona", icon: "npv", run: () => ui.togglePanel("npv") },
      { label: player.volume ? "Silencia" : "Activa el so", icon: player.volume ? "mute" : "volume", run: toggleMute },
    ]);
  }
</script>

<footer class="bar">
  <div class="now">
    {#if t}
      <button class="cover" onclick={() => ui.togglePanel("npv")} title="Ara sona" aria-label="Ara sona: {t.name}" class:dim={!!player.error}>
        <Cover src={t.album.thumb} size={56} radius={4} />
      </button>
      <div class="txt">
        <a class="name" href="#album" onclick={(e) => (e.preventDefault(), t.album.id && ui.go({ name: "collection", kind: "album", id: t.album.id }))}>{t.name}</a>
        <!-- Regió viva permanent: l'error de reproducció s'anuncia aquí mateix. -->
        <span class="artists" aria-live="polite">
          {#if player.error}
            <span class="error">{player.error}</span>
            <button class="link" onclick={() => player.retry()}>Torna-ho a provar</button>
            <button class="link" onclick={() => player.next()}>Salta</button>
          {:else}
            {#each t.artists as a, j (a.id + j)}
              {#if j > 0},&nbsp;{/if}<a href="#artist" onclick={(e) => (e.preventDefault(), a.id && ui.go({ name: "collection", kind: "artist", id: a.id }))}>{a.name}</a>
            {/each}
          {/if}
        </span>
      </div>
      {#if !t.src}<button
          class="ic like"
          class:on={liked}
          onclick={toggleLike}
          aria-pressed={liked}
          title={liked ? "Treu de Cançons que m'agraden (Ctrl S)" : "Desa a Cançons que m'agraden (Ctrl S)"}
        >
          <Icon name={liked ? "check" : "addCircle"} size={16} />
        </button>{/if}
    {/if}
  </div>

  <div class="mid">
    <div class="controls">
      <button class="ic wide" class:on={player.shuffle} aria-pressed={player.shuffle} onclick={() => player.toggleShuffle()} title="Ordre aleatori (Ctrl Maj S)"><Icon name="shuffle" size={16} /></button>
      <button class="ic" onclick={() => player.prev()} title="Anterior (Ctrl ←)"><Icon name="prev" size={16} /></button>
      <button class="play" class:loading={player.loading} onclick={() => player.toggle()} title={player.playing ? "Pausa (espai)" : "Reprodueix (espai)"} disabled={!t}>
        <Icon name={player.playing ? "pause" : "play"} size={16} />
      </button>
      <button class="ic" onclick={() => player.next()} title="Següent (Ctrl →)"><Icon name="next" size={16} /></button>
      <button class="ic wide" class:on={player.repeat !== "off"} aria-pressed={player.repeat !== "off"} aria-label={repeatLabel} onclick={() => player.cycleRepeat()} title="{repeatLabel} (Ctrl R)">
        <Icon name="repeat" size={16} />{#if player.repeat === "one"}<i class="one">1</i>{/if}
      </button>
      <button class="ic narrow-only" data-queue-btn onclick={more} title="Més opcions" aria-label="Més opcions" aria-haspopup="menu">
        <Icon name="more" size={18} />{#if player.upNext.length}<b class="badge">{player.upNext.length}</b>{/if}
      </button>
    </div>
    <div class="time">
      <span>{A.fmtTime(player.positionMs)}</span>
      <Slider value={player.progress} onchange={(v) => t && player.seekTo(v * t.duration_ms)} label="Progrés" valueText="{A.fmtTime(player.positionMs)} de {A.fmtTime(t?.duration_ms ?? 0)}" />
      <span>{A.fmtTime(t?.duration_ms ?? 0)}</span>
    </div>
  </div>

  <div class="right">
    <button class="ic" class:on={ui.view.name === "lyrics"} aria-pressed={ui.view.name === "lyrics"} onclick={() => ui.toggleLyrics()} title="Lletra (Ctrl L)" disabled={!t}><Icon name="lyrics" size={16} /></button>
    {#if !A.MOBILE}<button class="ic" class:on={mini.open} aria-pressed={mini.open} onclick={() => toggleMini()} title="Lletra flotant (Ctrl Maj L)" disabled={!t}><Icon name="note" size={16} /></button>{/if}
    <button class="ic" class:on={ui.panel === "queue"} aria-pressed={ui.panel === "queue"} data-queue-btn onclick={() => ui.togglePanel("queue")} title="Cua (Ctrl Q)">
      <Icon name="queue" size={16} />{#if player.upNext.length}<b class="badge">{player.upNext.length}</b>{/if}
    </button>
    <button class="ic" class:on={ui.panel === "npv"} aria-pressed={ui.panel === "npv"} onclick={() => ui.togglePanel("npv")} title="Vista Ara sona"><Icon name="npv" size={16} /></button>
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
    position: relative;
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
  .badge {
    position: absolute;
    top: 0;
    right: -2px;
    min-width: 15px;
    height: 15px;
    padding: 0 4px;
    box-sizing: border-box;
    border-radius: 8px;
    background: var(--accent);
    color: #000;
    font-size: 10px;
    font-weight: 700;
    line-height: 15px;
    text-align: center;
  }
  .narrow-only {
    display: none;
  }
  .cover {
    transition: opacity var(--t-quick);
  }
  .cover.dim {
    opacity: 0.5;
  }
  .error {
    display: inline-block;
    color: var(--error);
    animation: err-in var(--t-spring) var(--spring-soft) both;
  }
  @keyframes err-in {
    from {
      opacity: 0;
      transform: translateY(4px);
    }
  }
  .link {
    margin-left: 8px;
    font-size: 12px;
    font-weight: 700;
    color: var(--text);
    text-decoration: underline;
  }
  /* Mentre carrega: un anell que gira al voltant del play (només en aquest estat). */
  .play.loading::before {
    content: "";
    position: absolute;
    inset: -4px;
    border-radius: 50%;
    border: 2px solid transparent;
    border-top-color: var(--accent);
    animation: spin 0.8s linear infinite;
  }
  @keyframes spin {
    to {
      transform: rotate(1turn);
    }
  }
  @media (prefers-reduced-motion: reduce) {
    .play.loading::before {
      animation: none;
      border-color: var(--accent);
      opacity: 0.5;
    }
  }

  /* Pantalla estreta (608 px): dues files; la barra de temps a tota l'amplada i la resta al menú ⋯. */
  @media (max-width: 720px) {
    .bar {
      height: auto;
      grid-template-columns: minmax(0, 1fr) auto;
      grid-template-rows: auto auto;
      row-gap: 2px;
      padding: 6px 12px 8px;
    }
    .mid {
      display: contents;
    }
    .controls {
      grid-column: 2;
      grid-row: 1;
    }
    .time {
      grid-column: 1 / -1;
      grid-row: 2;
      max-width: none;
    }
    .right,
    .wide {
      display: none;
    }
    .narrow-only {
      display: grid;
    }
    .play {
      margin: 0 4px;
    }
  }
  .vol {
    width: 93px;
    flex: 0 1 93px;
  }
</style>
