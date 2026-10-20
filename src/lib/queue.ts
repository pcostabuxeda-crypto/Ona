/**
 * Lògica pura de la cua (sense Svelte), per poder-la provar amb `node --test`.
 * `order` són índexs del context en ordre de reproducció; `pos` és el que sona.
 */

/** Barreja només el que encara ha de sonar: el que ja ha sonat i el que s'ha tret no tornen. */
export function shuffleRest(order: number[], pos: number, rnd: () => number = Math.random): number[] {
  const rest = order.slice(pos + 1);
  for (let i = rest.length - 1; i > 0; i--) {
    const j = Math.floor(rnd() * (i + 1));
    [rest[i], rest[j]] = [rest[j], rest[i]];
  }
  return [...order.slice(0, pos + 1), ...rest];
}

/** Treu l'aleatori del que queda: torna a l'ordre del context, sense recuperar el que s'ha tret. */
export function unshuffleRest(order: number[], pos: number): number[] {
  return [...order.slice(0, pos + 1), ...order.slice(pos + 1).sort((a, b) => a - b)];
}

/**
 * Què sona després: primer la cua de l'usuari (en ordre d'arribada), després el context.
 * Retorna "user", la nova posició del context, o -1 si s'ha acabat.
 */
export function nextStep(userCount: number, pos: number, length: number, wrap: boolean): "user" | number {
  if (userCount > 0) return "user";
  if (pos + 1 < length) return pos + 1;
  return wrap && length > 0 ? 0 : -1;
}
