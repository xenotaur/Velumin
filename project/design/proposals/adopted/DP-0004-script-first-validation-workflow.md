---
id: DP-0004
title: Script-First Validation Workflow
status: adopted
owner: project maintainers
created: 2026-05-08
adopted: 2026-05-08
implementation_status: implemented
implemented_by:
  - WI-CI-0001
  - WI-CI-0002
evidence:
  - EV-0007
scope: developer workflow and continuous integration
depends_on:
  - DP-0001
---

# Script-First Validation Workflow

## Summary
Adopt a script-first validation workflow for Velumin. Repository-owned scripts should be the canonical contract for local development, CI, and agent environments. GitHub Actions should call those scripts instead of duplicating validation logic directly in workflow YAML.

This adapts the workflow shape used by Logical Robotics Harness while replacing Python-specific checks with Rust, WASM, npm, Vite, and browser-rendering checks appropriate for Velumin.

## Adoption Status
Adopted on 2026-05-08 as the active follow-up workstream after DP-0001. The core implementation is tracked by:

- `WI-CI-0001`: add script-first local validation;
- `WI-CI-0002`: add GitHub Actions validation.

Implementation evidence is recorded in `project/evidence/EV-0007.md`. Browser visual smoke, workflow linting, cargo-deny, Dependency Review, Dependabot, and broader supply-chain policy remain deferred follow-up work.

## Context
Velumin is currently a Rust/WASM/Vite project. The browser baseline is documented in the root `README.md`:

```sh
cargo check --target wasm32-unknown-unknown
wasm-pack build --target web
npm run baseline
npm run build
npm run dev
```

At proposal time, the project already had a useful `npm run baseline` wrapper under `webgpu_vector_lib/web/package.json`, but the repository did not yet have a top-level validation contract analogous to LRH's script workflow.

LRH's relevant design lesson is not Python itself. The useful pattern is that humans, CI, and coding agents all use the same repository scripts as the source of truth. Velumin should preserve that property while using idiomatic Rust and web tooling underneath.

## Problem
Without a canonical script layer, validation can fragment across:

- README commands;
- `package.json` scripts;
- direct `cargo` invocations;
- GitHub Actions YAML;
- ad hoc agent commands;
- later browser or visual smoke checks.

That fragmentation makes it easier for local validation to differ from CI validation. It also makes it harder to collect trustworthy evidence when a renderer change works in one environment but fails in another.

## Goals
- Define one script-based validation contract for local development, CI, and agent environments.
- Keep scripts thin and transparent, delegating to standard Rust, wasm-pack, npm, and Vite commands.
- Preserve the current browser baseline as the first validation target.
- Make future checks easy to add without rewriting CI workflows.
- Keep dependency installation and tool version diagnostics explicit.
- Keep CI workflow YAML small and focused on checkout, toolchain setup, caching, and script execution.

## Non-Goals
- Do not introduce a heavyweight task runner before the workflow needs one.
- Do not replace Cargo, rustfmt, Clippy, npm, Vite, or wasm-pack with custom logic.
- Do not require a Rust `xtask` crate in the first implementation.
- Do not require all visual/browser automation in the first implementation.
- Do not implement release publishing in this proposal.

## Proposed Direction
Add a top-level `scripts/` directory with executable script entrypoints. These scripts should be the commands documented in README, called by CI, and used by agents.

Initial script set:

| Script | Purpose |
| --- | --- |
| `scripts/develop` | Install or verify the local development prerequisites that the repository can reasonably manage. |
| `scripts/version` | Print versions for Rust, Cargo, rustfmt, Clippy, wasm-pack, Node, npm, and Vite. |
| `scripts/format` | Format Rust code and, later, any JS/WGSL files once formatting tools are selected. |
| `scripts/lint` | Run Clippy and lightweight static checks. |
| `scripts/test` | Run Rust tests and, later, wasm/browser tests. |
| `scripts/build` | Build the WASM package and Vite app. |
| `scripts/baseline` | Run the current browser-rendering build baseline. |
| `scripts/validate` | Run the canonical local validation sequence. |
| `scripts/check-workflows` | Validate GitHub Actions workflow syntax, with deeper semantic linting optional later. |

The initial canonical validation sequence should be:

```sh
scripts/version
scripts/format --check
scripts/lint
scripts/test
scripts/baseline
```

`scripts/validate` should run that sequence in order. It should fail fast on errors and print enough command context to make CI and agent logs useful.

## Suggested Command Mapping

### Rust Toolchain
Use a checked-in `rust-toolchain.toml` so Rust version, components, and targets are visible in the repository:

```toml
[toolchain]
channel = "1.87"
components = ["rustfmt", "clippy"]
targets = ["wasm32-unknown-unknown"]
```

