import type { Collection } from "./api";

/** Col·leccions sense portada pròpia: degradat i icona, com "Cançons que m'agraden" al client. */
export const TILES: Partial<Record<Collection["kind"], { icon: string; bg: string; tint: string }>> = {
  liked: { icon: "heart", bg: "linear-gradient(135deg, #450af5, #c4efd9)", tint: "#5038a0" },
  local: { icon: "note", bg: "linear-gradient(135deg, #1e3264, #1db954)", tint: "#1e5a5c" },
  audius: { icon: "album", bg: "linear-gradient(135deg, #7e1bcc, #cc0fe0)", tint: "#6a1bb0" },
};
