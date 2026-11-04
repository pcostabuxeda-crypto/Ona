<script lang="ts">
  /** Finestra d'aparença: estètica (Spotify, Liner o Ona) i, per a Ona, fons, efecte, disc i visualitzador. */
  import { settings, setSetting, type Effect, type Look } from "$lib/settings.svelte";
  import { THEMES } from "$lib/themes";
  import { ui } from "$lib/ui.svelte";
  import Icon from "./Icon.svelte";

  const LOOKS: { id: Look; name: string; hint: string; swatch: string }[] = [
    { id: "ona", name: "Ona", hint: "Fons de colors, disc de vinil i ona", swatch: "linear-gradient(160deg, #2b2fd9, #b521d4)" },
    { id: "spotify", name: "Spotify", hint: "Com el client oficial", swatch: "linear-gradient(160deg, #1ed760 0 30%, #121212 30%)" },
    { id: "liner", name: "Liner", hint: "Fosc i neutre, lletra a pantalla completa", swatch: "linear-gradient(160deg, #fff 0 22%, #0f0f11 22%)" },
  ];
  const EFFECTS: { id: Effect; name: string }[] = [
    { id: "stars", name: "Estrelles" },
    { id: "bubbles", name: "Bombolles" },
    { id: "none", name: "Cap" },
  ];

  let dialog: HTMLDialogElement;
  $effect(() => {
    if (ui.appearance && !dialog.open) dialog.showModal();
    else if (!ui.appearance && dialog.open) dialog.close();
  });
</script>

