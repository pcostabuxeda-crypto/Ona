<script lang="ts">
  /** Botó de play que canvia de forma: cercle en pausa, pastilla arrodonida mentre sona. */
  import { player } from "$lib/player.svelte";
  import Icon from "./Icon.svelte";

  let { size = 44 }: { size?: number } = $props();
</script>

<button
  class="pb"
  class:playing={player.playing}
  class:loading={player.loading}
  style:--sz="{size}px"
  onclick={() => player.toggle()}
  title={player.playing ? "Pausa" : "Reprodueix"}
>
  <span class="ic" class:show={!player.playing}><Icon name="play" size={size * 0.52} /></span>
  <span class="ic" class:show={player.playing}><Icon name="pause" size={size * 0.52} /></span>
</button>

<style>
  .pb {
    position: relative;
    width: var(--sz);
    height: var(--sz);
    border-radius: calc(var(--sz) / 2);
    display: grid;
    place-items: center;
    background: var(--text);
    color: #0b0b12;
    transition:
      width 0.5s var(--spring),
      border-radius 0.5s var(--spring),
      transform 0.35s var(--spring),
      background 0.3s;
  }
  .pb.playing {
    width: calc(var(--sz) * 1.35);
    border-radius: calc(var(--sz) * 0.3);
    background: var(--accent);
  }
  .pb:hover {
    transform: scale(1.06);
  }
  .pb:active {
    transform: scale(0.9);
  }
  .ic {
    grid-area: 1 / 1;
    display: grid;
    opacity: 0;
    transform: rotate(-90deg) scale(0.4);
    transition:
      opacity 0.2s,
      transform 0.45s var(--spring);
  }
  .ic.show {
    opacity: 1;
    transform: none;
  }
  .loading::before {
    content: "";
    position: absolute;
    inset: -5px;
    border-radius: inherit;
    border: 2px solid transparent;
    border-top-color: var(--accent);
    animation: spin 0.8s linear infinite;
  }
  @keyframes spin {
    to {
      transform: rotate(1turn);
    }
  }
</style>
