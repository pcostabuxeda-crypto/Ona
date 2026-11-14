<script lang="ts">
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
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
  import { settings, space } from "$lib/settings.svelte";
  import ThemeBackground from "$lib/components/ThemeBackground.svelte";
  import Appearance from "$lib/components/Appearance.svelte";

  // Estrelles de l'enlairament: al voltant del centre (vmax), es pinten un cop i només s'escala la capa.
  const CONVERGE = Array.from({ length: 110 }, () => {
    const a = Math.random() * Math.PI * 2;
    const r = 8 + Math.random() * 55;
    const s = Math.random() < 0.2 ? 1.5 : 0.75;
    return `${(Math.cos(a) * r).toFixed(2)}vmax ${(Math.sin(a) * r).toFixed(2)}vmax 0 ${s}px rgb(255 255 255 / ${(0.5 + Math.random() * 0.5).toFixed(2)})`;
  }).join(",");

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
    // Minimitzada: les animacions es paren (WebView2 no marca la pàgina com a amagada).
    const win = getCurrentWindow();
    win.onResized(async () => document.documentElement.classList.toggle("app-hidden", await win.isMinimized()));
    const slowTimer = setTimeout(() => (slow = true), 8000);
    // Que l'enlairament es vegi sencer (estrelles que s'ajunten + l'ona que s'encén), però no més.
    const liftoff = settings.look === "ona" && !settings.light && !matchMedia("(prefers-reduced-motion: reduce)").matches;
    const shown = new Promise((r) => setTimeout(r, liftoff ? 1500 : 0));
    await player.init();
    try {
      const user = await A.restore();
      clearTimeout(slowTimer);
      await shown;
      if (user) return enter();
    } catch (e) {
      error = loginError(e);
      expired = String(e).includes("session_expired");
    }
    clearTimeout(slowTimer);
    await shown;
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

  // En estrènyer la finestra, el panell obert passaria a tapar-ho tot: es tanca (es torna a obrir amb un clic).
  let wasNarrow = false;
  $effect(() => {
    if (narrow && !wasNarrow && phase === "app") ui.panel = null;
    wasNarrow = narrow;
  });

  /** Dreceres amb Ctrl (també funcionen amb el focus en un botó). */
  const ctrlKeys: Record<string, () => void> = {
    l: () => player.current && ui.toggleLyrics(),
    "shift+l": () => player.current && toggleMini(),
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
<div
  class="app"
  class:liner={settings.look === "liner"}
  class:ona={settings.look === "ona"}
  class:fsopen={phase === "app" && ui.view.name === "lyrics" && space()}
  style:--pb={player.palette.a}
>
  {#if settings.look === "ona"}<ThemeBackground />{/if}
  <TitleBar loggedIn={phase === "app"} onlogout={doLogout} />

  {#if phase === "boot"}
    <div class="boot" class:liftoff={settings.look === "ona" && !settings.light} data-tauri-drag-region>
      {#if settings.look === "ona" && !settings.light}
        <!-- Enlairament: les estrelles de tot el cel s'ajunten on naixerà l'ona. -->
        <span class="converge" style:box-shadow={CONVERGE} aria-hidden="true"></span>
      {/if}
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
      <Sidebar collapsed={width < 900 || settings.look === "liner"} />
      <main bind:this={scroller}>
        <!-- Cada vista nova arriba des del fons de l'espai (o, enrere, des de darrere nostre). -->
        {#key ui.view}
          <div class="travel" class:back={ui.dir < 0} class:on={settings.look === "ona" && !settings.light}>
            {#if ui.view.name === "home"}
              <Home />
            {:else if ui.view.name === "search"}
              <Search />
            {:else if ui.view.name === "lyrics" && settings.look === "spotify"}
              <Lyrics />
            {:else if ui.view.name === "collection"}
              <CollectionView kind={ui.view.kind} id={ui.view.id} {scroller} />
            {/if}
          </div>
        {/key}
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
    {#if ui.view.name === "lyrics" && settings.look !== "spotify"}<NowPlayingFull />{/if}
  {/if}

  <ContextMenu />
  <Appearance />

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
  .travel.on {
    /* `backwards`: en acabar no queda cap `scale` que faci de bloc contenidor dels `fixed`/`sticky`. */
    animation: travel-in var(--t-spring) var(--spring-soft) backwards;
    transform-origin: 50% 30%;
  }
  .travel.on.back {
    animation-name: travel-back;
  }
  @keyframes travel-in {
    from {
      opacity: 0;
      scale: 0.9;
      translate: 0 18px;
    }
  }
  @keyframes travel-back {
    from {
      opacity: 0;
      scale: 1.08;
    }
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
  /* A l'estètica Ona els panells són de vidre: el full que tapa el contingut ha de ser opac. */
  .ona .narrow :global(> aside.panel) {
    background: #120f1b;
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
  .boot.liftoff {
    position: relative;
    overflow: hidden;
  }
  .converge {
    position: absolute;
    left: 50%;
    top: 50%;
    width: 1px;
    height: 1px;
    border-radius: 50%;
    animation: converge 1300ms cubic-bezier(0.55, 0, 0.2, 1) both;
  }
  @keyframes converge {
    from {
      scale: 2.6;
      opacity: 0;
    }
    35% {
      opacity: 1;
    }
    to {
      scale: 0.05;
      opacity: 0;
    }
  }
  /* L'ona neix quan les estrelles ja s'han ajuntat, amb un esclat de llum. */
  .liftoff svg {
    animation: ignite 900ms 900ms var(--spring) backwards;
    filter: drop-shadow(0 0 14px var(--accent));
  }
  .liftoff .wave {
    animation-delay: 900ms;
  }
  @keyframes ignite {
    from {
      scale: 0.3;
      opacity: 0;
    }
  }
  /* Lletra a pantalla completa en mode espai: la resta de l'app no es pinta (el Cosmos es veu a través). */
  .fsopen > :global(header.bar),
  .fsopen > .body,
  .fsopen > :global(footer.bar) {
    visibility: hidden;
  }
  @media (prefers-reduced-motion: reduce) {
    .wave {
      animation: none;
      stroke-dashoffset: 0;
    }
    .converge {
      display: none;
    }
    .liftoff svg {
      animation: none;
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
