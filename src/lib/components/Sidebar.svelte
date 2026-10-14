<script lang="ts">
  import * as A from "$lib/api";
  import type { Collection } from "$lib/api";
  import { ui } from "$lib/ui.svelte";
  import Cover from "./Cover.svelte";
  import Icon from "./Icon.svelte";

  let { onlogout }: { onlogout: () => void } = $props();
  let playlists = $state<Collection[]>([]);
  let confirming = $state(false);

  $effect(() => {
    A.myPlaylists().then((r) => (playlists = r)).catch(() => {});
  });

  const links = [
    { name: "home", label: "Inici", icon: "home" },
    { name: "search", label: "Cerca", icon: "search" },
    { name: "library", label: "Biblioteca", icon: "library" },
    { name: "settings", label: "Configuració", icon: "settings" },
  ] as const;

  const isOpen = (kind: string, id: string) =>
    ui.view.name === "collection" && ui.view.kind === kind && ui.view.id === id;
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && (confirming = false)} />

<nav class="side">
  <div class="links">
    {#each links as l (l.name)}
      <button class="link" class:on={ui.view.name === l.name} onclick={() => ui.go({ name: l.name })}>
        <Icon name={l.icon} size={22} />
        <span class="lbl">{l.label}</span>
      </button>
    {/each}
  </div>

  <div class="lists">
    <button class="pl" class:on={isOpen("liked", "liked")} onclick={() => ui.go({ name: "collection", kind: "liked", id: "liked" })}>
      <span class="likedicon"><Icon name="heart" size={18} /></span>
      <span class="t"><b>M'agrada</b><small>Playlist</small></span>
    </button>
    {#each playlists as p (p.id)}
      <button class="pl" title={p.name} class:on={isOpen("playlist", p.id)} onclick={() => ui.go({ name: "collection", kind: "playlist", id: p.id })}>
        <Cover src={p.image} size={42} radius={8} />
        <span class="t"><b>{p.name}</b><small>{p.subtitle}</small></span>
      </button>
    {/each}
  </div>

  <div class="userwrap">
    {#if confirming}
      <div class="confirm" role="dialog" aria-label="Tancar la sessió">
        <p>Vols tancar la sessió?</p>
        <small>Hauràs de tornar a entrar amb Spotify.</small>
        <div class="actions">
          <button class="cancel" onclick={() => (confirming = false)}>Cancel·la</button>
          <button class="danger" onclick={() => ((confirming = false), onlogout())}>Tanca la sessió</button>
        </div>
      </div>
    {/if}
    <button class="user" class:open={confirming} onclick={() => (confirming = !confirming)} title="Tanca la sessió">
      <span class="avatar">{(ui.user ?? "?").slice(0, 1).toUpperCase()}</span>
      <span class="uname">{ui.user}</span>
      <Icon name="logout" size={18} />
    </button>
  </div>
</nav>

<style>
  .side {
    width: 260px;
    flex: none;
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-left: 10px;
    min-height: 0;
  }
  .links,
  .lists,
  .user {
    border-radius: 22px;
    background: var(--panel);
    border: 1px solid var(--panel-line);
  }
  .links {
    padding: 10px;
    display: flex;
    flex-direction: column;
  }
  .link {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 10px 12px;
    border-radius: 12px;
    font-weight: 600;
    color: var(--text-2);
    transition:
      color 0.15s,
      background 0.15s;
  }
  .link:hover {
    color: var(--text);
  }
  .link.on {
    color: var(--text);
    background: rgb(255 255 255 / 0.07);
  }
  .lists {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    padding: 8px;
  }
  .pl {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 6px;
    border-radius: 12px;
    text-align: left;
    transition: background 0.15s;
  }
  .pl:hover {
    background: rgb(255 255 255 / 0.05);
  }
  .pl.on {
    background: rgb(255 255 255 / 0.09);
  }
  .likedicon {
    width: 42px;
    height: 42px;
    flex: none;
    border-radius: 8px;
    display: grid;
    place-items: center;
    color: white;
    background: linear-gradient(135deg, #4b2bd6, #c03aa6 60%, #f08a5d);
  }
  .t {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  b,
  small {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  b {
    font-weight: 550;
    color: var(--text);
  }
  .pl.on b {
    color: var(--accent);
  }
  small {
    font-size: 12px;
    color: var(--text-2);
  }
  .userwrap {
    position: relative;
  }
  .user {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 14px;
    color: var(--text-2);
    text-align: left;
  }
  .user:hover {
    color: var(--text);
  }
  .avatar {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: grid;
    place-items: center;
    font-weight: 700;
    color: #0b0b12;
    background: linear-gradient(135deg, var(--pa), var(--pb));
  }
  .uname {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .user.open {
    color: var(--text);
    background: rgb(255 255 255 / 0.07);
  }
  .confirm {
    position: absolute;
    left: 0;
    right: 0;
    bottom: calc(100% + 8px);
    z-index: 20;
    padding: 16px;
    border-radius: 20px;
    background: rgb(22 20 32 / 0.97);
    border: 1px solid rgb(255 255 255 / 0.1);
    box-shadow: 0 20px 50px -14px rgb(0 0 0 / 0.9);
    transform-origin: bottom center;
    animation: pop 0.4s var(--spring) both;
  }
  .confirm p {
    margin: 0 0 4px;
    font-weight: 650;
    color: var(--text);
  }
  .confirm small {
    color: var(--text-2);
    white-space: normal;
  }
  .actions {
    display: flex;
    gap: 8px;
    margin-top: 14px;
  }
  .actions button {
    flex: 1;
    padding: 9px 10px;
    border-radius: 99px;
    font-weight: 600;
    font-size: 13px;
    transition:
      background 0.2s,
      transform 0.3s var(--spring);
  }
  .actions button:active {
    transform: scale(0.94);
  }
  .cancel {
    background: rgb(255 255 255 / 0.1);
    color: var(--text);
  }
  .cancel:hover {
    background: rgb(255 255 255 / 0.16);
  }
  .danger {
    background: #e5484d;
    color: white;
  }
  .danger:hover {
    background: #f2555a;
  }

  /* Finestra estreta (p. ex. pantalla vertical): barra d'icones. */
  @media (max-width: 900px) {
    .side {
      width: 68px;
    }
    .links {
      padding: 6px;
    }
    .link {
      justify-content: center;
      padding: 10px 0;
    }
    .lbl,
    .t,
    .uname,
    .user :global(svg) {
      display: none;
    }
    .lists {
      padding: 6px;
      scrollbar-width: none;
    }
    .pl {
      justify-content: center;
      padding: 4px 0;
    }
    .user {
      justify-content: center;
      padding: 10px 0;
    }
    .confirm {
      right: auto;
      width: 250px;
    }
  }
</style>
