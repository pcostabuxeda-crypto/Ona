import { invoke } from "@tauri-apps/api/core";
import { covers } from "./covers.svelte";
import { audiusTrending, localTracks } from "./sources";

export type Artist = { id: string; name: string };
export type Track = {
  id: string;
  uri: string;
  name: string;
  artists: Artist[];
  album: { id: string; name: string; image?: string; thumb?: string };
  duration_ms: number;
  explicit: boolean;
  /** Pistes que no són de Spotify (Audius, fitxers locals): URL que reprodueix l'<audio> de la interfície. */
  src?: string;
};
export type Collection = {
  kind: "playlist" | "album" | "liked" | "artist" | "audius" | "local";
  id: string;
  name: string;
  subtitle: string;
  image?: string;
  total?: number;
};
export type LyricLine = { t: number; text: string };
export type Lyrics = { synced: boolean; source: string; lines: LyricLine[]; ttml?: string } | null;
export type HomeSection = { title: string; items: Collection[] };

/* eslint-disable @typescript-eslint/no-explicit-any */

// Les dades venen dels endpoints spclient que fa servir el client d'escriptori de Spotify
// (metadades protobuf per lots, playlist4, col·lecció, context-resolve), via la sessió de librespot.

const idOf = (uri: string) => uri.split(":").pop() ?? uri;

const cache = new Map<string, { at: number; data: any }>();
const TTL = 5 * 60_000;
// Límits perquè la memòria no creixi amb la navegació (la queixa principal del client oficial).
const MAX_CACHE = 150;
const MAX_TRACKS = 20_000;

/** Treu les entrades més antigues d'un Map (l'ordre d'inserció fa de LRU). */
function trim<K, V>(map: Map<K, V>, max: number) {
  for (const k of map.keys()) {
    if (map.size <= max) break;
    map.delete(k);
  }
}

async function cached<T>(key: string, fresh: boolean, load: () => Promise<T>): Promise<T> {
  const hit = cache.get(key);
  if (!fresh && hit && Date.now() - hit.at < TTL) {
    cache.delete(key);
    cache.set(key, hit); // ara és la més recent
    return hit.data;
  }
  const data = await load();
  cache.set(key, { at: Date.now(), data });
  trim(cache, MAX_CACHE);
  return data;
}

/** Nom, artistes, àlbum i durada de cada pista (una crida per lot, com el client). */
const trackCache = new Map<string, Track>();
async function decorate(uris: string[]): Promise<Track[]> {
  const missing = [...new Set(uris.filter((u) => u.startsWith("spotify:track:") && !trackCache.has(u)))];
  if (missing.length) for (const t of await invoke<Track[]>("tracks", { uris: missing })) trackCache.set(t.uri, t);
  const out = uris.map((u) => trackCache.get(u)).filter(Boolean) as Track[];
  trim(trackCache, MAX_TRACKS); // després de fer servir les d'ara
  return out;
}

/** Portada de mosaic (4 àlbums diferents), com les playlists sense imatge pròpia del client. */
function mosaic(tracks: Track[]): string | undefined {
  const ids = [...new Set(tracks.map((t) => t.album.image?.split("/").pop()).filter(Boolean))];
  if (ids.length >= 4) return `https://mosaic.scdn.co/300/${ids.slice(0, 4).join("")}`;
  return tracks[0]?.album.image;
}

// ---------- Compte ----------

export const restore = () => invoke<string | null>("restore");
export const login = () => invoke<string>("login");
export const logout = () => {
  likedSet = null; // un altre compte tindrà uns altres M'agrada
  return invoke("logout");
};
export const me = () => cached("me", false, () => invoke<any>("me"));

// ---------- Reproductor ----------

export const playUri = (uri: string, position_ms = 0) => invoke("play", { uri, positionMs: position_ms });
export const preloadUri = (uri: string) => invoke("preload", { uri });
export const resume = () => invoke("resume");
/** Mode «només so»: la cua (a partir de la que sona) passa a Rust i es tanca la finestra. */
export const soundOnly = (tracks: Track[], repeatOne: boolean) =>
  invoke("sound_only", {
    items: tracks.map((t) => ({
      uri: t.uri,
      title: t.name,
      artist: t.artists.map((a) => a.name).join(", "),
      album: t.album.name ?? null,
      cover: t.album.image ?? null,
      durationMs: t.duration_ms ?? null,
    })),
    repeatOne,
  });
