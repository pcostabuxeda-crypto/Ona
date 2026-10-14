<script lang="ts">
  /** Cua editable: arrossegar per la nansa per reordenar, ✕ per treure, clic per saltar-hi. */
  import { flip } from "svelte/animate";
  import { fly } from "svelte/transition";
  import { fmtTime } from "$lib/api";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";
  import Cover from "./Cover.svelte";
  import Icon from "./Icon.svelte";

  const H = 58; // alçada de cada fila (px)

  let drag = $state<{ from: number; startY: number; dy: number; pointer: number } | null>(null);
  /** Durant un fotograma després de deixar anar, sense transicions (evita el salt). */
  let settling = $state(false);

  const n = $derived(player.upcoming.length);
  const over = $derived(drag ? Math.max(0, Math.min(n - 1, drag.from + Math.round(drag.dy / H))) : -1);

  function shift(j: number) {
    if (!drag) return 0;
    if (j === drag.from) return drag.dy;
    if (drag.from < j && j <= over) return -H;
    if (over <= j && j < drag.from) return H;
    return 0;
  }

  function start(e: PointerEvent, j: number) {
    e.preventDefault();
    e.stopPropagation();
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    drag = { from: j, startY: e.clientY, dy: 0, pointer: e.pointerId };
  }
  function moveDrag(e: PointerEvent) {
    if (drag && e.pointerId === drag.pointer) drag.dy = e.clientY - drag.startY;
  }
  function end(e: PointerEvent) {
    if (!drag || e.pointerId !== drag.pointer) return;
    const from = player.pos + 1 + drag.from;
    const to = player.pos + 1 + over;
    settling = true;
    drag = null;
    player.move(from, to);
    requestAnimationFrame(() => requestAnimationFrame(() => (settling = false)));
  }
</script>

