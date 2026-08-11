// Screenshot smoke check for the Velumin demos (WI-SMOKE-0001).
//
// Drives a scripted headless Chromium over the demo harness in freeze-frame
// mode (`?frame&t=<ms>`), captures the WebGPU canvas at deterministic tester
// frames and at 4:3 / wide / tall viewport sizes, and asserts structural
// properties of the rendered pixels (not all-black, not all-white, bright
// vector geometry present, pre-impact vs post-impact frames differ, and that
// default 4:3 scenes keep non-4:3 letterbox/pillarbox margins dark).
//
// It also compares each capture against a committed reference signature
// (a normalized coarse luminance grid) with a generous tolerance, to catch
// geometry that moves, disappears, or is recolored. It does NOT require
// pixel-perfect equality (adapter/driver variation is expected; see
// WI-SMOKE-0001 non-goals). The reference signatures live under
// smoke/reference/*.grid.json; the *.png files there are for human comparison.
//
// WebGPU requirement: this check needs a WebGPU-capable Chromium. When no
// WebGPU adapter is available (e.g. GitHub Actions runners without a GPU, or
// a headless-shell build), the check prints a SKIP notice and exits 0 rather
// than failing — it is a best-effort guard, not a hard CI gate.

import { chromium } from "playwright";
import { PNG } from "pngjs";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const BASE = (process.env.SMOKE_URL || "http://127.0.0.1:5173").replace(/\/$/, "");
const HERE = dirname(fileURLToPath(import.meta.url));
const OUT = resolve(HERE, "smoke-out");
const REF = resolve(HERE, "smoke", "reference");
const READY_TIMEOUT_MS = 15000;

// Spatial reference comparison: a coarse luminance grid, normalized per-frame
// to its brightest cell so adapter/driver brightness differences do not matter,
// compared against a committed reference with a generous tolerance. This catches
// geometry that moves, disappears, or is recolored/replaced (which aggregate
// luminance stats miss) without requiring pixel-perfect equality across GPUs
// (WI-SMOKE-0001 non-goals). Set SMOKE_UPDATE_REFS=1 to (re)write references.
const GRID_COLS = 16;
const GRID_ROWS = 12;
const REF_TOLERANCE = 0.16;
const UPDATE_REFS = /^(1|true|yes)$/i.test(process.env.SMOKE_UPDATE_REFS || "");

const LAUNCH_ARGS = [
  "--enable-unsafe-webgpu",
  "--enable-features=Vulkan,WebGPU",
  "--use-angle=metal",
  "--ignore-gpu-blocklist",
];

// Each check: a demo route in freeze-frame mode at a fixed viewport.
const CHECKS = [
  { name: "baseline-4x3", url: `${BASE}/?frame`, viewport: { width: 1024, height: 768 }, aspect: "4:3" },
  { name: "blasterites-4x3-pre", url: `${BASE}/?demo=blasterites&frame&t=2000`, viewport: { width: 1024, height: 768 }, aspect: "4:3" },
  { name: "blasterites-4x3-post", url: `${BASE}/?demo=blasterites&frame&t=4000`, viewport: { width: 1024, height: 768 }, aspect: "4:3" },
  { name: "blasterites-wide", url: `${BASE}/?demo=blasterites&frame&t=2000`, viewport: { width: 1280, height: 480 }, aspect: "wide" },
  { name: "blasterites-tall", url: `${BASE}/?demo=blasterites&frame&t=2000`, viewport: { width: 600, height: 900 }, aspect: "tall" },
  { name: "frame-api-4x3", url: `${BASE}/?demo=frame-api&frame&t=2000`, viewport: { width: 1024, height: 768 }, aspect: "4:3" },
  { name: "frame-api-wide-pixels", url: `${BASE}/?demo=frame-api&frame&t=2000`, viewport: { width: 1280, height: 480 }, aspect: "full-canvas" },
  { name: "tuner-4x3", url: `${BASE}/?demo=tuner&frame&t=2000`, viewport: { width: 1024, height: 768 }, aspect: "4:3" },
  // Non-default display presets (DP-0007 / WI-PRESET-0001), captured on the
  // deterministic pre-impact tester frame so each advertised look has evidence.
  { name: "preset-monochrome-beam", url: `${BASE}/?demo=blasterites&frame&t=2000&preset=monochrome-beam`, viewport: { width: 1024, height: 768 }, aspect: "4:3" },
  { name: "preset-color-quadra-scan", url: `${BASE}/?demo=blasterites&frame&t=2000&preset=color-quadra-scan`, viewport: { width: 1024, height: 768 }, aspect: "4:3" },
  { name: "preset-clean-neon", url: `${BASE}/?demo=blasterites&frame&t=2000&preset=clean-neon`, viewport: { width: 1024, height: 768 }, aspect: "4:3" },
];

