<script lang="ts">
  let {
    onlogin,
    busy,
    error,
    hasClientId,
  }: { onlogin: (clientId?: string) => void; busy: boolean; error: string; hasClientId: boolean } = $props();
  let id = $state("");
  let editing = $state(false);
  const askId = $derived(!hasClientId || editing);
</script>

<div class="login">
  <div class="aura" aria-hidden="true"><i></i><i></i></div>
  <div class="card">
    <svg class="logo" width="88" height="56" viewBox="0 0 44 28" aria-hidden="true">
      <path d="M2 14c5-10 10-10 15 0s10 10 15 0 7-8 10-6" fill="none" stroke="url(#lg)" stroke-width="3.2" stroke-linecap="round" />
      <defs>
        <linearGradient id="lg" x1="0" x2="1"><stop offset="0" stop-color="#8b6cff" /><stop offset="1" stop-color="#ff6fb5" /></linearGradient>
      </defs>
    </svg>
    <h1>ona</h1>
    <p>Tota la música de Spotify, en una app que no se't menja l'ordinador.</p>
    {#if askId}
      <ol class="steps">
        <li>Obre <b>developer.spotify.com/dashboard</b> i crea una app.</li>
        <li>Redirect URI: <code>http://127.0.0.1:5588/login</code> · marca <b>Web API</b>.</li>
        <li>Enganxa aquí el <b>Client ID</b>.</li>
      </ol>
      <input class="cid" bind:value={id} placeholder="Client ID (32 caràcters)" spellcheck="false" />
      <button onclick={() => onlogin(id)} disabled={id.trim().length !== 32}>
        {busy ? "Torna a obrir el navegador" : "Desa i entra"}
      </button>
    {:else}
      <button onclick={() => onlogin()}>
        {busy ? "Torna a obrir el navegador" : "Entra amb Spotify"}
      </button>
      <button class="link" onclick={() => (editing = true)}>Canvia el Client ID</button>
    {/if}
    {#if busy}<small>S'obrirà el navegador (potser dues vegades). Accepta l'accés i torna aquí.</small>{/if}
    {#if error}<small class="err">{error}</small>{/if}
    <small class="note">Cal un compte Premium.</small>
  </div>
</div>

<style>
  .login {
    position: relative;
    flex: 1;
    display: grid;
    place-items: center;
    overflow: hidden;
  }
  .aura {
    position: absolute;
    inset: 0;
  }
  .aura i {
    position: absolute;
    width: 60vmax;
    height: 60vmax;
    border-radius: 50%;
    animation: float 18s ease-in-out infinite alternate;
  }
  .aura i:first-child {
    left: -15vmax;
    top: -20vmax;
    background: radial-gradient(closest-side, rgb(110 80 255 / 0.45), transparent);
  }
  .aura i:last-child {
    right: -15vmax;
    bottom: -25vmax;
    background: radial-gradient(closest-side, rgb(255 90 170 / 0.35), transparent);
    animation-duration: 24s;
  }
  @keyframes float {
    to {
      transform: translate(8vmax, 6vmax) scale(1.1);
    }
  }
  .card {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 14px;
    text-align: center;
    max-width: 420px;
    animation: rise 0.8s cubic-bezier(0.2, 0.8, 0.2, 1) both;
  }
  .logo path {
    stroke-dasharray: 80;
    stroke-dashoffset: 80;
    animation: draw 1.4s 0.2s cubic-bezier(0.6, 0, 0.2, 1) forwards;
  }
  @keyframes draw {
    to {
      stroke-dashoffset: 0;
    }
  }
  h1 {
    font-size: 56px;
    margin: 0;
    letter-spacing: -0.05em;
  }
  p {
    margin: 0 0 12px;
    color: var(--text-2);
    font-size: 16px;
    line-height: 1.5;
  }
  button {
    padding: 14px 32px;
    border-radius: 999px;
    font-weight: 700;
    font-size: 15px;
    color: #0b0b12;
    background: linear-gradient(135deg, #a58bff, #ff8cc6);
    box-shadow: 0 14px 34px -12px rgb(200 120 255 / 0.8);
    transition: transform 0.2s cubic-bezier(0.2, 0.8, 0.2, 1);
  }
  button:hover:not(:disabled) {
    transform: translateY(-2px) scale(1.03);
  }
  button:disabled {
    opacity: 0.7;
  }
  small {
    color: var(--text-2);
  }
  .steps {
    text-align: left;
    margin: 0;
    padding-left: 20px;
    color: var(--text-2);
    line-height: 1.7;
    font-size: 13px;
  }
  .steps b {
    color: var(--text);
  }
  code {
    font-size: 12px;
    padding: 1px 6px;
    border-radius: 5px;
    background: rgb(255 255 255 / 0.1);
    color: var(--text);
    user-select: text;
  }
  .cid {
    width: 100%;
    box-sizing: border-box;
    padding: 12px 16px;
    border-radius: 12px;
    border: 1px solid rgb(255 255 255 / 0.12);
    background: rgb(255 255 255 / 0.05);
    color: var(--text);
    font: inherit;
    font-family: ui-monospace, Consolas, monospace;
    text-align: center;
    outline: none;
  }
  .cid:focus {
    border-color: #a58bff;
  }
  .link {
    padding: 0;
    background: none;
    box-shadow: none;
    color: var(--text-3);
    font-weight: 500;
    font-size: 13px;
  }
  .link:hover:not(:disabled) {
    transform: none;
    color: var(--text);
  }
  .err {
    color: #ff8a8a;
  }
  .note {
    color: var(--text-3);
    margin-top: 8px;
  }
</style>
