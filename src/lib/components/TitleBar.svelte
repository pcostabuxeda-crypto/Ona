<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import Icon from "./Icon.svelte";
  import { ui } from "$lib/ui.svelte";

  const win = getCurrentWindow();
</script>

<header class="bar" data-tauri-drag-region>
  <div class="left">
    <button class="nav" disabled={ui.stack.length < 2} onclick={() => ui.back()} title="Enrere">
      <Icon name="back" size={18} />
    </button>
    <span class="brand" data-tauri-drag-region>
      <svg width="22" height="14" viewBox="0 0 44 28" aria-hidden="true">
        <path d="M2 14c5-10 10-10 15 0s10 10 15 0 7-8 10-6" fill="none" stroke="url(#g)" stroke-width="4" stroke-linecap="round" />
        <defs>
          <linearGradient id="g" x1="0" x2="1">
            <stop offset="0" stop-color="var(--pa)" /><stop offset="1" stop-color="var(--pb)" />
          </linearGradient>
        </defs>
      </svg>
      ona
    </span>
  </div>
  <div class="win">
    <button onclick={() => win.minimize()} title="Minimitza"><Icon name="min" size={16} /></button>
    <button onclick={() => win.toggleMaximize()} title="Maximitza"><Icon name="max" size={14} /></button>
    <button class="close" onclick={() => win.close()} title="Tanca"><Icon name="close" size={16} /></button>
  </div>
</header>

<style>
  .bar {
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-left: 10px;
    position: relative;
    z-index: 50;
  }
  .left {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .brand {
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 650;
    letter-spacing: -0.02em;
    font-size: 15px;
    color: var(--text);
  }
  .nav {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    display: grid;
    place-items: center;
    color: var(--text-2);
    background: var(--glass);
  }
  .nav:disabled {
    opacity: 0.3;
  }
  .nav:not(:disabled):hover {
    color: var(--text);
  }
  .win {
    display: flex;
    height: 100%;
  }
  .win button {
    width: 46px;
    height: 100%;
    display: grid;
    place-items: center;
    color: var(--text-2);
    transition: background 0.15s;
  }
  .win button:hover {
    background: rgb(255 255 255 / 0.08);
    color: var(--text);
  }
  .win .close:hover {
    background: #c42b1c;
    color: white;
  }
</style>
