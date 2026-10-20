# Ona: direcció visual de les millores aprovades (fase 4)

**Abast:** P0 1-6 i P1-7 de `reports/ona-audit-2026-10-20.md`. Només especificació; la implementació és a la fase 5.
**Brief fix:** l'estètica és la de Spotify, amb tokens a `src/app.css` (i Liner com a opció). El color i la tipografia no es toquen. A l'usuari li agraden sobretot les transicions i les molles. Pantalla vertical de 608×1080. Màxim ~2 % de CPU sonant, sense `backdrop-filter`, i només s'animen `transform` i `opacity`.

## 1. Pla

**On posem l'atreviment:** al moviment que respon a una acció. Ona no pot destacar pel color (és el de Spotify), però sí per com es mou quan fas alguna cosa. Hi ha un sol moment memorable (la portada que vola cap a la cua); tota la resta és tranquil·la i ràpida.

**Color:** el mateix d'ara, sense afegir-hi res.

| Nom | Hex | Ús |
|---|---|---|
| base | `#121212` | panells |
| highlight | `#1f1f1f` | superfícies interactives |
| text | `#ffffff` | primari |
| text-2 | `#b3b3b3` | secundari |
| accent | `#1ed760` | estat actiu (amb Liner, `#fff`) |
| error | `#f3727f` | **nou al sistema**; ja existeix a `Login.svelte:80`, ara també per als errors de reproducció |

**Tipus:** el mateix (`--font`, `--font-title`). L'única regla nova: les dreceres als `title` dels botons van amb espai fi, com «Lletra (Ctrl L)».

**Moviment (tokens nous a `app.css`):**
```
--spring:      linear(0, 0.013 0.6%, 0.05 1.2%, 0.2 2.6%, 0.711 7.2%, 0.919 9.6%, 1.054 12.2%,
               1.106 14%, 1.124 15.7%, 1.117 17.6%, 1.057 22.4%, 1.016 26.6%, 0.995 32.4%, 0.996 42.3%, 1);
--spring-soft: linear(0, 0.009, 0.035 2.1%, 0.141 4.4%, 0.723 12.9%, 0.938 16.7%, 1.017 20%, 1.039 22.7%,
               1.042 25.4%, 1.026 31.5%, 1.004 40.6%, 0.998 51.9%, 1);
--t-spring: 520ms;   /* entrades i coses que "arriben" */
--t-quick:  180ms;   /* sortides, hover, canvis d'estat */
@media (prefers-reduced-motion: reduce) { --t-spring: 1ms; --t-quick: 1ms; }  /* i cap transform de desplaçament */
```
Són les molles que ja tenia l'Ona 0.1 (Material 3 Expressive) i que l'usuari va triar. Regla: **entrar amb molla, sortir de pressa i sense rebot**.

**Principis:**
1. El moviment explica què ha canviat i on ha anat; mai és decoració que es repeteix sola.
2. Res que soni en bucle de fons. L'únic bucle que es manté és l'equalitzador de la fila activa, que ja existia.
3. A 608 px res no tapa el contingut sense que l'hagis demanat.
4. Els errors diuen què ha passat i què pots fer, al mateix lloc on miraves.

## 2. Revisió contra el brief (què he canviat i per què)
- **Primer esborrany: obrir el panell «Ara sona» amb una lliscada i un fons enfosquit amb desenfocament.** És el calaix genèric i, a més, `backdrop-filter` està prohibit. **Canvi:** el panell surt de la dreta amb una molla, i el fons s'enfosqueix només amb `opacity` (un div negre al 45 %).
- **Primer esborrany: totes les seccions apareixen amb fosa i lliscada en carregar.** És el tic de les pàgines generades i gasta CPU en cada navegació. **Canvi:** les vistes canvien sense coreografia; només animen les accions de l'usuari.
- **Primer esborrany: la lletra flotant transparent, amb vidre.** Les finestres transparents de Windows fan treballar més la GPU i el vidre requereix `backdrop-filter`. **Canvi:** finestra opaca `#121212`, rodona i amb ombra del sistema.
- **Es manté:** «Playlist · Propietari» amb punt volat. És un tret genèric, però el brief demana l'estètica de Spotify i Spotify ho fa així.