// Computes luminance stats over a decoded RGBA screenshot, plus the darkness
// of the margins outside a centered 4:3 region (to check letterbox/pillarbox
// behavior). Stats are computed from the Playwright compositor screenshot
// rather than an in-page canvas readback: a 2D drawImage/getImageData of a
// WebGPU canvas returns black in headless Chromium, whereas the compositor
// screenshot captures the rendered frame reliably.
function bufferStats(png) {
  const { width: w, height: h, data: px } = png;

  let maxLum = 0;
  let sumLum = 0;
  let bright = 0;
  let litR = 0;
  let litG = 0;
  let litB = 0;
  for (let i = 0; i < px.length; i += 4) {
    const r = px[i];
    const g = px[i + 1];
    const b = px[i + 2];
    const lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    sumLum += lum;
    if (lum > maxLum) maxLum = lum;
    if (lum > 40) {
      bright += 1;
      litR += r;
      litG += g;
      litB += b;
    }
  }
  const total = w * h;
  // Mean colour of the lit (glow + core) pixels — sensitive to the per-preset
  // glow intensity and hue that the normalized spatial grid cannot see.
  const litColor = bright ? [litR / bright, litG / bright, litB / bright] : [0, 0, 0];

  // Centered 4:3 target region; anything outside is expected to be black.
  const target = 4 / 3;
  let rx = 0;
  let ry = 0;
  let rw = w;
  let rh = h;
  if (w / h > target) {
    rw = Math.round(h * target);
    rx = Math.round((w - rw) / 2);
  } else {
    rh = Math.round(w / target);
    ry = Math.round((h - rh) / 2);
  }
  let marginBright = 0;
  let marginPixels = 0;
  for (let y = 0; y < h; y += 2) {
    for (let x = 0; x < w; x += 2) {
      const inside = x >= rx && x < rx + rw && y >= ry && y < ry + rh;
      if (inside) continue;
      const idx = (y * w + x) * 4;
      const lum = 0.2126 * px[idx] + 0.7152 * px[idx + 1] + 0.0722 * px[idx + 2];
      marginPixels += 1;
      if (lum > 24) marginBright += 1;
    }
  }
  return {
    w,
    h,
    maxLum,
    avgLum: sumLum / total,
    brightFraction: bright / total,
    litColor,
    marginBrightFraction: marginPixels ? marginBright / marginPixels : 0,
  };
}

// A per-preset "look" distance between two captures, combining relative
// lit-area (glow spread/intensity) and mean lit colour. Both frames come from
// the same run/adapter, so this isolates the effect of the display preset. A
// value near 0 means the two frames look the same (e.g. a broken preset
// selector rendering the default everywhere).
function lookDistance(a, b) {
  const denom = Math.max(a.brightFraction, b.brightFraction, 1e-6);
  const brightnessDelta = Math.abs(a.brightFraction - b.brightFraction) / denom;
  const colorDelta =
    (Math.abs(a.litColor[0] - b.litColor[0]) +
      Math.abs(a.litColor[1] - b.litColor[1]) +
      Math.abs(a.litColor[2] - b.litColor[2])) /
    (3 * 255);
  return brightnessDelta + colorDelta;
}

