import { settings } from "./settings.svelte";

/**
 * Sense animacions decoratives: moviment reduït del sistema o mode lleuger. Per a les transicions
 * de Svelte (JS), que no veuen els tokens CSS; dins d'un $derived/plantilla segueix `settings.light`.
 */
export function calm() {
  return settings.light || (typeof matchMedia !== "undefined" && matchMedia("(prefers-reduced-motion: reduce)").matches);
}

/**
 * Portada en relleu (estètica Ona): s'inclina cap al punter i torna amb molla en sortir.
 * Només escriu variables CSS (`--rx`, `--ry`, `--gx`, `--gy`): el CSS les fa servir dins `transform`
 * (compositor). Res amb punter tàctil, moviment reduït o mode lleuger. Classe `tilting` mentre segueix.
 */
export function tilt(node: HTMLElement, max = 8) {
  const fine = matchMedia("(hover: hover) and (pointer: fine)");
  let raf = 0;
  let on = false;
  const set = (rx: number, ry: number, gx: number, gy: number) => {
    node.style.setProperty("--rx", `${rx}deg`);
    node.style.setProperty("--ry", `${ry}deg`);
    node.style.setProperty("--gx", `${gx}%`);
    node.style.setProperty("--gy", `${gy}%`);
  };
  const move = (e: PointerEvent) => {
    if (!fine.matches || settings.look !== "ona" || calm()) return;
    on = true;
    node.classList.add("tilting");
    cancelAnimationFrame(raf);
    raf = requestAnimationFrame(() => {
      const r = node.getBoundingClientRect();
      const x = (e.clientX - r.left) / r.width - 0.5; // -0.5 … 0.5
      const y = (e.clientY - r.top) / r.height - 0.5;
      // El reflex (el doble de gran) es desplaça amb el punter: -25 % … 25 %.
      set(-y * max * 2, x * max * 2, x * 50, y * 50);
    });
  };
  const leave = () => {
    if (!on) return;
    on = false;
    cancelAnimationFrame(raf);
    node.classList.remove("tilting");
    set(0, 0, 0, 0);
  };
  node.addEventListener("pointermove", move);
  node.addEventListener("pointerleave", leave);
  return {
    destroy() {
      cancelAnimationFrame(raf);
      node.removeEventListener("pointermove", move);
      node.removeEventListener("pointerleave", leave);
    },
  };
}