## 3. Especificació per millora

### P0-1 · Cua d'usuari (el moment memorable)
**Comportament:** «Afegeix a la cua» posa la cançó a la teva cua, en ordre d'arribada, i sona abans que la resta del context, com fa Spotify. «Reprodueix a continuació» la posa al davant de la teva cua. L'aleatori només barreja el context, mai la teva cua. Clicar una altra pista conserva la teva cua.

**El moment:** en afegir una cançó, la miniatura de la fila es clona i **vola fins al botó de la cua** de la barra inferior. Fa un arc i encongeix fins a 20 px (FLIP amb `transform`, 520 ms, `--spring`). En arribar, el botó fa un petit cop (escala 1 → 1,18 → 1, `--spring`) i el comptador s'actualitza. Amb moviment reduït, només s'actualitza el comptador i surt l'avís.

```
Panell de la cua (1280: a la dreta · 608: full lateral)
┌──────────────────────────────┐
│ Cua                     ✕     │
│ Ara sona                      │
│ ▣ Elefants · Oques Grasses    │
│ A continuació a la teva cua  2│ ← la secció només existeix si n'hi ha
│ ⋮▣ Serem Ocells          ✕    │   (arrossegar: ⋮ amb el teclat, Alt ↑/↓)
│ ⋮▣ Valentina             ✕    │
│ Després, de: Català      [Buida la cua]
│  ▣ …                          │
└──────────────────────────────┘
```
- Comptador al botó de la cua: un número petit a sobre de la icona, només quan la teva cua no és buida.
- Treure una cançó: la fila es plega amb `transform: scaleY` i `opacity` (180 ms) i la resta puja amb FLIP (`--spring-soft`).
- «Buida la cua»: avís amb «Desfés» durant 5 s, en lloc d'esborrar-la sense confirmació.

### P0-2 i P0-3 · Consum (sense canvi visual)
Desar només quan hi ha canvis, memòries cau amb límit i M'agrada en un conjunt. L'usuari no ho veu: és la condició perquè la resta de moviment càpiga dins el 2 %.

### P0-4 · Pantalla de 608×1080
Punts de tall nous: **<720 px**, la barra del reproductor passa a dues files; **<700 px**, la capçalera de les col·leccions s'apila. El panell «Ara sona» comença **tancat per sota de 1000 px**.

```
608 px · barra del reproductor (2 files, 88 px)
┌───────────────────────────────────────────┐
│ ▣ Elefants              ♡   ⏮  ⏯  ⏭   ⋯  │  ← ⋯ obre: aleatori, repetir, lletra,
│   Oques Grasses                           │     cua, Ara sona, lletra flotant, volum
│ 1:22 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 4:02  │  ← la barra de temps ocupa tota l'amplada
└───────────────────────────────────────────┘
```
- El menú `⋯` apareix des del botó amb `scale(.96) → 1` i `opacity`, en 180 ms, sense rebot (és un menú, no una arribada).
- **Panell «Ara sona» i cua <1000 px:** són un full lateral d'amplada `min(360px, 88vw)`. Entren amb `translateX(100%) → 0` i `--spring` (520 ms) i surten amb `--t-quick`. Tenen un fons negre al 45 % que es tanca en clicar-lo o amb Esc.

```
608 px · capçalera de col·lecció apilada
┌─────────────────────────────┐
│ ▣▣▣▣ 160 px                  │
│ ▣▣▣▣                         │
│ Playlist                     │
│ Català                       │  ← títol: clamp(28px, 8vw, 48px), text-wrap: balance
│ pop · 45 cançons, 2 h 38 min │
│ (▶)  ⤨                       │
└─────────────────────────────┘
```
- En fer scroll, la portada s'encongeix fins a 0,8 i s'esvaeix amb `animation-timeline: scroll()`. Ho fa el compositor, sense JS i amb cost pràcticament nul. Amb moviment reduït, no.

### P0-5 · Teclat
No hi ha cap canvi visual, però els `title` mostren les dreceres. L'espai només fa play/pausa si el focus no és en un botó. El control lliscant atura la tecla i no la deixa arribar a la drecera global.

