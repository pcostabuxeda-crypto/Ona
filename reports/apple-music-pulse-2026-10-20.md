# Customer Pulse Report: Apple Music (per a Ona)

**Generated:** 2026-10-20
**Sources searched:** Trustpilot, Apple Support Community (discussions.apple.com i communities.apple.com/fr), Microsoft Q&A, premsa (Notebookcheck, BGR, Windows Report, MacStories, 9to5Mac, gHacks, chip.pl, TechRepublic), GitHub Issues (Cider-2, AMWin-RP, FocalSonic). Reddit i X: cercats, cap resultat directe; Reddit només de segona mà (BGR, Windows Report, Notebookcheck).
**Period focus:** Recent (last ~3 months, weighted toward most recent)

> ⚠️ **Confiança mitjana.** 24 punts de dades verificats; 11 són del 2026 (Trustpilot, Apple Community, BGR, GitHub). L'app de Windows té poca veu el 2026: gairebé tot és del 2023-2025, però cap font diu que estigui arreglat. No s'ha instal·lat l'app ni iniciat sessió: no hi ha mesura pròpia de RAM.

---

## Top 3 This Week

> Tres tasques d'una hora o menys, concretes per a Ona.

1. **Comprova que Ona avança la cua amb un temporitzador, no amb `requestAnimationFrame`.** A Cider 2 ([#1510](https://github.com/ciderapp/Cider-2/issues/1510), 17 juny 2026, 6 👍) les tecles multimèdia i el pas a la cançó següent fallaven amb la finestra sense focus, i el van arreglar amb un "timer-based watchdog". WebView2 també frena les pàgines en segon pla: prova Ona minimitzada 20 min amb una playlist i tecles multimèdia.
2. **Fes una prova de 2 h de RAM amb Ona minimitzada.** Cider 2 té tres fils oberts de fuites el 2026 ([#1370](https://github.com/ciderapp/Cider-2/issues/1370), 24 comentaris; [#1865](https://github.com/ciderapp/Cider-2/issues/1865), "close to 2gb"; [#1946](https://github.com/ciderapp/Cider-2/issues/1946), memòria del nucli que creix GB/hora pel servidor WebSocket local). L'app oficial de Windows també ([Notebookcheck](https://www.notebookcheck.net/Apple-Music-for-Windows-Why-its-garbage-listening-experience-has-been-failing-loyal-users-since-2023.1110071.0.html)). És el mateix avantatge que Ona té davant Spotify.
3. **Llegeix el fil [New songs disappear from playlists](https://discussions.apple.com/thread/256302374)** (iOS 26.5, 24 «me too», maig-juny 2026, sense solució) i el [resum de BGR](https://www.bgr.com/2257028/why-users-ditch-apple-music/) (22 set 2026). Són la millor llista de què **no** ha de fer la versió d'iPhone d'Ona: biblioteca que no és fiable, botons amagats darrere de menús, no poder continuar al Mac el que sonava a l'iPhone.

---

## Executive Summary

### Top Insights (ranked by priority)

1. **`[CONFIRMED]` La biblioteca i les playlists no són fiables** (cançons que desapareixen, sincronització infinita, música baixada o comprada que s'esborra).
   - Category: Bug | Mentions: 8 | Severity: High | Trend: Worsening. Fil d'iOS 26.5 (maig 2026, 24 «me too», darrera resposta 18 juny); a Trustpilot, 4 ressenyes de 1★ entre agost i octubre 2026 ("they deleted over 2,000 songs I had downloaded", 27 ag).
   - Sources: Apple Community, Trustpilot, iMore, Yahoo/9to5Mac, BGR, Windows Report
   - Recommended Action (Ona): a la versió d'iPhone, la biblioteca local (fitxers, Audius) i les playlists han de ser de l'usuari i no dependre d'un núvol que les pot reescriure. Desa l'estat en local i fes que qualsevol sincronització sigui només additiva.

2. **`[CONFIRMED]` L'app de Windows no reprodueix bé** (talls d'àudio als 2 s, volum que no respon, cançons que se salten, penjades).
   - Category: Bug | Mentions: 5 | Severity: High | Trend: Stable. Queixes del 2023 a l'octubre del 2025 i cap font de correcció; el 2026 la Store mostra 3,9/5 amb 16,4k valoracions.
   - Sources: Notebookcheck (set 2025), Apple Community FR (10 oct 2025), Microsoft Q&A (30 set 2025), MacStories, Microsoft Store
   - Recommended Action (Ona): és l'argument d'Ona a Windows: un client nadiu i petit que sona sempre. No hi ha res a copiar; només cal no perdre la fiabilitat actual.

3. **`[CONFIRMED]` RAM i rendiment: fuites a l'app oficial i també a Cider 2**.
   - Category: Performance | Mentions: 6 | Severity: High | Trend: Worsening a Cider (3 fils nous el 2026); Stable a l'oficial.
   - Sources: Notebookcheck, Microsoft Q&A, GitHub Cider-2 (#1370, #1865, #1946, #2087, #1425 "Slow to Start")
   - Recommended Action (Ona): Top 3, punt 2. Tingues present que Cider (que es ven com a alternativa ràpida) fa el mateix error: un client de tercers no és lleuger només pel fet de ser-ho.

4. **`[NEW]` Pausar a l'iPhone i continuar al Mac no és fàcil (no hi ha equivalent de Spotify Connect)**.
   - Category: Feature Request / Churn | Mentions: 2 | Severity: Medium | Trend: New. Post de maig 2026 amb ~2.000 vots; un altre usuari diu que és l'única raó per seguir a Spotify ([BGR](https://www.bgr.com/2257028/why-users-ditch-apple-music/)). YouTube Music ja sincronitza la cua entre Android i iOS ([TechRadar](https://www.techradar.com/tag/apple-music)).
   - Recommended Action (Ona): Ona (escriptori + iPhone) ja parteix de Spotify Connect amb librespot. Fes que la versió d'iPhone pugui controlar l'escriptori i reprendre la seva cua: és la funció que més vots té en aquesta recerca.

5. **`[NEW]` Interfície amb massa tocs: botons amagats en menús, barra de pestanyes que es plega (Liquid Glass)**.
   - Category: UX | Mentions: 5 | Severity: Medium | Trend: Worsening (feb i juny 2026; Apple planeja retocar-ho a iOS 27 segons Gurman, maig 2026).
   - Sources: BGR (feb i juny 2026, r/AppleMusic), chip.pl (maig 2026), TechRepublic (beta 3 d'iOS 26), MacRumors Forums
   - Recommended Action (Ona): a l'iPhone, accions principals sempre visibles (afegir a la biblioteca, a la cua, a continuació); res de barres que es plegen en fer scroll.

6. **`[CONFIRMED]` La cua i les playlists no es poden gestionar bé** (no es pot reordenar a Windows; treure un àlbum esborrava les cançons de totes les playlists; multiselecció de la cua trencada a macOS 26.3; la reproducció s'atura en acabar la cançó).
   - Category: UX/Bug | Mentions: 5 | Severity: Medium | Trend: Improving (la beta d'iOS 27 ja no esborra de les playlists, segons [BGR](https://www.bgr.com/2257028/why-users-ditch-apple-music/)).
   - Sources: Notebookcheck, Windows Report (r/AppleMusic, 2024), Apple Community (macOS 26.3), BGR
   - Recommended Action (Ona): la cua d'usuari de la v0.5.0 ja respon a això. Afegeix-hi reordenar arrossegant i multiselecció si no hi són, i prova-ho amb 500+ cançons (Cider va tenir el límit de ~1.000 cançons a la cua, [#117](https://github.com/ciderapp/Cider-2/issues/117)).

7. **`[NEW]` Integració amb el sistema: tecles multimèdia, safata i controls a la barra de tasques**.
   - Category: UX | Mentions: 5 | Severity: Medium | Trend: Stable.
   - Sources: MacStories i Notebookcheck (app oficial, sense controls a la barra de tasques ni Ctrl+F); Cider-2 [#1510](https://github.com/ciderapp/Cider-2/issues/1510), [#889](https://github.com/ciderapp/Cider-2/issues/889) ("Close to tray" no va), [#1291](https://github.com/ciderapp/Cider-2/issues/1291) (MPRIS)
   - Recommended Action (Ona): comprova SMTC (controls multimèdia de Windows), la miniatura de la barra de tasques i la drecera per cercar.

8. **`[CONFIRMED]` Lletres limitades fora d'Apple** (Windows sense sincronitzar; traducció en pocs idiomes, sense català).
   - Category: Feature Request | Mentions: 3 | Severity: Low-Medium | Trend: Improving (lletres offline a iOS 26.2).
   - Sources: gHacks, PhoneArena, 9to5Mac. FocalSonic i Cider ho resolen amb lletra per síl·laba (TTML).
   - Recommended Action (Ona): la lletra per síl·laba ja és un punt fort d'Ona; a l'iPhone, desa-la per a ús offline com fa iOS 26.2.

9. **`[CONFIRMED]` Pujada de preu sense res de nou** (11,99 € a Espanya des de juliol 2026, igual que Spotify).
   - Category: Pricing | Mentions: 4 | Severity: Medium | Trend: New (juliol 2026).
   - Sources: Moncloa.com, Que.es, Teknófilo, Android Authority; a Trustpilot, problemes de cobrament després de la prova gratuïta (10 ag 2026).
   - Recommended Action (Ona): cap d'immediata; vegeu «Què en pot aprendre Ona» sobre el cost real d'admetre Apple Music.

10. **`[NEW]` Suport i compte: restabliment de contrasenya forçat, música comprada que no surt, compte hackejat sense ajuda**.
    - Category: Churn | Mentions: 4 | Severity: High per a qui ho pateix | Trend: Insufficient data (només Trustpilot, ag-oct 2026).
    - Sources: Trustpilot
    - Recommended Action (Ona): fora de l'abast d'un client; indica que si Ona admet Apple Music, l'usuari encara dependrà del compte i del suport d'Apple.

### Sentiment Overview
- **Overall sentiment:** Negative a Windows i a Trustpilot (TrustScore 1,7 amb 260 ressenyes; 3,9/5 a la Store); Mixed a iPhone (es valora la qualitat de so i el catàleg, "Great Music Quality, Premium feel", 14 ag 2026).
- **Trending:** Worse a biblioteca i interfície (fils del 2026); Better en lletres (offline) i en el comportament de les playlists a iOS 27 beta.

---

## Detailed Feedback by Category

### Bugs & Reliability Issues

| Issue | Sources | Mentions | Severity | Representative Quote |
|-------|---------|----------|----------|---------------------|
| Cançons noves que desapareixen de playlists (iOS 26.5) | Apple Community | 24 «me too» | High | — (sense solució confirmada a 18 juny 2026) |
| Música baixada/comprada esborrada | Trustpilot (27 ag, 5 set, 15 set 2026) | 3 | High | "they deleted over 2,000 songs I had downloaded" |
| Talls d'àudio i volum que no respon (Windows) | Notebookcheck, Apple Community FR | 2+ | High | "play for about 2 seconds and then the sound will stop" |
| "Loading Library" / "Syncing Library" infinit | Notebookcheck, Apple Community ES | 2 | High | — |
| Cançons d'altres a les meves playlists | Trustpilot (5 ag 2026), Yahoo/9to5Mac | 2 | Medium | — |
| Penjades i reinicis de sessió freqüents (Windows 11) | Apple Community FR (10 oct 2025) | 1 | Medium | — |

La biblioteca és el problema més greu i més recent: afecta iPhone i Windows i fa perdre feina de l'usuari (playlists, baixades). A Windows, el problema de base és que l'app no reprodueix de manera fiable.

### Feature Requests

| Issue | Sources | Mentions | Severity | Representative Quote |
|-------|---------|----------|----------|---------------------|
| Continuar la reproducció entre dispositius | BGR (maig 2026, ~2.000 vots) | 2 | Medium | — |
| Sincronitzar la cua entre dispositius | TechRadar (comparació amb YouTube Music) | 1 | Low | — |
| Funcions d'iTunes a Windows (reordenar, Home Sharing, columnes) | Notebookcheck, Apple Community | 2 | Medium | — |
| Recomanacions més fidels a l'estat d'ànim | BGR (Reddit) | 2 | Low | "It should be pretty simple to just play similar vibes" |

### UX & Usability

| Issue | Sources | Mentions | Severity | Representative Quote |
|-------|---------|----------|----------|---------------------|
| Accions amagades darrere de menús | BGR (feb i juny 2026) | 3 | Medium | "constant friction with the user interface across all platforms" |
| Barra de pestanyes que es plega (iOS 26) | chip.pl, TechRepublic, MacRumors | 3 | Medium | — |
| Cerca que necessita l'ortografia exacta | BGR, Windows Report | 2 | Medium | "needs the exact spelling to return any results" |
| Sense controls a la barra de tasques ni Ctrl+F (Windows) | MacStories, Notebookcheck | 2 | Low | — |

### Performance

| Issue | Sources | Mentions | Severity | Representative Quote |
|-------|---------|----------|----------|---------------------|
| Fuita de RAM (app oficial, Windows 11) | Notebookcheck | 1 | High | "consumes ever-increasing amounts of RAM" |
| Interfície que es congela uns segons | Microsoft Q&A (30 set 2025) | 1 | Medium | — |
| Fuites a Cider 2 (fins a ~2 GB; memòria del nucli) | GitHub Cider-2 | 4 fils (2026) | High | "close to 2gb of ram after running the app for a while" |
| Àudio que empitjora amb apps que gasten CPU (Cider) | GitHub Cider-2 #2026 | 1 | Medium | — |

### Pricing & Value

| Issue | Sources | Mentions | Severity | Representative Quote |
|-------|---------|----------|----------|---------------------|
| Pujada a 11,99 € (juliol 2026) | Moncloa.com, Que.es, Teknófilo | 3 | Medium | — |
| Cobrament després de la prova gratuïta | Trustpilot (10 ag 2026) | 1 | Medium | — |

### Praise & Positive Sentiment

| Issue | Sources | Mentions | Severity | Representative Quote |
|-------|---------|----------|----------|---------------------|
| Qualitat de so i catàleg | Trustpilot (14 ag 2026), FreeYourMusic | 2 | — | "Great Music Quality, Premium feel" |
| Smart playlists per regles, lossless/Atmos sense cost extra | TechRadar, DigitalCitizen (2026) | 2 | — | — |
| A Windows "funciona hores sense problemes" (minoria) | Microsoft Q&A (3 oct 2025) | 1 | — | "A little slow at times but really no issues" |

### Churn Risks & Competitive Pressure

| Issue | Sources | Mentions | Severity | Representative Quote |
|-------|---------|----------|----------|---------------------|
| Usuaris que es queden a Spotify per Connect | BGR | 1 | Medium | — |
| Amenaça de deixar Apple One per Spotify (Windows) | Notebookcheck (cita d'Apple Community) | 1 | Medium | — |
| Suport de Microsoft que recomana la web o Cider | Microsoft Q&A | 1 | Medium | — |
| El flux del 2026 va **cap a** Apple Music (smart playlists, lossless) | TechRadar, DigitalCitizen | 2 | — | — |

---

## Known vs. New Findings

### Confirmed Known Issues

| Theme | Known Pain Point | Status vs. Known |
|-------|-----------------|-----------------|
| Biblioteca i playlists | Biblioteca i sincronització | **Escalating**: el 2026 s'hi sumen pèrdues de baixades i compres (Trustpilot) |
| Reproducció Windows | Reproducció a Windows | Consistent; cap correcció trobada |
| RAM | Rendiment i RAM a Windows | Consistent a l'oficial; **nou** a Cider 2 |
| Cua i playlists | Cua; Funcions que falten | Improving a iOS 27 beta |
| Lletres | Lletres | Improving (offline a iOS 26.2) |
| Preu | Preu | Consistent |

### New Discoveries

| Theme | Category | Severity | Why It Matters |
|-------|----------|----------|---------------|
| Continuar entre dispositius | Feature/Churn | Medium | ~2.000 vots; Ona ja té la base (Spotify Connect) |
| Massa tocs / Liquid Glass | UX | Medium | Guia directa per a la UI d'iPhone d'Ona |
| Tecles multimèdia sense focus (Cider) | Bug | Medium | Mateix risc tècnic que Ona amb WebView2 |
| Suport i compte | Churn | High | Si Ona admet Apple Music, hereta aquesta dependència |

---

## Blind Spots & Gaps

### Source Gaps

| Source Category | Useful Data? | Notes |
|-----------------|-------------|-------|
| Reddit          | Indirecte | Cap fil de r/AppleMusic al WebSearch; només a través de BGR, Windows Report i Notebookcheck |
| Twitter / X     | No | Cap resultat del 2026 |
| Review Sites    | Sí | Trustpilot amb 11 ressenyes d'ag-oct 2026; cap de G2/Capterra |
| Product Forum   | Sí | Apple Community es llegeix amb WebFetch; Windows Report va fer 403 i es va llegir amb el navegador integrat |
| General Web     | Sí | Notebookcheck (403 a WebFetch, llegit amb navegador), BGR, MacStories |
| Competitor / Churn | Parcial | El 2026 la premsa parla més de passar-se **a** Apple Music que de deixar-lo |

### Customer Segment Gaps

| Segment | Feedback Found? | Notes |
|---------|----------------|-------|
| iPhone + PC Windows | Sí | El segment amb més queixes |
| Ecosistema Apple (iPhone + Mac) | Sí | Biblioteca i cua a macOS 26.3 |
| Audiòfils | Poc | Només de passada (lossless a Windows) |
| Col·leccionistes (antics usuaris d'iTunes) | Sí | Funcions perdudes, biblioteques grans |
| Linux / personalitzadors | Sí (GitHub Cider) | — |
| Discord / Last.fm | Sí (GitHub AMWin-RP) | Problemes de scrobbling i d'arrencada, 2026 |
| Pantalles verticals | No | Cap dada pública |

### Product Area Gaps

- **AutoMix**: cap queixa ni elogi trobat. Pot ser poc ús o que funciona.
- **Traducció i pronunciació de lletres**: cap opinió d'usuari, només premsa.
- **Web music.apple.com**: cap opinió directa (Microsoft la recomana com a alternativa).
- **Apple Devices (sincronitzar l'iPhone des de Windows)**: només a Notebookcheck.
- **Android**: fora de l'abast; només es menciona de passada.

---

## Methodology

- **Sources searched:** 8 WebSearch (Reddit 1, X 1, Trustpilot 1, fòrum 1, iOS UI 1, desitjos 1, migració 1, Windows 1) + 12 del pas de product-context.
- **Pages fetched:** 9 (WebFetch 6, navegador integrat 2, GitHub CLI 1 lot d'issues).
- **Total data points:** 24.
- **Source URLs:**
  - https://www.trustpilot.com/review/applemusic.com?sort=recency
  - https://discussions.apple.com/thread/256302374
  - https://communities.apple.com/fr/thread/256161417
  - https://learn.microsoft.com/en-us/answers/a/12268668
  - https://www.bgr.com/2257028/why-users-ditch-apple-music/
  - https://windowsreport.com/apple-music-nightmare-windows-android-users-say/ (navegador)
  - https://www.notebookcheck.net/Apple-Music-for-Windows-Why-its-garbage-listening-experience-has-been-failing-loyal-users-since-2023.1110071.0.html (navegador)
  - https://www.macstories.net/stories/hands-on-with-apple-music-for-windows/
  - https://9to5mac.com/2025/12/12/ios-26-2-adds-new-apple-music-features-and-fixes-a-viral-bug/
  - https://apps.microsoft.com/detail/9pfhdd62mxs1 (navegador; galetes opcionals rebutjades)
  - GitHub: ciderapp/Cider-2 issues #117, #889, #1291, #1370, #1425, #1510, #1865, #1946, #2026, #2087; PKBeam/AMWin-RP issues (feb-oct 2026); SamsidParty/FocalSonic #5-#7
- **Limitations:** TechRadar (article de canvi de Spotify a Apple Music) no es va poder llegir; ressenyes de Trustpilot sense plataforma; cap mesura pròpia de l'app (no s'ha iniciat sessió a cap compte).

---

## Què en pot aprendre Ona

Creua aquest informe, [product-context/apple-music.md](../product-context/apple-music.md) (inclosa l'anàlisi de codi) i l'estat d'Ona v0.5.0. Només idees; no s'ha copiat codi (Cider 1 és AGPL-3.0, AMWin-RP i FocalSonic són GPL-3.0).

### Idees per a l'escriptori (Windows, 608×1080)
1. **Avançar la cua amb un temporitzador al costat de Rust, no amb el bucle d'animació de la pàgina.** Cider 2 hi va caure ([#1510](https://github.com/ciderapp/Cider-2/issues/1510)). Ona ho té fàcil perquè el reproductor és librespot (Rust), però cal comprovar que la UI no és qui decideix el pas a la cançó següent.
2. **Mode "només so" amb la finestra tancada.** FocalSonic tanca la finestra i crida el GC del WebView quan passa a segon pla (`FocalSonic/Program.cs:100-110`) i alenteix les actualitzacions de temps de 200 a 750 ms (`AudioPlayer/PlayerThread.cs:29`). scratter va més lluny: dimoni que sona sense UI i "0 MB VRAM". Ona ja té el mode lleuger; el pas següent és destruir el WebView en minimitzar a la safata i recrear-lo en obrir-lo (això resoldria els 222 MB minimitzada).
3. **Integració amb Windows:** SMTC (tecles multimèdia i la targeta de volum de Windows), controls a la miniatura de la barra de tasques, Ctrl+F/Ctrl+L per cercar. Són queixes concretes a l'app oficial (MacStories, Notebookcheck).
4. **Proves de RAM llargues** (2 h, minimitzada) a cada versió, perquè tant l'oficial com Cider 2 fallen aquí.
5. **Discord/Last.fm com a extra opcional**: AMWin-RP existeix només perquè l'app oficial no ho fa, i té 988 ★. Ona podria fer scrobbling a Last.fm/ListenBrainz sense dependre de res extern (AMWin-RP llegeix la UI de l'app oficial amb UI Automation, `AppleMusicClientScraper.cs:174-204`: fràgil, i Ona no ho necessita perquè ja sap què sona).
6. **Lletra amb diversos proveïdors i que l'usuari pugui triar** quan no hi ha coincidència perfecta (FocalSonic retorna candidats per proveïdor, `Lyrics/LyricSearchService.cs:13-19`). Ona ja té LRCLib i lletra per síl·laba.

### Idees per a la futura versió d'iPhone
1. **Accions sempre visibles** (afegir a la biblioteca, a continuació, al final), sense barres que es pleguin en fer scroll. És la queixa d'interfície més repetida del 2026 (BGR, chip.pl).
2. **Biblioteca local de l'usuari, mai reescrita pel núvol.** Les queixes més greus d'Apple Music són playlists i baixades que desapareixen.
3. **Comandament de l'escriptori** (veure i controlar el que sona a l'Ona del PC, i continuar la cua). Cider en fa una app a part, Cider Remote ([cider.sh](https://cider.sh)); és la funció amb més vots (~2.000) d'aquesta recerca.
4. **Lletres offline** per a les cançons desades, com iOS 26.2.

### Té sentit que Ona admeti Apple Music? Requisits reals
| Requisit | Què implica | Font |
|---|---|---|
| Developer token de MusicKit | Cal l'Apple Developer Program (~99-100 $/any), un identificador MusicKit i una clau privada; JWT ES256 | [Guia MusicKit JS](https://dev.to/areknawo/apple-music-javascript-integration-guide-518b), [Apple Developer Forums](https://developer.apple.com/forums/thread/661313), [Classmethod](https://dev.classmethod.jp/articles/ios-11-apple-music-api-intro/) |
| Music User Token | L'usuari inicia sessió amb el seu Apple ID i ha de tenir subscripció activa (no hi ha pla gratuït) | [Apple Developer Forums](https://developer.apple.com/forums/thread/703942) |
| Reproducció amb DRM a Windows | MusicKit JS reprodueix amb Widevine: Cider 1 necessita l'Electron de castlabs amb Widevine (`Cider/package.json:83`); FocalSonic demana llicències a `play.itunes.apple.com/.../acquireWebPlaybackLicense` (`FocalMK/src/helpers/constants.ts:1-2`) | codi dels repos |
| Lossless a Windows | **No hi ha via legal**: FocalSonic ho declara "Not planned" | [FocalSonic README](https://github.com/SamsidParty/FocalSonic) |
| iPhone | MusicKit natiu (Swift) amb `ApplicationMusicPlayer`; requereix el mateix programa de desenvolupador i, per publicar, App Store. La sessió de l'iPhone sense Mac ho complica encara més | <!-- TODO: verificar amb la documentació oficial de MusicKit per a Swift --> |

**Com ho esquiven els clients oberts (i per què no és un model per a Ona):** cap dels tres paga el token propi. LitoMusic extreu cada dia el token de la web d'Apple amb un script (`LitoMusic/scripts/update-developer-token.sh:5-7`); Cider 1 el demana al servidor de Cider (`Cider/src/renderer/index.js:92-105`); FocalSonic el captura de la sessió de music.apple.com dins del WebView (`FocalMK/src/runtime/token-grabber.ts:13-31`). Tot això depèn de credencials d'Apple que no són seves i es pot trencar o tancar en qualsevol moment. Cider 2 va passar a ser de pagament i tancat.

**Recomanació:** ara mateix **no**. Cost fix de 99 $/any, sense lossless a Windows, una via oficial (MusicKit JS dins WebView2) que cal verificar que reprodueix el DRM <!-- TODO: provar si el WebView2 d'Ona té CDM Widevine; LitoMusic diu que funcionava amb WebView2 però està arxivat des del 2023 -->, i els usuaris d'Apple Music es queixen sobretot de la biblioteca d'iCloud, que Ona no podria arreglar perquè és al servidor. El que Ona pot aprofitar sense cap d'aquests costos és el que falla a l'app d'Apple: fiabilitat, consum baix, cua controlable, lletra per síl·laba i comandament entre l'iPhone i el PC. Si mai es fa, la manera seriosa és un mòdul opcional amb el token propi (programa de desenvolupador) i MusicKit JS, i a l'iPhone MusicKit natiu, sense prometre lossless a Windows.
