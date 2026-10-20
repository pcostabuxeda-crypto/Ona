/**
 * El moment de la cua: la portada de la fila vola fins al botó de la cua (o al ⋯ en pantalla estreta)
 * i el botó fa un petit cop. Només transform/opacity; sense moviment si l'usuari el té reduït.
 */
export function flyToQueue(from: Element | null | undefined) {
  const target = [...document.querySelectorAll<HTMLElement>("[data-queue-btn]")].find((el) => el.offsetParent);
  if (!target) return;
  const spring = getComputedStyle(document.documentElement).getPropertyValue("--spring").trim() || "ease-out";
  const bump = () =>
    target.animate([{ transform: "scale(1)" }, { transform: "scale(1.18)" }, { transform: "scale(1)" }], { duration: 520, easing: spring });
  if (!from || matchMedia("(prefers-reduced-motion: reduce)").matches) return bump();

  const a = from.getBoundingClientRect();
  const b = target.getBoundingClientRect();
  const ghost = from.cloneNode(true) as HTMLElement;
  Object.assign(ghost.style, {
    position: "fixed",
    left: `${a.left}px`,
    top: `${a.top}px`,
    width: `${a.width}px`,
    height: `${a.height}px`,
    margin: "0",
    zIndex: "300",
    pointerEvents: "none",
    borderRadius: "4px",
    overflow: "hidden",
  });
  document.body.append(ghost);
  const dx = b.left + b.width / 2 - (a.left + a.width / 2);
  const dy = b.top + b.height / 2 - (a.top + a.height / 2);
  const s = 20 / Math.max(1, a.width);
  ghost
    .animate(
      [
        { transform: "translate(0, 0) scale(1)", opacity: 1 },
        // Fa un arc cap amunt abans de baixar al botó.
        { transform: `translate(${dx * 0.45}px, ${dy * 0.45 - 70}px) scale(${(1 + s) / 1.6})`, opacity: 1, offset: 0.45 },
        { transform: `translate(${dx}px, ${dy}px) scale(${s})`, opacity: 0.4 },
      ],
      { duration: 520, easing: "cubic-bezier(.3,0,.2,1)" },
    )
    .finished.then(() => {
      ghost.remove();
      bump();
    }, () => ghost.remove());
}
