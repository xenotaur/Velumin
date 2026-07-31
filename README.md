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

## Browser Demos

Build the WASM package and start the local Vite server:

```sh
scripts/demos
```

The script prints the available demo routes, then starts Vite. Open the localhost URL shown by the server.

Available demo routes:

- `/` renders the baseline white-line smoke scene.
- `/?demo=blasterites` renders the deterministic Blasterites-inspired tester scene with a rotating ship, bullet, approaching asteroid, spark explosion, glow, scanlines, and subtle pulse/wobble.
- `/?demo=tuner` renders the Blasterites tester with live sliders for vector line width and glow-layer tuning.

The Blasterites tester and tuner are renderer validation harnesses, not playable games. The tester is deterministic from elapsed time so future browser or screenshot smoke checks can target known moments in the animation.

Add `?frame` (optionally `?frame=<ms>` or `?t=<ms>`) to any demo route to render a single deterministic frame at a fixed elapsed time instead of animating — this is the freeze-frame mode the screenshot smoke check uses.

## Browser Screenshot Smoke Check

```sh
scripts/smoke
```

`scripts/smoke` builds the WASM package, serves the demos, and drives a scripted headless Chromium (Playwright) over the deterministic tester frames at 4:3, wide, and tall viewports, asserting that each scene renders visible vector geometry on a black field and that non-4:3 sizes letterbox rather than distort. It requires a WebGPU-capable Chromium and **skips cleanly on GPU-less environments** (such as CI), so it is not part of `scripts/validate`. Run `npm install` in `webgpu_vector_lib/web/` and `npx playwright install chromium` first. See `scripts/README.md` for details.

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
