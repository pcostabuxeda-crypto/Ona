<script lang="ts" module>
  // Una sinusoide llarga (2400 px) dibuixada un sol cop; la barra en mostra un tros i la desplaça.
  const WL = 26;
  const AMP = 3.2;
  const WAVE_D = (() => {
    let d = `M 0 6`;
    for (let x = 0; x < 2400; x += WL / 2) {
      const dir = (x / (WL / 2)) % 2 === 0 ? -1 : 1;
      const c = WL / 2 / 2.6;
      d += ` C ${x + c} ${6 + dir * AMP * 1.3} ${x + WL / 2 - c} ${6 + dir * AMP * 1.3} ${x + WL / 2} 6`;
    }
    return d;
  })();
</script>

<script lang="ts">
  import { fade } from "svelte/transition";
  /**
   * Barra arrossegable estil Material 3 Expressive: la part feta és una ona que avança
   * i es torna plana en pausa; el polze és una pastilla.
   */
  let {
    value,
    onseek,
    smooth = false,
    wavy = false,
    playing = false,
    label = (v: number) => "",
    height = 4,
  }: {
    value: number;
    onseek: (v: number) => void;
    smooth?: boolean;
    wavy?: boolean;
    playing?: boolean;
    label?: (v: number) => string;
    height?: number;
  } = $props();

  let el: HTMLDivElement;
  let drag = $state<number | null>(null);
  let hover = $state<number | null>(null);

  const shown = $derived(drag ?? value);
  const waving = $derived(wavy && playing && drag === null);

  function at(e: PointerEvent) {
    const r = el.getBoundingClientRect();
    return Math.min(1, Math.max(0, (e.clientX - r.left) / r.width));
  }
  function down(e: PointerEvent) {
    el.setPointerCapture(e.pointerId);
    drag = at(e);
  }
  function move(e: PointerEvent) {
    hover = at(e);
    if (drag !== null) drag = hover;
  }
  function up(e: PointerEvent) {
    if (drag === null) return;
    onseek(at(e));
    drag = null;
  }
  function key(e: KeyboardEvent) {
    if (e.key === "ArrowRight") onseek(Math.min(1, value + 0.02));
    if (e.key === "ArrowLeft") onseek(Math.max(0, value - 0.02));
  }
</script>

<div
  class="bar"
  class:active={drag !== null}
  class:smooth={smooth && drag === null}
  bind:this={el}
  style:--h="{height}px"
  style:--p={shown}
  role="slider"
  tabindex="0"
  aria-valuemin={0}
  aria-valuemax={1}
  aria-valuenow={shown}
  onpointerdown={down}
  onpointermove={move}
  onpointerup={up}
  onpointerleave={() => (hover = null)}
  onkeydown={key}
>
  <div class="done">
    {#if waving}
      <!-- Ona: es repinta (zona petita) en lloc de fer una capa de GPU, que donava vores brutes. -->
      <svg class="wave" height="12" width="2400" aria-hidden="true" in:fade={{ duration: 250 }} out:fade={{ duration: 150 }}>
        <path class="wpath" d={WAVE_D} fill="none" stroke="currentColor" stroke-width={height - 1} stroke-linecap="round" />
      </svg>
    {:else}
      <div class="flat" in:fade={{ duration: 250 }}></div>
    {/if}
  </div>
  <div class="rest"></div>
  <div class="thumb"></div>
  {#if hover !== null && label(hover)}
    <span class="tip" style:left="{hover * 100}%">{label(drag ?? hover)}</span>
  {/if}
</div>

<style>
  .bar {
    --gap: 5px;
    --wl: 26px;
    position: relative;
    height: 22px;
    cursor: pointer;
    touch-action: none;
    outline: none;
    container-type: inline-size;
  }
  .done {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: max(0px, calc(var(--p) * 100% - var(--gap)));
    overflow: hidden;
  }
  .smooth .done {
    transition: width 0.25s linear;
  }
  .smooth .rest {
    transition: left 0.25s linear;
  }
  .smooth .thumb {
    transition:
      left 0.25s linear,
      height 0.2s,
      margin 0.2s,
      width 0.2s;
  }
  .flat {
    position: absolute;
    left: 0;
    right: 0;
    top: 50%;
    height: var(--h);
    margin-top: calc(var(--h) / -2);
    border-radius: 99px;
    background: var(--accent);
  }
  .wave {
    position: absolute;
    left: calc(var(--wl) * -1);
    top: 50%;
    margin-top: -6px;
    color: var(--accent);
  }
  .wpath {
    animation: travel 1.1s linear infinite;
  }
  @keyframes travel {
    to {
      transform: translateX(var(--wl));
    }
  }
  .rest {
    position: absolute;
    right: 0;
    top: 50%;
    height: var(--h);
    margin-top: calc(var(--h) / -2);
    left: min(100%, calc(var(--p) * 100% + var(--gap)));
    border-radius: 99px;
    background: rgb(255 255 255 / 0.16);
  }
  .thumb {
    position: absolute;
    top: 50%;
    left: calc(var(--p) * 100%);
    width: 5px;
    height: 16px;
    margin: -8px 0 0 -2.5px;
    border-radius: 99px;
    background: var(--accent);
    transition:
      height 0.2s,
      margin 0.2s,
      width 0.2s;
  }
  .bar:hover .thumb,
  .active .thumb {
    height: 20px;
    margin-top: -10px;
  }
  .active .thumb {
    width: 7px;
    margin-left: -3.5px;
  }
  .bar:focus-visible {
    outline: none;
  }
  .tip {
    position: absolute;
    bottom: 22px;
    transform: translateX(-50%);
    font-size: 11px;
    padding: 3px 7px;
    border-radius: 6px;
    background: rgb(10 10 16 / 0.92);
    color: var(--text);
    pointer-events: none;
    white-space: nowrap;
    font-variant-numeric: tabular-nums;
  }
</style>
