# Customer Pulse Report: Spotify Desktop (per a Ona)

**Generated:** 2026-10-20
**Sources searched:** Spotify Community (fòrum oficial, llegit amb navegador), Trustpilot, premsa tecnològica (PCGH, Android Authority, OMG! Ubuntu, Linux Links), AlternativeTo, GitHub (Spotube, Psst, ncspot, spotify-player), Lemmy. Reddit i X: cercats, sense resultats indexats.
**Period focus:** Recent (last ~3 months, weighted toward most recent)

> ⚠️ **Confiança mitjana.** 14 punts de dades verificats. Reddit i X no han donat cap resultat, i el fòrum oficial només es pot llegir amb navegador (el fetch només en torna el títol). Els temes de RAM, lletres i cua són sòlids; els d'interfície tenen una base de 2023-2025.

---

## Top 3 This Week

> Tres tasques d'una hora o menys, concretes per a Ona.

1. **Mesura la RAM d'Ona en una sessió llarga de navegació.** El fil [High ram usage](https://community.spotify.com/forum/en/discussion/comment/6377281) descriu exactament el patró: la memòria puja amb cada cerca o enrere/endavant i no baixa (d'uns 250 MB en arrencar a 1,4 GB). El 2 de març de 2026 encara hi havia algú que es queixava que talla la música. Reprodueix el mateix recorregut amb Ona (50 navegacions i 30 minuts) i mira si la privada creix. Ona ja té un pendent relacionat: 222 MB minimitzada, que no baixen.
2. **Prototipa una finestra de lletra flotant "sempre a sobre".** Hi ha dues idees al fòrum del 2026 ([3 de febrer, 10 vots](https://community.spotify.com/forum/en/discussion/7327311/feature-request-floating-subtitle-style-lyrics-window-picture-in-picture-for-desktop) i [17 de juliol](https://community.spotify.com/forum/en/discussion/7503404/feature-idea-keep-lyrics-visible-while-multitasking), amb un comentari del 23 de juliol que la demana al Miniplayer), i Spotify no ho té. Ona ja té la lletra per síl·laba i el mode lleuger: una segona finestra de Tauri petita i transparent reaprofitaria `Lyrics.svelte`.
3. **Comprova que "Reprodueix a continuació" i "Afegeix a la cua" es trobin fàcilment a Ona.** La idea [Queue to Next or Last](https://community.spotify.com/forum/en/discussion/comment/5313023) té 1.677 kudos i 572 comentaris, i encara se'n parlava el 2021. A Trustpilot (Mason, 5 de setembre de 2026) es queixen que organitzar la cua és de pagament. Ona ja té totes dues opcions al menú contextual i un botó per a la segona, però no té drecera de teclat.

---

## Executive Summary

### Top Insights (ranked by priority)

1. **`[CONFIRMED]` La RAM és alta i creix durant la sessió**: el client oficial va d'uns 600 MB a més d'1 GB, i creix amb l'ús fins que la música talla.
   - Category: Performance | Mentions: 5 | Severity: High | Trend: Stable. Queixes des del 2021, una de nova el 2 de març de 2026, i una mesura de PCGH de l'octubre de 2026 (1.124 MB).
   - Sources: Spotify Community, PCGH, mesura pròpia (1.136 MB, 15 d'octubre)
   - Recommended Action: és el principal avantatge d'Ona (216 MB sonant). Cal blindar-lo: provar sessions llargues i el cas de la finestra minimitzada (222 MB) abans d'afegir funcions.

2. **`[CONFIRMED]` CPU alta en repòs** (~25-30 %) a la versió Store 1.2.86.
   - Category: Performance | Mentions: 2 (el fil i la resolució) | Severity: High | Trend: Improving. Obert el 29 de març i resolt per Spotify el 17 d'abril de 2026.
   - Sources: Spotify Community
   - Recommended Action: Ona gasta 0,16 % sonant. Cal mantenir el límit del 2 % i mesurar-lo en cada versió (A/B, finestra visible).

3. **`[NEW]` Veure la lletra mentre fas altres coses** (finestra flotant o lletra al miniplayer).
   - Category: Feature Request | Mentions: 3 (2 idees i 1 comentari) | Severity: Medium | Trend: New. Febrer i juliol de 2026.
   - Sources: Spotify Community
   - Recommended Action: és la petició més recent que Ona pot complir amb poc codi, reaprofitant la lletra per síl·laba. Veure el Top 3, punt 2.

4. **`[CONFIRMED]` Control de la cua**: "a continuació" contra "al final", cua llarga i difícil de reordenar, i bloquejada per als comptes gratuïts.
   - Category: Feature Request / UX | Mentions: 3 | Severity: Medium | Trend: Stable. Idea del 2012 amb 1.677 kudos; Trustpilot, setembre de 2026.
   - Sources: Spotify Community, Trustpilot
   - Recommended Action: Ona ja ho fa. Falta fer-ho visible i afegir-hi tecles.

5. **`[CONFIRMED]` Fiabilitat**: es congela, pantalles en blanc o negres, la música s'atura.
   - Category: Bug | Mentions: 4 | Severity: High | Trend: Stable. Un fil sobre la pantalla negra a Windows 11 després de la instal·lació del 15 de juny de 2026 té 35 respostes.
   - Sources: Spotify Community
   - Recommended Action: per a Ona, assegurar-se que cap error de xarxa deixa una vista en blanc (avui surt "No s'ha pogut carregar").

6. **`[CONFIRMED]` Redisseny que malbarata espai, també en monitors verticals.** Algú diu que la portada ampliada els aixafa la pantalla en un monitor vertical.
   - Category: UX | Mentions: 3 | Severity: Medium | Trend: Stable (queixes del 2023 al 2025).
   - Sources: Spotify Community (cercador), Digital Music News
   - Recommended Action: és el teu cas (608×1080). Ona ja s'adapta; a la fase 3, auditar-ho en estret.

7. **`[CONFIRMED]` Funcions que desapareixen o falten a escriptori**: canviar la portada de les playlists, multiselecció, ordenar, equalitzador.
   - Category: Feature Request | Mentions: 3 | Severity: Medium | Trend: Stable.
   - Sources: Spotify Community
   - Recommended Action: a Ona hi ha la multiselecció, ordenar les llistes i l'equalitzador (no en té cap). Prioritzar a la fase 3.

8. **`[NEW]` Els clients de tercers que fan servir la Web API topen amb límits i quotes.** El 9 d'octubre de 2026, Spotifast donava "rate limiting" i deixava playlists buides. Psst exigeix un Client ID propi des del febrer de 2026.
   - Category: Churn Risk / Competitive | Mentions: 2 | Severity: High (per als competidors) | Trend: New.
   - Sources: PCGH, GitHub de Psst
   - Recommended Action: és un avantatge d'Ona, que fa servir els endpoints interns i no depèn de la quota. Cal mantenir la regla de no tornar a la Web API.

9. **`[NEW]` Apareixen clients natius lleugers**: Spotifast (Rust/egui, pic de 168 MB) i Fastpotify.
   - Category: Competitive | Mentions: 3 | Severity: Medium | Trend: New (octubre de 2026).
   - Sources: PCGH, SourceForge, zeli.app
   - Recommended Action: a Spotifast li falten coses que Ona ja té (fitxers locals) i coses que Ona tampoc té (baixades offline, crossfade). A la fase 2, comparar-ne el codi.

10. **`[CONFIRMED]` Preu i anuncis**: increment del 2026, anuncis repetits al pla gratuït, cobraments.
    - Category: Pricing | Mentions: 7 | Severity: Medium | Trend: Stable o pitjor (ressenyes de setembre i octubre de 2026).
    - Sources: Trustpilot
    - Recommended Action: fora de l'abast d'Ona (cal Premium). El mode sense Spotify (Audius i locals) respon en part als qui no paguen.

### Sentiment Overview
- **Overall sentiment:** Negatiu. Al fòrum i a Trustpilot dominen el rendiment, la fiabilitat, el preu i els anuncis. Els comentaris positius són sobretot sobre el catàleg.
- **Trending:** Estable. La CPU ha millorat (resolta l'abril), la RAM continua igual (queixa del març de 2026) i les lletres són una petició nova del 2026.

---

## Detailed Feedback by Category

### Bugs & Reliability Issues

| Issue | Sources | Mentions | Severity | Representative Quote |
|-------|---------|----------|----------|---------------------|
| La RAM creix fins que l'app falla | Community | 3 | High | "Spotify eats more and more memory. It never release it." |
| Es congela, pantalla en blanc o negra | Community (cercador) | 3 | High | — (fils sense text llegible; resum del cercador) |
| Retard en acabar una cançó | Trustpilot | 1 | Low | "sometimes a couple of seconds delay when song ends" |

Per al fòrum, la solució habitual és "reinstal·la". La fuita de memòria per navegació és l'error més documentat i el que més afecta l'ús llarg.

### Feature Requests

| Issue | Sources | Mentions | Severity | Representative Quote |
|-------|---------|----------|----------|---------------------|
| Lletra flotant o sempre visible | Community | 3 | Medium | "A floating, resizable lyrics window" |
| Cua: a continuació o al final | Community, Trustpilot | 3 | Medium | "Present two options on rightclick, Queue Last and Queue Next." |
| Equalitzador a escriptori | Community (cercador) | 1 | Low | — |

### UX & Usability

| Issue | Sources | Mentions | Severity | Representative Quote |
|-------|---------|----------|----------|---------------------|
| La barra lateral malbarata espai; problemes en monitor vertical; aspecte de xarxa social | Community (cercador), Digital Music News | 3 | Medium | — (el fil demana iniciar sessió; resum del cercador) |

### Performance

| Issue | Sources | Mentions | Severity | Representative Quote |
|-------|---------|----------|----------|---------------------|
| CPU en repòs ~30 % (1.2.86 Store) | Community | 2 | High | "using alot of my CPU while just idle" |
| RAM ~1 GB (mesurat) | PCGH, pròpia | 2 | High | (xifra) 1.124 MB contra 168 MB de Spotifast |
| RAM encara alta el 2026 | Community | 1 | High | "2026 still using a wild amount of memory" |

### Pricing & Value

| Issue | Sources | Mentions | Severity | Representative Quote |
|-------|---------|----------|----------|---------------------|
| Anuncis repetits al pla gratuït | Trustpilot | 5 | Medium | "the exact same advert over 20 times in a row" |
| Funcions bàsiques de pagament | Trustpilot | 1 | Medium | "organizing your queue are locked behind a paywall" |
| Cobraments i reemborsaments | Trustpilot | 2 | Medium | — |

### Praise & Positive Sentiment

| Issue | Sources | Mentions | Severity | Representative Quote |
|-------|---------|----------|----------|---------------------|
| Catàleg i biblioteca | Trustpilot | 1 | — | (ressenya de 5★, març de 2026, sobre el catàleg; resum del cercador) |
| Agrada la lletra en temps real | Community | 1 | — | "I love Spotify's real-time lyrics feature" |

### Churn Risks & Competitive Pressure

| Issue | Sources | Mentions | Severity | Representative Quote |
|-------|---------|----------|----------|---------------------|
| Clients natius lleugers (Spotifast, Fastpotify) | PCGH, SourceForge | 3 | Medium | — |
| Clients de terminal per eficiència (ncspot) | OMG! Ubuntu | 1 | Low | "super resource efficient" |
| Spotube per escoltar sense Premium, però amb pistes equivocades | Linux Links | 1 | Medium | — |
| Abandonament a Android cap a YouTube Music o Apple Music (agost de 2026) | Android Authority | 1 | Medium | — (mòbil, no escriptori) |

---

## Known vs. New Findings

### Confirmed Known Issues

| Theme | Known Pain Point | Status vs. Known |
|-------|-----------------|-----------------|
| RAM alta i creixent | Rendiment i recursos | Consistent; s'hi afegeix la mesura de PCGH (octubre de 2026) |
| CPU en repòs | Rendiment i recursos | Millorant (resolt el 17 d'abril de 2026) |
| Es congela, s'atura | Fiabilitat de reproducció | Consistent |
| Funcions que falten | Funcions que falten a escriptori | Consistent |
| Preu i anuncis | Preu | S'agreuja als comptes gratuïts (ressenyes de setembre i octubre de 2026) |
| Interfície i monitor vertical | (parcial: Personalització) | Evoluciona: no hi era com a dolor propi; cal afegir-ho |

### New Discoveries

| Theme | Category | Severity | Why It Matters |
|-------|----------|----------|---------------|
| Lletra flotant o sempre visible | Feature Request | Medium | Petició del 2026 que Spotify no fa i que Ona pot fer amb poc (té la lletra per síl·laba) |
| Límits de la Web API per als clients de tercers | Competitive | High | Els competidors lleugers fallen per quota; Ona no, i és un avantatge clar |
| Nous clients natius lleugers | Competitive | Medium | Estableixen la referència de RAM (168 MB): Ona hi ha d'estar a prop |

---

## Blind Spots & Gaps

### Source Gaps

| Source Category | Useful Data? | Notes |
|-----------------|-------------|-------|
| Reddit | No | 3 cerques (amb `site:` i sense), cap fil indexat: el cercador no torna Reddit |
| Twitter / X | No | 2 cerques, cap publicació indexada |
| Review Sites | Sí (Trustpilot) | Són de l'app en general; pràcticament cap esment de l'escriptori |
| Product Forum | Sí | Ha migrat a una plataforma nova; el fetch només en torna el títol, i s'ha llegit amb navegador (5 fils). La secció App-Features demana iniciar sessió (no consultada) |
| General Web | Sí | PCGH (octubre de 2026) amb mesures reals |
| Competitor / Churn | Parcial | Sobretot Linux i terminal; poc d'usuaris de Windows |

### Customer Segment Gaps

| Segment | Feedback Found? | Notes |
|---------|----------------|-------|
| General Premium Windows/Mac | Sí | La majoria del fòrum |
| Maquinari modest o portàtils | Mínim | Ningú parla de bateria; buit |
| Personalitzadors (Spicetify) | No | No s'ha cercat al Discord ni al subreddit de Spicetify |
| Terminal i Linux | Sí | OMG! Ubuntu, Lemmy |
| Sense Premium (Spotube) | Parcial | Linux Links (pistes equivocades) |
| Amants de les lletres i el karaoke | Sí | 2 idees del 2026 |
| Pantalles verticals | Mínim | 1 esment indirecte (portada aixafada en monitor vertical) |

### Product Area Gaps
- **Listening Activity (2026):** cap esment. Pot ser poc ús o que encara no ha arribat.
- **Baixades offline a escriptori:** cap queixa recent. Pot ser satisfacció o poc ús; a Spotifast és una de les funcions que falten.
- **Spotify Connect a escriptori:** cap esment.
- **Lletres per síl·laba:** ningú demana síl·labes explícitament; demanen veure la lletra mentre fan altres coses.

---

## Methodology

- **Sources searched:** 11 WebSearch: Reddit 3, X 2, Trustpilot 1, fòrum 2, web 1, competència 1 i 1 de RAM.
- **Pages fetched:** 12 WebFetch (2 útils: PCGH i Trustpilot; 10 del fòrum només amb el títol) i 7 amb el navegador (5 útils i 1 que redirigeix a l'inici de sessió).
- **Total data points:** 14 elements de feedback diferents.
- **Source URLs:**
  - https://community.spotify.com/forum/en/discussion/7390422/high-idle-cpu-usage-30
  - https://community.spotify.com/forum/en/discussion/comment/6377281
  - https://community.spotify.com/forum/en/discussion/7327311/feature-request-floating-subtitle-style-lyrics-window-picture-in-picture-for-desktop
  - https://community.spotify.com/forum/en/discussion/7503404/feature-idea-keep-lyrics-visible-while-multitasking
  - https://community.spotify.com/forum/en/discussion/comment/5313023
  - https://www.pcgameshardware.de/RAM-Hardware-154108/News/Spotify-Alternative-Spotifast-RAM-Verbrauch-Premium-Download-1556271/
  - https://www.trustpilot.com/review/www.spotify.com?page=3
  - https://www.omgubuntu.co.uk/2020/02/spotify-cli-client-ncspot
  - https://linuxlinks.com/spotube-flutter-based-lightweight-spotify-client
  - https://androidauthority.com/spotify-android-app-lag-performance-issues-3698268
  - https://github.com/jpochyla/psst
  - Cercador (no llegits a fons): fils "New desktop interface is ugly" (demana iniciar sessió), "Desktop app unusable" i "Desktop app (Windows) not working".
- **Limitations:** sense Reddit ni X. Els fils de la interfície són dels anys 2023-2025 i el resum ve del cercador. Trustpilot és tota l'app (sobretot el mòbil). No hi ha dades de bateria ni de pantalles verticals.
