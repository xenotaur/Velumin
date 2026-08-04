# Validation Scripts

Repository-owned validation scripts are the canonical command contract for local development, CI, and agent workflows.

Run scripts from the repository root.

## Quick Reference

| Command | Purpose |
| --- | --- |
| `scripts/develop` | Setup/bootstrap local dependencies and CI tools. |
| `scripts/version` | Print validation tool versions. |
| `scripts/format` | Format Rust code with rustfmt. |
| `scripts/format --check` | Check Rust formatting without rewriting files. |
| `scripts/lint` | Run Clippy for the WASM target with warnings denied. |
| `scripts/test` | Run Rust tests. |
| `scripts/baseline` | Rebuild the Rust/WASM/Vite browser baseline. |
| `scripts/demos` | Build the WASM package and start the Vite demo server. |
| `scripts/smoke` | Browser screenshot smoke check of the demos (requires a WebGPU-capable Chromium; skips otherwise). |
| `scripts/validate` | Run the full canonical validation sequence. |

## Recommended Workflow

For ordinary code review and edit cycles:

```sh
scripts/version
scripts/format --check
scripts/lint
scripts/test
```

For full local validation:

```sh
scripts/validate
```

For setup/bootstrap:

```sh
scripts/develop
```

Do not run `scripts/develop` routinely during ordinary validation. It is for setup, CI bootstrap, or setup debugging.

## Script Details

### `scripts/develop`

Ensures the development environment has:

- Rust `rustfmt` and Clippy components;
- the `wasm32-unknown-unknown` target;
- npm dependencies from `webgpu_vector_lib/web/package-lock.json`;
- pinned `wasm-pack` version `0.13.1`.

In CI, missing or mismatched `wasm-pack` is installed with:

```sh
cargo install wasm-pack --version 0.13.1 --locked
```

Locally, a missing or mismatched `wasm-pack` prints an explicit install command and exits. This keeps routine validation from unexpectedly installing tools on contributor machines.

### `scripts/version`

Prints versions for:

- `rustc`;
- Cargo;
- rustfmt;
- Clippy;
- `wasm-pack`;
- Node;
- npm;
- Vite.

Use this first when debugging validation failures. Missing or mismatched tools usually indicate setup/bootstrap mismatch rather than code regression.

The repository is a Cargo workspace (root `Cargo.toml`) of two crates: `velumin-core` (platform-neutral vector/scene/style types, no `wasm-bindgen`/`web-sys`/`wgpu` dependency) and `webgpu_vector_lib` (the `wgpu` renderer and browser adapter, depending on `velumin-core`). `scripts/format`, `scripts/lint`, and `scripts/test` operate against the workspace root manifest, covering both crates in one pass.

### `scripts/format`

Runs rustfmt against the workspace root `Cargo.toml` (all members).

Use `scripts/format --check` in validation and review repair. Use `scripts/format` when intentionally applying formatter output.

### `scripts/lint`

Runs:

```sh
cargo clippy --manifest-path Cargo.toml --target wasm32-unknown-unknown --all-targets -- -D warnings
```

Warnings are treated as errors to keep CI strict and deterministic. `velumin-core` has no `wasm32`-specific dependencies, so it compiles cleanly for this target alongside `webgpu_vector_lib`.

### `scripts/test`

Runs Rust tests for the whole workspace, on the host target:

```sh
cargo test --manifest-path Cargo.toml
```

`velumin-core`'s tests run and pass here with no `wasm32` target required, satisfying its own acceptance criterion independently of `webgpu_vector_lib`.

### `scripts/baseline`

Rebuilds the browser baseline:

```sh
cargo check --manifest-path webgpu_vector_lib/Cargo.toml --target wasm32-unknown-unknown
wasm-pack build webgpu_vector_lib --target web
npm run build --prefix webgpu_vector_lib/web
```

Run this when Rust/WASM/Vite build behavior, browser harness behavior, shaders, or renderer setup may be affected.

### `scripts/demos`

Builds the WASM package, prints the available browser demo routes, and starts the local Vite server. The script uses a strict port so the printed routes match the running server.

### `scripts/smoke`

Builds the WASM package, serves the Vite harness, and drives a scripted headless
Chromium (Playwright) over the demo scenes in freeze-frame mode
(`?frame&t=<ms>`). It captures the WebGPU canvas at deterministic Blasterites
tester frames (`t=2000ms` pre-impact, `t=4000ms` post-impact) across 4:3, wide,
and tall viewports, plus a 4:3 public frame-API harness, then asserts structural
properties of the rendered pixels
(not all-black, not all-white, geometry present, pre- vs post-impact frames
differ, and the non-4:3 letterbox/pillarbox margins are dark).

This check requires a **WebGPU-capable Chromium**. Where no WebGPU adapter is
available (for example CI runners without a GPU), it prints a SKIP notice and
exits 0 rather than failing — it is a best-effort guard, not a hard CI gate, and
is intentionally **not** part of `scripts/validate`. Captured frames are written
to `webgpu_vector_lib/web/smoke-out/` (gitignored); committed reference frames
for human comparison live in `webgpu_vector_lib/web/smoke/reference/`. Requires
`npm install` in `webgpu_vector_lib/web/` first (installs Playwright); the
Chromium browser is fetched with `npx playwright install chromium`.

### `scripts/validate`

Runs the complete validation lane:

```sh
scripts/version
scripts/format --check
scripts/lint
scripts/test
scripts/baseline
```

This is the canonical local validation command and the command CI runs after setup.

## Known Notes

- `npm ci` may report audit findings. Audit policy and supply-chain gates are intentionally deferred until there is an explicit work item.
- Browser visual smoke validation is available via `scripts/smoke` but is not part of `scripts/validate`: it needs a WebGPU-capable Chromium and skips on GPU-less runners, so it is an explicit, opt-in check rather than a hard CI gate.
- `wasm-pack` may write helper-tool cache data outside the repository. In restricted sandboxes, that can require additional permissions even when validation is otherwise correct.
