<script lang="ts">
  import { onMount } from "svelte";
  import * as A from "$lib/api";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import PlayerBar from "$lib/components/PlayerBar.svelte";
  import NowPlayingPanel from "$lib/components/NowPlayingPanel.svelte";
  import Queue from "$lib/components/Queue.svelte";
  import Login from "$lib/components/Login.svelte";
  import Home from "$lib/views/Home.svelte";
  import Search from "$lib/views/Search.svelte";
  import Lyrics from "$lib/views/Lyrics.svelte";
  import CollectionView from "$lib/views/CollectionView.svelte";
  import ContextMenu from "$lib/components/ContextMenu.svelte";
  import NowPlayingFull from "$lib/components/NowPlayingFull.svelte";
  import { settings } from "$lib/settings.svelte";

  let phase = $state<"boot" | "login" | "app">("boot");
  let busy = $state(false);
  let error = $state("");
  let scroller = $state<HTMLElement>();
  let width = $state(1200);

  onMount(async () => {
    await player.init();
    try {
      const user = await A.restore();
      if (user) return enter();
    } catch (e) {
      error = String(e);
    }
    if (freeSaved()) return enterFree();
    phase = "login";
  });

  const FREE_KEY = "ona.free";
  function freeSaved() {
    try {
      return localStorage.getItem(FREE_KEY) === "1";
    } catch {
      return false;
    }
  }
  function setFree(free: boolean) {
    ui.free = free;
    try {
      if (free) localStorage.setItem(FREE_KEY, "1");
      else localStorage.removeItem(FREE_KEY);
    } catch {}
  }

  /** Sense compte de Spotify: Audius i fitxers locals. */
  function enterFree() {
    setFree(true);
    ui.user = { name: "Sense Spotify" };
    ui.stack = [{ name: "home" }];
    phase = "app";
  }

  async function enter() {
    setFree(false);
    phase = "app";
    try {
      const me = await A.me();
      ui.user = { name: me.name ?? me.uri ?? "", image: me.image_url };
    } catch (e) {
      if (String(e).includes("not_logged_in")) phase = "login";
    }
  }

  async function doLogin() {
    busy = true;
    error = "";
    try {
      await A.login();
      await enter();
    } catch (e) {
      // Un clic nou cancel·la l'intent anterior: aquell error no s'ha de mostrar.
      if (String(e) === "cancelled") return;
      error = `No s'ha pogut entrar: ${e}`;
    }
    busy = false;
  }

  async function doLogout() {
    if (!ui.free) await A.logout();
    setFree(false);
    phase = "login";
  }

  // Tornar a dalt quan canvia la vista.
  $effect(() => {
    void ui.view;
    scroller?.scrollTo({ top: 0 });
  });

  // Com el client: amb la finestra estreta, el panell de la dreta flota per sobre.
  const narrow = $derived(width < 1000);

  function onKey(e: KeyboardEvent) {
    const typing = (e.target as HTMLElement)?.closest("input, textarea");
    if (e.ctrlKey && e.key.toLowerCase() === "k") {
      e.preventDefault();
      document.querySelector<HTMLInputElement>("#search")?.focus();
      return;
    }
    if (typing || phase !== "app") return;
    if (e.code === "Space") {
      e.preventDefault();
      player.toggle();
    } else if (e.key === "ArrowRight" && e.ctrlKey) player.next();
    else if (e.key === "ArrowLeft" && e.ctrlKey) player.prev();
    else if (e.key === "ArrowRight" && e.shiftKey) player.seekTo(player.positionMs + 5000);
    else if (e.key === "ArrowLeft" && e.shiftKey) player.seekTo(Math.max(0, player.positionMs - 5000));
    else if (e.key === "ArrowUp" && e.ctrlKey) player.setVolume(player.volume + 0.1);
    else if (e.key === "ArrowDown" && e.ctrlKey) player.setVolume(player.volume - 0.1);
    else if (e.altKey && e.key === "ArrowLeft") ui.back();
    else if (e.altKey && e.key === "ArrowRight") ui.fwd();
  }
</script>

<svelte:window onkeydown={onKey} bind:innerWidth={width} />

<div class="app" class:liner={settings.liner}>
  <TitleBar loggedIn={phase === "app"} onlogout={doLogout} />

  {#if phase === "login"}
    <Login onlogin={doLogin} onfree={enterFree} {busy} {error} />
  {:else if phase === "app"}
    <div class="body" class:narrow>
      <Sidebar collapsed={width < 900 || settings.liner} />
      <main bind:this={scroller}>
        {#if ui.view.name === "home"}
          <Home />
        {:else if ui.view.name === "search"}
          <Search />
        {:else if ui.view.name === "lyrics" && !settings.liner}
          <Lyrics />
        {:else if ui.view.name === "collection"}
          {#key ui.view}
            <CollectionView kind={ui.view.kind} id={ui.view.id} {scroller} />
          {/key}
        {/if}
      </main>
      {#if ui.panel === "npv" && player.current}
        <NowPlayingPanel />
      {:else if ui.panel === "queue"}
        <Queue />
      {/if}
    </div>
    <PlayerBar />
    {#if ui.view.name === "lyrics" && settings.liner}<NowPlayingFull />{/if}
  {/if}

  <ContextMenu />

  {#if ui.toast}
    <div class="toast" role="status">{ui.toast}</div>
  {/if}
</div>

<style>
  .app {
    height: 100vh;
    display: flex;
    flex-direction: column;
    background: var(--bg);
  }
  .body {
    position: relative;
    flex: 1;
    min-height: 0;
    display: flex;
    gap: var(--gap);
    padding: 0 var(--gap);
  }
  main {
    position: relative;
    flex: 1;
    min-width: 0;
    border-radius: var(--radius);
    background: var(--base);
    overflow-y: auto;
    overflow-x: hidden;
  }
  .body :global(> aside.panel) {
    width: 280px;
    flex: none;
  }
  .narrow :global(> aside.panel) {
    position: absolute;
    right: var(--gap);
    top: 0;
    bottom: 0;
    z-index: 20;
    box-shadow: -8px 0 24px rgb(0 0 0 / 0.5);
  }
  .toast {
    position: fixed;
    left: 50%;
    bottom: 96px;
    transform: translateX(-50%);
    z-index: 100;
    padding: 8px 16px;
    border-radius: 4px;
    background: #3d91f4;
    color: #fff;
    font-weight: 600;
    box-shadow: 0 8px 24px rgb(0 0 0 / 0.5);
  }
</style>
