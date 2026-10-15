<script lang="ts">
  /** Entrada amb el compte de Spotify al navegador (la sessió és la de librespot: no cal cap Client ID). */
  let { onlogin, onfree, busy, error }: { onlogin: () => void; onfree: () => void; busy: boolean; error: string } = $props();
</script>

<section class="login" data-tauri-drag-region>
  <div class="box">
    <svg width="48" height="48" viewBox="0 0 24 24" aria-hidden="true">
      <path fill="#fff" d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20m4.6 14.4a.6.6 0 0 1-.9.2c-2.4-1.4-5.3-1.8-8.8-1a.6.6 0 1 1-.3-1.2c3.8-.9 7-.5 9.7 1.1.3.2.4.6.3.9m1.2-2.7a.8.8 0 0 1-1 .3c-2.7-1.7-6.9-2.2-10.2-1.2a.8.8 0 0 1-.4-1.5c3.7-1.1 8.3-.6 11.4 1.4.3.2.4.7.2 1m.1-2.8C14.7 9 9.4 8.8 6.3 9.7a.9.9 0 1 1-.5-1.8C9.3 6.9 15.2 7.1 18.9 9.3a.9.9 0 0 1-1 1.6" />
    </svg>
    <h1>Milions de cançons.<br />Sense pagar en RAM.</h1>
    <p class="muted">Necessites un compte de Spotify Premium. S'obrirà el navegador per iniciar la sessió.</p>
    <button class="go" onclick={onlogin} disabled={busy}>{busy ? "Esperant el navegador…" : "Inicia sessió amb Spotify"}</button>
    {#if busy}<button class="retry" onclick={onlogin}>Torna a obrir el navegador</button>{/if}
    <button class="free" onclick={onfree}>Continua sense Spotify</button>
    <p class="muted small">Sense compte: música lliure d'Audius i els fitxers de la teva carpeta Música.</p>
    {#if error}<p class="err">{error}</p>{/if}
  </div>
</section>

<style>
  .login {
    flex: 1;
    display: grid;
    place-items: center;
    margin: 0 8px 8px;
    border-radius: var(--radius);
    background: linear-gradient(#2a2a2a, var(--base) 60%);
  }
  .box {
    max-width: 420px;
    padding: 32px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    text-align: center;
  }
  h1 {
    font-size: 32px;
    line-height: 1.15;
    letter-spacing: -0.02em;
  }
  .go {
    margin-top: 8px;
    padding: 14px 32px;
    border-radius: 999px;
    background: var(--accent);
    color: #000;
    font-size: 16px;
    font-weight: 700;
    transition: transform 0.1s, background 0.1s;
  }
  .go:hover:not(:disabled) {
    transform: scale(1.04);
    background: #3be477;
  }
  .go:disabled {
    opacity: 0.7;
    cursor: default;
  }
  .retry {
    font-size: 14px;
    color: var(--text-2);
    text-decoration: underline;
  }
  .free {
    padding: 12px 28px;
    border-radius: 999px;
    border: 1px solid #727272;
    font-weight: 700;
  }
  .free:hover {
    border-color: #fff;
    transform: scale(1.04);
  }
  .small {
    font-size: 13px;
    margin: 0;
  }
  .err {
    color: #f3727f;
  }
</style>
