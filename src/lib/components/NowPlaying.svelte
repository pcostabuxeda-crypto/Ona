<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import * as A from "$lib/api";
  import type { Lyrics } from "$lib/api";
  import { player } from "$lib/player.svelte";
  import { settings } from "$lib/settings.svelte";
  import { ui } from "$lib/ui.svelte";
  import { onFrame, setSensitivity, useSpectrum } from "$lib/viz.svelte";
  import { perf } from "$lib/perf.svelte";
  import Disc from "./Disc.svelte";
  import Icon from "./Icon.svelte";
  import PlayButton from "./PlayButton.svelte";
  import Progress from "./Progress.svelte";
  import SettingsPanel from "./SettingsPanel.svelte";
  import Visualizer from "./Visualizer.svelte";

  const LEAD_MS = 250;
  const memo = new Map<string, Lyrics>();

  let lyrics = $state<Lyrics | undefined>(undefined);
  let box = $state<HTMLDivElement>();
  let art = $state<HTMLDivElement>();
  let blobs: HTMLElement[] = [];
  let showSettings = $state(false);
  let userScrollAt = 0;
  let lastVol = 0.7;

  // ---------- Lletra ----------

  $effect(() => {
    const t = player.current;
    if (!t) return;
    if (memo.has(t.id)) {
      lyrics = memo.get(t.id);
      return;
    }
    lyrics = undefined;
    A.getLyrics(t)
      .then((l) => {
        memo.set(t.id, l);
        if (t === player.current) lyrics = l;
      })
      .catch(() => t === player.current && (lyrics = null));
  });

  const active = $derived.by(() => {
    if (!lyrics?.synced) return -1;
    const p = player.positionMs + LEAD_MS;
    let lo = 0,
      hi = lyrics.lines.length - 1,
      ans = -1;
    while (lo <= hi) {
      const mid = (lo + hi) >> 1;
      if (lyrics.lines[mid].t <= p) {
        ans = mid;
        lo = mid + 1;
      } else hi = mid - 1;
    }
    return ans;
  });

  // Centra la línia activa, tret que l'usuari estigui fent scroll ell mateix.
  $effect(() => {
    const i = active;
    if (!box || i < 0 || performance.now() - userScrollAt < 3000) return;
    const el = box.querySelector<HTMLElement>(`[data-i="${i}"]`);
    if (!el) return;
    box.scrollTo({ top: el.offsetTop - box.clientHeight * 0.4, behavior: "smooth" });
  });

  // ---------- Espectre i fons viu ----------

  $effect(() => {
    if (!(perf.viz || perf.liveBg) || !player.playing) return;
    return useSpectrum();
  });
  $effect(() => setSensitivity(settings.vizSensitivity));
  $effect(() => {
    if (!perf.liveBg) {
      blobs.forEach((b) => b && (b.style.scale = ""));
      return;
    }
    return onFrame((bass) => {
      const k = settings.bgReactivity;
      const amount = [0.6, 0.35, 0.45];
      blobs.forEach((b, i) => b && (b.style.scale = String(1 + bass * k * amount[i])));
    });
  });

  // ---------- Entrada: la portada surt de la barra, fa dues voltes i s'engrandeix ----------

  /** Camí de la portada entre la barra de baix i el disc gran: dues voltes i després creix. */
  function flight(): Keyframe[] | null {
    const mini = document.querySelector("[data-mini-cover]")?.getBoundingClientRect();
    if (!mini || !art || matchMedia("(prefers-reduced-motion: reduce)").matches) return null;
    // Mida i posició de l'art sense cap animació aplicada.
    const to = art.getBoundingClientRect();
    const fromAnim = art.getAnimations().length > 0;
    if (fromAnim) art.getAnimations().forEach((a) => a.cancel());
    const r = fromAnim ? art.getBoundingClientRect() : to;
    const dx = mini.left + mini.width / 2 - (r.left + r.width / 2);
    const dy = mini.top + mini.height / 2 - (r.top + r.height / 2);
    const s0 = mini.width / r.width;
    return [
      { transform: `translate(${dx}px, ${dy}px) scale(${s0}) rotate(0turn)`, easing: "cubic-bezier(.45,0,.2,1)" },
      { offset: 0.58, transform: `translate(0, 0) scale(${s0 * 1.5}) rotate(2turn)`, easing: "cubic-bezier(.2,.9,.25,1.25)" },
      { offset: 0.86, transform: "scale(1.05) rotate(2turn)", easing: "ease-out" },
      { transform: "scale(1) rotate(2turn)" },
    ];
  }

  let leaving = $state(false);

  onMount(() => {
    const frames = flight();
    if (frames) art!.animate(frames, { duration: 1250 });

    // Sortida: el mateix camí a l'inrevés, mentre la resta s'esvaeix i torna a aparèixer l'app.
    ui.npCloser = () => {
      if (leaving) return;
      leaving = true;
      showSettings = false;
      const back = flight();
      const done = () => {
        ui.npCloser = null;
        ui.setNowPlaying(false);
      };
      if (!back) return done();
      art!.animate(back, { duration: 1050, direction: "reverse", fill: "forwards" }).finished.then(done, done);
    };
    return () => {
      if (ui.npCloser) ui.npCloser = null;
    };
  });

  function close() {
    ui.setNowPlaying(false);
  }
  function toggleMute() {
    if (player.volume > 0) {
      lastVol = player.volume;
      player.setVolume(0);
    } else player.setVolume(lastVol || 0.7);
  }

  /** Entrada del disc nou en canviar de cançó: gira i rebota. */
  function spinIn(_: Element, { duration = 900 } = {}) {
    return {
      duration,
      easing: (t: number) => 1 - Math.cos(t * Math.PI * 2.2) * Math.exp(-t * 6.5),
      css: (t: number, u: number) => `transform: rotate(${-u * 140}deg) scale(${0.55 + 0.45 * t}); opacity: ${Math.min(1, t * 2)}`,
    };
  }
