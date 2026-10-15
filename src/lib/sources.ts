/**
 * Fonts que no necessiten Spotify: Audius (API pública i gratuïta, música d'artistes independents)
 * i els fitxers de la carpeta Música. Es reprodueixen amb l'<audio> de la interfície (Track.src).
 */
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import type { Track } from "./api";

/* eslint-disable @typescript-eslint/no-explicit-any */

const AUDIUS = "https://api.audius.co/v1";
const APP = "app_name=Ona";

function audiusTrack(t: any): Track {
  return {
    id: `audius-${t.id}`,
    uri: `audius:track:${t.id}`,
    name: t.title,
    artists: [{ id: "", name: t.user?.name ?? "" }],
    album: { id: "", name: "Audius", image: t.artwork?.["480x480"], thumb: t.artwork?.["150x150"] },
    duration_ms: (t.duration ?? 0) * 1000,
    explicit: false,
    src: `${AUDIUS}/tracks/${encodeURIComponent(t.id)}/stream?${APP}`,
  };
}

async function audius(path: string): Promise<Track[]> {
  const r = await fetch(`${AUDIUS}${path}${path.includes("?") ? "&" : "?"}${APP}`);
  if (!r.ok) throw new Error(`Audius ${r.status}`);
  const { data } = await r.json();
  return (data ?? []).filter((t: any) => t.is_streamable !== false).map(audiusTrack);
}

export const audiusSearch = (q: string) => audius(`/tracks/search?query=${encodeURIComponent(q)}&limit=20`);
export const audiusTrending = () => audius("/tracks/trending?limit=50");

let localCache: Promise<Track[]> | null = null;

/** Cançons de la carpeta Música ("Artista - Títol.mp3"; la carpeta fa d'àlbum). */
export function localTracks(fresh = false): Promise<Track[]> {
  if (!localCache || fresh)
    localCache = invoke<{ path: string; title: string; artist: string; folder: string }[]>("local_tracks").then((files) =>
      files.map((f) => ({
        id: `local-${f.path}`,
        uri: `local:${f.path}`,
        name: f.title,
        artists: [{ id: "", name: f.artist || "Desconegut" }],
        album: { id: "", name: f.folder || "Fitxers locals" },
        duration_ms: 0, // l'<audio> la sap quan carrega la pista
        explicit: false,
        src: convertFileSrc(f.path),
      })),
    );
  return localCache;
}
