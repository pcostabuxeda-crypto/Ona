/** Treu una paleta (3 colors) d'una portada, reduint-la a 24x24 en un canvas. */
export type Palette = { a: string; b: string; c: string };

export const DEFAULT_PALETTE: Palette = { a: "#5b3cc4", b: "#c43c8a", c: "#1d6fa3" };

const memo = new Map<string, Palette>();

export async function paletteFrom(url?: string): Promise<Palette> {
  if (!url) return DEFAULT_PALETTE;
  const hit = memo.get(url);
  if (hit) return hit;
  try {
    const img = new Image();
    img.crossOrigin = "anonymous";
    img.src = url;
    await img.decode();
    const size = 24;
    const canvas = new OffscreenCanvas(size, size);
    const ctx = canvas.getContext("2d", { willReadFrequently: true })!;
    ctx.drawImage(img, 0, 0, size, size);
    const { data } = ctx.getImageData(0, 0, size, size);

    // Agrupa per tonalitat (12 cubetes) i puntua per saturació, per evitar grisos.
    const buckets = Array.from({ length: 12 }, () => ({ r: 0, g: 0, b: 0, w: 0 }));
    for (let i = 0; i < data.length; i += 4) {
      const [r, g, b] = [data[i], data[i + 1], data[i + 2]];
      const [h, s, l] = hsl(r, g, b);
      if (l < 0.08 || l > 0.95) continue;
      const w = 0.15 + s * s;
      const k = Math.floor(h * 12) % 12;
      buckets[k].r += r * w;
      buckets[k].g += g * w;
      buckets[k].b += b * w;
      buckets[k].w += w;
    }
    const top = buckets
      .filter((x) => x.w > 0)
      .sort((x, y) => y.w - x.w)
      .slice(0, 3)
      .map((x) => tune(x.r / x.w, x.g / x.w, x.b / x.w));
    while (top.length < 3) top.push(top[0] ?? DEFAULT_PALETTE.a);
    const p = { a: top[0], b: top[1], c: top[2] };
    memo.set(url, p);
    return p;
  } catch {
    return DEFAULT_PALETTE;
  }
}

/** Ajusta el color perquè llueixi sobre fons fosc: saturació mínima i lluminositat mitjana. */
function tune(r: number, g: number, b: number) {
  const [h, s, l] = hsl(r, g, b);
  const s2 = Math.min(1, Math.max(0.45, s * 1.15));
  const l2 = Math.min(0.6, Math.max(0.38, l));
  return `hsl(${Math.round(h * 360)} ${Math.round(s2 * 100)}% ${Math.round(l2 * 100)}%)`;
}

function hsl(r: number, g: number, b: number): [number, number, number] {
  r /= 255;
  g /= 255;
  b /= 255;
  const max = Math.max(r, g, b);
  const min = Math.min(r, g, b);
  const l = (max + min) / 2;
  if (max === min) return [0, 0, l];
  const d = max - min;
  const s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
  let h = max === r ? (g - b) / d + (g < b ? 6 : 0) : max === g ? (b - r) / d + 2 : (r - g) / d + 4;
  return [h / 6, s, l];
}
