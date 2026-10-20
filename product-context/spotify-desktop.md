# Product: Spotify Desktop

<!-- Generat amb /product-context el 20 oct 2026 per a Ona (C:\Claude\Ona). Cada fet porta la font; el que no s'ha pogut verificar és TODO. -->

## Description
El client d'escriptori oficial de Spotify per a Windows, macOS i Linux. És una app CEF/Chromium (interfície web `xpui` dins de Chromium; a la Microsoft Store, el codi principal ve compilat dins `v8_context_snapshot.bin`) per escoltar el catàleg de Spotify, amb Premium o amb el pla gratuït amb anuncis. Font: anàlisi pròpia del paquet Store 1.304.258 (15 oct 2026).

**Producte propi a comparar: Ona** (C:\Claude\Ona, v0.4.0). És un client d'escriptori lleuger per a Windows fet amb Tauri 2 (WebView2), Svelte 5 i Rust/librespot 0.8. Utilitza els endpoints spclient del client natiu i no necessita Client ID. Té lletra per síl·laba, un mode lleuger, Audius i fitxers locals sense compte, i una estètica Liner opcional.

## Key Features
- Biblioteca anclada a l'esquerra ("Your Library") i vista "Now Playing" a la dreta, redissenyades. Font: [Spotify Newsroom](https://newsroom.spotify.com/?p=23515). <!-- TODO: data exacta; la cobertura apunta al 2021 -->
- Cua editable i "recently played"; baixades per escoltar offline amb Premium (des del redisseny d'escriptori). Fonts: [TechCrunch](https://techcrunch.com/?p=2129990), [What Hi-Fi](https://www.whathifi.com/news/spotifys-overhauled-desktop-app-lets-subscribers-listen-offline).
- Spotify Connect, lletres sincronitzades per línia (`color-lyrics`, `LINE_SYNCED` en 37 de 40 pistes provades; cap amb síl·labes), fitxers locals. Font: proves pròpies amb librespot (15 oct 2026).
- 2026: "Listening Activity" opcional (mostrar què escoltes a Missatges). Font: [BGR](https://www.bgr.com/2179163/best-features-added-spotify-2026/).

## Pricing (as of October 2026)
Increment als EUA a partir de la facturació de febrer de 2026 ([Android Authority](https://www.androidauthority.com/spotify-premium-us-price-hike-3632761), [Audio Advice](https://www.audioadvice.com/blogs/expert-advice/spotify-is-raising-prices-again-in-2026)):

| Pla | Abans | 2026 |
|---|---|---|
| Individual | $11.99 | $12.99 |
| Student | $5.99 | $6.99 |
| Duo | $16.99 | $18.99 |
| Family | $19.99 | $21.99 |

Gratuït amb anuncis: no permet reproducció a clients de tercers basats en librespot (Psst, ncspot, spotify-player i Ona exigeixen Premium). <!-- TODO: preus a Espanya/UE 2026 -->

## Recent Releases
<!-- Keep this section updated with recent launches and changes -->
- 2026: Listening Activity a Missatges ([BGR](https://www.bgr.com/2179163/best-features-added-spotify-2026/)).
- 2026: versió Windows Store 1.2.86 amb CPU alta en repòs (~30 %), corregida en una versió posterior segons el personal de Spotify ([Community](https://community.spotify.com/t5/Desktop-Windows/High-idle-CPU-usage-30/td-p/7390422/highlight/true)).
- Febrer 2026: la Web API de Spotify exigeix un Client ID propi per a funcions de cerca, biblioteca i playlists en clients de tercers (Psst ho documenta: [psst](https://github.com/jpochyla/psst)).
- Octubre 2026: versió Store 1.304.258 (analitzada per a Ona).
<!-- TODO: no s'han trobat notes de versió oficials del 2026 per al client d'escriptori -->

## Known Pain Points
**Rendiment i recursos (Alt, persistent)**
- CPU alta en repòs: ~30 % a la 1.2.86 Store (2026, resolt amb una actualització) ([fil](https://community.spotify.com/t5/Desktop-Windows/High-idle-CPU-usage-30/td-p/7390422/highlight/true)).
- RAM: mesurat el 15 oct 2026 en aquesta màquina, sonant: **1.136 MB de working set / ~840 MB de privada en 9 processos** (Ona: 216 MB). Hi ha històric de fuites de memòria a Windows i Mac, que el suport resol "reinstal·lant" ([Windows](https://community.spotify.com/t5/Desktop-Windows/High-CPU-Usage-amp-Memory-Leak/td-p/4441639), [Mac](https://community.spotify.com/t5/Desktop-Mac/High-memory-usage/m-p/4626609)).
- Escriptura massiva al disc en repòs (antic, corregit a la 1.0.42) ([Hypebot](https://www.hypebot.com/spotify-is-dumping-huge-amounts-of-junk-data-on-your-hard-drive-fixed-promised/)).

**Fiabilitat de reproducció (Mitjà-Alt, recurrent)**
- La música s'atura als 5-6 s; l'app peta al cap d'uns 5 minuts; "Can't play this right now"; els fitxers locals s'aturen a 0 s ([Community](https://community.spotify.com/t5/Desktop-Windows/Music-stop-playing-after-5-seconds/m-p/4747155), [crashes](https://community.spotify.com/t5/Desktop-Windows/Desktop-app-constantly-crashing/m-p/5238408/highlight/true), [Ongoing Issues](https://community.spotify.com/t5/Ongoing-Issues/idb-p/ongoing_issues/tab/most-recent/page/3?l=problems)).
- Mac (febrer 2026): l'Inici surt en blanc i els M'agrada no se sincronitzen amb el mòbil ([Community](https://community.spotify.com/t5/Desktop-Mac/Spotify-Desktop-App-Not-Working-on-MacBook/td-p/7343715)).

**Funcions que falten a escriptori (Mitjà, persistent)**
- Equalitzador (sí que n'hi ha al mòbil), feed "What's New", opcions de Family Mix, cercar dins d'una playlist, amplada de columnes ([idees](https://community.spotify.com/t5/Desktop-Windows/Please-add-an-equalizer-to-pc-desktop-spotify/td-p/4872606), [What's New](https://community.spotify.com/t5/Desktop-Windows/quot-What-s-new-quot-on-Spotify-for-PC/m-p/5634960/highlight/true)).
- Funcions que desapareixen en proves A/B (canviar la portada de les playlists, multiselecció) ([Community](https://community.spotify.com/t5/Desktop-Windows/Desktop-app-unusable/m-p/4981833/highlight/true)).

**Personalització (Mitjà)**
- No es pot personalitzar; per això existeix Spicetify, que es pot trencar amb les actualitzacions ([font de poca autoritat](https://factually.co/fact-checks/electronics-tech/safer-ways-to-customize-spotify-interface-tos-safe-options-a58176)).

**Lletres (Baix-Mitjà)**
- Només per línia, sense síl·labes; algunes pistes no en tenen (3 de 40 en la prova pròpia). Font: prova pròpia, 15 oct 2026.

**Preu (Mitjà, recent)**
- Increment del 2026 sense funcions noves associades ([Android Authority](https://www.androidauthority.com/spotify-premium-us-price-hike-3632761)).

## Customer Segments
- **Usuari general Premium a Windows/Mac:** escolta en segon pla; li preocupen la RAM i la CPU i que l'app no peti. És el segment amb més queixes al fòrum.
- **Usuaris amb maquinari modest o portàtils:** els afecten més la RAM, la CPU en repòs i la bateria. <!-- TODO: poques dades directes -->
- **Personalitzadors (Spicetify):** volen temes, extensions i lletres millorades; toleren que s'espatlli en actualitzar.
- **Usuaris de terminal i Linux/BSD (ncspot, spotify-player):** busquen poc consum, Vim i control per IPC.
- **Usuaris sense Premium (Spotube):** volen escoltar sense pagar, amb fonts alternatives com YouTube o plugins.
- **Amants de les lletres i el karaoke (Liner, Better Lyrics):** volen lletra per síl·laba i fons visuals.
- **Pantalles verticals o estretes:** és el cas de l'usuari d'Ona (608×1080). <!-- TODO: cap dada pública; segment probablement sense veu als fòrums -->

## Competitors
- **Spicetify:** personalitza el client oficial (temes, extensions, apps pròpies, marketplace de més de 200 elements, CLI) per a Windows, macOS i Linux ([spicetify.app](https://spicetify.app/)). No el substitueix, i per tant no redueix el consum.
- **Liner** (tryliner/desktop): Electron, React 19 i un signador en Rust/WASM. Lletra per síl·laba (braccato) i fons WebGL (kawarp). Funciona sense compte de Spotify, contra un backend tancat (`api.tryliner.fun`). Llicència *source-available* no comercial, i actualitza el codi remotament (OTA) cada 15 min. Font: lectura del repositori el 15 oct 2026. Per això, com a inspiració visual però no com a font de codi.
- **Spotube:** Flutter/Dart, BSD-4-Clause, 50,1k estrelles. Escriptori i mòbil. L'àudio i les metadades venen de plugins (eines de YouTube: Piped, Invidious, yt-dlp), amb lletra sincronitzada (LRCLib), baixades i sense telemetria. No necessita Premium. ([GitHub](https://github.com/KRTirtho/spotube))
- **Psst:** Rust amb GUI nativa (Druid), MIT. Windows, Linux i macOS. Necessita Premium, i des del febrer de 2026 també un Client ID propi per a cerca, biblioteca i playlists. Està en una fase inicial i li falten funcions (no té Spotify Connect). ([GitHub](https://github.com/jpochyla/psst))
- **ncspot:** Rust, librespot i ncurses, BSD-2-Clause. Necessita Premium. "Small resource footprint", tecles de Vim, IPC, també als BSD. ([GitHub](https://github.com/hrkfdn/ncspot))
- **spotify-player:** Rust, ratatui, rspotify i librespot, MIT. Necessita Premium. Spotify Connect, lletra sincronitzada, integració amb els controls multimèdia del sistema (MPRIS i Windows), portades al terminal, mode dimoni i CLI amb sortida JSON. ([GitHub](https://github.com/aome510/spotify-player))
- **Reproductor web oficial** (open.spotify.com): alternativa sense instal·lació. <!-- TODO: consum comparat -->

## Community Forum
- Oficial: https://community.spotify.com/t5/Desktop-Windows/bd-p/desktop_windows (subfòrum Desktop-Windows, que segons diversos fils és només de lectura per als usuaris). Les idees van a l'Ideas Exchange, i els problemes reconeguts, a [Ongoing Issues](https://community.spotify.com/t5/Ongoing-Issues/idb-p/ongoing_issues).
- Alternatives: el subreddit r/spotify, el Discord i el subreddit de Spicetify, i els GitHub Issues de Spotube, Psst, ncspot i spotify-player. <!-- TODO: verificar l'URL exacta de r/spotify -->

## Additional Search Terms
- "Spotify desktop RAM" / "Spotify memory leak Windows" / "Spotify high CPU idle"
- "Spotify desktop app slow" / "Spotify Electron" / "Spotify CEF"
- "Spotify lyrics not showing desktop" / "Spotify synced lyrics" / "Spotify karaoke lyrics"
- "Spotify queue bug" / "Spotify queue disappears"
- "Spotify offline downloads desktop not working"
- "Spotify desktop equalizer"
- "Spotify can't play this right now"
- "Spotify lightweight client" / "Spotify alternative client Windows"
- "Spicetify broke after update"
- "Spotube vs Spotify" / "ncspot" / "spotify-player" / "psst spotify"
- "Spotify Windows Store version CPU"
- "Spotify desktop media keys not working"

## Anàlisi de competidors amb codi (20 oct 2026)
Codi a `~/workspace/competitors/ona/` (clons `--depth 1`). Commits analitzats: spotify-player 7dc9d17 (19 set), ncspot cec0f53 (9 oct), psst 3c3621a (17 ago), Spotifast 7c78448 (20 oct), Spicetify cli 37ea81c (18 oct), Spotube 69a310c (5 jun; el repo s'ha mogut a team-spotube/spotube, el clon pot ser antic). Llicències: MIT (psst, spotify-player, Spotifast), BSD-2 (ncspot), BSD-4 (Spotube), LGPL-2.1 (Spicetify) → només idees, cap codi.

Judicis que canvien decisions (inferència, amb el que els faria caure):
- **La RAM ja no és un avantatge únic**: Spotifast (egui, librespot, MIT, 8,5k estrelles en 2 mesos) declara 100–250 MB (README:26), el rang d'Ona (216 MB). Ona ha de guanyar per lletra, estètica i pantalla estreta. Cauria si una mesura A/B a la mateixa màquina dona a Spotifast clarament per sota de 150 MB.
- **La cua viu a la WebView** (`player.svelte.ts:123` avança en JS): no es pot tancar la WebView per estalviar RAM en segon pla, cosa que Spotifast sí fa (safata + SMTC en finestra pròpia, `thumbbar.rs:4-10`). Moure l'avanç de cua a Rust és el requisit previ per a una safata de ~20-40 MB. 待验证: RAM d'ona.exe sol sonant amb la WebView destruïda.
- **"Afegeix a la cua" posa la pista al final del context** (`player.svelte.ts:261-264`), no just després de la cua d'usuari com Spotify/Psst (`psst-core/src/player/queue.rs:16-60`), i l'aleatori la barreja. Cal una cua d'usuari FIFO separada.
- **Lletra**: Spotifast cacheja a disc 30 dies i, si falla `/api/get`, cerca a LRCLIB i puntua per durada (`lyrics.rs:21,121-135,283-309`); Ona no cacheja ni cerca. Spotube i Spicetify tenen retard ajustable; Spicetify fa lletra flotant amb canvas→`captureStream`→Picture-in-Picture (`Extensions/popupLyrics.js:411-422`), i Spotube una mini-finestra de lletra sempre a sobre de 400×150 (`pages/lyrics/mini_lyrics.dart:107-153`).