</script>

<section
  class="np"
  style:--a={player.palette.a}
  style:--b={player.palette.b}
  style:--c={player.palette.c}
  style:--blur="{Math.round(20 + settings.bgBlur * 100)}px"
  class:leaving
  in:fade|global={{ duration: 260 }}
  out:fade|global={{ duration: 120 }}
>
  <div class="bg" aria-hidden="true">
    {#if player.current}
      {#key player.current.album.image}<img src={player.current.album.image} alt="" class="blur" />{/key}
    {/if}
    <div class="aura" class:still={!perf.liveBg}>
      <i bind:this={blobs[0]}></i><i bind:this={blobs[1]}></i><i bind:this={blobs[2]}></i>
    </div>
  </div>

  {#if perf.viz && settings.vizStyle === "normal"}
    <div class="vizline">
      <Visualizer style="normal" shape={settings.vizShape} colorA={player.palette.a} colorB={player.palette.b} />
    </div>
  {/if}

  <div class="top">
    <button class="round" class:on={showSettings} onclick={() => (showSettings = !showSettings)} title="Configuració">
      <Icon name="settings" size={20} />
    </button>
    <button class="round" onclick={close} title="Tanca (Esc)"><Icon name="down" size={22} /></button>
  </div>

  {#if player.current}
    {@const t = player.current}
    <div class="card">
      <div class="art" bind:this={art}>
        {#if perf.viz && settings.vizStyle === "circular"}
          <div class="vizring">
            <Visualizer style="circular" shape={settings.vizShape} colorA={player.palette.a} colorB={player.palette.b} inner={0.31} />
          </div>
        {/if}
        {#key t.id}
          <div class="cookiewrap" in:spinIn>
            <Disc src={t.album.image} playing={player.playing} size="230px" />
          </div>
        {/key}
      </div>

      {#key t.id}
        <div class="meta">
          <h1 title={t.name}>{t.name}</h1>
          <p>{t.artists.map((a) => a.name).join(", ")}</p>
        </div>
      {/key}

      <div class="time">
        <span>{A.fmtTime(player.positionMs)}</span>
        <Progress
          value={player.progress}
          smooth={player.playing}
          wavy={perf.wave}
          playing={player.playing}
          onseek={(v) => player.seekTo(v * t.duration_ms)}
          label={(v) => A.fmtTime(v * t.duration_ms)}
        />
        <span>{A.fmtTime(t.duration_ms)}</span>
      </div>

      <div class="buttons">
        <button class="ib" class:on={player.shuffle} onclick={() => player.toggleShuffle()} title="Aleatori"><Icon name="shuffle" size={19} /></button>
        <button class="ib" onclick={() => player.prev()} title="Anterior"><Icon name="prev" size={24} /></button>
        <PlayButton size={52} />
        <button class="ib" onclick={() => player.next()} title="Següent"><Icon name="next" size={24} /></button>
        <button class="ib" class:on={player.repeat !== "off"} onclick={() => player.cycleRepeat()} title="Repeteix">
          <Icon name="repeat" size={19} />{#if player.repeat === "one"}<b class="one">1</b>{/if}
        </button>
      </div>

      <div class="vol">
        <button class="ib small" onclick={toggleMute} title="Volum"><Icon name={player.volume === 0 ? "mute" : "volume"} size={18} /></button>
        <Progress value={player.volume} onseek={(v) => player.setVolume(v)} label={(v) => `${Math.round(v * 100)}%`} />
      </div>
    </div>

    <div class="lyrics" bind:this={box} onwheel={() => (userScrollAt = performance.now())}>
      {#if lyrics === undefined}
        <p class="state">Buscant la lletra…</p>
      {:else if lyrics === null}
        <p class="state">No hem trobat la lletra d'aquesta cançó.</p>
      {:else}
        <div class="lines" class:synced={lyrics.synced}>
          {#each lyrics.lines as line, i (i)}
            {#if lyrics.synced}
              <button
                data-i={i}
                class="line"
                class:past={i < active}
                class:now={i === active}
                class:gap={!line.text}
                onclick={() => player.seekTo(line.t)}
              >
                {line.text || "♪"}
              </button>
            {:else}
              <p class="line plain">{line.text || " "}</p>
            {/if}
          {/each}
        </div>
        <span class="source">Lletra: {lyrics.source}</span>
      {/if}
    </div>
  {/if}

  {#if showSettings}
    <div class="popover" transition:fade={{ duration: 160 }}>
      <SettingsPanel compact />
    </div>
  {/if}
</section>

<style>
  .np {
    position: absolute;
    inset: 40px 0 0 0;
    z-index: 60;
    display: grid;
    grid-template-columns: minmax(320px, 380px) 1fr;
    align-items: center;
    gap: 5vw;
    padding: 3vh 5vw;
    overflow: hidden;
    background: #08080d;
  }
  .bg {
    position: absolute;
    inset: 0;
    overflow: hidden;
    pointer-events: none;
  }

  /* Sortida: tot s'esvaeix (i es veu l'app de darrere) menys el disc, que torna a la barra. */
  .np {
    transition: background-color 0.55s ease;
  }
  .np.leaving {
    background-color: transparent;
    pointer-events: none;
  }
  /* Animacions (i no transicions) perquè substitueixin les d'entrada, que tenen `fill: both`. */
  .leaving .bg,
  .leaving.np::after,
  .leaving .vizline,
  .leaving .top,
  .leaving .lyrics,
  .leaving .vizring,
  .leaving .card > :not(.art) {
    animation: leaveFade 0.4s ease forwards;
  }
  .leaving .card {
    animation: cardOut 0.45s ease forwards;
  }
  @keyframes leaveFade {
    from {
      opacity: 1;
    }
    to {
      opacity: 0;
    }
  }
  @keyframes cardOut {
    to {
      background: transparent;
      border-color: transparent;
      box-shadow: none;
    }
  }
  /* Portada borrosa: es pinta un cop i no s'anima. */
  .blur {
    position: absolute;
    inset: -15%;
    width: 130%;
    height: 130%;
    object-fit: cover;
    filter: blur(var(--blur)) saturate(1.5) brightness(0.5);
    animation: fadeIn 1.2s ease both;
  }
  @keyframes fadeIn {
    from {
      opacity: 0;
    }
  }
  /* Fons viu: taques de color que es mouen (CSS) i polsen amb els greus (JS, propietat `scale`). */
  .aura {
    position: absolute;
    inset: -20%;
    opacity: 0.6;
    mix-blend-mode: screen;
  }
  .aura i {
    position: absolute;
    width: 65%;
    height: 65%;
    border-radius: 50%;
    will-change: transform, scale;
    transition: background 1.2s;
  }
  .aura i:nth-child(1) {
    left: -5%;
    top: 0;
    background: radial-gradient(closest-side, color-mix(in oklab, var(--a) 70%, transparent), transparent);
    animation: drift1 26s ease-in-out infinite alternate;
  }
  .aura i:nth-child(2) {
    right: -10%;
    top: 20%;
    background: radial-gradient(closest-side, color-mix(in oklab, var(--b) 60%, transparent), transparent);
    animation: drift2 32s ease-in-out infinite alternate;
  }
  .aura i:nth-child(3) {
    left: 25%;
    bottom: -15%;
    background: radial-gradient(closest-side, color-mix(in oklab, var(--c) 55%, transparent), transparent);
    animation: drift3 38s ease-in-out infinite alternate;
  }
  .aura.still i {
    animation-play-state: paused;
  }
  @keyframes drift1 {
    to {
      transform: translate(18%, 14%);
    }
  }
  @keyframes drift2 {
    to {
      transform: translate(-22%, -10%);
    }
  }
  @keyframes drift3 {
    to {
      transform: translate(10%, -18%);
    }
  }
  .np::after {
    content: "";
    position: absolute;
    inset: 0;
    background: radial-gradient(120% 90% at 30% 50%, transparent, rgb(5 5 10 / 0.55));
    pointer-events: none;
  }
  .vizline {
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    height: 30vh;
    z-index: 1;
    opacity: 0.7;
    mask-image: linear-gradient(to top, black 30%, transparent);
  }
  .top {
    position: absolute;
    top: 16px;
    right: 20px;
    z-index: 5;
    display: flex;
    gap: 8px;
    animation: fadeIn 0.6s 0.4s both;
  }
  .round {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    display: grid;
    place-items: center;
    color: var(--text);
    background: rgb(255 255 255 / 0.1);
    transition:
      background 0.2s,
      transform 0.4s var(--spring);
  }
  .round:hover {
    background: rgb(255 255 255 / 0.2);
  }
  .round.on {
    background: var(--accent);
    color: #0b0b12;
    transform: rotate(60deg);
  }

  /* ---------- Reproductor compacte ---------- */
  .card {
    position: relative;
    z-index: 3;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 14px;
    padding: 30px 26px 22px;
    border-radius: 34px;
    background: rgb(14 13 22 / 0.55);
    border: 1px solid rgb(255 255 255 / 0.08);
    box-shadow: 0 30px 80px -30px rgb(0 0 0 / 0.8);
    animation: cardIn 0.7s 0.35s var(--spring-soft) both;
  }
  @keyframes cardIn {
    from {
      background: transparent;
      border-color: transparent;
      box-shadow: none;
    }
  }
  .card > :not(.art) {
    animation: rise 0.6s var(--spring-soft) both;
  }
  .card > .meta {
    animation-delay: 0.55s;
  }
  .card > .time {
    animation-delay: 0.62s;
  }
  .card > .buttons {
    animation-delay: 0.68s;
  }
  .card > .vol {
    animation-delay: 0.74s;
  }
  .art {
    position: relative;
    width: 230px;
    height: 230px;
    margin-bottom: 6px;
  }
  .vizring {
    position: absolute;
    inset: -45%;
    pointer-events: none;
  }
  .cookiewrap {
    position: relative;
  }
  .meta {
    text-align: center;
    width: 100%;
    min-width: 0;
  }
  .meta h1 {
    margin: 0 0 4px;
    font-size: 22px;
    line-height: 1.2;
    letter-spacing: 0.04em;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .meta p {
    margin: 0;
    color: var(--text-2);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .time {
    width: 100%;
    display: grid;
    grid-template-columns: 38px 1fr 38px;
    align-items: center;
    gap: 8px;
    font-size: 11px;
    color: var(--text-2);
    font-variant-numeric: tabular-nums;
  }
  .time span:first-child {
    text-align: right;
  }
  .buttons {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .ib {
    position: relative;
    width: 42px;
    height: 42px;
    border-radius: 50%;
    display: grid;
    place-items: center;
    color: var(--text);
    transition:
      color 0.2s,
      background 0.2s,
      transform 0.35s var(--spring);
  }
  .ib:hover {
    background: rgb(255 255 255 / 0.08);
  }
  .ib:active {
    transform: scale(0.85);
  }
  .ib.on {
    color: var(--accent);
  }
  .ib.small {
    width: 34px;
    height: 34px;
    color: var(--text-2);
  }
  .one {
    position: absolute;
    top: 6px;
    right: 6px;
    font-size: 9px;
  }
  .vol {
    width: 100%;
    display: grid;
    grid-template-columns: 34px 1fr;
    align-items: center;
    gap: 6px;
    padding: 0 8px 0 0;
  }

  /* ---------- Lletra ---------- */
  .lyrics {
    position: relative;
    z-index: 2;
    align-self: stretch;
    overflow-y: auto;
    scrollbar-width: none;
    mask-image: linear-gradient(to bottom, transparent, black 18%, black 78%, transparent);
    padding: 34vh 0 44vh;
    animation: rise 0.7s 0.5s var(--spring-soft) both;
  }
  .lines {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 6px;
  }
  .line {
    text-align: left;
    font-size: clamp(24px, 2.8vw, 40px);
    font-weight: 750;
    letter-spacing: -0.02em;
    line-height: 1.22;
    padding: 6px 0;
    color: var(--text);
    opacity: 0.3;
    transform-origin: left center;
    transform: scale(0.93);
    transition:
      opacity 0.45s,
      transform 0.7s var(--spring),
      text-shadow 0.6s;
  }
  .synced .line:hover {
    opacity: 0.7;
  }
  .line.past {
    opacity: 0.45;
  }
  .line.now {
    opacity: 1;
    transform: none;
    text-shadow: 0 0 34px color-mix(in oklab, var(--a) 75%, transparent);
  }
  .line.gap {
    font-size: 26px;
  }
  .line.plain {
    margin: 0;
    opacity: 0.85;
    transform: none;
    font-size: 26px;
    font-weight: 650;
  }
  .state {
    font-size: 22px;
    color: var(--text-2);
  }
  .source {
    display: block;
    margin-top: 32px;
    font-size: 12px;
    color: var(--text-3);
  }

  .popover {
    position: absolute;
    top: 66px;
    right: 20px;
    z-index: 10;
    width: min(420px, 90vw);
    max-height: calc(100% - 90px);
    overflow-y: auto;
    padding: 12px;
    border-radius: 26px;
    background: rgb(16 15 24 / 0.97);
    border: 1px solid rgb(255 255 255 / 0.08);
    box-shadow: 0 30px 70px -20px rgb(0 0 0 / 0.9);
    transform-origin: top right;
    animation: pop 0.45s var(--spring) both;
  }
  @media (prefers-reduced-motion: reduce) {
    .aura i {
      animation: none;
    }
  }

  /* Pantalla vertical o finestra estreta: reproductor compacte a dalt (en horitzontal) i lletra a sota. */
  @media (max-width: 860px) {
    .np {
      grid-template-columns: 1fr;
      grid-template-rows: auto minmax(0, 1fr);
      align-items: stretch;
      gap: 1.5vh;
      padding: 60px 4vw 0;
    }
    .card {
      display: grid;
      grid-template-columns: 150px minmax(0, 1fr);
      grid-template-areas:
        "art meta"
        "art time"
        "art buttons"
        "art vol";
      column-gap: 22px;
      row-gap: 6px;
      align-items: center;
      padding: 18px 20px;
      border-radius: 28px;
    }
    .art {
      grid-area: art;
      width: 150px;
      height: 150px;
      margin: 0;
    }
    .art :global(.disc) {
      width: 150px !important;
      height: 150px !important;
    }
    .meta {
      grid-area: meta;
      text-align: left;
    }
    .meta h1 {
      font-size: 19px;
    }
    .time {
      grid-area: time;
    }
    .buttons {
      grid-area: buttons;
      justify-content: flex-start;
      gap: 4px;
    }
    .vol {
      grid-area: vol;
    }
    .lyrics {
      padding: 14vh 0 40vh;
    }
    .line {
      font-size: clamp(20px, 4.6vw, 30px);
    }
  }
  /* Molt estret: el disc a sobre de tot, centrat. */
  @media (max-width: 520px) {
    .card {
      grid-template-columns: 1fr;
      grid-template-areas: "art" "meta" "time" "buttons" "vol";
      justify-items: center;
    }
    .meta {
      text-align: center;
    }
    .buttons {
      justify-content: center;
    }
  }
</style>