/** Quantes cançons ha avançat Rust sense finestra (0 si no n'hi havia). */
export const soundOnlyTake = () => invoke<number>("sound_only_take");
export const quit = () => invoke("quit");
/** Ordre al Spotify Connect quan el que sona ho ha posat el mòbil. */
export const connectCmd = (action: "toggle" | "next" | "prev" | "seek", ms?: number) =>
  invoke("connect_cmd", { action, ms: ms == null ? null : Math.round(ms) }).catch(() => {});
export const pause = () => invoke("pause");
export const seek = (position_ms: number) => invoke("seek", { positionMs: Math.round(position_ms) });
export const setVolume = (volume: number) => invoke("set_volume", { volume });

export const mediaUpdate = (t: Track | undefined) =>
  invoke("media_update", {
    title: t?.name ?? null,
    artist: t ? t.artists.map((a) => a.name).join(", ") : null,
    album: t?.album.name ?? null,
    cover: t?.album.image ?? null,
    durationMs: t?.duration_ms ?? null,
  }).catch(() => {});
export const mediaState = (playing: boolean, positionMs: number) =>
  invoke("media_state", { playing, positionMs: Math.max(0, Math.round(positionMs)) }).catch(() => {});

export const playerState = () =>
  invoke<{ uri: string; position_ms: number; playing: boolean } | null>("player_state");

export async function trackByUri(uri: string): Promise<Track | null> {
  return (await decorate([uri]))[0] ?? null;
}

/** `syllables`: demanar també la lletra per síl·laba (Better Lyrics); el mode lleuger no la fa servir. */
export const getLyrics = (t: Track, syllables = true) =>
  invoke<Lyrics>("get_lyrics", {
    id: t.id,
    title: t.name,
    artist: t.artists[0]?.name ?? "",
    album: t.album.name ?? "",
    durationMs: t.duration_ms,
    syllables,
  });

// ---------- Contingut ----------

const LIKED: Collection = { kind: "liked", id: "liked", name: "Cançons que m'agraden", subtitle: "Playlist" };

/**
 * Inici fet amb la teva biblioteca: sense la consulta GraphQL "home" (pathfinder no és accessible),
 * no hi ha les recomanacions personalitzades del client.
 */
export async function home(free = false): Promise<{ shortcuts: Collection[]; sections: HomeSection[] }> {
  if (free) return { shortcuts: [LOCAL, AUDIUS], sections: [] };
  const [playlists, artists, albums] = await Promise.all([
    myPlaylists(),
    followedArtists().catch(() => []),
    savedAlbums().catch(() => []),
  ]);
  return {
    shortcuts: [LIKED, ...playlists.slice(0, 5), LOCAL, AUDIUS],
    sections: [
      { title: "Les teves playlists", items: playlists },
      { title: "Artistes que segueixes", items: artists },
      { title: "Àlbums desats", items: albums },
    ].filter((s) => s.items.length),
  };
}

/** Cerca de cançons; els artistes i àlbums surten de les cançons trobades. */
export async function search(q: string) {
  const tracks = await cached(`search:${q}`, false, () => invoke<Track[]>("search", { q }));
  for (const t of tracks) trackCache.set(t.uri, t);
  const uniq = (xs: string[]) => [...new Set(xs.filter((x) => x && !x.endsWith(":")))].slice(0, 10);
  const [artists, albums] = await Promise.all([
    invoke<Collection[]>("artists", { uris: uniq(tracks.flatMap((t) => t.artists.map((a) => `spotify:artist:${a.id}`))) }).catch(() => []),
    invoke<Collection[]>("albums", { uris: uniq(tracks.map((t) => `spotify:album:${t.album.id}`)) }).catch(() => []),
  ]);
  return { tracks, artists, albums, playlists: [] as Collection[] };
}

export async function myPlaylists(): Promise<Collection[]> {
  return cached("rootlist", false, async () => {
    const [list, user] = await Promise.all([invoke<any[]>("rootlist"), me().catch(() => null)]);
    const mine = user?.uri?.split(":").pop();
    return list.map((p) => ({
      kind: "playlist" as const,
      id: idOf(p.uri),
      name: p.name,
      subtitle: `Playlist · ${p.owner === mine ? user.name : p.owner}`,
      image: p.image ?? undefined,
      total: p.total,
    }));
  });
}