This matches Velumin's current minimum Rust requirement while making the WASM target and lint/format components explicit.

### Formatting
Initial behavior:

```sh
cargo fmt --all
cargo fmt --all -- --check
```

The script should support both mutating and check-only modes:

```sh
scripts/format
scripts/format --check
```

WGSL and JavaScript formatting should be added only after the project selects a formatter and configuration. Until then, Rust formatting should be the enforced baseline.

### Linting
Initial behavior:

```sh
cargo clippy --manifest-path webgpu_vector_lib/Cargo.toml --target wasm32-unknown-unknown --all-targets -- -D warnings
```

If native targets are added under DP-0002, linting should expand to cover the workspace and relevant target feature combinations.

### Testing
Initial behavior:

```sh
cargo test --manifest-path webgpu_vector_lib/Cargo.toml
```

As the renderer grows, add:

- `wasm-pack test --headless --chrome` or another agreed browser target;
- focused CPU-side tests for geometry/tessellation;
- smoke tests for unsupported-browser and no-adapter paths where practical.

### Build and Baseline
Initial behavior:

```sh
cargo check --manifest-path webgpu_vector_lib/Cargo.toml --target wasm32-unknown-unknown
wasm-pack build webgpu_vector_lib --target web
npm ci --prefix webgpu_vector_lib/web
npm run build --prefix webgpu_vector_lib/web
```

The baseline script should preserve the current README signal: a clean Rust/WASM/Vite build path that protects the visible white-line-on-black browser demo.

### Browser Smoke Checks
Browser automation should be added after the script layer exists. A later phase should add a visual smoke command that:

- starts the Vite dev server;
- opens the page in a browser with WebGPU support where available;
- verifies the canvas is nonblank;
- checks for expected black background and white-line pixels;
- records browser, OS, and adapter information when available.

Until then, `scripts/baseline` should remain a build-level smoke check rather than claiming visual verification.

## CI Shape
GitHub Actions should install toolchains and dependencies, then call repository scripts.

Recommended fast PR workflow shape:

```yaml
name: Validate

on:
  pull_request:
  push:
    branches: [main]
  workflow_dispatch:

permissions:
  contents: read

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - uses: actions/setup-node@v6
        with:
          node-version: 24
          cache: npm
          cache-dependency-path: webgpu_vector_lib/web/package-lock.json
      - run: scripts/develop
      - run: scripts/validate
```

The CI workflow may perform setup directly at first if `scripts/develop` is not yet ready, but validation logic should still live in repository scripts.

## Optional Security and Supply-Chain Checks
Add these as separate checks after the core workflow is stable:

- `cargo deny check` for Rust dependency advisories, licenses, duplicate crates, and sources.
- GitHub Dependency Review on pull requests that change dependency manifests or lockfiles.
- npm audit policy only after the project defines an acceptable severity threshold and false-positive handling policy.
- Dependabot configuration for Cargo, npm, and GitHub Actions updates.

These should be separate from the first fast validation lane if they are noisy or slow.

## Options Considered

### Thin Top-Level Scripts
Use simple executable shell scripts under `scripts/`.

Advantages:

- Closely matches LRH's useful script-first workflow.
- Keeps local, CI, and agent validation aligned.
- Uses standard Rust and web tooling directly.
- Low implementation cost.
- Easy to inspect and debug.

Costs:

- Shell scripts are less portable to native Windows shells.
- Argument parsing can become awkward if scripts grow too complex.
- Discipline is needed to keep scripts thin.

This is the recommended first implementation.

### `package.json` as the Main Control Plane
Use npm scripts as the canonical validation interface.

Advantages:

- Natural for web contributors.
- Builds on the existing `webgpu_vector_lib/web/package.json`.
- Easy to run in GitHub Actions after `npm ci`.

Costs:

- Makes npm the control plane for a Rust-first renderer.
- Rust-only validation becomes nested under the web package.
- Toolchain setup, release checks, and security tooling are harder to express cleanly.

This is acceptable for local web subcommands, but should not be the top-level canonical workflow.

### Cargo `xtask`
Create a Rust helper crate that implements validation commands.

Advantages:

- Better cross-platform behavior than shell scripts.
- Stronger structure if orchestration becomes complex.
- Natural for a mature Rust workspace.

Costs:

- Adds a helper crate and workspace structure before Velumin needs it.
- Slower to bootstrap.
- More code to maintain for simple command sequencing.

This is a good later migration path if shell scripts become too complex.

### `justfile` or `Makefile`
Use a task runner for validation aliases.

Advantages:

- Nice local developer ergonomics.
- Good for short command names and grouped workflows.

Costs:

- Adds another required tool if used as the canonical interface.
- CI must install or assume the tool.
- Less direct than executable scripts.

This can be added as a convenience layer, but should not replace scripts as the canonical contract.

