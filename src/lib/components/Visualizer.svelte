<script lang="ts">
  /** Dibuixa l'espectre en un canvas: barres o ona, en línia (a baix) o en cercle (al voltant de la portada). */
  import { onMount } from "svelte";
  import { BANDS, bands, onFrame, useSpectrum } from "$lib/viz.svelte";

  type VizStyle = "line" | "circular";
  type VizShape = "bars" | "wave";

  let {
    style,
    shape,
    colorA,
    colorB,
    inner = 0.33,
  }: { style: VizStyle; shape: VizShape; colorA: string; colorB: string; inner?: number } = $props();

  let canvas: HTMLCanvasElement;
  let w = 0,
    h = 0,
    dpr = 1;
  let idle = false;

  onMount(() => {
    // Rust només calcula l'espectre mentre algun visualitzador és a la pantalla.
    const leave = useSpectrum();
    const ctx = canvas.getContext("2d")!;
    const ro = new ResizeObserver(() => {
      dpr = Math.min(2, devicePixelRatio || 1);
      w = canvas.clientWidth;
      h = canvas.clientHeight;
      canvas.width = Math.round(w * dpr);
      canvas.height = Math.round(h * dpr);
    });
    ro.observe(canvas);

    const stop = onFrame(() => {
      let energy = 0;
      for (let i = 0; i < BANDS; i++) energy += bands[i];
      // En silenci no es redibuixa res (estalvia GPU).
      if (energy < 0.02) {
        if (!idle) ctx.clearRect(0, 0, canvas.width, canvas.height);
        idle = true;
        return;
      }
      idle = false;
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      ctx.clearRect(0, 0, w, h);
      if (style === "circular") drawCircle(ctx);
      else drawLine(ctx);
    });
    return () => {
      stop();
      ro.disconnect();
      leave();
    };
  });

  function gradient(ctx: CanvasRenderingContext2D, x0: number, y0: number, x1: number, y1: number) {
    const g = ctx.createLinearGradient(x0, y0, x1, y1);
    g.addColorStop(0, colorA);
    g.addColorStop(1, colorB);
    return g;
  }

  /** Barres o ona al llarg de la part de baix, simètriques des del centre. */
  function drawLine(ctx: CanvasRenderingContext2D) {
    const n = BANDS;
    const max = h * 0.9;
    const cx = w / 2;
    const step = cx / n;
    ctx.fillStyle = gradient(ctx, 0, 0, w, 0);
    if (shape === "bars") {
      const bw = Math.max(2, step * 0.55);
      ctx.globalAlpha = 0.75;
      for (let i = 0; i < n; i++) {
        const bh = Math.max(2, bands[i] * max);
        for (const x of [cx + i * step, cx - (i + 1) * step]) {
          ctx.beginPath();
          ctx.roundRect(x + (step - bw) / 2, h - bh, bw, bh, bw / 2);
          ctx.fill();
        }
      }
    } else {
      ctx.globalAlpha = 0.55;
      ctx.beginPath();
      ctx.moveTo(0, h);
      const pts: [number, number][] = [];
      for (let i = n - 1; i >= 0; i--) pts.push([cx - (i + 0.5) * step, h - bands[i] * max]);
      for (let i = 0; i < n; i++) pts.push([cx + (i + 0.5) * step, h - bands[i] * max]);
      smooth(ctx, pts);
      ctx.lineTo(w, h);
      ctx.closePath();
      const fill = ctx.createLinearGradient(0, h - max, 0, h);
      fill.addColorStop(0, colorB);
      fill.addColorStop(1, colorA);
      ctx.fillStyle = fill;
      ctx.fill();
    }
    ctx.globalAlpha = 1;
  }

  /** Al voltant d'un cercle (la portada queda al mig). Simètric esquerra/dreta. */
  function drawCircle(ctx: CanvasRenderingContext2D) {
    const cx = w / 2,
      cy = h / 2;
    const r0 = Math.min(w, h) * inner;
    const len = Math.min(w, h) * (0.5 - inner) * 0.95;
    const n = BANDS;
    const total = n * 2;
    if (shape === "bars") {
      ctx.strokeStyle = gradient(ctx, cx - r0, cy - r0, cx + r0, cy + r0);
      ctx.lineCap = "round";
      ctx.lineWidth = Math.max(2, ((Math.PI * 2 * r0) / total) * 0.5);
      ctx.globalAlpha = 0.85;
      ctx.beginPath();
      for (let k = 0; k < total; k++) {
        const i = k < n ? k : total - 1 - k;
        const a = (k / total) * Math.PI * 2 - Math.PI / 2;
        const l = 3 + bands[i] * len;
        ctx.moveTo(cx + Math.cos(a) * (r0 + 4), cy + Math.sin(a) * (r0 + 4));
        ctx.lineTo(cx + Math.cos(a) * (r0 + 4 + l), cy + Math.sin(a) * (r0 + 4 + l));
      }
      ctx.stroke();
    } else {
      const pts: [number, number][] = [];
      for (let k = 0; k < total; k++) {
        const i = k < n ? k : total - 1 - k;
        const a = (k / total) * Math.PI * 2 - Math.PI / 2;
        const r = r0 + 6 + bands[i] * len;
        pts.push([cx + Math.cos(a) * r, cy + Math.sin(a) * r]);
      }
      ctx.globalAlpha = 0.5;
      ctx.beginPath();
      smooth(ctx, [...pts, pts[0], pts[1]], true);
      ctx.closePath();
      ctx.fillStyle = gradient(ctx, cx - r0, cy - r0, cx + r0, cy + r0);
      ctx.fill();
    }
    ctx.globalAlpha = 1;
  }

  /** Corba suau que passa pels punts (punts mitjos + quadràtiques). */
  function smooth(ctx: CanvasRenderingContext2D, pts: [number, number][], move = false) {
    const mid = (a: [number, number], b: [number, number]) => [(a[0] + b[0]) / 2, (a[1] + b[1]) / 2];
    const [sx, sy] = mid(pts[0], pts[1]);
    if (move) ctx.moveTo(sx, sy);
    else ctx.lineTo(pts[0][0], pts[0][1]);
    for (let i = 1; i < pts.length - 1; i++) {
      const [mx, my] = mid(pts[i], pts[i + 1]);
      ctx.quadraticCurveTo(pts[i][0], pts[i][1], mx, my);
    }
  }
</script>

<canvas bind:this={canvas} class="viz {style}"></canvas>

<style>
  canvas {
    display: block;
    width: 100%;
    height: 100%;
    pointer-events: none;
  }
</style>
