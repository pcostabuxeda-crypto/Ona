import { invoke } from "@tauri-apps/api/core";

export type Artist = { id: string; name: string };
export type Track = {
  id: string;
  uri: string;
  name: string;
  artists: Artist[];
  album: { id: string; name: string; image?: string; thumb?: string };
  duration_ms: number;
  explicit: boolean;
};
export type Collection = {
  kind: "playlist" | "album" | "liked" | "artist";
  id: string;
  name: string;
  subtitle: string;
  image?: string;
  total?: number;
};
export type LyricLine = { t: number; text: string };
export type Lyrics = { synced: boolean; source: string; lines: LyricLine[] } | null;

/* eslint-disable @typescript-eslint/no-explicit-any */

function pickImage(images: any[] | undefined, target: number) {
  if (!images?.length) return undefined;
  const sorted = [...images].sort((a, b) => (a.width ?? 0) - (b.width ?? 0));
  return (sorted.find((i) => (i.width ?? 640) >= target) ?? sorted[sorted.length - 1]).url;
}

export function toTrack(t: any, album?: any): Track | null {
  if (!t || !t.id || t.type === "episode") return null;
  const al = t.album ?? album ?? {};
  return {
    id: t.id,
    uri: t.uri,
    name: t.name,
    artists: (t.artists ?? []).map((a: any) => ({ id: a.id, name: a.name })),
    album: {
      id: al.id,
      name: al.name,
      image: pickImage(al.images, 600),
      thumb: pickImage(al.images, 64),
    },
    duration_ms: t.duration_ms,
    explicit: !!t.explicit,
  };
}

function toCollection(x: any, kind: Collection["kind"]): Collection {
  return {
    kind,
    id: x.id,
    name: x.name,
    subtitle:
      kind === "playlist"
        ? (x.owner?.display_name ?? "Playlist")
        : kind === "album"
          ? (x.artists ?? []).map((a: any) => a.name).join(", ")
          : "Artista",
    image: pickImage(x.images, 300),
    total: x.items?.total ?? x.tracks?.total ?? x.total_tracks,
  };
}

// ---------- Crides crues ----------

const cache = new Map<string, { at: number; data: any }>();
const TTL = 5 * 60_000;

export async function api<T = any>(path: string, opts: { method?: string; body?: unknown; fresh?: boolean } = {}): Promise<T> {
  const method = opts.method ?? "GET";
  if (method === "GET" && !opts.fresh) {
    const hit = cache.get(path);
    if (hit && Date.now() - hit.at < TTL) return hit.data;
  }
  const data = await invoke<T>("api", { path, method, body: opts.body ?? null });
  if (method === "GET") cache.set(path, { at: Date.now(), data });
  else cache.clear();
  return data;
}

/** Segueix la paginació (`next`) fins a `max` elements. */
async function paged(path: string, max = 2000): Promise<any[]> {
  const items: any[] = [];
  let next: string | null = path;
  while (next && items.length < max) {
    const page: any = await api(next);
    items.push(...(page.items ?? []));
    next = page.next ? new URL(page.next).pathname + new URL(page.next).search : null;
  }
  return items;
}

// ---------- Compte ----------

export const restore = () => invoke<string | null>("restore");
export const login = () => invoke<string>("login");
export const logout = () => invoke("logout");
export const getClientId = () => invoke<string | null>("get_client_id");
export const setClientId = (clientId: string) => invoke("set_client_id", { clientId });
export const me = () => api("/v1/me");

// ---------- Reproductor ----------

export const playUri = (uri: string, position_ms = 0) => invoke("play", { uri, positionMs: position_ms });
export const preloadUri = (uri: string) => invoke("preload", { uri });
export const resume = () => invoke("resume");
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
  const id = uri.split(":").pop();
  return id ? toTrack(await api(`/v1/tracks/${id}`)) : null;
}

export const getLyrics = (t: Track) =>
  invoke<Lyrics>("get_lyrics", {
    id: t.id,
    title: t.name,
    artist: t.artists[0]?.name ?? "",
    album: t.album.name ?? "",
    durationMs: t.duration_ms,
  });

// ---------- Contingut ----------

