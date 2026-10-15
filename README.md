# Ona

Client d'escriptori lleuger per a Spotify Premium (Windows). Poc consum de RAM/CPU i la mateixa disposició que el client oficial (biblioteca, vista central, panell Ara sona, barra del reproductor, lletra), sense extres pesats.

## Stack
- **Tauri 2** (WebView2) + **SvelteKit / Svelte 5** (adapter-static)
- **Rust** amb `librespot` 0.8 per a la reproducció i les dades: els mateixos endpoints spclient que el client d'escriptori (metadades protobuf per lots, playlist4, col·lecció, context-resolve). No cal cap Client ID ni la Web API.

## Funcions
- Lletra per síl·laba ([Better Lyrics](https://better-lyrics.boidu.dev), API gratuïta) pintada amb `@braccato/core`, sobre un fons fluid fet amb la portada (`@kawarp/core`, WebGL). Si no n'hi ha, la de Spotify (per línies) o LRCLIB.
- **Estètica Liner** (menú del perfil): interfície fosca i neutra, barra d'icones i lletra a pantalla completa amb la portada i els controls a l'esquerra. Inspirada en [Liner](https://github.com/tryliner/desktop), però amb codi propi.
- **Mode lleuger** (menú del perfil): lletra simple per línies i fons de color pla, sense WebGL ni animació per síl·laba.
- **Sense Spotify**: música lliure d'[Audius](https://audius.co) (API pública) i els fitxers de la carpeta Música (`Artista - Títol.mp3`). Es pot entrar sense compte amb «Continua sense Spotify».

## Requisits
- Node 22+, Rust (rustup) i Visual Studio Build Tools amb C++ (MSVC)
- Compte Spotify Premium per al catàleg de Spotify (l'inici de sessió s'obre al navegador); sense compte, només Audius i fitxers locals

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
