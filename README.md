# Velumin

Velumin is a retro vector-graphics library.

## Repository Orientation

Velumin is currently a Rust/WASM/WebGPU project with a Vite browser harness. The Rust code is a Cargo workspace of two crates: `velumin-core` (DP-0002 Phase 1: platform-neutral vector/scene/style types, no `wasm-bindgen`/`web-sys`/`wgpu` dependency) and `webgpu_vector_lib` (the `wgpu` renderer and browser adapter, depending on `velumin-core`). Within `webgpu_vector_lib`, the `Renderer` (DP-0002 Phase 2) is itself platform-neutral and host-buildable — it takes a generic `wgpu::Surface`/`Adapter` and its own error type has no `wasm-bindgen` dependency — while the `WebGPU` browser adapter (canvas lookup, resize-from-window) stays `wasm32`-gated. The repository also uses LRH-style project-control documents under `project/` to record intent, design decisions, evidence, current focus, and work items.

Important entrypoints:

- `AGENTS.md`: guidance for AI coding agents working in this repository.
- `REVIEWS.md`: protocol for addressing pull request review feedback.
- `STYLE.md`: minimal style guidance and links to canonical Rust style references.
- `.lrh/`: project-local LRH templates that adapt reusable harness prompts to Velumin's Rust/WASM/WebGPU workflow.
- `scripts/`: repository-owned local validation commands.
- `.github/workflows/`: GitHub Actions validation workflows.
- `project/`: project-control artifacts; treat this as the source of truth for roadmap, design, focus, evidence, and status.

## Browser Rendering Baseline

The current browser baseline is a Rust/WASM/Vite demo that renders a white line on a black canvas. Preserve this signal before changing renderer dependencies or architecture.

The workspace currently requires Rust 1.87 or newer. The repository pins the local toolchain to Rust 1.87.0 through `rust-toolchain.toml` and uses `wasm32-unknown-unknown` for the browser WASM build. `velumin-core` builds and tests on the host target with no `wasm32` requirement; `webgpu_vector_lib` builds for both host (its pure-logic tests, and now `Renderer` itself) and `wasm32-unknown-unknown` (the browser artifact, including the `WebGPU` browser adapter that stays wasm32-only).

## Display Presets

Velumin exposes a small public API for choosing a classic-inspired display look (DP-0007). Pick a named `VectorDisplayPreset` — `ArcadeBalanced` (default), `MonochromeBeam`, `ColorQuadraScan`, or `CleanNeon` — at renderer creation or at runtime:

```js
import init, { WebGPU, VectorDisplayPreset } from "@pkg/webgpu_vector_lib.js";
await init();

// Choose a preset up front (or use WebGPU.create for the ArcadeBalanced default):
const gpu = await WebGPU.createWithPreset("canvas", VectorDisplayPreset.MonochromeBeam);

// ...or switch at runtime; it applies on the next render call:
gpu.setDisplayPreset(VectorDisplayPreset.CleanNeon);
```

The preset **names** are a stable contract and `VectorDisplayPreset` is `#[non_exhaustive]` (presets may be added later). The numeric glow/stroke tuning behind each preset is internal and may be re-tuned. The demos expose a preset dropdown (baseline and Blasterites), and `?preset=<name>` (e.g. `?demo=blasterites&preset=clean-neon`) selects one via query parameter.

## Browser Frame API

Velumin exposes a browser-first immediate frame API (DP-0008) for JavaScript games that own their simulation and want Velumin to render vector geometry. Build a `VectorFrame`, append commands for the current frame, and submit it through the existing WebGPU renderer:

```js
import init, { VectorFrame, VectorFrameView, WebGPU, VectorDisplayPreset } from "@pkg/webgpu_vector_lib.js";
await init();

const gpu = await WebGPU.createWithPreset("canvas", VectorDisplayPreset.ArcadeBalanced);
const canvas = document.getElementById("canvas");
const canvasPixelSize = () => {
  const scale = window.devicePixelRatio || 1;
  return {
    width: Math.max(1, Math.round(canvas.clientWidth * scale)),
    height: Math.max(1, Math.round(canvas.clientHeight * scale)),
  };
};
const frame = new VectorFrame();

frame.line(-0.6, 0.0, 0.2, 0.0, 0.4, 0.9, 1.0, 1.0, 0.012, 1.4);
frame.closedPolyline(
  new Float32Array([-0.72, 0.0, -0.86, 0.075, -0.8, 0.0, -0.86, -0.075]),
  0.72, 0.92, 1.0, 1.0, 0.018, 1.25,
);
gpu.renderFrame(frame);
frame.clear();

// Opt into browser-style canvas pixels when your game already draws in
// top-left-origin pixel coordinates.
const pixelFrame = new VectorFrame();
pixelFrame.line(80, 60, 240, 60, 0.4, 0.9, 1.0, 1.0, 4, 1.4);
const { width, height } = canvasPixelSize();
const pixelView = VectorFrameView.canvasPixels(width, height);
gpu.renderFrameWithView(pixelFrame, pixelView);
```

The v1 geometry surface is stroke-first: `line(x1, y1, x2, y2, r, g, b, a, width, intensity)`, `polyline(points, r, g, b, a, width, intensity)`, and `closedPolyline(points, r, g, b, a, width, intensity)`. Polyline point arrays are flat x/y pairs, usually a `Float32Array`; `closedPolyline` repeats the first point when the submitted array is not already closed. Malformed point arrays and non-finite values throw JavaScript errors before anything is appended.

