<script lang="ts">
  import { getContext } from "svelte";
  import { settings } from "$lib/settings.svelte";
  import { THEMES, saveCustomImage, type Theme } from "$lib/themes";
  import { ui } from "$lib/ui.svelte";
  import Icon from "./Icon.svelte";
  import Progress from "./Progress.svelte";

  const bgImage = getContext<{ value: string | null } | undefined>("bgImage");
  let fileInput = $state<HTMLInputElement>();

  function pick(t: Theme) {
    if (t.id === "custom" && !bgImage?.value) return fileInput?.click();
    settings.theme = t.id;
    settings.effect = t.effect;
  }

  async function onFile(e: Event) {
    const file = (e.currentTarget as HTMLInputElement).files?.[0];
    if (!file || !bgImage) return;
    try {
      bgImage.value = await saveCustomImage(file);
      settings.theme = "custom";
    } catch {
      ui.say("No s'ha pogut fer servir aquesta imatge");
    }
  }

  let { compact = false }: { compact?: boolean } = $props();
</script>

{#snippet toggle(on: boolean, set: (v: boolean) => void, label: string)}
  <button class="toggle" class:on role="switch" aria-checked={on} aria-label={label} onclick={() => set(!on)}>
    <span class="knob"></span>
  </button>
{/snippet}

{#snippet segmented<T extends string>(value: T, options: [T, string][], set: (v: T) => void)}
  <div class="seg">
    {#each options as [v, label] (v)}
      <button class:on={value === v} onclick={() => set(v)}>{label}</button>
    {/each}
  </div>
{/snippet}

<div class="panel" class:compact>
  <section class="eco">
    <div class="row">
      <span>
        <strong>Mode estalvi</strong>
        <small>Treu el visualitzador, el fons viu, els efectes i l'ona de la barra. Gasta el mínim.</small>
      </span>
      {@render toggle(settings.eco, (v) => (settings.eco = v), "Mode estalvi")}
    </div>
  </section>

  {#if !compact}
    <section>
      <h3>Tema</h3>
      <div class="themes">
        {#each THEMES as t (t.id)}
          <button class="theme" class:on={settings.theme === t.id} onclick={() => pick(t)}>
            <span
              class="swatch"
              style:background={t.id === "custom" && bgImage?.value ? `center / cover url(${bgImage.value})` : t.background}
            >
              {#if t.id === "cover"}<Icon name="album" size={22} />{/if}
              {#if t.id === "custom" && !bgImage?.value}<Icon name="queueAdd" size={22} />{/if}
              {#if t.accent}<i style:background={t.accent}></i>{/if}
            </span>
            <span class="tname">{t.name}</span>
          </button>
        {/each}
      </div>
      <input bind:this={fileInput} type="file" accept="image/*" hidden onchange={onFile} />
      {#if settings.theme === "custom"}
        <button class="change" onclick={() => fileInput?.click()}>Canvia la imatge…</button>
      {/if}
      <div class="row">
        <span>Efecte</span>
        {@render segmented(settings.effect, [["none", "Cap"], ["stars", "Estrelles"], ["bubbles", "Bombolles"]], (v) => (settings.effect = v))}
      </div>
      <div class="row slider" class:dim={settings.theme === "ona"}>
        <span>Opacitat dels panells <b>{Math.round(settings.panelAlpha * 100)}%</b></span>
        <Progress
          value={(settings.panelAlpha - 0.15) / 0.75}
          onseek={(v) => (settings.panelAlpha = 0.15 + v * 0.75)}
          label={(v) => `${Math.round((0.15 + v * 0.75) * 100)}%`}
        />
      </div>
      <div class="row slider" class:dim={settings.theme === "ona"}>
        <span>Enfosquiment del fons <b>{Math.round((settings.bgDim / 0.8) * 100)}%</b></span>
        <Progress value={settings.bgDim / 0.8} onseek={(v) => (settings.bgDim = v * 0.8)} label={(v) => `${Math.round(v * 100)}%`} />
      </div>
    </section>
  {/if}

  <section>
    <h3>Visualitzador</h3>
    <div class="row">
      <span>Mostra les ones de la música</span>
      {@render toggle(settings.vizEnabled, (v) => (settings.vizEnabled = v), "Visualitzador")}
    </div>
    <div class="row" class:dim={!settings.vizEnabled}>
      <span>Estil</span>
      {@render segmented(settings.vizStyle, [["normal", "Normal"], ["circular", "Circular"]], (v) => (settings.vizStyle = v))}
    </div>
    <div class="row" class:dim={!settings.vizEnabled}>
      <span>Forma</span>
      {@render segmented(settings.vizShape, [["bars", "Barres"], ["wave", "Ona"]], (v) => (settings.vizShape = v))}
    </div>
    <div class="row slider" class:dim={!settings.vizEnabled}>
      <span>Sensibilitat <b>{Math.round(settings.vizSensitivity * 100)}%</b></span>
      <Progress
        value={(settings.vizSensitivity - 0.5) / 1.5}
        onseek={(v) => (settings.vizSensitivity = 0.5 + v * 1.5)}
        label={(v) => `${Math.round((0.5 + v * 1.5) * 100)}%`}
      />
    </div>
  </section>

  <section>
    <h3>Fons</h3>
    <div class="row">
      <span>Fons viu (es mou amb el ritme)</span>
      {@render toggle(settings.liveBg, (v) => (settings.liveBg = v), "Fons viu")}
    </div>
    <div class="row slider">
      <span>Desenfocament <b>{Math.round(settings.bgBlur * 100)}%</b></span>
      <Progress value={settings.bgBlur} onseek={(v) => (settings.bgBlur = v)} label={(v) => `${Math.round(v * 100)}%`} />
    </div>
    <div class="row slider" class:dim={!settings.liveBg}>
      <span>Reactivitat <b>{Math.round(settings.bgReactivity * 100)}%</b></span>
      <Progress value={settings.bgReactivity} onseek={(v) => (settings.bgReactivity = v)} label={(v) => `${Math.round(v * 100)}%`} />
    </div>
  </section>
  {#if !compact}
    <p class="note">
      Només decoratiu: no canvia la qualitat del so. L'espectre només es calcula mentre la pantalla de lletra és oberta i
      sona música; en pausa no gasta res.
    </p>
  {/if}
</div>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    gap: 22px;
    max-width: 720px;
  }
  section {
    padding: 18px 20px;
    border-radius: 20px;
    background: rgb(255 255 255 / 0.04);
    border: 1px solid rgb(255 255 255 / 0.06);
  }
  .compact section {
    padding: 14px 16px;
    background: rgb(255 255 255 / 0.05);
  }
  h3 {
    margin: 0 0 12px;
    font-size: 16px;
  }
  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    min-height: 40px;
    color: var(--text);
    transition: opacity 0.2s;
  }
  .row.slider {
    flex-direction: column;
    align-items: stretch;
    gap: 2px;
    padding-top: 6px;
  }
  .row b {
    color: var(--text-2);
    font-weight: 500;
    margin-left: 6px;
  }
  .dim {
    opacity: 0.4;
    pointer-events: none;
  }
  .toggle {
    position: relative;
    width: 52px;
    height: 30px;
    flex: none;
    border-radius: 99px;
    background: rgb(255 255 255 / 0.12);
    border: 2px solid rgb(255 255 255 / 0.2);
    transition:
      background 0.25s,
      border-color 0.25s;
  }
  .toggle.on {
    background: var(--accent);
    border-color: var(--accent);
  }
  .knob {
    position: absolute;
    top: 50%;
    left: 6px;
    width: 14px;
    height: 14px;
    margin-top: -7px;
    border-radius: 50%;
    background: rgb(255 255 255 / 0.7);
    transition:
      transform 0.45s var(--spring),
      width 0.3s var(--spring),
      height 0.3s var(--spring),
      margin 0.3s var(--spring),
      background 0.25s;
  }
  .toggle.on .knob {
    width: 22px;
    height: 22px;
    margin-top: -11px;
    transform: translateX(18px);
    background: #0b0b12;
  }
  .toggle:active .knob {
    width: 26px;
  }
  .seg {
    display: flex;
    padding: 3px;
    border-radius: 99px;
    background: rgb(255 255 255 / 0.07);
  }
  .seg button {
    padding: 6px 14px;
    border-radius: 99px;
    color: var(--text-2);
    font-weight: 550;
    font-size: 13px;
    transition:
      background 0.3s var(--spring),
      color 0.2s;
  }
  .seg button.on {
    background: var(--accent);
    color: #0b0b12;
  }
  .themes {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(104px, 1fr));
    gap: 10px;
    margin-bottom: 10px;
  }
  .theme {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 6px;
    border-radius: 16px;
    transition:
      background 0.2s,
      transform 0.35s var(--spring);
  }
  .theme:hover {
    background: rgb(255 255 255 / 0.05);
    transform: translateY(-2px);
  }
  .swatch {
    position: relative;
    aspect-ratio: 16 / 10;
    border-radius: 12px;
    display: grid;
    place-items: center;
    color: var(--text-2);
    box-shadow: inset 0 0 0 1px rgb(255 255 255 / 0.1);
    transition: box-shadow 0.25s;
  }
  .swatch i {
    position: absolute;
    right: 7px;
    bottom: 7px;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    box-shadow: 0 0 0 2px rgb(0 0 0 / 0.35);
  }
  .theme.on .swatch {
    box-shadow:
      inset 0 0 0 1px rgb(255 255 255 / 0.1),
      0 0 0 3px var(--accent);
  }
  .tname {
    font-size: 13px;
    font-weight: 550;
    color: var(--text-2);
    text-align: left;
  }
  .theme.on .tname {
    color: var(--text);
  }
  .change {
    margin: 0 0 8px;
    padding: 7px 14px;
    border-radius: 99px;
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
    background: rgb(255 255 255 / 0.08);
  }
  .eco strong {
    display: block;
    font-size: 15px;
  }
  .eco small {
    display: block;
    margin-top: 2px;
    color: var(--text-2);
    font-size: 12.5px;
  }
  .note {
    margin: 0;
    font-size: 13px;
    line-height: 1.5;
    color: var(--text-3);
  }
</style>