<dialog bind:this={dialog} onclose={() => (ui.appearance = false)} onclick={(e) => e.target === dialog && dialog.close()} aria-labelledby="ap-title">
  <div class="sheet">
    <header>
      <h2 id="ap-title">Aparença</h2>
      <button class="x" onclick={() => dialog.close()} title="Tanca"><Icon name="close" size={16} /></button>
    </header>

    <fieldset>
      <legend>Estètica</legend>
      <div class="looks">
        {#each LOOKS as l (l.id)}
          <label class="look" class:on={settings.look === l.id}>
            <input type="radio" name="look" value={l.id} checked={settings.look === l.id} onchange={() => setSetting("look", l.id)} />
            <span class="swatch" style:background={l.swatch}></span>
            <b>{l.name}</b>
            <span class="hint">{l.hint}</span>
          </label>
        {/each}
      </div>
    </fieldset>

    {#if settings.look === "ona"}
      <fieldset>
        <legend>Fons</legend>
        <div class="themes">
          {#each THEMES as t (t.id)}
            <label class="theme" class:on={settings.theme === t.id} title={t.name}>
              <input type="radio" name="theme" value={t.id} checked={settings.theme === t.id} onchange={() => setSetting("theme", t.id)} />
              <span class="dot" style:background={t.id === "cover" ? "conic-gradient(#f59b73, #5b3cc4, #1d6fa3, #f59b73)" : t.background}></span>
              <span class="tname">{t.name}</span>
            </label>
          {/each}
        </div>
      </fieldset>

      <fieldset>
        <legend>Efecte del fons</legend>
        <div class="chips">
          {#each EFFECTS as e (e.id)}
            <label class="chip" class:on={settings.effect === e.id}>
              <input type="radio" name="effect" value={e.id} checked={settings.effect === e.id} onchange={() => setSetting("effect", e.id)} />
              {e.name}
            </label>
          {/each}
        </div>
      </fieldset>

      <fieldset>
        <legend>Extres</legend>
        <label class="row">
          <span><b>Disc de vinil</b><span class="hint">La portada gira mentre sona</span></span>
          <input type="checkbox" role="switch" checked={settings.disc} onchange={(e) => setSetting("disc", e.currentTarget.checked)} />
        </label>
        <label class="row">
          <span><b>Visualitzador</b><span class="hint">Barres que es mouen amb la música (gasta una mica més)</span></span>
          <input type="checkbox" role="switch" checked={settings.viz} onchange={(e) => setSetting("viz", e.currentTarget.checked)} />
        </label>
      </fieldset>
    {/if}

    <fieldset>
      <legend>Consum</legend>
      <label class="row">
        <span><b>Mode lleuger</b><span class="hint">Lletra simple, sense efectes ni visualitzador</span></span>
        <input type="checkbox" role="switch" checked={settings.light} onchange={(e) => setSetting("light", e.currentTarget.checked)} />
      </label>
    </fieldset>
  </div>
</dialog>

<style>
  dialog {
    padding: 0;
    border: 0;
    border-radius: 24px;
    background: #15121e;
    color: var(--text);
    width: min(460px, calc(100vw - 32px));
    max-height: calc(100vh - 48px);
    box-shadow: 0 30px 80px -20px rgb(0 0 0 / 0.8), 0 0 0 1px rgb(255 255 255 / 0.08);
  }
  dialog[open] {
    animation: pop var(--t-spring) var(--spring) both;
  }
  dialog::backdrop {
    background: rgb(4 3 10 / 0.6);
  }
  @keyframes pop {
    from {
      opacity: 0;
      transform: scale(0.94) translateY(12px);
    }
  }
  .sheet {
    padding: 20px 22px 24px;
  }
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 8px;
  }
  h2 {
    font-size: 22px;
    letter-spacing: -0.03em;
  }
  .x {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    display: grid;
    place-items: center;
    color: var(--text-2);
  }
  .x:hover {
    background: rgb(255 255 255 / 0.08);
    color: var(--text);
  }
  fieldset {
    border: 0;
    padding: 0;
    margin: 18px 0 0;
  }
  legend {
    font-size: 13px;
    font-weight: 700;
    color: var(--text-2);
    margin-bottom: 10px;
  }
  input[type="radio"] {
    position: absolute;
    opacity: 0;
    pointer-events: none;
  }
  label:has(input:focus-visible) {
    outline: 2px solid #fff;
    outline-offset: 2px;
  }
  .looks {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }
  .look {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 10px;
    border-radius: 16px;
    background: rgb(255 255 255 / 0.05);
    box-shadow: inset 0 0 0 1px rgb(255 255 255 / 0.06);
    cursor: pointer;
    transition: scale var(--t-spring) var(--spring);
  }
  .look:active {
    scale: 0.95;
  }
  .look.on {
    box-shadow: inset 0 0 0 2px #fff;
  }
  .swatch {
    height: 54px;
    border-radius: 10px;
    margin-bottom: 4px;
  }
  .hint {
    display: block;
    font-size: 12px;
    color: var(--text-2);
    line-height: 1.35;
  }
  .themes {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: 6px;
  }
  .theme {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    cursor: pointer;
    font-size: 11px;
    color: var(--text-2);
    text-align: center;
  }
  .dot {
    width: 44px;
    height: 44px;
    border-radius: 50%;
    box-shadow: 0 0 0 1px rgb(255 255 255 / 0.12);
    transition:
      scale var(--t-spring) var(--spring),
      box-shadow var(--t-quick);
  }
  .theme:active .dot {
    scale: 0.88;
  }
  .theme.on .dot {
    box-shadow:
      0 0 0 2px #15121e,
      0 0 0 4px #fff;
  }
  .theme.on {
    color: var(--text);
  }
  .chips {
    display: flex;
    gap: 6px;
    flex-wrap: wrap;
  }
  .chip {
    position: relative;
    padding: 7px 14px;
    border-radius: 99px;
    background: rgb(255 255 255 / 0.07);
    cursor: pointer;
    font-size: 13px;
    transition: scale var(--t-spring) var(--spring);
  }
  .chip:active {
    scale: 0.93;
  }
  .chip.on {
    background: #fff;
    color: #000;
  }
  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 8px 0;
    cursor: pointer;
  }
  /* Interruptor: el polze llisca amb molla. */
  input[type="checkbox"] {
    appearance: none;
    flex: none;
    position: relative;
    width: 44px;
    height: 26px;
    margin: 0;
    border-radius: 99px;
    background: rgb(255 255 255 / 0.16);
    cursor: pointer;
    transition: background var(--t-quick);
  }
  input[type="checkbox"]::after {
    content: "";
    position: absolute;
    top: 3px;
    left: 3px;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: #fff;
    transition: translate var(--t-spring) var(--spring);
  }
  input[type="checkbox"]:checked {
    background: #fff;
  }
  input[type="checkbox"]:checked::after {
    translate: 18px 0;
    background: #15121e;
  }
  input[type="checkbox"]:focus-visible {
    outline: 2px solid #fff;
    outline-offset: 2px;
  }
  @media (max-width: 480px) {
    .themes {
      grid-template-columns: repeat(3, 1fr);
    }
  }
</style>