// A coarse GRID_COLS x GRID_ROWS grid of average luminance, normalized to the
// brightest cell (so it encodes spatial distribution, not absolute brightness).
function gridSignature(png) {
  const { width: w, height: h, data: px } = png;
  const sum = new Array(GRID_COLS * GRID_ROWS).fill(0);
  const count = new Array(GRID_COLS * GRID_ROWS).fill(0);
  for (let y = 0; y < h; y++) {
    const gy = Math.min(GRID_ROWS - 1, Math.floor((y / h) * GRID_ROWS));
    for (let x = 0; x < w; x++) {
      const gx = Math.min(GRID_COLS - 1, Math.floor((x / w) * GRID_COLS));
      const i = (y * w + x) * 4;
      const lum = 0.2126 * px[i] + 0.7152 * px[i + 1] + 0.0722 * px[i + 2];
      const gi = gy * GRID_COLS + gx;
      sum[gi] += lum;
      count[gi] += 1;
    }
  }
  const grid = sum.map((s, i) => (count[i] ? s / count[i] : 0));
  const max = Math.max(1e-6, ...grid);
  return grid.map((v) => v / max);
}

// Compares a frame's grid signature against its committed reference (or writes
// the reference when SMOKE_UPDATE_REFS is set). Returns { failure } on a
// tolerance breach, { note } when there is nothing to compare against, or
// { mad } with the match distance.
async function referenceCheck(name, grid) {
  const file = resolve(REF, `${name}.grid.json`);
  if (UPDATE_REFS) {
    await mkdir(REF, { recursive: true });
    const payload = { cols: GRID_COLS, rows: GRID_ROWS, grid: grid.map((v) => Math.round(v * 1000) / 1000) };
    await writeFile(file, `${JSON.stringify(payload)}\n`);
    return { note: "updated reference" };
  }
  let ref;
  try {
    ref = JSON.parse(await readFile(file, "utf8"));
  } catch {
    return { note: "no committed reference; spatial comparison skipped" };
  }
  if (!Array.isArray(ref.grid) || ref.grid.length !== grid.length) {
    return { failure: "reference grid shape mismatch (re-baseline with SMOKE_UPDATE_REFS=1)" };
  }
  let acc = 0;
  for (let i = 0; i < grid.length; i++) acc += Math.abs(grid[i] - ref.grid[i]);
  const mad = acc / grid.length;
  if (mad > REF_TOLERANCE) {
    return { failure: `differs from committed reference: grid MAD ${mad.toFixed(3)} > ${REF_TOLERANCE}` };
  }
  return { mad };
}

async function detectWebGpu(browser) {
  const page = await browser.newPage();
  try {
    await page.goto(`${BASE}/?frame`, { waitUntil: "domcontentloaded" });
    return await page.evaluate(async () => {
      if (!navigator.gpu) return false;
      try {
        return !!(await navigator.gpu.requestAdapter());
      } catch {
        return false;
      }
    });
  } finally {
    await page.close();
  }
}

function assertCheck(check, stats) {
  const failures = [];
  if (stats.w < 2 || stats.h < 2) failures.push(`canvas too small (${stats.w}x${stats.h})`);
  if (stats.maxLum < 60) failures.push(`all-black: maxLum ${stats.maxLum.toFixed(1)} < 60`);
  if (stats.avgLum > 90) failures.push(`too bright / washed out: avgLum ${stats.avgLum.toFixed(1)} > 90`);
  if (stats.brightFraction < 0.0002) failures.push(`no geometry: brightFraction ${stats.brightFraction.toExponential(2)}`);
  if (stats.brightFraction > 0.5) failures.push(`geometry fills screen: brightFraction ${stats.brightFraction.toFixed(3)}`);
  if (check.aspect !== "4:3" && check.aspect !== "full-canvas" && stats.marginBrightFraction > 0.02) {
    failures.push(`letterbox margins not dark: marginBrightFraction ${stats.marginBrightFraction.toFixed(3)}`);
  }
  return failures;
}