| Acció | Drecera |
|---|---|
| Lletra | Ctrl L |
| Lletra flotant | Ctrl Maj L |
| Cua | Ctrl Q |
| M'agrada | Ctrl S (com Spotify) |
| Aleatori / Repetir | Ctrl Maj S / Ctrl R |
| Afegeix a la cua (fila amb el focus) | Q · Reprodueix a continuació: Maj Q |

### P0-6 · Errors de reproducció i d'inici de sessió
**A la barra, al mateix lloc on miraves:** si una cançó falla, la línia de l'artista es converteix en el missatge.
```
│ ▣ Elefants                                   │
│   No s'ha pogut reproduir. [Torna-ho a provar] · [Salta]  │   ← color --error, aria-live="polite"
```
- La portada baixa a `opacity: .5` en 180 ms i el missatge entra amb `translateY(4px) → 0` i `--spring-soft`.
- Mentre carrega (`player.loading`), el botó de play mostra un anell que gira amb `transform: rotate` (només mentre carrega, mai en bucle de fons).
- Si una cançó no està disponible i se salta, surt l'avís «S'ha saltat "Títol": no està disponible al teu país» (o el motiu que tinguem).
- **Credencials que no valen:** l'inici de sessió esborra les credencials i torna al navegador sol. La pantalla diu «La sessió ha caducat. Torna a iniciar sessió amb Spotify.», amb un sol botó.
- **Arrencada:** en lloc d'una finestra buida, el logotip de l'ona dibuixa el traç una sola vegada (`stroke-dashoffset`, 900 ms) mentre connecta. Si triga més de 8 s, apareix «Connectant amb Spotify…» i el botó «Continua sense Spotify».

### P1-7 · Lletra flotant
**Format:** una segona finestra de Tauri `lyrics-mini` de 380×120 (redimensionable entre 280×90 i 640×240), sempre a sobre, sense marc, opaca, amb cantonades de 12 px i ombra del sistema. Es pot arrossegar per qualsevol punt.

```
┌────────────────────────────────────────┐
│ Tinc un cel i un infern a dins          │  ← línia actual: 20 px, 700, blanc
│ Milers de nits que…                     │  ← línia següent: 15 px, text-2
└────────────────────────────────────────┘
  en passar-hi el ratolí:  ⏮ ⏯ ⏭      Aa− Aa+   ✕   (fosa de 180 ms)
```
- **Canvi de línia (el detall que l'usuari notarà):** la línia actual puja i s'esvaeix (`translateY(-100%)`, `opacity 0`, 180 ms); la següent puja al seu lloc i creix de 15 a 20 px. El creixement es fa amb `scale(.75) → 1` des de `transform-origin: left`, amb `--spring` (520 ms). Una línia nova entra per sota. Només hi ha dues línies al DOM, i només es mouen en canviar de línia (cada pocs segons), amb cost gairebé zero.
- **Síl·labes:** amb lletra de Better Lyrics i sense el mode lleuger, la línia actual s'omple paraula per paraula amb un color pla (`data-word-state`, sense l'escombrat amb `mask-image`, que era el que costava CPU).
- **Fons:** sense kawarp. Un degradat estàtic de `--pa` (la paleta de la portada) a `#121212`, que canvia per fosa de 600 ms amb cada cançó.
- **Com s'obre:** botó a la barra (`⋯` a 608 px), Ctrl Maj L i entrada al menú de la cançó. Es tanca amb la ✕, Esc o el mateix botó. Recorda la posició i la mida.
- **Sense lletra:** «Aquesta cançó no té lletra», en `text-2`, i la finestra no s'amaga.

## 4. Coses que NO farem
- Animar l'entrada de cada vista o de cada targeta.
- Desenfocament, vidre o `backdrop-filter`.
- Bucles decoratius (un disc que gira, el visualitzador): ja es van treure per consum.
- Canviar la paleta, la tipografia o les cantonades globals.

## 5. Com ho mesurarem (fase 5)
Release, finestra visible, A/B seguit, ona.exe més els WebView2 `cat.ona.app`:
- sonant a l'Inici: igual que ara (~0,16 %) o menys;
- amb la lletra flotant oberta i la finestra principal minimitzada: objectiu ≤ 0,5 %;
- RAM privada després de 50 navegacions: no ha de créixer més d'un 10 % respecte a l'inici.
