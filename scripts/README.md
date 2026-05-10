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

### `scripts/format`

Runs rustfmt against `webgpu_vector_lib/Cargo.toml`.

Use `scripts/format --check` in validation and review repair. Use `scripts/format` when intentionally applying formatter output.

### `scripts/lint`

Runs:

```sh
cargo clippy --manifest-path webgpu_vector_lib/Cargo.toml --target wasm32-unknown-unknown --all-targets -- -D warnings
```

Warnings are treated as errors to keep CI strict and deterministic.

### `scripts/test`

Runs Rust tests through the crate manifest:

```sh
cargo test --manifest-path webgpu_vector_lib/Cargo.toml
```

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
- Browser visual smoke validation is not part of the current script lane. The current baseline is build-level validation plus manual browser demo checks.
- `wasm-pack` may write helper-tool cache data outside the repository. In restricted sandboxes, that can require additional permissions even when validation is otherwise correct.
