# Ona

Client d'escriptori lleuger per a Spotify Premium (Windows). Poc consum de RAM/CPU i la mateixa disposició que el client oficial (biblioteca, vista central, panell Ara sona, barra del reproductor, lletra), sense extres pesats.

## Stack
- **Tauri 2** (WebView2) + **SvelteKit / Svelte 5** (adapter-static)
- **Rust** amb `librespot` 0.8 per a la reproducció i les dades: els mateixos endpoints spclient que el client d'escriptori (metadades protobuf per lots, playlist4, col·lecció, context-resolve). No cal cap Client ID ni la Web API.

## Requisits
- Node 22+, Rust (rustup) i Visual Studio Build Tools amb C++ (MSVC)
- Compte Spotify Premium (l'inici de sessió s'obre al navegador)

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
`%APPDATA%\cat.ona.app\` (credencials i memòria cau d'àudio). No es versionen mai.