/** Portades que falten (mosaics): es calculen amb les primeres pistes de cada playlist. */
const pendingCovers = new Set<string>();
export async function fillCovers(list: Collection[]) {
  for (const p of list.filter((p) => p.kind === "playlist" && !p.image && !pendingCovers.has(p.id))) {
    pendingCovers.add(p.id);
    try {
      const d = await invoke<any>("playlist", { id: p.id });
      const img = mosaic(await decorate(d.uris.slice(0, 12)));
      if (img) covers[p.id] = img;
    } catch {}
  }
}

/** URIs desades a la biblioteca: M'agrada i àlbums ("collection"), artistes ("artist"). */
const library = (set: "collection" | "artist", fresh = false) =>
  cached(`collection:${set}`, fresh, () => invoke<{ uri: string; added_at: number }[]>("collection", { set }));

export async function savedAlbums(): Promise<Collection[]> {
  const uris = (await library("collection")).map((i) => i.uri).filter((u) => u.startsWith("spotify:album:"));
  return uris.length ? invoke<Collection[]>("albums", { uris: uris.slice(0, 100) }) : [];
}

export async function followedArtists(): Promise<Collection[]> {
  const uris = (await library("artist")).map((i) => i.uri);
  return uris.length ? invoke<Collection[]>("artists", { uris: uris.slice(0, 100) }) : [];
}

export type Loaded = { info: Collection; tracks: Track[] };

const artistData = (id: string) =>
  cached(`artist:${id}`, false, () => invoke<{ info: Collection; top: Track[]; albums: Collection[] }>("artist", { id }));

export const AUDIUS: Collection = { kind: "audius", id: "trending", name: "Tendències a Audius", subtitle: "Música lliure i gratuïta" };
export const LOCAL: Collection = { kind: "local", id: "music", name: "Fitxers locals", subtitle: "La carpeta Música" };

export async function loadCollection(kind: Collection["kind"], id: string): Promise<Loaded> {
  if (kind === "audius") return { info: AUDIUS, tracks: await audiusTrending() };
  if (kind === "local") {
    const tracks = await localTracks(true);
    return { info: { ...LOCAL, total: tracks.length }, tracks };
  }
  if (kind === "liked") {
    const uris = (await library("collection", true)).map((i) => i.uri).filter((u) => u.startsWith("spotify:track:"));
    const tracks = await decorate(uris);
    return { info: { ...LIKED, total: tracks.length }, tracks };
  }
  if (kind === "album") {
    const a = await cached(`album:${id}`, false, () => invoke<Loaded>("album", { id }));
    return { info: { ...a.info, subtitle: `Àlbum · ${a.info.subtitle}` }, tracks: a.tracks };
  }
  if (kind === "artist") {
    const a = await artistData(id);
    return { info: a.info, tracks: a.top };
  }
  const p = await invoke<any>("playlist", { id });
  const tracks = await decorate(p.uris);
  const mine = (await me().catch(() => null)) as any;
  return {
    info: {
      kind,
      id,
      name: p.name,
      subtitle: `Playlist · ${mine?.uri?.endsWith(`:${p.owner}`) ? mine.name : p.owner}`,
      image: p.image ?? mosaic(tracks),
      total: p.uris.length,
    },
    tracks,
  };
}

export async function artistAlbums(id: string): Promise<Collection[]> {
  return (await artistData(id)).albums;
}

/** M'agrada en memòria: es carrega un cop i s'actualitza en marcar (abans: tota la col·lecció per cançó). */
let likedSet: Promise<Set<string>> | null = null;
let likedAt = 0;
/** Es recarrega com a molt cada 5 min (per veure els M'agrada fets des del mòbil), no a cada cançó. */
function liked() {
  if (!likedSet || Date.now() - likedAt > TTL) {
    likedAt = Date.now();
    likedSet = library("collection", true)
      .then((l) => new Set(l.map((i) => i.uri)))
      .catch((e) => ((likedSet = null), Promise.reject(e)));
  }
  return likedSet;
}

export async function isLiked(ids: string[]): Promise<boolean[]> {
  if (!ids.length) return [];
  const set = await liked();
  return ids.map((id) => set.has(`spotify:track:${id}`));
}

export async function setLiked(id: string, like: boolean) {
  const uri = `spotify:track:${id}`;
  await invoke("collection_write", { set: "collection", uris: [uri], remove: !like });
  cache.delete("collection:collection");
  const set = await liked().catch(() => null);
  if (like) set?.add(uri);
  else set?.delete(uri);
}

export function fmtTime(ms: number) {
  const s = Math.max(0, Math.floor(ms / 1000));
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
}
