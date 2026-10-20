<script lang="ts">
  /**
   * Cua en dues parts, com Spotify: «A continuació a la teva cua» (el que has afegit, en ordre
   * d'arribada) i «Després, de: <context>». Es reordena arrossegant la nansa o amb Alt+↑/↓.
   */
  import { flip } from "svelte/animate";
  import { fly } from "svelte/transition";
  import type { Track } from "$lib/api";
  import { fmtTime } from "$lib/api";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";
  import Cover from "./Cover.svelte";
  import Icon from "./Icon.svelte";

  type ListId = "user" | "ctx";
  const H = 58; // alçada de cada fila (px)

  let drag = $state<{ list: ListId; from: number; startY: number; dy: number; pointer: number } | null>(null);
  /** Durant un fotograma després de deixar anar, sense transicions (evita el salt). */
  let settling = $state(false);

  const rows = $derived({
    user: player.upNext.map((track, i) => ({ track, key: `u${i}:${track.uri}` })),
    ctx: player.upcoming.map((u) => ({ track: u.track, key: `c${u.key}` })),
  });
  const over = $derived(drag ? Math.max(0, Math.min(rows[drag.list].length - 1, drag.from + Math.round(drag.dy / H))) : -1);

  function shift(list: ListId, j: number) {
    if (!drag || drag.list !== list) return 0;
    if (j === drag.from) return drag.dy;
    if (drag.from < j && j <= over) return -H;
    if (over <= j && j < drag.from) return H;
    return 0;
  }

  /** Mou dins d'una llista (índexs relatius a la llista). */
  function moveIn(list: ListId, from: number, to: number) {
    if (list === "user") player.moveUser(from, to);
    else player.move(player.pos + 1 + from, player.pos + 1 + to);
  }

  function start(e: PointerEvent, list: ListId, j: number) {
    e.preventDefault();
    e.stopPropagation();
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    drag = { list, from: j, startY: e.clientY, dy: 0, pointer: e.pointerId };
  }
  function moveDrag(e: PointerEvent) {
    if (drag && e.pointerId === drag.pointer) drag.dy = e.clientY - drag.startY;
  }
  function end(e: PointerEvent) {
    if (!drag || e.pointerId !== drag.pointer) return;
    const { list, from } = drag;
    const to = over;
    settling = true;
    drag = null;
    moveIn(list, from, to);
    requestAnimationFrame(() => requestAnimationFrame(() => (settling = false)));
  }

  /** Alt+↑/↓ sobre una fila: alternativa de teclat a arrossegar. */
  function keyMove(e: KeyboardEvent, list: ListId, j: number) {
    if (!e.altKey || (e.key !== "ArrowUp" && e.key !== "ArrowDown")) return;
    e.preventDefault();
    e.stopPropagation();
    const to = j + (e.key === "ArrowUp" ? -1 : 1);
    if (to < 0 || to >= rows[list].length) return;
    moveIn(list, j, to);
    requestAnimationFrame(() => document.querySelector<HTMLElement>(`[data-q="${list}${to}"]`)?.focus());
  }

  function play(list: ListId, j: number) {
    if (drag) return;
    if (list === "user") player.jumpToUser(j);
    else player.jumpTo(player.pos + 1 + j);
  }
  function remove(list: ListId, j: number) {
    if (list === "user") player.removeUser(j);
    else player.removeAt(player.pos + 1 + j);
  }

  function clearUser() {
    const was = player.clearUser();
    ui.say(was.length === 1 ? "S'ha buidat la cua (1 cançó)" : `S'ha buidat la cua (${was.length} cançons)`, {
      label: "Desfés",
      run: () => player.restoreUser(was),
    });
  }
</script>

