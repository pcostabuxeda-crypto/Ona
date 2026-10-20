<script lang="ts">
  /** Barra prima de Spotify: blanca, verda en passar-hi per sobre, amb el botó rodó visible només aleshores. */
  let { value, onchange, label, valueText }: { value: number; onchange: (v: number) => void; label: string; valueText?: string } = $props();

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
  <div class="track"><div class="fill" style:width="{shown * 100}%"></div></div>
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
</style>
