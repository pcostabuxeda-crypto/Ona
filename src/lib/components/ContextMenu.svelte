<script lang="ts">
  /**
   * Menú contextual flotant (clic dret). Es tanca amb clic fora, Esc, Tab o en fer scroll.
   * Teclat: ↑/↓/Inici/Fi mouen el focus; en tancar, el focus torna on era.
   */
  import { scale } from "svelte/transition";
  import { calm } from "$lib/motion";
  import { settings } from "$lib/settings.svelte";
  import { ui } from "$lib/ui.svelte";
  import Icon from "./Icon.svelte";

  let el = $state<HTMLDivElement>();
  let pos = $state({ x: 0, y: 0, up: false, left: false });
  /** Element que tenia el focus abans d'obrir el menú. */
  let restore: HTMLElement | null = null;

  // Que no surti de la finestra; i el focus, a dins.
  $effect(() => {
    const m = ui.menu;
    if (!m || !el) return;
    const r = el.getBoundingClientRect();
    const left = m.x + r.width > innerWidth - 8;
    const up = m.y + r.height > innerHeight - 8;
    pos = {
      x: left ? Math.max(8, m.x - r.width) : m.x,
      y: up ? Math.max(8, m.y - r.height) : m.y,
      up,
      left,
    };
    const prev = document.activeElement as HTMLElement | null;
    if (!el.contains(prev)) restore = prev;
    // Obert des del teclat (el focus es veia): directament al primer element; amb el ratolí, al menú.
    const first = el.querySelector<HTMLElement>("[role=menuitem]");
    (prev?.matches(":focus-visible") ? first : el)?.focus({ preventScroll: true });
  });

  function close() {
    if (!ui.menu) return;
    ui.menu = null;
    restore?.focus({ preventScroll: true });
    restore = null;
  }

  function onKey(e: KeyboardEvent) {
    const items = [...(el?.querySelectorAll<HTMLElement>("[role=menuitem]") ?? [])];
    const i = items.indexOf(document.activeElement as HTMLElement);
    const go = (n: number) => {
      e.preventDefault();
      items[(n + items.length) % items.length]?.focus();
    };
    if (e.key === "ArrowDown") go(i + 1);
    else if (e.key === "ArrowUp") go(i < 0 ? -1 : i - 1);
    else if (e.key === "Home") go(0);
    else if (e.key === "End") go(-1);
    else if (e.key === "Tab") {
      e.preventDefault();
      close();
    }
  }
</script>

<svelte:window
  onpointerdown={(e) => ui.menu && !el?.contains(e.target as Node) && close()}
  onkeydown={(e) => e.key === "Escape" && close()}
  onwheel={close}
  onblur={close}
/>

{#if ui.menu}
  <div
    class="menu"
    class:still={settings.light}
    bind:this={el}
    style:left="{pos.x}px"
    style:top="{pos.y}px"
    style:transform-origin="{pos.left ? 'right' : 'left'} {pos.up ? 'bottom' : 'top'}"
    role="menu"
    tabindex="-1"
    onkeydown={onKey}
    out:scale={{ start: 0.96, duration: calm() ? 0 : 120 }}
  >
    {#each ui.menu.items as item, i (item.label)}
      <button
        role="menuitem"
        tabindex="-1"
        style:--i={i}
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
    outline: none;
    /* Neix del punt del clic amb molla; els elements arriben un rere l'altre. */
    animation: menu-in var(--t-spring) var(--spring) both;
  }
  :global(.app.ona) .menu {
    border-radius: 14px;
    background: #1d1928;
  }
  :global(.app.liner) .menu {
    border-radius: 12px;
    background: var(--elevated);
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
    animation: item-in var(--t-spring) var(--spring-soft) both;
    animation-delay: calc(min(var(--i), 8) * 18ms);
  }
  :global(.app.ona) button,
  :global(.app.liner) button {
    border-radius: 8px;
  }
  .still,
  .still button {
    animation: none;
  }
  button :global(svg) {
    color: var(--text-2);
  }
  button:hover,
  button:focus-visible {
    background: rgb(255 255 255 / 0.1);
    color: #fff;
    outline: none;
  }
  @keyframes menu-in {
    from {
      opacity: 0;
      scale: 0.9;
    }
  }
  @keyframes item-in {
    from {
      opacity: 0;
      translate: 0 -4px;
    }
  }
</style>