{#snippet list(id: ListId, items: { track: Track; key: string }[])}
  <div class="list" class:dragging={drag?.list === id} class:settling>
    {#each items as u, j (u.key)}
      <div
        class="row"
        class:lifted={drag?.list === id && drag.from === j}
        style:transform="translateY({shift(id, j)}px)"
        animate:flip={{ duration: settling ? 0 : 300 }}
        out:fly={{ x: 40, duration: 180 }}
      >
        <span
          class="handle"
          role="button"
          tabindex="-1"
          aria-label="Arrossega per moure"
          onpointerdown={(e) => start(e, id, j)}
          onpointermove={moveDrag}
          onpointerup={end}
          onpointercancel={end}
        >
          <Icon name="grip" size={16} />
        </span>
        <button
          class="item"
          data-q="{id}{j}"
          title="Reprodueix ara · Alt ↑/↓ per moure"
          onclick={() => play(id, j)}
          onkeydown={(e) => keyMove(e, id, j)}
        >
          <Cover src={u.track.album.thumb} size={48} radius={4} />
          <span class="txt">
            <b>{u.track.name}</b>
            <small>{u.track.artists.map((a) => a.name).join(", ")}</small>
          </span>
          <small class="dur">{u.track.duration_ms ? fmtTime(u.track.duration_ms) : ""}</small>
        </button>
        <button class="remove" onclick={() => remove(id, j)} title="Treu de la cua" aria-label="Treu «{u.track.name}» de la cua">
          <Icon name="close" size={16} />
        </button>
      </div>
    {/each}
  </div>
{/snippet}

<aside class="panel queue" aria-label="Cua">
  <header>
    <h3>Cua</h3>
    <button class="x" onclick={() => (ui.panel = null)} title="Tanca" aria-label="Tanca la cua"><Icon name="close" size={18} /></button>
  </header>

  {#if player.current}
    <span class="label">Ara sona</span>
    <div class="item now">
      <Cover src={player.current.album.thumb} size={48} radius={4} />
      <span class="txt">
        <b>{player.current.name}</b>
        <small>{player.current.artists.map((a) => a.name).join(", ")}</small>
      </span>
    </div>
  {/if}

  {#if rows.user.length}
    <span class="label">
      A continuació a la teva cua
      <button class="clear" onclick={clearUser}>Buida la cua</button>
    </span>
    {@render list("user", rows.user)}
  {/if}

  {#if rows.ctx.length}
    <span class="label">
      Després{player.context ? `, de: ${player.context}` : ""}
      <em>{player.upcomingTotal}</em>
    </span>
    {@render list("ctx", rows.ctx)}
    {#if player.upcomingTotal > rows.ctx.length}
      <p class="more">… i {player.upcomingTotal - rows.ctx.length} més</p>
    {/if}
  {/if}

  {#if !rows.user.length && !rows.ctx.length}
    <p class="empty">
      No hi ha res més a la cua. Afegeix-hi cançons amb el botó <Icon name="queueAdd" size={14} /> d'una fila o amb Q.
    </p>
  {/if}
</aside>

<style>
  .queue {
    padding: 16px 8px;
    border-radius: var(--radius);
    background: var(--base);
    overflow-y: auto;
    overflow-x: hidden;
  }
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 8px 8px;
  }
  .clear {
    margin-left: auto;
    padding: 5px 12px;
    border-radius: 99px;
    font-size: 12px;
    font-weight: 600;
    color: var(--text-2);
    background: var(--tinted);
  }
  .clear:hover,
  .clear:focus-visible {
    color: var(--text);
    background: var(--tinted-hi);
  }
  .x {
    color: var(--text-2);
    display: grid;
  }
  .x:hover {
    color: var(--text);
  }
  h3 {
    font-size: 16px;
  }
  .label {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 16px 8px 8px;
    font-size: 16px;
    font-weight: 700;
    color: var(--text);
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
    border-radius: 6px;
    transition: transform var(--t-quick) ease;
  }
  .settling .row {
    transition: none;
  }
  .row:hover {
    background: var(--highlight);
  }
  .row.lifted {
    z-index: 5;
    transition: none;
    background: var(--elevated-hi);
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
  .row:focus-within .handle,
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
    font-weight: 400;
    font-size: 16px;
    color: var(--text);
  }
  small {
    color: var(--text-2);
    font-size: 14px;
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
    transition: opacity 0.15s;
  }
  .row:hover .remove,
  .remove:focus-visible,
  .row:focus-within .remove {
    opacity: 1;
  }
  .remove:hover {
    color: var(--text);
    background: rgb(255 255 255 / 0.1);
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
</style>