There are two boundaries. JavaScript uses `VectorFrame` because nested Rust enums and structs such as `VectorCommand` are not the browser JS ABI. Rust/WASM consumers that already own Velumin command data can avoid serializing through JavaScript and call `WebGPU::render_commands(&[VectorCommand])`; `renderFrame` delegates to the same normal CRT renderer path. Both routes render command slices through the existing `Renderer`, and both can use the same `VectorFrameView` mapping.

By default, coordinates use Velumin's centered 4:3 logical playfield: center origin, y-up, with visible coordinates roughly from `-1.0..=1.0` horizontally and `-0.75..=0.75` vertically. Browser resize keeps that playfield centered with letterbox or pillarbox margins rather than stretching geometry.

Use `VectorFrameView` when your game already owns another coordinate model:

- `VectorFrameView.centered4x3()` is the default view.
- `VectorFrameView.logicalExtents(left, bottom, right, top)` maps explicit logical bounds into the full canvas at render time.
- `VectorFrameView.canvasPixels(width, height)` is a convenience for browser-style top-left-origin pixel coordinates where `(0, 0)` is the canvas top-left and `(width, height)` is the bottom-right.
- `gpu.setFrameView(view)` changes the default view used by `renderFrame` and the Rust/WASM `render_commands` path; `gpu.renderFrameWithView(frame, view)` applies a view for one JavaScript frame submission.

Color channels and alpha use `0.0..=1.0`; stroke width is a positive width in the submitted coordinate model; intensity is a non-negative multiplier applied by the renderer. Display presets remain renderer-global: commands carry stroke color, width, and intensity, while `VectorDisplayPreset` controls the overall glow/CRT look.

This is an immediate-frame API, not a retained scene graph. Rebuild and submit the commands visible for each game frame. View mapping is render-time coordinate conversion only; gameplay camera following, wrapping, collision, and object ownership remain in the consuming game.

## Browser Demos

Build the WASM package and start the local Vite server:

```sh
scripts/demos
```

The script prints the available demo routes, then starts Vite. Open the localhost URL shown by the server.

Available demo routes:

- `/` renders the baseline white-line smoke scene.
- `/?demo=blasterites` renders the deterministic Blasterites-inspired tester scene with a rotating ship, bullet, approaching asteroid, spark explosion, glow, scanlines, and subtle pulse/wobble.
- `/?demo=frame-api` renders a deterministic Replication Vector / Blasterites-style harness built from the public `VectorFrame` JavaScript API and `VectorFrameView.canvasPixels`.
- `/?demo=tuner` renders the Blasterites tester with live sliders for vector line width and glow-layer tuning.

The Blasterites tester, tuner, and frame-API route are renderer validation harnesses, not playable games. The tester and frame-API harness are deterministic from elapsed time so future browser or screenshot smoke checks can target known moments in the animation.

Add `?frame` (optionally `?frame=<ms>` or `?t=<ms>`) to any demo route to render a single deterministic frame at a fixed elapsed time instead of animating — this is the freeze-frame mode the screenshot smoke check uses.

## Browser Screenshot Smoke Check

```sh
scripts/smoke
```

`scripts/smoke` builds the WASM package, serves the demos, and drives a scripted headless Chromium (Playwright) over deterministic tester frames at 4:3, wide, and tall viewports plus public frame-API harness captures, asserting that each scene renders visible vector geometry on a black field, that default non-4:3 scenes letterbox rather than distort, and that the public frame-API harness can render through a full-canvas pixel view. It requires a WebGPU-capable Chromium and **skips cleanly on GPU-less environments** (such as CI), so it is not part of `scripts/validate`. Run `npm install` in `webgpu_vector_lib/web/` and `npx playwright install chromium` first. See `scripts/README.md` for details.

## Canonical Local Validation

Velumin uses a script-first validation workflow. The top-level `scripts/` directory is the shared contract for local development, CI, and agent environments.

For ordinary validation from the repository root:

```sh
scripts/validate
```

`scripts/validate` runs:

1. `scripts/version`
2. `scripts/format --check`
3. `scripts/lint`
4. `scripts/test`
5. `scripts/baseline`

For smaller review or edit cycles, this sequence is preferred:

```sh
scripts/version
scripts/format --check
scripts/lint
scripts/test
```

Run `scripts/baseline` when Rust/WASM/Vite browser build behavior may be affected.

Use `scripts/develop` only for setup/bootstrap or when debugging setup. It installs or verifies Rust components, the WASM target, npm dependencies, and the pinned `wasm-pack` version.

From the repository root:

```sh
scripts/develop
scripts/validate
```

See `scripts/README.md` for the full script reference.

## Continuous Integration

GitHub Actions runs the same validation contract as local development:

```sh
scripts/develop
scripts/validate
```

The workflow lives at `.github/workflows/validate.yml`. It runs on pull requests, pushes to `main`, and manual dispatch. It uses read-only repository permissions, Node 24 with npm lockfile caching, the checked-in Rust toolchain, and the repository scripts.

See `.github/workflows/README.md` for workflow details and maintenance notes.

## Manual Baseline Commands

The scripts above are canonical, but these lower-level commands are useful when debugging a specific layer.

From `webgpu_vector_lib/`:

```sh
cargo check --target wasm32-unknown-unknown
wasm-pack build --target web
```

From `webgpu_vector_lib/web/`:

```sh
npm run baseline
npm run build
npm run dev
```

Open the Vite localhost URL and verify:

- the page background is black;
- the canvas fills the viewport;
- a horizontal white line is visible near the center;
- browser logs reach setup, pipeline creation, render call, and frame presentation without errors.

For the active roadmap and implementation phases, see `project/roadmap/roadmap.md` and `project/work_items/`.
