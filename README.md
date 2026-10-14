# Ona

Client d'escriptori lleuger per a Spotify Premium (Windows). Poc consum de RAM/CPU i una interfície pròpia amb animacions treballades (disc de vinil, barra ondulada, lletres a pantalla completa, temes de fons).

## Stack
- **Tauri 2** (WebView2) + **SvelteKit / Svelte 5** (adapter-static)
- **Rust** amb `librespot` 0.8 per a la reproducció; Web API de Spotify amb un Client ID propi (OAuth PKCE, redirect `http://127.0.0.1:5588/login`)

## Requisits
- Node 22+, Rust (rustup) i Visual Studio Build Tools amb C++ (MSVC)
- Compte Spotify Premium i una app a https://developer.spotify.com (el Client ID s'introdueix a la pantalla d'inici de sessió)

## Arrencar en local
```bash
npm ci
npm run tauri dev
```

## Instal·lador
```bash
npm run tauri build
```
Genera `src-tauri/target/release/bundle/nsis/Ona_<versió>_x64-setup.exe`. Tanca Ona abans de compilar (l'exe en marxa bloqueja el build).

## Dades
`%APPDATA%\cat.ona.app\` (credencials, client ID, memòria cau d'àudio). No es versionen mai.
