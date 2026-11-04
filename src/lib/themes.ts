/** Temes de fons de l'estètica Ona (de l'Ona 0.1). `accent: null` vol dir que l'accent surt de la portada. */
import type { Effect, ThemeId } from "./settings.svelte";

export type Theme = {
  id: ThemeId;
  name: string;
  /** Fons CSS (degradats). Els temes "cover" i "custom" el posen amb una imatge. */
  background: string;
  accent: string | null;
  /** Efecte que se suggereix en triar el tema. */
  effect: Effect;
};

export const THEMES: Theme[] = [
  {
    id: "ona",
    name: "Ona",
    background: "radial-gradient(1200px 600px at 100% 0%, color-mix(in oklab, var(--pb) 10%, transparent), transparent), #09090f",
    accent: null,
    effect: "none",
  },
  {
    id: "galaxy",
    name: "Galàxia",
    background:
      "radial-gradient(90% 70% at 85% 100%, #e52fd6 0%, transparent 60%), radial-gradient(70% 60% at 10% 0%, #2b2fd9 0%, transparent 70%), linear-gradient(160deg, #1a1a8a 0%, #4a1fc4 45%, #b521d4 100%)",
    accent: "#ff6be6",
    effect: "stars",
  },
  {
    id: "aurora",
    name: "Aurora",
    background:
      "radial-gradient(60% 50% at 20% 10%, #12c79a 0%, transparent 70%), radial-gradient(50% 45% at 80% 20%, #3b7bff 0%, transparent 70%), radial-gradient(70% 60% at 60% 100%, #6a2bd1 0%, transparent 70%), #04121a",
    accent: "#6ef7c8",
    effect: "stars",
  },
  {
    id: "sunset",
    name: "Posta de sol",
    background:
      "radial-gradient(80% 60% at 50% 110%, #ffb15e 0%, transparent 60%), linear-gradient(175deg, #1d0b33 0%, #6b1f5e 45%, #d9485f 80%, #f58b4c 100%)",
    accent: "#ffc07a",
    effect: "none",
  },
  {
    id: "ocean",
    name: "Oceà",
    background:
      "radial-gradient(70% 50% at 50% 0%, #1fa6d6 0%, transparent 70%), linear-gradient(180deg, #03243b 0%, #04476e 55%, #021725 100%)",
    accent: "#6fe0ff",
    effect: "bubbles",
  },
  { id: "cover", name: "Portada", background: "#09090f", accent: null, effect: "none" },
];

export const themeById = (id: string) => THEMES.find((t) => t.id === id) ?? THEMES[0];

const IMAGE_KEY = "ona.bgimage";

export function loadCustomImage(): string | null {
  try {
    return localStorage.getItem(IMAGE_KEY);
  } catch {
    return null;
  }
}

/** Redueix la imatge triada (màx. 1920 px, JPEG) perquè ocupi poc i es pinti ràpid. */
export async function saveCustomImage(file: File): Promise<string> {
  const url = URL.createObjectURL(file);
  try {
    const img = new Image();
    img.src = url;
    await img.decode();
    const k = Math.min(1, 1920 / Math.max(img.naturalWidth, img.naturalHeight));
    const canvas = document.createElement("canvas");
    canvas.width = Math.round(img.naturalWidth * k);
    canvas.height = Math.round(img.naturalHeight * k);
    canvas.getContext("2d")!.drawImage(img, 0, 0, canvas.width, canvas.height);
    const data = canvas.toDataURL("image/jpeg", 0.85);
    localStorage.setItem(IMAGE_KEY, data);
    return data;
  } finally {
    URL.revokeObjectURL(url);
  }
}
