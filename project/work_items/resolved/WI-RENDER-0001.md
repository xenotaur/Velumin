---
id: WI-RENDER-0001
title: Preserve Browser Rendering Baseline
status: resolved
priority: high
owner: project maintainers
depends_on: WI-BOOTSTRAP-0001---

# WI-RENDER-0001: Preserve Browser Rendering Baseline

## Objective
- Keep the current browser white-line smoke demo visible and repeatable before renderer modernization begins.

## Scope
- Document the clean local build and demo workflow for the current Rust/WASM/Vite browser harness.
- Verify that the browser demo renders a visible white line on a black canvas after a clean rebuild.
- Add or document a tiny visual validation path that can detect a black-canvas regression.
- Record browser, OS, and GPU/adapter information when available during validation.

## Evidence
- DP-0001: `project/design/proposals/DP-0001-modern-webgpu-rendering.md`
- Current Rust entrypoint: `webgpu_vector_lib/src/lib.rs`
- Current shader smoke signal: `webgpu_vector_lib/shaders/line.wgsl`
- Current browser harness: `webgpu_vector_lib/web/index.html`

## Acceptance Criteria
- `cargo check --target wasm32-unknown-unknown` is documented and passes from a clean checkout.
- `wasm-pack build --target web` is documented and passes from a clean checkout.
- Vite build/run commands are documented and exercise the browser harness.
- The browser demo shows a visible white line on a black canvas.
- Visual verification can distinguish the expected smoke output from an all-black or all-white canvas.
- No renderer upgrade work begins without this baseline being recoverable.

## Status
- Done: build commands are documented, npm baseline scripts are available, and browser visual verification is recorded in `project/evidence/EV-0002.md`.
