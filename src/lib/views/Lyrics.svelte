<script lang="ts">
  /** Lletra a la vista central, com al client: fons del color de la portada i la línia activa en blanc. */
  import * as A from "$lib/api";
  import type { Lyrics } from "$lib/api";
  import { player } from "$lib/player.svelte";

  const LEAD_MS = 250;
  const memo = new Map<string, Lyrics>();

  let lyrics = $state<Lyrics | undefined>(undefined);
  let box = $state<HTMLElement>();
  let userScrollAt = 0;

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
    const main = box.closest("main");
    if (el && main) main.scrollTo({ top: el.offsetTop - main.clientHeight * 0.35, behavior: "smooth" });
  });
</script>

<svelte:window onwheel={() => (userScrollAt = performance.now())} />

<section class="lyrics" style:--bg={player.palette.a} bind:this={box}>
  {#if !player.current}
    <p class="msg">Posa alguna cançó.</p>
  {:else if lyrics === undefined}
    <p class="msg">Carregant la lletra…</p>
  {:else if lyrics === null || !lyrics.lines.length}
    <p class="msg">Encara no tenim la lletra d'aquesta cançó.</p>
  {:else}
    {#each lyrics.lines as line, i (i)}
      <button
        class="line"
        class:past={lyrics.synced && i < active}
        class:now={i === active}
        class:future={lyrics.synced && i > active}
        data-i={i}
        onclick={() => lyrics?.synced && player.seekTo(line.t)}
      >
        {line.text || "♪"}
      </button>
    {/each}
    <p class="src">Lletra proporcionada per {lyrics.source}</p>
  {/if}
</section>

<style>
  .lyrics {
    min-height: 100%;
    box-sizing: border-box;
    padding: 48px 40px 30vh;
    background: var(--bg);
    transition: background 0.6s;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }
  .line {
    font-family: var(--font-title);
    font-size: clamp(24px, 3vw, 32px);
    font-weight: 700;
    line-height: 1.35;
    letter-spacing: -0.01em;
    text-align: left;
    color: #fff;
    padding: 4px 0;
    transition: color 0.2s;
  }
  .line.future {
    color: rgb(0 0 0 / 0.75);
  }
  .line.past {
    color: rgb(255 255 255 / 0.7);
  }
  .line:hover {
    color: #fff;
    text-decoration: underline;
  }
  .msg {
    font-size: 24px;
    font-weight: 700;
  }
  .src {
    margin-top: 32px;
    font-size: 14px;
    color: rgb(255 255 255 / 0.7);
  }
</style>
