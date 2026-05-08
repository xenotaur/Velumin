# Velumin

Velumin is a retro vector-graphics library.

## Browser Rendering Baseline

The current browser baseline is a Rust/WASM/Vite demo that renders a white line on a black canvas. Preserve this signal before changing renderer dependencies or architecture. The crate currently requires Rust 1.87 or newer.

## Canonical Local Validation

From the repository root:

```sh
scripts/develop
scripts/validate
```

`scripts/validate` is the canonical local validation command. It reports tool versions, checks formatting, runs Clippy with warnings denied, runs Rust tests, and rebuilds the Rust/WASM/Vite browser baseline.

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
