/**
 * Cerca tolerant per filtrar la biblioteca (queixa d'Apple Music: "needs the exact spelling").
 * Sense accents ni majúscules, i cada paraula admet una lletra equivocada (dues si és llarga).
 */

export const norm = (s: string) =>
  s
    .normalize("NFD")
    .replace(/\p{Diacritic}/gu, "")
    .toLowerCase();

/** Distància d'edició (dues lletres girades compten com un error) amb tall: max+1 si la supera. */
function dist(a: string, b: string, max: number): number {
  if (Math.abs(a.length - b.length) > max) return max + 1;
  let prev2: number[] = [];
  let prev = Array.from({ length: b.length + 1 }, (_, j) => j);
  for (let i = 1; i <= a.length; i++) {
    const cur = [i];
    let best = i;
    for (let j = 1; j <= b.length; j++) {
      cur[j] = Math.min(prev[j] + 1, cur[j - 1] + 1, prev[j - 1] + (a[i - 1] === b[j - 1] ? 0 : 1));
      if (i > 1 && j > 1 && a[i - 1] === b[j - 2] && a[i - 2] === b[j - 1]) cur[j] = Math.min(cur[j], prev2[j - 2] + 1);
      best = Math.min(best, cur[j]);
    }
    if (best > max) return max + 1;
    prev2 = prev;
    prev = cur;
  }
  return prev[b.length];
}

/** Totes les paraules de `query` hi són (o gairebé: un error, o dos si tenen 8+ lletres). */
export function matches(query: string, text: string): boolean {
  const q = norm(query).split(/\s+/).filter(Boolean);
  if (!q.length) return true;
  const t = norm(text);
  const words = t.split(/[^\p{L}\p{N}]+/u).filter(Boolean);
  return q.every((tok) => {
    if (t.includes(tok)) return true;
    if (tok.length < 4) return false;
    const max = tok.length >= 8 ? 2 : 1;
    // Contra la paraula sencera o el seu principi (es pot estar escrivint encara).
    return words.some((w) => dist(tok, w, max) <= max || dist(tok, w.slice(0, tok.length), max) <= max);
  });
}
