import { settings } from "./settings.svelte";

/**
 * Sense animacions decoratives: moviment reduït del sistema o mode lleuger. Per a les transicions
 * de Svelte (JS), que no veuen els tokens CSS; dins d'un $derived/plantilla segueix `settings.light`.
 */
export function calm() {
  return settings.light || (typeof matchMedia !== "undefined" && matchMedia("(prefers-reduced-motion: reduce)").matches);
}