### CI-Only Validation
Put validation commands directly in GitHub Actions.

Advantages:

- Fastest to write initially.
- No local script maintenance.

Costs:

- Local validation and CI validation can drift.
- Agents lose a reliable source-of-truth command.
- Debugging environment mismatches becomes harder.

This option is rejected for Velumin because it fails the core reproducibility goal.

## Implementation Staging

### Phase 0: Document the Contract
- Add this design proposal.
- Update README once scripts exist.
- Treat current README commands as the temporary baseline until scripts are implemented.

### Phase 1: Add Minimal Scripts
- Add `scripts/version`.
- Add `scripts/format`.
- Add `scripts/lint`.
- Add `scripts/test`.
- Add `scripts/baseline`.
- Add `scripts/validate`.
- Keep each script thin and readable.

### Phase 2: Add Toolchain Pinning
- Add `rust-toolchain.toml`.
- Include `rustfmt`, `clippy`, and `wasm32-unknown-unknown`.
- Confirm `scripts/version` reports the selected toolchain.

### Phase 3: Add CI
- Add a fast validation workflow that calls `scripts/validate`.
- Use Node setup with npm cache keyed to `webgpu_vector_lib/web/package-lock.json`.
- Use read-only repository permissions unless a job needs more.

### Phase 4: Add Workflow Validation
- Add `.github/workflows/`.
- Add `scripts/check-workflows`.
- Start with YAML syntax validation.
- Consider `actionlint` later for deeper semantic checks.

### Phase 5: Add Visual Browser Smoke
- Add browser automation after the build baseline is stable.
- Verify visible rendering, not only successful compilation.
- Record evidence useful for renderer regressions.

### Phase 6: Add Supply-Chain Checks
- Add `cargo-deny` after a `deny.toml` policy is agreed.
- Add GitHub Dependency Review for pull requests.
- Add Dependabot for Cargo, npm, and GitHub Actions.

## Acceptance Criteria
- A clean checkout can run `scripts/validate` as the canonical local validation command.
- GitHub Actions calls `scripts/validate` rather than duplicating its command sequence.
- `scripts/format --check`, `scripts/lint`, `scripts/test`, and `scripts/baseline` can be run independently.
- Version diagnostics include Rust, Cargo, rustfmt, Clippy, wasm-pack, Node, npm, and Vite when available.
- The current Rust/WASM/Vite browser baseline remains protected.
- CI uses frozen npm installs through `npm ci`.
- CI uses the checked-in Rust toolchain target/components or equivalent explicit setup.
- Any visual smoke test clearly distinguishes build success from actual browser-render verification.

## Risks
- Shell scripts may accumulate too much logic if not kept thin.
- `wasm-pack` installation may vary across developer machines and CI images.
- Headless browser WebGPU support may be inconsistent, making early visual CI checks flaky.
- Clippy warnings can change as the Rust toolchain changes, especially if the toolchain is not pinned.
- Security tooling can create noisy failures before the project defines policy.

## Guardrails
- Prefer repository scripts over direct tool invocations in CI and agent workflows.
- Keep scripts small and explicit; migrate to `xtask` only if complexity justifies it.
- Use `npm ci` in CI and avoid lockfile mutation during validation.
- Use `cargo fmt --check` and Clippy with warnings denied in CI.
- Do not claim visual validation unless a browser smoke check actually inspects rendered output.
- Keep heavyweight or flaky checks separate from the fast PR lane until stable.

## References
- Current project files: `README.md`, `webgpu_vector_lib/Cargo.toml`, `webgpu_vector_lib/web/package.json`, `webgpu_vector_lib/web/vite.config.js`.
- LRH script workflow and canonical validation: https://raw.githubusercontent.com/xenotaur/logical_robotics_harness/main/README.md
- Cargo continuous integration guide: https://doc.rust-lang.org/cargo/guide/continuous-integration.html
- Cargo test documentation: https://doc.rust-lang.org/cargo/commands/cargo-test.html
- Clippy continuous integration guidance: https://doc.rust-lang.org/stable/clippy/continuous_integration/index.html
- rustfmt formatting and CI check behavior: https://github.com/rust-lang/rustfmt
- rustup toolchain file documentation: https://rust-lang.github.io/rustup/overrides.html
- wasm-pack build documentation: https://rustwasm.github.io/docs/wasm-pack/commands/build.html
- npm clean install documentation: https://docs.npmjs.com/cli/v10/commands/npm-ci/
- Vite production build documentation: https://vite.dev/guide/build
- GitHub setup-node dependency caching: https://github.com/actions/setup-node
- cargo-deny checks: https://embarkstudios.github.io/cargo-deny/checks/index.html
- GitHub Dependency Review Action: https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/manage-your-dependency-security/configuring-the-dependency-review-action
