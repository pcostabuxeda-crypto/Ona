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
  import MiniLyrics from "$lib/components/MiniLyrics.svelte";
  import { toggleMini } from "$lib/mini.svelte";
  import { settings } from "$lib/settings.svelte";

  // La finestra de lletra flotant carrega la mateixa app amb ?mini: només hi mostra la lletra.
  const isMini = typeof location !== "undefined" && new URLSearchParams(location.search).has("mini");

  let phase = $state<"boot" | "login" | "app">("boot");
  let busy = $state(false);
  let error = $state("");
  let scroller = $state<HTMLElement>();
  let width = $state(1200);

  /** Arrencada lenta: després de 8 s es diu què passa i es pot continuar sense Spotify. */
  let slow = $state(false);
  let expired = false;

  onMount(async () => {
    if (isMini) return;
    const slowTimer = setTimeout(() => (slow = true), 8000);
    await player.init();
    try {
      const user = await A.restore();
      clearTimeout(slowTimer);
      if (user) return enter();
    } catch (e) {
      error = loginError(e);
      expired = String(e).includes("session_expired");
    }
    clearTimeout(slowTimer);
    if (freeSaved() && !expired) return enterFree();
    phase = "login";
  });

  /** Errors d'inici de sessió en llenguatge de l'usuari, amb què pot fer. */
  function loginError(e: unknown) {
    const m = String(e);
    if (m.includes("session_expired")) return "La sessió ha caducat. Torna a iniciar sessió amb Spotify.";
    if (m.includes("Premium account required")) return "Cal un compte de Spotify Premium per reproduir. Sense, pots continuar amb Audius i els teus fitxers.";
    if (m.includes("port 5588")) return "Una altra app fa servir el port 5588. Tanca-la i torna-ho a provar.";
    if (m.includes("temps d'espera")) return "El navegador no ha respost a temps. Torna-ho a provar.";
    return `No s'ha pogut entrar (${m}). Comprova la connexió i torna-ho a provar.`;
  }

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
    // A la pantalla estreta, el panell «Ara sona» tapa el contingut: comença tancat.
    if (width < 1000) ui.panel = null;
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
      error = loginError(e);
    }
    busy = false;
  }

  async function doLogout() {
    player.forget(); // abans de tancar la sessió: la pausa encara funciona i la cua ja no serviria
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

  /** Dreceres amb Ctrl (també funcionen amb el focus en un botó). */
  const ctrlKeys: Record<string, () => void> = {
    l: () => player.current && ui.toggleLyrics(),
    "shift+l": () => player.current && !A.MOBILE && toggleMini(),
    q: () => ui.togglePanel("queue"),
    s: () => document.querySelector<HTMLButtonElement>("footer .like")?.click(),
    "shift+s": () => player.toggleShuffle(),
    r: () => player.cycleRepeat(), // i evita que WebView2 recarregui la pàgina
  };

  function onKey(e: KeyboardEvent) {
    const el = e.target as HTMLElement;
    const typing = el?.closest("input, textarea");
    if (e.ctrlKey && e.key.toLowerCase() === "k") {
      e.preventDefault();
      document.querySelector<HTMLInputElement>("#search")?.focus();
      return;
    }
    if (typing || phase !== "app") return;
    const combo = (e.shiftKey ? "shift+" : "") + e.key.toLowerCase();
    if (e.ctrlKey && !e.altKey && ctrlKeys[combo]) {
      e.preventDefault();
      ctrlKeys[combo]();
      return;
    }
    // L'espai només és play/pausa si el focus no és en un control que ja l'usa.
    if (e.code === "Space" && !el?.closest("button, a, [role=button], [role=slider], [role=menuitem]")) {
      e.preventDefault();
      player.toggle();
    } else if (e.key === "Escape" && ui.panel && narrow) ui.panel = null;
    else if (e.key === "ArrowRight" && e.ctrlKey) player.next();
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

{#if isMini}
  <MiniLyrics />
{:else}
<div class="app" class:liner={settings.liner}>
  <TitleBar loggedIn={phase === "app"} onlogout={doLogout} />

  {#if phase === "boot"}
    <div class="boot" data-tauri-drag-region>
      <svg width="88" height="56" viewBox="0 0 44 28" aria-hidden="true">
        <path class="wave" d="M2 14c5-10 10-10 15 0s10 10 15 0 7-8 10-6" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" />
      </svg>
      {#if slow}
        <p class="muted" role="status">Connectant amb Spotify…</p>
        <button class="free" onclick={enterFree}>Continua sense Spotify</button>
      {/if}
    </div>
  {:else if phase === "login"}
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
      {#if narrow && ui.panel && (ui.panel === "queue" || player.current)}
        <!-- Fons enfosquit només amb opacitat (sense backdrop-filter): clicar-lo tanca el panell. -->
        <button class="scrim" aria-label="Tanca el panell" tabindex="-1" onclick={() => (ui.panel = null)}></button>
      {/if}
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

  <!-- Regió viva permanent: si es creés amb el text, el lector de pantalla podria no anunciar-lo. -->
  <div class="toasts" role="status" aria-live="polite">
    {#if ui.toast}
      <div class="toast">
        {ui.toast}
        {#if ui.toastAction}
          <button
            onclick={() => {
              ui.toastAction?.run();
              ui.toast = null;
              ui.toastAction = null;
            }}>{ui.toastAction.label}</button
          >
        {/if}
      </div>
    {/if}
  </div>
</div>
{/if}

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
    /* Les vistes s'adapten a l'amplada real del contingut (amb panells oberts o no), no a la finestra. */
    container-type: inline-size;
  }
  .body :global(> aside.panel) {
    width: 280px;
    flex: none;
  }
  /* <1000 px: el panell és un full lateral que entra amb una molla i surt de pressa. */
  .narrow :global(> aside.panel) {
    position: absolute;
    right: var(--gap);
    top: 0;
    bottom: 0;
    z-index: 20;
    width: min(360px, 88vw);
    box-shadow: -8px 0 24px rgb(0 0 0 / 0.5);
    animation: sheet-in var(--t-spring) var(--spring) both;
  }
  @keyframes sheet-in {
    from {
      transform: translateX(105%);
    }
  }
  .scrim {
    position: absolute;
    inset: 0;
    z-index: 15;
    background: #000;
    opacity: 0.45;
    border-radius: var(--radius);
    animation: scrim-in var(--t-quick) ease-out both;
  }
  @keyframes scrim-in {
    from {
      opacity: 0;
    }
  }
  .boot {
    flex: 1;
    display: grid;
    place-content: center;
    justify-items: center;
    gap: 16px;
    color: var(--accent);
  }
  .wave {
    stroke-dasharray: 60;
    stroke-dashoffset: 60;
    animation: draw 900ms var(--spring-soft) forwards;
  }
  @keyframes draw {
    to {
      stroke-dashoffset: 0;
    }
  }
  @media (prefers-reduced-motion: reduce) {
    .wave {
      animation: none;
      stroke-dashoffset: 0;
    }
  }
  .boot .free {
    padding: 10px 24px;
    border-radius: 999px;
    border: 1px solid #727272;
    font-weight: 700;
    color: var(--text);
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
    display: flex;
    align-items: center;
    gap: 16px;
    animation: toast-in var(--t-spring) var(--spring) both;
  }
  .toast button {
    font-weight: 700;
    text-decoration: underline;
  }
  @keyframes toast-in {
    from {
      opacity: 0;
      transform: translate(-50%, 12px);
    }
  }
</style>
