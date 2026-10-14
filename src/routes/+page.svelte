<script lang="ts">
  import { onMount } from "svelte";
  import * as A from "$lib/api";
  import { player } from "$lib/player.svelte";
  import { ui } from "$lib/ui.svelte";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import PlayerBar from "$lib/components/PlayerBar.svelte";
  import NowPlaying from "$lib/components/NowPlaying.svelte";
  import Queue from "$lib/components/Queue.svelte";
  import Login from "$lib/components/Login.svelte";
  import Home from "$lib/views/Home.svelte";
  import Search from "$lib/views/Search.svelte";
  import Library from "$lib/views/Library.svelte";
  import CollectionView from "$lib/views/CollectionView.svelte";
  import Settings from "$lib/views/Settings.svelte";
  import ContextMenu from "$lib/components/ContextMenu.svelte";
  import { persistSettings, settings } from "$lib/settings.svelte";
  import ThemeBackground from "$lib/components/ThemeBackground.svelte";
  import { loadCustomImage, themeById } from "$lib/themes";
  import { setContext } from "svelte";

  // La imatge pròpia es comparteix amb la configuració (que la pot canviar).
  const bgImage = $state({ value: loadCustomImage() });
  setContext("bgImage", bgImage);
  const theme = $derived(themeById(settings.theme));

  persistSettings();

  let phase = $state<"boot" | "login" | "app">("boot");
  let busy = $state(false);
  let error = $state("");
  let scroller = $state<HTMLElement>();
  let clientId = $state<string | null>(null);

  onMount(async () => {
    await player.init();
    clientId = await A.getClientId();
    try {
      const user = await A.restore();
      if (user) return enter();
    } catch (e) {
      error = String(e);
    }
    phase = "login";
  });

  async function enter() {
    phase = "app";
    try {
      const me = await A.me();
      ui.user = me.display_name ?? me.id;
    } catch (e) {
      if (String(e).includes("not_logged_in")) phase = "login";
      else ui.user = "";
    }
  }

  async function doLogin(newId?: string) {
    busy = true;
    error = "";
    try {
      if (newId) {
        await A.setClientId(newId);
        clientId = newId.trim();
      }
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
    await A.logout();
    phase = "login";
  }

  // Tornar a dalt quan canvia la vista.
  $effect(() => {
    void ui.view;
    scroller?.scrollTo({ top: 0 });
  });

  function onKey(e: KeyboardEvent) {
    const typing = (e.target as HTMLElement)?.closest("input, textarea");
    if ((e.ctrlKey && e.key.toLowerCase() === "k") || (!typing && e.key === "/")) {
      e.preventDefault();
      ui.go({ name: "search" });
      return;
    }
    if (typing || phase !== "app") return;
    if (e.code === "Space") {
      e.preventDefault();
      player.toggle();
    } else if (e.key === "ArrowRight" && e.ctrlKey) player.next();
    else if (e.key === "ArrowLeft" && e.ctrlKey) player.prev();
    else if (e.key === "ArrowRight") player.seekTo(player.positionMs + 5000);
    else if (e.key === "ArrowLeft") player.seekTo(Math.max(0, player.positionMs - 5000));
    else if (e.key === "ArrowUp") player.setVolume(player.volume + 0.05);
    else if (e.key === "ArrowDown") player.setVolume(player.volume - 0.05);
    else if (e.key.toLowerCase() === "l") ui.setNowPlaying(!ui.nowPlaying);
    else if (e.key === "Escape" && ui.nowPlaying) ui.setNowPlaying(false);
    else if (e.key === "Backspace" || (e.altKey && e.key === "ArrowLeft")) ui.back();
  }
</script>

<svelte:window onkeydown={onKey} />


<div
  class="app"
  class:themed={theme.id !== "ona"}
  style:--pa={player.palette.a}
  style:--pb={player.palette.b}
  style:--theme-accent={theme.accent}
  style:--pa-alpha={settings.panelAlpha}
>
  <ThemeBackground customImage={bgImage.value} />
  <TitleBar />

  {#if phase === "login"}
    <Login onlogin={doLogin} {busy} {error} hasClientId={!!clientId} />
  {:else if phase === "app"}
    <div class="body">
      <Sidebar onlogout={doLogout} />
      <div class="stage">
        <main bind:this={scroller}>
          {#key ui.view}
            <div class="view">
              {#if ui.view.name === "home"}
                <Home />
              {:else if ui.view.name === "search"}
                <Search />
              {:else if ui.view.name === "library"}
                <Library />
              {:else if ui.view.name === "settings"}
                <Settings />
              {:else if ui.view.name === "collection"}
                <CollectionView kind={ui.view.kind} id={ui.view.id} {scroller} />
              {/if}
            </div>
          {/key}
        </main>
      </div>
      {#if ui.queueOpen}<Queue />{/if}
    </div>
    <PlayerBar />
    {#if ui.nowPlaying}<NowPlaying />{/if}
  {/if}

  <ContextMenu />

  {#if ui.toast}
    <div class="toast" role="status">{ui.toast}</div>
  {/if}
</div>

<style>
  .app {
    --accent: var(--theme-accent, color-mix(in oklab, var(--pa) 55%, white));
    /* Fons dels panells: gairebé invisibles al tema Ona, vidre fosc als temes amb fons. */
    --panel: rgb(255 255 255 / 0.03);
    --panel-solid: #13121c;
    --panel-line: rgb(255 255 255 / 0.05);
    position: relative;
    height: 100vh;
    display: flex;
    flex-direction: column;
    isolation: isolate;
  }
  .app.themed {
    --panel: rgb(8 6 18 / var(--pa-alpha));
    --panel-solid: rgb(8 6 18 / calc(var(--pa-alpha) + 0.25));
    --panel-line: rgb(255 255 255 / 0.1);
  }

  .body {
    position: relative;
    flex: 1;
    min-height: 0;
    display: flex;
    gap: 10px;
    padding-bottom: 10px;
  }
  .stage {
    position: relative;
    flex: 1;
    min-width: 0;
    border-radius: 22px;
    overflow: hidden;
    background: var(--panel);
    border: 1px solid var(--panel-line);
  }
  main {
    view-transition-name: stage;
    height: 100%;
    overflow-y: auto;
    overflow-x: hidden;
  }
  .view {
    min-height: 100%;
  }
  .toast {
    position: fixed;
    left: 50%;
    bottom: 112px;
    transform: translateX(-50%);
    z-index: 100;
    padding: 10px 18px;
    border-radius: 999px;
    background: var(--text);
    color: #0b0b12;
    font-weight: 600;
    box-shadow: 0 12px 30px -10px rgb(0 0 0 / 0.8);
    animation: rise 0.3s cubic-bezier(0.2, 0.8, 0.2, 1);
  }
</style>
