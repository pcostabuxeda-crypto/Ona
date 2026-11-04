/** Lletra paraula per paraula quan només tenim el temps de cada línia. */
export type Word = { t: number; text: string };

/**
 * Reparteix la durada de la línia entre les paraules segons quantes lletres tenen (les paraules
 * llargues es canten més estona). Es canta el 85 % del temps fins a la línia següent, com a molt 8 s.
 */
export function estimateWords(text: string, t: number, nextT?: number): Word[] {
  const words = text.split(/(\s+)/).filter(Boolean);
  const total = words.reduce((n, w) => n + (w.trim() ? w.length : 0), 0);
  if (!total) return [];
  const span = Math.min((nextT ?? t + 4000) - t, 8000) * 0.85;
  const out: Word[] = [];
  let done = 0;
  for (const w of words) {
    if (!w.trim()) {
      // L'espai va enganxat a la paraula anterior: així no queda cap espai «per cantar».
      if (out.length) out[out.length - 1].text += w;
      continue;
    }
    out.push({ t: Math.round(t + (span * done) / total), text: w });
    done += w.length;
  }
  return out;
}
