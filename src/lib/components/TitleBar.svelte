<script lang="ts">
  /** Barra de dalt com la del client: enrere/endavant, inici, cercador, perfil i botons de finestra. */
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import Icon from "./Icon.svelte";
  import { ui } from "$lib/ui.svelte";

  let { loggedIn, onlogout }: { loggedIn: boolean; onlogout: () => void } = $props();
  const win = getCurrentWindow();

  function onSearchInput() {
    if (ui.view.name !== "search") ui.go({ name: "search" });
  }
  function profileMenu(e: MouseEvent) {
    ui.openMenu(e, [{ label: "Tanca la sessió", icon: "logout", run: onlogout }]);
  }
</script>

<header class="bar" data-tauri-drag-region>
  {#if loggedIn}
    <div class="nav" data-tauri-drag-region>
      <button class="arrow" disabled={ui.stack.length < 2} onclick={() => ui.back()} title="Enrere"><Icon name="back" size={16} /></button>
      <button class="arrow" disabled={!ui.forward.length} onclick={() => ui.fwd()} title="Endavant"><Icon name="forward" size={16} /></button>
    </div>
    <div class="center" data-tauri-drag-region>
      <button class="home" class:on={ui.view.name === "home"} onclick={() => ui.go({ name: "home" })} title="Inici">
        <Icon name={ui.view.name === "home" ? "homeFilled" : "home"} size={24} />
      </button>
      <label class="search">
        <Icon name="search" size={24} />
        <input
          id="search"
          type="search"
          placeholder="Què vols escoltar?"
          bind:value={ui.query}
          oninput={onSearchInput}
          onfocus={onSearchInput}
          autocomplete="off"
          spellcheck="false"
        />
      </label>
    </div>
    <button class="avatar" onclick={profileMenu} oncontextmenu={profileMenu} title={ui.user?.name ?? ""}>
      {#if ui.user?.image}<img src={ui.user.image} alt="" />{:else}{(ui.user?.name ?? "?").slice(0, 1).toUpperCase()}{/if}
    </button>
  {/if}
  <div class="win">
    <button onclick={() => win.minimize()} title="Minimitza"><Icon name="min" size={16} /></button>
    <button onclick={() => win.toggleMaximize()} title="Maximitza"><Icon name="max" size={14} /></button>
    <button class="close" onclick={() => win.close()} title="Tanca"><Icon name="close" size={16} /></button>
  </div>
</header>

<style>
  .bar {
    height: 64px;
    flex: none;
    display: flex;
    align-items: center;
    gap: 8px;
    padding-left: 16px;
  }
  .nav {
    display: flex;
    gap: 8px;
  }
  .arrow {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    display: grid;
    place-items: center;
    color: var(--text);
  }
  .arrow:disabled {
    color: var(--text-3);
    cursor: default;
  }
  .center {
    flex: 1;
    min-width: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 8px;
  }
  .home {
    width: 48px;
    height: 48px;
    flex: none;
    border-radius: 50%;
    display: grid;
    place-items: center;
    background: var(--highlight);
    color: var(--text-2);
    transition: transform 0.1s;
  }
  .home:hover,
  .home.on {
    color: var(--text);
    transform: scale(1.04);
  }
  .search {
    flex: 0 1 474px;
    min-width: 0;
    height: 48px;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 0 16px 0 12px;
    border-radius: 500px;
    background: var(--highlight);
    color: var(--text-2);
    box-shadow: inset 0 0 0 1px transparent;
    transition: box-shadow 0.2s, background 0.2s;
  }
  .search:hover {
    background: var(--elevated-hi);
    box-shadow: inset 0 0 0 1px #535353;
  }
  .search:focus-within {
    box-shadow: inset 0 0 0 2px #fff;
    color: var(--text);
  }
  .search input {
    flex: 1;
    min-width: 0;
    background: none;
    border: 0;
    outline: 0;
    color: var(--text);
    font: inherit;
    font-size: 16px;
  }
  .search input::placeholder {
    color: var(--text-2);
  }
  .avatar {
    width: 48px;
    height: 48px;
    flex: none;
    border-radius: 50%;
    display: grid;
    place-items: center;
    background: var(--highlight);
    padding: 0;
  }
  .avatar img,
  .avatar:not(:has(img))::before {
    width: 32px;
    height: 32px;
    border-radius: 50%;
  }
  .avatar img {
    object-fit: cover;
  }
  .avatar:not(:has(img)) {
    font-weight: 700;
    color: #000;
    background: radial-gradient(circle, #f59b73 15px, var(--highlight) 16px);
  }
  .win {
    display: flex;
    align-self: stretch;
    margin-left: 8px;
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
</style>
