<script lang="ts">
  /**
   * Estètica Liner: lletra a pantalla completa amb la portada i els controls a l'esquerra.
   * Reaprofita la vista de lletra (fons fluid o color pla, segons el mode) amb un vel fosc.
   */
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import * as A from "$lib/api";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";
  import Lyrics from "$lib/views/Lyrics.svelte";
  import Cover from "./Cover.svelte";
  import Icon from "./Icon.svelte";
  import Slider from "./Slider.svelte";
  import Disc from "./Disc.svelte";
  import Visualizer from "./Visualizer.svelte";
  import { settings, space } from "$lib/settings.svelte";

  // Estètica Ona: disc de vinil, ona a la barra i visualitzador a baix.
  const ona = $derived(settings.look === "ona");
  // Mode espai: el fons Cosmos es veu a través (sense fons propi) i el disc és un planeta.
  const cosmos = $derived(space());

  const win = getCurrentWindow();
  const t = $derived(player.current);
  let width = $state(1200);
  // Finestra estreta (pantalla vertical): la columna passa a dalt, petita.
  const stacked = $derived(width < 860);
</script>

<svelte:window bind:innerWidth={width} onkeydown={(e) => e.key === "Escape" && ui.back()} />

<div class="fs" class:stacked class:cosmos>
  {#if ona && settings.viz && !settings.light}
    <div class="viz"><Visualizer style="line" shape="wave" colorA={player.palette.a} colorB={player.palette.b} /></div>
  {/if}
  <main class="scroll">
    <Lyrics inset={stacked ? "32px" : "52%"} dark space={cosmos} aura={ona && !settings.light} />
  </main>

  <header data-tauri-drag-region>
    <button class="round" onclick={() => ui.back()} title="Tanca la lletra"><Icon name="down" size={20} /></button>
    <div class="win">
      <button onclick={() => win.minimize()} title="Minimitza"><Icon name="min" size={16} /></button>
      <button onclick={() => win.toggleMaximize()} title="Maximitza"><Icon name="max" size={14} /></button>
      <button class="close" onclick={() => win.close()} title="Tanca"><Icon name="close" size={16} /></button>
    </div>
  </header>

  {#if t}
    <aside class="side">
      {#if ona && settings.disc}
        <Disc src={t.album.image} playing={player.playing} size={stacked ? "96px" : "min(28vw, 340px)"} planet={cosmos && !stacked} />
      {:else}
        <Cover src={t.album.image} size={stacked ? 96 : "min(28vw, 340px)"} radius={ona ? 20 : 12} />
      {/if}
      <div class="meta">
        <b>{t.name}</b>
        <span>{#if t.explicit}<i class="e">E</i>{/if}{t.artists.map((a) => a.name).join(", ")}</span>
      </div>
      <div class="time">
        <span>{A.fmtTime(player.positionMs)}</span>
        <Slider value={player.progress} onchange={(v) => player.seekTo(v * t.duration_ms)} label="Progrés" wavy={ona && !settings.light} playing={player.playing} />
        <span>{A.fmtTime(t.duration_ms)}</span>
      </div>
      <div class="controls">
        <button class:on={player.shuffle} onclick={() => player.toggleShuffle()} title="Ordre aleatori"><Icon name="shuffle" size={18} /></button>
        <button onclick={() => player.prev()} title="Anterior"><Icon name="prev" size={24} /></button>
        <button class="play" onclick={() => player.toggle()} title={player.playing ? "Pausa" : "Reprodueix"}>
          <Icon name={player.playing ? "pause" : "play"} size={30} />
        </button>
        <button onclick={() => player.next()} title="Següent"><Icon name="next" size={24} /></button>
        <button class:on={player.repeat !== "off"} onclick={() => player.cycleRepeat()} title="Repeteix"><Icon name="repeat" size={18} /></button>
      </div>
      {#if !stacked}
        <div class="vol">
          <Icon name={player.volume ? "volume" : "mute"} size={16} />
          <Slider value={player.volume} onchange={(v) => player.setVolume(v)} label="Volum" />
        </div>
      {/if}
    </aside>
  {/if}
</div>

<style>
  .fs {
    position: fixed;
    inset: 0;
    z-index: 60;
    background: #0a0a0b;
  }
  /* Mode espai: el Cosmos de l'app es veu a través (la resta de l'app s'amaga a +page). */
  .fs.cosmos {
    background: transparent;
    animation: arrive var(--t-spring) var(--spring-soft) backwards;
  }
  @keyframes arrive {
    from {
      opacity: 0;
      scale: 0.94;
    }
  }
  .stacked.cosmos .side {
    background: linear-gradient(rgb(3 2 8 / 0.92) 80%, transparent);
  }
  .cosmos .side :global(.disc.planet) {
    margin-bottom: 28px;
  }
  .viz {
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    height: 22vh;
    z-index: 1;
    pointer-events: none;
    opacity: 0.8;
  }
  .scroll {
    position: absolute;
    inset: 0;
    overflow-y: auto;
  }
  header {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 64px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-left: 16px;
    z-index: 2;
  }
  .round {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    display: grid;
    place-items: center;
    background: rgb(255 255 255 / 0.08);
    color: var(--text);
  }
  .round:hover {
    background: rgb(255 255 255 / 0.16);
  }
  .win {
    display: flex;
    align-self: stretch;
  }
  .win button {
    width: 46px;
    display: grid;
    place-items: center;
    color: var(--text-2);
  }
  .win button:hover {
    background: rgb(255 255 255 / 0.1);
    color: var(--text);
  }
  .win .close:hover {
    background: #c42b1c;
    color: #fff;
  }
  .side {
    position: absolute;
    z-index: 2;
    left: 0;
    top: 0;
    bottom: 0;
    width: 48%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 16px;
    pointer-events: none;
  }
  .side > * {
    pointer-events: auto;
  }
  .side :global(.cover) {
    box-shadow: 0 16px 48px rgb(0 0 0 / 0.5);
  }
  .meta {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    max-width: min(28vw, 340px);
    text-align: center;
  }
  .meta b {
    font-size: 16px;
  }
  .meta span {
    font-size: 14px;
    color: var(--text-2);
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
    font-style: normal;
    font-weight: 700;
    vertical-align: 1px;
  }
  .time,
  .vol {
    width: min(28vw, 300px);
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 12px;
    color: var(--text-2);
    font-variant-numeric: tabular-nums;
  }
  .vol {
    width: min(18vw, 180px);
  }
  .controls {
    display: flex;
    align-items: center;
    gap: 24px;
  }
  .controls button {
    color: var(--text);
    display: grid;
    place-items: center;
  }
  .controls button:not(.play) {
    opacity: 0.75;
  }
  .controls button:hover {
    opacity: 1;
  }
  .controls .on {
    opacity: 1;
    color: var(--accent);
  }

  /* Pantalla estreta: portada petita i controls a dalt, la lletra a sota. */
  .stacked .side {
    width: auto;
    right: 0;
    bottom: auto;
    top: 56px;
    flex-direction: row;
    flex-wrap: wrap;
    justify-content: flex-start;
    padding: 0 24px;
    gap: 12px 16px;
    background: linear-gradient(#0a0a0b 80%, transparent);
    padding-bottom: 24px;
  }
  .stacked .meta {
    align-items: flex-start;
    text-align: left;
    flex: 1;
    min-width: 0;
  }
  .stacked .time {
    width: 100%;
  }
  .stacked .scroll {
    /* Per sota de la columna apilada (portada de 96 px, temps i controls), perquè no tapi la lletra. */
    top: 264px;
  }
</style>