<aside class="queue">
  <header>
    <h3>Cua</h3>
    <div class="hactions">
      {#if n}
        <button class="clear" onclick={() => player.clearUpcoming()}>Buida</button>
      {/if}
      <button class="x" onclick={() => (ui.queueOpen = false)} title="Tanca"><Icon name="close" size={18} /></button>
    </div>
  </header>

  {#if player.current}
    <span class="label">Ara sona</span>
    <div class="item now">
      <Cover src={player.current.album.thumb} size={44} radius={8} />
      <span class="txt">
        <b>{player.current.name}</b>
        <small>{player.current.artists.map((a) => a.name).join(", ")}</small>
      </span>
    </div>
  {/if}

  {#if n}
    <span class="label">
      Següents{player.context ? ` · ${player.context}` : ""}
      <em>{player.upcomingTotal}</em>
    </span>
    <div class="list" class:dragging={!!drag} class:settling>
      {#each player.upcoming as u, j (u.key)}
        <div
          class="row"
          class:lifted={drag?.from === j}
          style:transform="translateY({shift(j)}px)"
          animate:flip={{ duration: settling ? 0 : 300 }}
          out:fly={{ x: 40, duration: 220 }}
        >
          <span
            class="handle"
            role="button"
            tabindex="-1"
            aria-label="Arrossega per moure"
            onpointerdown={(e) => start(e, j)}
            onpointermove={moveDrag}
            onpointerup={end}
            onpointercancel={end}
          >
            <Icon name="grip" size={16} />
          </span>
          <button class="item" onclick={() => !drag && player.jumpTo(u.at)}>
            <Cover src={u.track.album.thumb} size={44} radius={8} />
            <span class="txt">
              <b>{u.track.name}</b>
              <small>{u.track.artists.map((a) => a.name).join(", ")}</small>
            </span>
            <small class="dur">{fmtTime(u.track.duration_ms)}</small>
          </button>
          <button class="remove" onclick={() => player.removeAt(u.at)} title="Treu de la cua">
            <Icon name="close" size={16} />
          </button>
        </div>
      {/each}
    </div>
    {#if player.upcomingTotal > n}
      <p class="more">… i {player.upcomingTotal - n} més</p>
    {/if}
  {:else}
    <p class="empty">No hi ha res més a la cua. Afegeix-hi cançons amb el clic dret o el botó <Icon name="queueAdd" size={14} />.</p>
  {/if}
</aside>

<style>
  .queue {
    width: 340px;
    flex: none;
    margin: 0 10px 0 0;
    padding: 16px 10px;
    border-radius: 22px;
    background: var(--panel);
    border: 1px solid var(--panel-line);
    overflow-y: auto;
    overflow-x: hidden;
    animation: slide 0.45s var(--spring-soft) both;
  }
  @keyframes slide {
    from {
      opacity: 0;
      transform: translateX(28px);
    }
  }
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 8px 8px;
  }
  .hactions {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .clear {
    padding: 5px 12px;
    border-radius: 99px;
    font-size: 12px;
    font-weight: 600;
    color: var(--text-2);
    background: rgb(255 255 255 / 0.07);
    transition:
      background 0.2s,
      color 0.2s;
  }
  .clear:hover {
    color: var(--text);
    background: rgb(255 255 255 / 0.13);
  }
  .x {
    color: var(--text-2);
    display: grid;
  }
  .x:hover {
    color: var(--text);
  }
  h3 {
    margin: 0;
    font-size: 18px;
  }
  .label {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 14px 8px 6px;
    font-size: 12px;
    font-weight: 600;
    color: var(--text-2);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .label em {
    font-style: normal;
    margin-left: auto;
    color: var(--text-3);
  }
  .list {
    position: relative;
  }
  .row {
    position: relative;
    height: 58px;
    display: grid;
    grid-template-columns: 22px 1fr 30px;
    align-items: center;
    border-radius: 12px;
    transition:
      transform 0.28s var(--spring-soft),
      background 0.15s,
      box-shadow 0.2s;
  }
  .settling .row {
    transition: none;
  }
  .row:hover {
    background: rgb(255 255 255 / 0.05);
  }
  .row.lifted {
    z-index: 5;
    transition: none;
    background: #24212f;
    box-shadow: 0 14px 30px -10px rgb(0 0 0 / 0.8);
    scale: 1.03;
  }
  .dragging .row:not(.lifted) {
    pointer-events: none;
  }
  .handle {
    display: grid;
    place-items: center;
    height: 100%;
    color: var(--text-3);
    cursor: grab;
    opacity: 0;
    touch-action: none;
    transition: opacity 0.15s;
  }
  .row:hover .handle,
  .lifted .handle {
    opacity: 1;
  }
  .lifted .handle {
    cursor: grabbing;
    color: var(--text);
  }
  .item {
    width: 100%;
    min-width: 0;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 6px 4px;
    border-radius: 10px;
    text-align: left;
  }
  .now {
    padding: 6px 8px;
  }
  .now b {
    color: var(--accent);
  }
  .txt {
    display: flex;
    flex-direction: column;
    min-width: 0;
    flex: 1;
  }
  b,
  small {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  b {
    font-weight: 550;
    color: var(--text);
  }
  small {
    color: var(--text-2);
    font-size: 12px;
  }
  .dur {
    color: var(--text-3);
    flex: none;
  }
  .row:hover .dur {
    display: none;
  }
  .remove {
    display: grid;
    place-items: center;
    width: 28px;
    height: 28px;
    border-radius: 50%;
    color: var(--text-2);
    opacity: 0;
    transition:
      opacity 0.15s,
      background 0.15s,
      transform 0.3s var(--spring);
  }
  .row:hover .remove {
    opacity: 1;
  }
  .remove:hover {
    color: var(--text);
    background: rgb(255 255 255 / 0.1);
  }
  .remove:active {
    transform: scale(0.85);
  }
  .more,
  .empty {
    padding: 8px;
    color: var(--text-2);
    font-size: 13px;
    line-height: 1.5;
  }
  .empty :global(svg) {
    vertical-align: -2px;
  }

  /* Finestra estreta: la cua flota per sobre del contingut en lloc de fer-li lloc. */
  @media (max-width: 1000px) {
    .queue {
      position: absolute;
      top: 0;
      right: 10px;
      bottom: 10px;
      z-index: 40;
      width: min(340px, calc(100% - 20px));
      margin: 0;
      background: #15131f;
      box-shadow: -20px 0 60px -10px rgb(0 0 0 / 0.8);
    }
  }
</style>
