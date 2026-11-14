<script lang="ts">
  /** Estat buit o d'error: icona, títol, què pot fer l'usuari i, si cal, un botó. */
  import type { Snippet } from "svelte";
  import { settings } from "$lib/settings.svelte";
  import Icon from "./Icon.svelte";

  let {
    icon,
    title,
    action,
    onaction,
    small = false,
    children,
  }: { icon: string; title: string; action?: string; onaction?: () => void; small?: boolean; children?: Snippet } = $props();
</script>

<div class="empty" class:small class:still={settings.light}>
  <span class="ic"><Icon name={icon} size={small ? 20 : 28} /></span>
  <h3>{title}</h3>
  {#if children}<p>{@render children()}</p>{/if}
  {#if action && onaction}<button onclick={onaction}>{action}</button>{/if}
</div>

<style>
  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 48px 24px;
    text-align: center;
    animation: rise var(--t-spring) var(--spring-soft) both;
  }
  .small {
    padding: 16px 8px;
    gap: 4px;
  }
  .still {
    animation: none;
  }
  .ic {
    width: 64px;
    height: 64px;
    margin-bottom: 8px;
    border-radius: 50%;
    display: grid;
    place-items: center;
    background: var(--tinted);
    color: var(--text-2);
  }
  .small .ic {
    width: 40px;
    height: 40px;
    margin-bottom: 4px;
  }
  h3 {
    font-size: 20px;
  }
  .small h3 {
    font-size: 15px;
  }
  p {
    max-width: 360px;
    margin: 0;
    color: var(--text-2);
    line-height: 1.5;
  }
  .small p {
    font-size: 13px;
  }
  p :global(svg) {
    vertical-align: -2px;
  }
  button {
    margin-top: 12px;
    padding: 10px 24px;
    border-radius: 999px;
    background: var(--text);
    color: #000;
    font-weight: 700;
    transition: scale var(--t-spring) var(--spring);
  }
  button:hover {
    scale: 1.04;
  }
  @keyframes rise {
    from {
      opacity: 0;
      translate: 0 12px;
    }
  }
</style>
