/**
 * Què es pot animar ara mateix: combina les preferències, el mode estalvi i si la finestra es veu.
 * Minimitzada (o tapada del tot) no es calcula espectre ni s'anima res.
 */
import { settings } from "./settings.svelte";

class Perf {
  visible = $state(typeof document === "undefined" ? true : !document.hidden);

  viz = $derived(this.visible && !settings.eco && settings.vizEnabled);
  liveBg = $derived(this.visible && !settings.eco && settings.liveBg);
  effect = $derived(settings.eco ? "none" : settings.effect);
  wave = $derived(!settings.eco);

  constructor() {
    if (typeof document === "undefined") return;
    document.addEventListener("visibilitychange", () => {
      this.visible = !document.hidden;
      document.documentElement.classList.toggle("app-hidden", document.hidden);
    });
  }
}

export const perf = new Perf();
