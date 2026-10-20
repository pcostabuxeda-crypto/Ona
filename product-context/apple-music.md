# Product: Apple Music

<!-- Generat amb /product-context el 20 oct 2026 per a Ona (C:\Claude\Ona). Cada fet porta la font; el que no s'ha pogut verificar és TODO. -->

## Description
Servei de música en streaming d'Apple (catàleg + biblioteca iCloud) sense pla gratuït. Té apps pròpies a iPhone/iPad/Mac (app Música), Android, Windows (app "Apple Music" de la Microsoft Store, des de gener 2023 com a preview, successora d'iTunes) i la web music.apple.com. Fonts: [MacStories](https://www.macstories.net/stories/hands-on-with-apple-music-for-windows/), [Microsoft Store](https://apps.microsoft.com/detail/9pfhdd62mxs1).

- **Windows (Store):** editor Apple Inc., valoració **3,9/5 amb 16,4k valoracions** (llegit el 20 oct 2026); "Active Apple Music subscription required"; sincronitzar amb iPhone requereix l'app separada Apple Devices ([Store](https://apps.microsoft.com/detail/9pfhdd62mxs1)). Requereix Windows 11 22621+ ([TechCrunch](https://techcrunch.com/2023/01/12/apple-tv-apple-music-apps-windows-microsoft-store/amp/)). App UWP ([MacStories](https://www.macstories.net/stories/hands-on-with-apple-music-for-windows/)).
- **macOS / iPhone:** app Música integrada al sistema (iOS 26 / macOS Tahoe 26).
- **Web:** music.apple.com, construïda sobre MusicKit JS (la mateixa base que fan servir Cider 1 i LitoMusic). <!-- TODO: capacitats exactes de la web 2026 (lossless/Atmos) no verificades amb font oficial -->

**Producte propi a comparar: Ona** (C:\Claude\Ona, v0.5.0): client d'escriptori lleuger de Spotify (Tauri 2 + Svelte 5 + Rust/librespot), pantalla vertical 608×1080, prioritat consum baix.

## Key Features
- Catàleg amb lossless (ALAC) i hi-res lossless sense cost extra, Dolby Atmos/àudio espacial ([FreeYourMusic, abr 2026](https://freeyourmusic.com/blog/best-music-streaming-services)).
- Windows: lossless i hi-res en streaming i baixades, crossfade, "Sound Enhancer" ([MacStories](https://www.macstories.net/stories/hands-on-with-apple-music-for-windows/)); lletres afegides després (inicialment sense sincronitzar) i dreceres de teclat parcials ([gHacks](https://www.ghacks.net/?p=193006)).
- iOS 26 (2025): **AutoMix** (transicions tipus DJ amb time-stretch i beat-match, Apple Intelligence; no funciona per AirPlay), **traducció i pronunciació de lletres** (només algunes cançons i parells d'idiomes), importació de playlists d'altres serveis ([MusicTech](https://musictech.com/news/music/apple-music-ios-26-updates/), [PhoneArena](https://www.phonearena.com/news/apple-music-updates-automix-lyrics-translation-more-new-features_id171166)).
- iOS 26.2 (12 des 2025): **lletres offline** per a cançons baixades; Favorite Songs a Top Picks ([9to5Mac](https://9to5mac.com/2025/12/12/ios-26-2-adds-new-apple-music-features-and-fixes-a-viral-bug/)).
- Curació editorial humana i interfície "music-first"; descobriment algorítmic considerat inferior al de Spotify ([FreeYourMusic](https://freeyourmusic.com/blog/best-music-streaming-services)).

## Pricing (as of October 2026)
Primera pujada des d'octubre de 2022, juliol 2026, sense anunci formal d'Apple ([Android Authority](https://www.androidauthority.com/apple-music-price-increases-3688951/), [BGR](https://www.bgr.com/2265143/how-much-is-apple-music-2026/)).

| Pla | EUA abans | EUA 2026 | Espanya abans | Espanya 2026 |
|---|---|---|---|---|
| Individual | $10.99 | $11.99 | 10,99 € | **11,99 €** (iguala Spotify) |
| Estudiant | $5.99 | $6.99 | 5,99 € | 6,99 € (amb Apple TV+) |
| Familiar (fins a 6) | $16.99 | $19.99 | 16,99 € | 19,99 € |
| Anual individual | — | — | — | 119 € |

Fonts Espanya: [Moncloa.com](https://www.moncloa.com/2026/07/24/apple-music-subida-precio-espana-3404944/), [Que.es](https://www.que.es/2026/07/18/apple-music-sube-precio-espana/), [Teknófilo](https://www.teknofilo.com/apple-music-sube-precio-espana-nuevas-tarifas/) (Teknófilo diu que el familiar no canvia; la resta diu 16,99→19,99). Apple One Familiar $27.95, Premier $39.95 (EUA). **Sense pla gratuït**: tot client (oficial o de tercers) exigeix subscripció activa ([Store](https://apps.microsoft.com/detail/9pfhdd62mxs1)).

## Recent Releases
<!-- Keep this section updated with recent launches and changes -->
- Gener 2023: apps Apple Music, TV i Apple Devices per a Windows en preview ([TechCrunch](https://techcrunch.com/2023/01/12/apple-tv-apple-music-apps-windows-microsoft-store/amp/)).
- Posterior: Windows rep lletres (no sincronitzades) i dreceres; corregit el salt de pista als 15 s en lossless ([gHacks](https://www.ghacks.net/?p=193006)). <!-- TODO: data -->
- Juny 2025 (WWDC) / set 2025: iOS 26 amb AutoMix, traducció de lletres, importació de playlists ([MusicTech](https://musictech.com/news/music/apple-music-ios-26-updates/)).
- 12 des 2025: iOS 26.2, lletres offline ([9to5Mac](https://9to5mac.com/2025/12/12/ios-26-2-adds-new-apple-music-features-and-fixes-a-viral-bug/)).
- Juliol 2026: pujada de preus als EUA, Espanya, Índia, Brasil.
<!-- TODO: notes de versió oficials de l'app de Windows al 2026 no trobades -->

## Known Pain Points
**Reproducció a Windows (Alt, persistent des del 2023)**
- La pista sona ~2 s i l'àudio es talla mentre la barra continua; volum que no respon durant desenes de segons; mute que deixa el volum al 50 % ([Notebookcheck, 10 set 2025](https://www.notebookcheck.net/Apple-Music-for-Windows-Why-its-garbage-listening-experience-has-been-failing-loyal-users-since-2023.1110071.0.html)).

**Rendiment i RAM a Windows (Alt, persistent)**
- Fuites de memòria a Windows 11 fins que l'app no respon; Browse lent fins i tot en un PC potent; penjades que requereixen el Gestor de tasques ([Notebookcheck](https://www.notebookcheck.net/Apple-Music-for-Windows-Why-its-garbage-listening-experience-has-been-failing-loyal-users-since-2023.1110071.0.html)). Cerques de >15 s i pàgines en blanc ([MacStories, gen 2023](https://www.macstories.net/stories/hands-on-with-apple-music-for-windows/)). <!-- TODO: mesura pròpia de RAM no feta (no s'ha instal·lat ni iniciat sessió) -->

**Biblioteca i sincronització (Alt, persistent, totes les plataformes)**
- Windows: pantalla "Loading Library" infinita; deixa de reconèixer fitxers locals; sincronitzar amb iOS falla repetidament ([Notebookcheck](https://www.notebookcheck.net/Apple-Music-for-Windows-Why-its-garbage-listening-experience-has-been-failing-loyal-users-since-2023.1110071.0.html)); sincronització que peta amb biblioteques de 65–70 GB ([Apple Community](https://discussions-kr-prz.apple.com/thread/255664936)).
- iPhone: "Syncing Library…" infinit a iOS 26 ([Apple Community](https://communities.apple.com/es/thread/256338722)); cançons noves que desapareixen de playlists a iOS 26.5 (maig 2026) ([Apple Community](https://discussions.apple.com/thread/256302374)); playlists d'altres persones apareixent a la biblioteca ([Yahoo/9to5Mac](https://autos.yahoo.com/apple-music-bug-on-ios-is-reportedly-mixing-up-peoples-playlists-073003381.html)); biblioteques perdudes ([iMore](https://www.imore.com/music-movies-tv/apple-music/having-library-syncing-problems-in-apple-music-youre-not-alone-and-some-are-losing-hundreds-of-hours-of-curated-music)).

**Integració amb Windows (Mitjà, persistent)**
- Sense controls a la miniatura de la barra de tasques; tecles multimèdia que no funcionaven al llançament; sense Ctrl+F per cercar ([MacStories](https://www.macstories.net/stories/hands-on-with-apple-music-for-windows/), [Notebookcheck](https://www.notebookcheck.net/Apple-Music-for-Windows-Why-its-garbage-listening-experience-has-been-failing-loyal-users-since-2023.1110071.0.html)).

**Funcions que falten respecte d'iTunes (Mitjà)**
- No es pot reordenar una playlist, ni treure una cançó sense esborrar-la de la biblioteca; sense Home Sharing ni navegació per columnes ([Notebookcheck](https://www.notebookcheck.net/Apple-Music-for-Windows-Why-its-garbage-listening-experience-has-been-failing-loyal-users-since-2023.1110071.0.html)).

**Cua (Mitjà)**
- "Play Next/Play Later" que de vegades no surten (Windows, [MacStories](https://www.macstories.net/stories/hands-on-with-apple-music-for-windows/)); multiselecció per moure/esborrar a la cua que falla a macOS Tahoe 26.3 ([Apple Community](https://discussions.apple.com/thread/256249638)).

**Lletres (Mitjà)**
- Windows va arribar sense lletres i després només no sincronitzades ([gHacks](https://www.ghacks.net/?p=193006)); traducció limitada a pocs idiomes (cap català) ([PhoneArena](https://www.phonearena.com/news/apple-music-updates-automix-lyrics-translation-more-new-features_id171166)).

**Preu (Mitjà, recent)**
- Pujada de juliol 2026 sense funcions noves associades; ja costa el mateix que Spotify a Espanya ([Moncloa.com](https://www.moncloa.com/2026/07/24/apple-music-subida-precio-espana-3404944/)).

## Customer Segments
- **Usuaris d'iPhone que treballen amb PC Windows:** el segment amb més queixes (app de Windows, sincronització amb iOS via Apple Devices).
- **Usuaris d'ecosistema Apple (iPhone + Mac):** els afecta la sincronització de la biblioteca iCloud i els errors de cua de macOS.
- **Audiòfils:** trien Apple Music pel lossless/hi-res sense cost extra; els afecten els talls d'àudio de Windows.
- **Col·leccionistes de biblioteca pròpia (antics usuaris d'iTunes):** fitxers locals, biblioteques de desenes de GB, funcions d'iTunes perdudes.
- **Usuaris de Linux i personalitzadors:** depenen de Cider (de pagament) o de la web.
- **Usuaris de Discord / Last.fm a Windows:** fan servir AMWin-RP perquè l'app oficial no ho fa.
- **Pantalles verticals o estretes:** cas d'Ona (608×1080). <!-- TODO: cap dada pública -->

## Competitors
- **Spotify:** 12,99 $/mes als EUA (2026), únic gran servei amb pla gratuït, millor descobriment; sense lossless comparable ([FreeYourMusic](https://freeyourmusic.com/blog/best-music-streaming-services)). És el que fa servir Ona.
- **YouTube Music:** Premium Music $11.99/mes als EUA des d'abril 2026 (abans $10.99), familiar $18.99; inclòs a YouTube Premium ($15.99) ([EDMTunes](https://www.edmtunes.com/2026/04/youtube-raises-subscription-prices-for-premium-and-music-plans/), [Tom's Guide](https://www.tomsguide.com/entertainment/streaming/google-just-announced-a-price-hike-for-youtube-premium-heres-what-youll-pay-now)).
- **Tidal:** orientat a audiòfils, lossless i Atmos, ~$10.99 ([FreeYourMusic, abr 2026](https://freeyourmusic.com/blog/best-music-streaming-services)). <!-- TODO: preu Tidal 2026 sense font primària -->
- **Cider** (cider.sh): client de tercers per a Windows (.NET), macOS i Linux (Electron) i Android beta; Quasar + Vue; lletres sincronitzades en mode Immersive, EQ i espacialització, plugins i marketplace, Last.fm/Libre.fm/Maloja, Discord, Cider Remote per a iPhone ([cider.sh](https://cider.sh)). **Cider 2 és de pagament amb clau de llicència** ([AUR](https://aur.archlinux.org/packages/cider-2)) i descrit com a tancat ([Gappsy](https://gappsy.com/tools/cider/)); **Cider 1** és AGPL-3.0, 7,1k ★, arxivat ([GitHub](https://github.com/ciderapp/Cider)).
- **LitoMusic** (lujjjh): client lleuger Windows/macOS amb MusicKit JS + **Edge WebView2** / WKWebView + React; MIT, arxivat 2023 ([GitHub](https://github.com/lujjjh/LitoMusic)). El més semblant a l'arquitectura d'Ona.
- **AMWin-RP** (PKBeam): Discord Rich Presence + scrobbling Last.fm per a l'app *nativa* de Windows; C#, GPL-3.0, 988 ★, actiu (oct 2026) ([GitHub](https://github.com/PKBeam/AMWin-RP)).
- **FocalSonic** (SamsidParty): client lliure per a Windows 11, GPL-3.0, actiu oct 2026 ([GitHub](https://github.com/SamsidParty/FocalSonic)).
- **Apple Music Electron** (ciderapp): predecessor de Cider, MIT, deprecat ([GitHub](https://github.com/ciderapp/Apple-Music-Electron)).
- Projectes petits 2026: scratter (C++23, dimoni que continua sonant sense UI, "0 MB VRAM" amb la finestra tancada; MIT, 2 ★), tuffahi (Electron + SolidJS + MusicKit JS), musictron, helix ([cerca GitHub, 20 oct 2026](https://github.com/topics/apple-music)).

## Community Forum
- Oficial: Apple Support Community, https://discussions.apple.com/ (fils citats a dalt; també edicions locals, p. ex. communities.apple.com/es).
- Alternatives: r/AppleMusic (citat com a font per [Notebookcheck](https://www.notebookcheck.net/Apple-Music-for-Windows-Why-its-garbage-listening-experience-has-been-failing-loyal-users-since-2023.1110071.0.html)), Microsoft Q&A (learn.microsoft.com), GitHub Issues de Cider i AMWin-RP. <!-- TODO: URL exacta del subreddit no verificada -->

## Additional Search Terms
- "Apple Music Windows" / "Apple Music for Windows" / "Apple Music PC"
- "Apple Music Windows audio cuts out" / "volume slider"
- "Apple Music Loading Library stuck"
- "Apple Music memory leak Windows 11"
- "Apple Music Syncing Library iOS 26"
- "Apple Music songs disappearing from playlist"
- "Apple Music queue bug" / "Play Next missing"
- "Apple Music lyrics not synced Windows"
- "Apple Devices app sync iPhone Windows"
- "Apple Music AutoMix" / "lyrics translation"
- "Apple Music price increase 2026" / "subida precio Apple Music"
- "Cider Apple Music" / "Cider 2 license"
- "MusicKit JS" / "Apple Music API developer token"
- "AMWin-RP"

## Anàlisi de competidors amb codi (20 oct 2026)
Codi llegit el 20 oct 2026 als clons de `scratchpad/comp/` de la sessió (no al workspace durable `~/workspace/competitors`). Les cites `fitxer:línia` són relatives a l'arrel de cada repo al commit indicat. Les inferències van marcades com a **(inferència)**.

### Registre de fonts
| Competidor | Remote | Commit analitzat | Data commit | Estat |
|---|---|---|---|---|
| Cider 1 | https://github.com/ciderapp/Cider | `953bb5a9e260` | 2024-12-10 | Arxivat (`README.md:11`) |
| LitoMusic | https://github.com/lujjjh/LitoMusic | `a351c95b707b` | 2023-06-16 | L'últim commit és l'actualització automàtica del token |
| FocalSonic | https://github.com/SamsidParty/FocalSonic | `bb1cd0faf7e9` | 2026-07-24 | Actiu (v1.3.0) |
| AMWin-RP | https://github.com/PKBeam/AMWin-RP | `b05a49957713` | 2026-09-29 | Actiu |
| Ona (propi) | C:\Claude\Ona (local) | — | v0.5.0 (`src-tauri/tauri.conf.json:4`) | — |

### Fets de codi per competidor
**Cider 1** (Electron + Vue, AGPL-3.0 `package.json:7`, v1.6.4 `package.json:5`)
- Runtime: Electron de castlabs amb Widevine (`package.json:83`, `electron-releases#v32.2.6+wvcus`); espera els components DRM abans de crear la finestra (`src/main/index.ts:37`) i té una acció per reinstal·lar el CDM (`src/main/base/browserwindow.ts:862-866`).
- Token de desenvolupador: el baixa d'un servidor propi `api.cider.sh/v1/` (`src/renderer/index.js:105`), en desa una còpia a `localStorage` com a reserva (`src/renderer/index.js:60-67`) i inicialitza MusicKit JS (`src/renderer/index.js:68-69`).
- Àudio: cadena Web Audio pròpia amb convolvers d'espacialització (`src/renderer/audio/audio.js:247`) i filtres biquad per a l'EQ (`src/renderer/audio/audio.js:806`, `:895-899`).
- Integracions com a plugins (`src/main/plugins/`): Discord, Last.fm, MPRIS, Chromecast, AirPlay/RAOP (dependències `package.json:38-53`), botons de miniatura de la barra de tasques (`src/main/plugins/thumbar.ts:82-84`). API WebSocket de control remot al port 26369 (`src/main/base/wsapi.ts:13`). Fitxers locals amb `music-metadata` (`src/main/providers/local/index.ts:3`).
- Lletres: token de Musixmatch tret de l'endpoint de l'app d'escriptori (`src/renderer/main/vueapp.js:3475-3486`).

**LitoMusic** (host natiu + React, MIT `LICENSE:1`)
- Windows: host Rust propi sobre WebView2 amb el crate `windows` 0.21 (`windows/Cargo.toml:9`, `windows/src/webview.rs`); macOS: WKWebView en Swift (`darwin/LitoMusic/LitoMusic/WebView.swift`). Carrega `musickit.js` (`packages/lito/index.html:16`).
- Token: un CI diari (`.github/workflows/update-developer-token.yml:4-5`) extreu el token del bundle JS de music.apple.com (`scripts/update-developer-token.sh:5-7`) i el fusiona sol (`.github/workflows/update-developer-token.yml:24-30`); en execució mira l'expiració del JWT i en demana un de nou a `api.litomusic.org/token` (`packages/lito/src/utils/auth.ts:7-22`).
- Lletres: TTML del catàleg d'Apple (`packages/lito/src/api/lyrics.ts:43`, `:52`). Dreceres de teclat pròpies (`packages/lito/src/a11y/shortcuts.ts:7`, `:79`).
- Cap codi SMTC explícit trobat (grep a `windows/src` i `packages/lito/src`). `per verificar`: si WebView2 exposa sol el MediaSession de MusicKit a les tecles multimèdia.

**FocalSonic** (.NET 9 + webview IgniteView + Vite, GPL-3.0 `LICENSE.txt:1`)
- DRM: la reproducció passa per una finestra web oculta a `beta.music.apple.com` amb Widevine "Due to DRM" (`FocalSonic/AppleMusic/AppleMusicAudioPlayer.cs:15-16`, `:42-44`); la UI comprova Widevine amb robustesa `HW_SECURE_ALL` (`FocalSonic/src-vite/src/utils/widevine.ts:6-17`). Per a Atmos reescriu els key-ID del contenidor (`FocalSonic/Streaming/StreamingCDM.cs:15-56`). Lossless descartat per manca de via legal a Windows (`README.md:23-24`).
- Credencials: developer token + media-user token desats com a credencial local (`FocalSonic/AppleMusic/AppleMusicAudioPlayer.cs:358-362`).
- **Mode en segon pla**: tanca la finestra principal, posa `IsRunningInForeground=false` i força `gc()` a les webviews que queden mentre l'àudio continua (`FocalSonic/Program.cs:100-111`; flag a `FocalSonic/Performance.cs:12`); el README ho ven com a estalvi de recursos (`README.md:10`).
- Àudio: SoundFlow/miniaudio (`FocalSonic/FocalSonic.csproj:47`, `FocalSonic/AudioPlayer/SoundFlowAudioPlayer.cs:6-23`), control de velocitat amb time-stretch (`README.md:11`).
- SMTC via `MediaPlayer.SystemMediaTransportControls` amb timeline (`FocalSonic/Windows/WindowsPresenceProvider.cs:23`, `:35`, `:102`). Discord RPC, Chromecast, AirPlay (`FocalSonic/FocalSonic.csproj:42-46`), Last.fm (`FocalSonic/LastFM/`).
- Negoci: "Free and open source" (`README.md:3`), però la build Windows comprova llicència de la Microsoft Store i dona una prova de 7 dies (`FocalSonic/Licensing.cs:16`, `:32-38`, `:47-59`); fora de Windows retorna sense límit (`FocalSonic/Licensing.cs:60-62`).

**AMWin-RP** (WPF .NET 10, `AMWin-RichPresence/AMWin-RichPresence.csproj:5`)
- No reprodueix: llegeix l'app oficial `AppleMusic.exe` amb UI Automation/FlaUI (`AMWin-RichPresence/AppleMusicClientScraper.cs:142`, `:186`, `:204-205`, `:280-301`; `TECHNICAL_DOCS.md:4`). La caràtula no surt d'UIA i la busca a Last.fm o a la web d'Apple (`TECHNICAL_DOCS.md:16`; `AMWin-RichPresence/AppleMusicWebScraper.cs:71`, `:328`).
- Scrobbling Last.fm + ListenBrainz (`AMWin-RichPresence/AMWin-RichPresence.csproj:34-35`): cançons ≥30 s, quan passa la meitat o un màxim configurable (`AMWin-RichPresence/AppleMusicScrobbler.cs:64-69`). Credencials al Windows Credential Manager (`AMWin-RichPresence/SettingsWindow.xaml.cs:344`, `:409`).
- Lletres de LRCLIB (`AMWin-RichPresence/LRCLibClient.cs:84`). Requereix Windows 11 24H2 (`README.md:12`).

### Comparació amb Ona
| Dimensió | Cider 1 | LitoMusic | FocalSonic | AMWin-RP | Ona |
|---|---|---|---|---|---|
| Runtime | Electron castlabs (`package.json:83`) | Rust + WebView2 (`windows/Cargo.toml:9`) | .NET + webview (`FocalSonic.csproj:36`) | WPF (`AMWin-RichPresence.csproj:5`) | Tauri 2 (`src-tauri/Cargo.toml:21`) |
| Motor d'àudio | MusicKit JS + Web Audio | MusicKit JS | webview Widevine + SoundFlow | cap (llegeix l'app oficial) | librespot natiu, 320 kbps, normalització (`src-tauri/Cargo.toml:25`, `src-tauri/src/lib.rs:126-129`) |
| Token / sessió | servidor propi (`src/renderer/index.js:105`) | scraping per CI + servidor (`scripts/update-developer-token.sh:5-7`) | credencial local (`AppleMusicAudioPlayer.cs:360-361`) | n/a | credencials a la cache de librespot (`src-tauri/src/lib.rs:89-96`, `:101`, `:143`) |
| SMTC / tecles | MPRIS + thumbar (`src/main/plugins/`) | `per verificar` | sí (`WindowsPresenceProvider.cs:35`) | n/a | sí, souvlaki (`src-tauri/src/media.rs:1-2`, `:17-23`) |
| Lletres | Musixmatch (`vueapp.js:3475`) | TTML Apple (`lyrics.ts:52`) | per síl·laba (`README.md:12`) | LRCLIB (`LRCLibClient.cs:84`) | BetterLyrics TTML per síl·laba + Spotify + LRCLIB (`src-tauri/src/lyrics.rs:7-17`, `:36`, `:58-67`) |
| Segon pla sense UI | — | — | sí (`Program.cs:100-111`) | n/a | no: tancar la principal surt de l'app (`src-tauri/src/lib.rs:624-628`) |
| Discord / scrobbling | sí / Last.fm | — | sí / Last.fm | sí / Last.fm + ListenBrainz | cap (grep `discord`, `scrobbl`, `lastfm` sense resultats a `src` i `src-tauri/src`) |
| Finestra | — | — | — | — | 1280×820, mín. 380×560, sense decoració (`src-tauri/tauri.conf.json:17-21`); disseny de 608 px (`src/lib/components/PlayerBar.svelte:320`) |

### Decisions que canvia l'evidència
**1. No afegir Apple Music com a segon backend d'Ona ara.**
- Observacions: els tres clients que reprodueixen depenen d'un runtime web amb Widevine (Cider `package.json:83`; FocalSonic `AppleMusicAudioPlayer.cs:15-16`; Lito via MusicKit JS `packages/lito/index.html:16`) i d'un developer token que Apple no els emet: servidor propi (Cider `src/renderer/index.js:105`, Lito `packages/lito/src/utils/auth.ts:18`) o scraping del bundle web (Lito `scripts/update-developer-token.sh:5-7`). Cap ofereix lossless a Windows (FocalSonic `README.md:23-24`). Cider 1 està arxivat (`README.md:11`) i Lito no té cap commit humà des del 2023 (registre).
- **(inferència)** Per a Ona voldria dir executar MusicKit JS dins la WebView2 de Tauri (patró Lito): una segona pila d'àudio fora del pipeline Rust de librespot, que és el que dona normalització i consum baix (`src-tauri/src/lib.rs:126-129`). El manteniment se'n va al token, el punt que ha fet dependre d'un servidor els dos projectes més semblants.
- Cost: Ona no capta usuaris d'Apple Music (el segment "iPhone + PC" amb més queixes, vegeu més amunt).
- Explicació alternativa: Lito pot haver mort per manca de temps de l'autor i no pel token; el seu CI continuava funcionant el 2023.
- Què la tombaria: que Apple publiqui un SDK MusicKit d'escriptori amb token per app, o que un prototip MusicKit JS dins la WebView2 de Tauri reprodueixi el catàleg complet (FocalSonic demana `HW_SECURE_ALL`, `widevine.ts:12`; `per verificar` si MusicKit l'exigeix de veritat).

**2. Afegir un mode "només àudio" que destrueixi la WebView mentre librespot continua (oportunitat principal).**
- Observacions: FocalSonic tanca la finestra i força GC per estalviar recursos (`Program.cs:100-111`, `README.md:10`); scratter (vegeu Competitors) ven el mateix. A Ona l'àudio ja viu al procés Rust (`src-tauri/src/lib.rs:126-134`) i el SMTC no depèn de la UI (`src-tauri/src/media.rs:13-23`), però tancar la finestra principal surt de l'app a propòsit (`src-tauri/src/lib.rs:624-628`).
- **(inferència)** Ona hi guanya més que FocalSonic, perquè FocalSonic encara necessita la finestra web oculta per al DRM (`AppleMusicAudioPlayer.cs:42-44`) i Ona no. Encaixa amb la prioritat de consum baix.
- Cost: icona de safata (no hi ha cap feature `tray-icon` a `src-tauri/Cargo.toml:21`), recrear la finestra amb l'estat del reproductor (`player_state`, `src-tauri/src/lib.rs:234`) i decidir què fa la lletra flotant.
- Contraexemple: si WebView2 ja allibera prou memòria amb la finestra minimitzada, el guany pot ser petit.
- Què la tombaria (propera comprovació): mesurar el working set d'`ona.exe` + `msedgewebview2.exe` sonant amb la finestra oberta, minimitzada i destruïda; si destruir-la estalvia menys de ~50 MB, no val la pena. `per verificar`.

**3. Scrobbling i Discord: prioritat baixa; si es fa, copiar la regla d'AMWin-RP.**
- Observacions: tots els competidors amb UI pròpia ho tenen (Cider `package.json:41`, `:51`; FocalSonic `FocalSonic.csproj:42`; AMWin-RP n'és el producte). Ona no (grep). Regla estàndard a `AppleMusicScrobbler.cs:64-69`; credencials al Credential Manager (`SettingsWindow.xaml.cs:344`).
- **(inferència)** Amb Spotify el scrobbling a Last.fm sol venir del compte connectat, així que el valor per a Ona és menor que per a Apple Music, on l'app oficial no ho fa. `per verificar` si les reproduccions via librespot arriben a Last.fm pel connector de Spotify; si no hi arriben, puja de prioritat.

**4. Lletres: cap canvi.** Ona ja cobreix síl·laba (BetterLyrics TTML), Spotify i LRCLIB (`src-tauri/src/lyrics.rs:7-17`, `:36`, `:58-67`), igual o per sobre de Lito (`lyrics.ts:52`) i AMWin-RP (`LRCLibClient.cs:84`); Cider depèn d'un token Musixmatch no oficial (`vueapp.js:3475-3486`).

### Riscos i supòsits
| Element | Què se sap | Què falta | Propera comprovació |
|---|---|---|---|
| Estalvi del mode només àudio | patró FocalSonic (`Program.cs:100-111`) | RAM real d'Ona per estat de finestra | `Get-Process ona,msedgewebview2` en els tres estats |
| Patrons de DRM/token | FocalSonic reescriu key-ID (`StreamingCDM.cs:15-56`); Lito fa scraping del token | posició d'Apple | no adoptar-los a Ona |
| Freshness | commits fixats al registre | canvis posteriors al 20 oct 2026 | `git fetch` als clons |
| Ubicació dels clons | són al scratchpad de la sessió | es perdran | moure'ls a `~/workspace/competitors/ona/` si cal repetir l'anàlisi |