export async function search(q: string) {
  const r = await api(`/v1/search?q=${encodeURIComponent(q)}&type=track,artist,album,playlist&limit=10`);
  return {
    tracks: (r.tracks?.items ?? []).map((t: any) => toTrack(t)).filter(Boolean) as Track[],
    artists: (r.artists?.items ?? []).filter(Boolean).map((a: any) => ({
      ...toCollection(a, "artist"),
      image: pickImage(a.images, 300),
    })) as Collection[],
    albums: (r.albums?.items ?? []).filter(Boolean).map((a: any) => toCollection(a, "album")) as Collection[],
    playlists: (r.playlists?.items ?? []).filter(Boolean).map((p: any) => toCollection(p, "playlist")) as Collection[],
  };
}

export async function myPlaylists(): Promise<Collection[]> {
  const items = await paged("/v1/me/playlists?limit=50", 500);
  return items.filter(Boolean).map((p) => toCollection(p, "playlist"));
}

export async function topTracks(): Promise<Track[]> {
  const r = await api("/v1/me/top/tracks?limit=20&time_range=short_term");
  return (r.items ?? []).map((t: any) => toTrack(t)).filter(Boolean);
}

export async function recentTracks(): Promise<Track[]> {
  const r = await api("/v1/me/player/recently-played?limit=30");
  const seen = new Set<string>();
  return (r.items ?? [])
    .map((i: any) => toTrack(i.track))
    .filter((t: Track | null) => t && !seen.has(t.id) && seen.add(t.id));
}

export async function savedAlbums(): Promise<Collection[]> {
  const r = await api("/v1/me/albums?limit=20");
  return (r.items ?? []).map((i: any) => toCollection(i.album, "album"));
}

export type Loaded = { info: Collection; tracks: Track[] };

export async function loadCollection(kind: Collection["kind"], id: string): Promise<Loaded> {
  if (kind === "liked") {
    const items = await paged("/v1/me/tracks?limit=50", 5000);
    const tracks = items.map((i) => toTrack(i.track)).filter(Boolean) as Track[];
    return { info: { kind, id, name: "M'agrada", subtitle: "La teva música", total: tracks.length }, tracks };
  }
  if (kind === "album") {
    const a = await api(`/v1/albums/${id}`);
    const tracks = (a.tracks?.items ?? []).map((t: any) => toTrack(t, a)).filter(Boolean);
    return { info: toCollection(a, "album"), tracks };
  }
  if (kind === "artist") {
    // L'API ja no dona les "populars" d'un artista: es busquen les seves cançons.
    const a = await api(`/v1/artists/${id}`);
    const r = await api(`/v1/search?q=${encodeURIComponent(`artist:"${a.name}"`)}&type=track&limit=10`);
    return {
      info: { ...toCollection(a, "artist"), image: pickImage(a.images, 600), subtitle: "Artista" },
      tracks: (r.tracks?.items ?? [])
        .filter((t: any) => t?.artists?.some((x: any) => x.id === id))
        .map((t: any) => toTrack(t))
        .filter(Boolean),
    };
  }
  const [p, items] = await Promise.all([
    api(`/v1/playlists/${id}?fields=id,name,images,owner(display_name),items(total)`),
    paged(`/v1/playlists/${id}/items?limit=50`, 5000),
  ]);
  return {
    info: toCollection(p, "playlist"),
    tracks: items.map((i) => toTrack(i.item ?? i.track)).filter(Boolean) as Track[],
  };
}

export async function artistAlbums(id: string): Promise<Collection[]> {
  const r = await api(`/v1/artists/${id}/albums?include_groups=album,single&limit=20`);
  return (r.items ?? []).map((a: any) => toCollection(a, "album"));
}

export async function isLiked(ids: string[]): Promise<boolean[]> {
  if (!ids.length) return [];
  const uris = ids.map((id) => `spotify:track:${id}`).join(",");
  return api(`/v1/me/library/contains?uris=${uris}`, { fresh: true });
}

export async function setLiked(id: string, liked: boolean) {
  await api(`/v1/me/library?uris=spotify:track:${id}`, { method: liked ? "PUT" : "DELETE" });
}

export function fmtTime(ms: number) {
  const s = Math.max(0, Math.floor(ms / 1000));
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
}
