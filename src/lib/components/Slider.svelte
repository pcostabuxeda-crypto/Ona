<script lang="ts" module>
  // Ona (estètica Ona, de l'Ona 0.1): una sinusoide llarga dibuixada un sol cop; la barra en mostra un tros i la desplaça.
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
  /**
   * Barra prima de Spotify: blanca, verda en passar-hi per sobre, amb el botó rodó visible només aleshores.
   * Amb `wavy` (estètica Ona): la part feta és una ona que avança mentre sona i es torna plana en pausa.
   */
  let {
    value,
    onchange,
    label,
    valueText,
    wavy = false,
    playing = false,
  }: { value: number; onchange: (v: number) => void; label: string; valueText?: string; wavy?: boolean; playing?: boolean } = $props();

  let el: HTMLDivElement;
  let drag = $state<number | null>(null);
  const shown = $derived(drag ?? Math.min(1, Math.max(0, value || 0)));

  const at = (e: PointerEvent) => {
    const r = el.getBoundingClientRect();
    return Math.min(1, Math.max(0, (e.clientX - r.left) / r.width));
  };
  function down(e: PointerEvent) {
    el.setPointerCapture(e.pointerId);
    drag = at(e);
  }
  function move(e: PointerEvent) {
    if (drag !== null) drag = at(e);
  }
  function up() {
    if (drag === null) return;
    onchange(drag);
    drag = null;
  }
  function key(e: KeyboardEvent) {
    // Amb Ctrl/Maj/Alt és una drecera global (cançó següent, avançar 5 s...): no fem res aquí.
    if (e.ctrlKey || e.shiftKey || e.altKey || e.metaKey) return;
    const step = e.key === "ArrowRight" ? 0.05 : e.key === "ArrowLeft" ? -0.05 : 0;
    if (step) {
      e.preventDefault();
      e.stopPropagation();
      onchange(Math.min(1, Math.max(0, shown + step)));
    }
  }
</script>

<div
  class="slider"
  class:wavy
  class:waving={wavy && playing && drag === null}
  class:dragging={drag !== null}
  bind:this={el}
  role="slider"
  tabindex="0"
  aria-label={label}
  aria-valuemin={0}
  aria-valuemax={100}
  aria-valuenow={Math.round(shown * 100)}
  aria-valuetext={valueText}
  onpointerdown={down}
  onpointermove={move}
  onpointerup={up}
  onkeydown={key}
>
  {#if wavy}
    <div class="done" style:width="max(0px, calc({shown * 100}% - 5px))">
      <!-- Estela del cometa: s'esvaeix cap enrere des del cap (la barreta), sempre enganxada a la dreta. -->
      <div class="trail"></div>
      <svg class="wave" height="12" width="2400" aria-hidden="true">
        <path d={WAVE_D} fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" />
      </svg>
      <div class="flat"></div>
    </div>
    <div class="rest" style:left="min(100%, calc({shown * 100}% + 5px))"></div>
  {:else}
    <div class="track"><div class="fill" style:width="{shown * 100}%"></div></div>
  {/if}
  <div class="knob" style:left="{shown * 100}%"></div>
</div>

<style>
  .slider {
    position: relative;
    flex: 1;
    height: 12px;
    display: flex;
    align-items: center;
    cursor: pointer;
    touch-action: none;
  }
  .track {
    width: 100%;
    height: 4px;
    border-radius: 2px;
    background: #4d4d4d;
    overflow: hidden;
  }
  .fill {
    height: 100%;
    border-radius: 2px;
    background: #fff;
  }
  .knob {
    position: absolute;
    width: 12px;
    height: 12px;
    margin-left: -6px;
    border-radius: 50%;
    background: #fff;
    box-shadow: 0 2px 4px rgb(0 0 0 / 0.5);
    display: none;
  }
  .slider:hover .fill,
  .dragging .fill,
  .slider:focus-visible .fill {
    background: var(--accent);
  }
  .slider:hover .knob,
  .dragging .knob,
  .slider:focus-visible .knob {
    display: block;
  }

  /* ---------- Ona ---------- */
  .wavy {
    height: 22px;
  }
  .done {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    overflow: hidden;
    color: var(--accent);
  }
  .wave,
  .flat {
    position: absolute;
    top: 50%;
    transition: opacity 0.25s;
  }
  .wave {
    left: -26px;
    margin-top: -6px;
    opacity: 0;
  }
  .flat {
    left: 0;
    right: 0;
    height: 3px;
    margin-top: -1.5px;
    border-radius: 99px;
    background: currentColor;
  }
  .waving .wave {
    opacity: 1;
  }
  .waving .flat {
    opacity: 0;
  }
  /* Només es mou mentre sona: en pausa l'ona s'aplana i no hi ha cap animació en marxa.
     Es mou l'<svg> sencer (capa del compositor), no el <path> (que es repintaria a cada fotograma). */
  .waving .wave {
    animation: travel 1.1s linear infinite;
  }
  @keyframes travel {
    to {
      transform: translateX(26px);
    }
  }
  /* Cometa: color de la portada aclarit. L'estela i l'halo són degradats quiets; només canvia l'opacitat. */
  .wavy {
    --comet: color-mix(in oklab, var(--pb, var(--accent)) 55%, white);
  }
  .trail {
    position: absolute;
    right: 0;
    top: 50%;
    width: min(140px, 100%);
    height: 12px;
    margin-top: -6px;
    border-radius: 99px;
    background: linear-gradient(90deg, transparent, color-mix(in oklab, var(--comet) 45%, transparent));
    opacity: 0.5;
    transition: opacity var(--t-spring) ease-out;
  }
  .waving .trail {
    opacity: 1;
  }
  .rest {
    position: absolute;
    right: 0;
    top: 50%;
    height: 3px;
    margin-top: -1.5px;
    border-radius: 99px;
    background: rgb(255 255 255 / 0.16);
  }
  .wavy .knob {
    display: block;
    width: 5px;
    height: 16px;
    margin-left: -2.5px;
    border-radius: 99px;
    background: var(--comet);
    box-shadow: none;
    transition:
      height 0.3s var(--spring),
      width 0.3s var(--spring),
      margin 0.3s var(--spring);
  }
  /* Cap del cometa: halo de llum al voltant de la barreta, més viu mentre sona. */
  .wavy .knob::before {
    content: "";
    position: absolute;
    left: 50%;
    top: 50%;
    width: 22px;
    height: 22px;
    margin: -11px 0 0 -11px;
    border-radius: 50%;
    background: radial-gradient(circle, color-mix(in oklab, var(--comet) 70%, transparent), transparent 65%);
    opacity: 0.45;
    pointer-events: none;
    transition:
      opacity var(--t-spring) ease-out,
      scale var(--t-spring) var(--spring);
  }
  .waving .knob::before {
    opacity: 1;
  }
  .wavy:hover .knob::before,
  .wavy.dragging .knob::before {
    opacity: 1;
  }
  .wavy:hover .knob,
  .wavy.dragging .knob {
    height: 22px;
  }
  .wavy.dragging .knob {
    width: 7px;
    margin-left: -3.5px;
  }
  @media (prefers-reduced-motion: reduce) {
    .waving .wave {
      animation: none;
    }
  }
</style>