async function runCheck(browser, check) {
  const page = await browser.newPage({ viewport: check.viewport });
  try {
    await page.goto(check.url, { waitUntil: "domcontentloaded" });
    await page.waitForFunction(
      () => window.__smokeReady === true || typeof window.__smokeError === "string",
      { timeout: READY_TIMEOUT_MS },
    );
    const err = await page.evaluate(() => window.__smokeError || null);
    if (err) return { failures: [`render error: ${err}`], stats: null };
    await mkdir(OUT, { recursive: true });
    const shot = await page.locator("#canvas").screenshot();
    await writeFile(resolve(OUT, `${check.name}.png`), shot);
    const png = PNG.sync.read(shot);
    const stats = bufferStats(png);
    const failures = assertCheck(check, stats);
    const ref = await referenceCheck(check.name, gridSignature(png));
    if (ref.failure) failures.push(ref.failure);
    return { failures, stats, note: ref.note, mad: ref.mad };
  } finally {
    await page.close();
  }
}

async function main() {
  const browser = await chromium.launch({ headless: true, args: LAUNCH_ARGS });
  try {
    const hasWebGpu = await detectWebGpu(browser);
    if (!hasWebGpu) {
      console.log(
        "SKIP: no WebGPU adapter available in this browser environment; " +
          "skipping screenshot smoke check. It requires a WebGPU-capable Chromium " +
          "(a developer machine with a GPU/display, or a GPU-enabled runner).",
      );
      return 0;
    }

    const results = [];
    for (const check of CHECKS) {
      const { failures, stats, note, mad } = await runCheck(browser, check);
      results.push({ check, failures, stats, note, mad });
    }

    // Cross-frame: the deterministic pre- and post-impact frames must differ.
    const pre = results.find((r) => r.check.name === "blasterites-4x3-pre");
    const post = results.find((r) => r.check.name === "blasterites-4x3-post");
    if (pre?.stats && post?.stats) {
      const delta = Math.abs(pre.stats.brightFraction - post.stats.brightFraction);
      if (delta < 1e-4) {
        pre.failures.push(
          `frame did not change across impact: brightFraction delta ${delta.toExponential(2)} (static render?)`,
        );
      }
    }

    // Preset distinctness: the ArcadeBalanced default frame and each non-default
    // preset frame (same scene, same t) must render a visibly different look.
    // This is what actually validates preset SELECTION — the per-frame grid
    // reference cannot, since the presets share geometry and it normalizes away
    // brightness. If the selector regressed so every route renders one preset,
    // these frames would collapse together and fail here.
    const PRESET_LOOK_MIN = 0.08;
    const presetFrames = [
      { label: "arcade-balanced (default)", result: pre },
      ...results.filter((r) => r.check.name.startsWith("preset-")).map((r) => ({ label: r.check.name, result: r })),
    ].filter((f) => f.result?.stats);
    for (let a = 0; a < presetFrames.length; a++) {
      for (let b = a + 1; b < presetFrames.length; b++) {
        const dist = lookDistance(presetFrames[a].result.stats, presetFrames[b].result.stats);
        if (dist < PRESET_LOOK_MIN) {
          presetFrames[b].result.failures.push(
            `preset look not distinct from ${presetFrames[a].label}: distance ${dist.toFixed(3)} < ${PRESET_LOOK_MIN} (preset selection may have regressed)`,
          );
        }
      }
    }

    let failed = 0;
    for (const { check, failures, stats, note, mad } of results) {
      if (failures.length) {
        failed += 1;
        console.log(`FAIL ${check.name}: ${failures.join("; ")}`);
      } else {
        const parts = [];
        if (stats) parts.push(`maxLum ${stats.maxLum.toFixed(0)}, bright ${(stats.brightFraction * 100).toFixed(2)}%`);
        if (typeof mad === "number") parts.push(`ref MAD ${mad.toFixed(3)}`);
        if (note) parts.push(note);
        console.log(`ok   ${check.name}${parts.length ? ` (${parts.join("; ")})` : ""}`);
      }
    }
    const mode = UPDATE_REFS ? " [reference update]" : "";
    console.log(`\nScreenshot smoke${mode}: ${results.length - failed}/${results.length} checks passed; captures in ${OUT}`);
    return failed ? 1 : 0;
  } finally {
    await browser.close();
  }
}

main()
  .then((code) => process.exit(code))
  .catch((err) => {
    console.error("smoke check crashed:", err);
    process.exit(2);
  });
