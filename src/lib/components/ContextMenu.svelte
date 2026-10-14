<script lang="ts">
  /** Menú contextual flotant (clic dret). Es tanca amb clic fora, Esc o en fer scroll. */
  import { ui } from "$lib/ui.svelte";
  import Icon from "./Icon.svelte";

  let el = $state<HTMLDivElement>();
  let pos = $state({ x: 0, y: 0 });

  // Que no surti de la finestra.
  $effect(() => {
    const m = ui.menu;
    if (!m || !el) return;
    const r = el.getBoundingClientRect();
    pos = {
      x: Math.min(m.x, innerWidth - r.width - 8),
      y: m.y + r.height > innerHeight - 8 ? Math.max(8, m.y - r.height) : m.y,
    };
  });

  const close = () => (ui.menu = null);
</script>

<svelte:window
  onpointerdown={(e) => ui.menu && !el?.contains(e.target as Node) && close()}
  onkeydown={(e) => e.key === "Escape" && close()}
  onwheel={close}
  onblur={close}
/>

{#if ui.menu}
  <div class="menu" bind:this={el} style:left="{pos.x}px" style:top="{pos.y}px" role="menu">
    {#each ui.menu.items as item, i (item.label)}
      <button
        role="menuitem"
        style:--d="{i * 22}ms"
        onclick={() => {
          close();
          item.run();
        }}
      >
        <Icon name={item.icon} size={18} />
        {item.label}
      </button>
    {/each}
  </div>
{/if}

<style>
  .menu {
    position: fixed;
    z-index: 200;
    min-width: 230px;
    padding: 6px;
    border-radius: 16px;
    background: #1c1a27;
    border: 1px solid rgb(255 255 255 / 0.09);
    box-shadow: 0 18px 44px -12px rgb(0 0 0 / 0.85);
    transform-origin: top left;
    animation: pop 0.32s var(--spring) both;
  }
  button {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 9px 12px;
    border-radius: 10px;
    text-align: left;
    font-weight: 500;
    color: var(--text);
    animation: rise 0.3s var(--spring-soft) both;
    animation-delay: var(--d);
    transition: background 0.12s;
  }
  button :global(svg) {
    color: var(--text-2);
  }
  button:hover {
    background: rgb(255 255 255 / 0.08);
  }
  button:hover :global(svg) {
    color: var(--accent);
  }
</style>
