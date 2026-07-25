// Screenshot smoke check for the Velumin demos (WI-SMOKE-0001).
//
// Drives a scripted headless Chromium over the demo harness in freeze-frame
// mode (`?frame&t=<ms>`), captures the WebGPU canvas at deterministic tester
// frames and at 4:3 / wide / tall viewport sizes, and asserts structural
// properties of the rendered pixels (not all-black, not all-white, bright
// vector geometry present, pre-impact vs post-impact frames differ, and the
// non-4:3 letterbox/pillarbox margins are dark).
//
// It does NOT require pixel-perfect equality with committed reference frames
// (adapter/driver variation is expected; see WI-SMOKE-0001 non-goals). Committed
// reference frames under smoke/reference/ are for human comparison only.
//
// WebGPU requirement: this check needs a WebGPU-capable Chromium. When no
// WebGPU adapter is available (e.g. GitHub Actions runners without a GPU, or
// a headless-shell build), the check prints a SKIP notice and exits 0 rather
// than failing — it is a best-effort guard, not a hard CI gate.

import { chromium } from "playwright";
import { PNG } from "pngjs";
import { mkdir, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const BASE = (process.env.SMOKE_URL || "http://127.0.0.1:5173").replace(/\/$/, "");
const HERE = dirname(fileURLToPath(import.meta.url));
const OUT = resolve(HERE, "smoke-out");
const READY_TIMEOUT_MS = 15000;

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
  { name: "tuner-4x3", url: `${BASE}/?demo=tuner&frame&t=2000`, viewport: { width: 1024, height: 768 }, aspect: "4:3" },
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
  for (let i = 0; i < px.length; i += 4) {
    const lum = 0.2126 * px[i] + 0.7152 * px[i + 1] + 0.0722 * px[i + 2];
    sumLum += lum;
    if (lum > maxLum) maxLum = lum;
    if (lum > 40) bright += 1;
  }
  const total = w * h;

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
    marginBrightFraction: marginPixels ? marginBright / marginPixels : 0,
  };
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
  if (check.aspect !== "4:3" && stats.marginBrightFraction > 0.02) {
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
    const stats = bufferStats(PNG.sync.read(shot));
    return { failures: assertCheck(check, stats), stats };
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
      const { failures, stats } = await runCheck(browser, check);
      results.push({ check, failures, stats });
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

    let failed = 0;
    for (const { check, failures, stats } of results) {
      if (failures.length) {
        failed += 1;
        console.log(`FAIL ${check.name}: ${failures.join("; ")}`);
      } else {
        const s = stats
          ? ` (maxLum ${stats.maxLum.toFixed(0)}, bright ${(stats.brightFraction * 100).toFixed(2)}%)`
          : "";
        console.log(`ok   ${check.name}${s}`);
      }
    }
    console.log(`\nScreenshot smoke: ${results.length - failed}/${results.length} checks passed; captures in ${OUT}`);
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
