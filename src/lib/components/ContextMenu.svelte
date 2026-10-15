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
    {#each ui.menu.items as item (item.label)}
      <button
        role="menuitem"
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
    min-width: 196px;
    max-width: 350px;
    padding: 4px;
    border-radius: 4px;
    background: #282828;
    box-shadow: 0 16px 24px rgb(0 0 0 / 0.3), 0 6px 8px rgb(0 0 0 / 0.2);
  }
  button {
    width: 100%;
    height: 40px;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 0 12px 0 8px;
    border-radius: 2px;
    text-align: left;
    color: rgb(255 255 255 / 0.9);
  }
  button :global(svg) {
    color: var(--text-2);
  }
  button:hover {
    background: rgb(255 255 255 / 0.1);
    color: #fff;
  }
</style>
