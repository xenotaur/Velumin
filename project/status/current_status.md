---
id: STATUS-CURRENT
title: Current Project Status
scope: project
status: active
health: yellow
---

# Current Status

## Summary
- Velumin is an early-stage retro vector-graphics library with an adopted Rust/WASM/WebGPU browser rendering baseline.
- LRH project-control artifacts make intent, constraints, evidence, and uncertainty explicit.
- DP-0001, DP-0004, and DP-0005 are adopted and implemented. DP-0005 shipped the deterministic Blasterites tester and live tuner browser demos (merged across PRs #3-#6).
- DP-0006 (Vector CRT renderer migration) is partially implemented and is the active renderer workstream: a fixed 4:3 viewport, additive multi-layer glow, and an internal display-preset set have landed alongside the demos, but the visual model is not yet backed by recorded browser evidence.
- DP-0002 and DP-0003 remain proposed follow-up design directions.

## Evidence Basis
- `README.md` identifies Velumin as a retro vector-graphics library and documents the `/`, `/?demo=blasterites`, and `/?demo=tuner` demo routes.
- `webgpu_vector_lib/src/lib.rs` shows WebGPU canvas setup, capability handling, renderer state, vector primitive tessellation, additive multi-layer glow, a centered 4:3 viewport (`RenderViewport::centered_4_3`), an internal `VectorDisplayPreset` set, and the `render`, `render_blasterites_tester`, and `render_blasterites_tuner` entrypoints exposed through `wasm-bindgen`.
- `webgpu_vector_lib/shaders/` shows the crisp line pass (`line.wgsl`), glow and composite passes (`glow.wgsl`, `composite.wgsl`), and the tester-only scanline composite (`tester_composite.wgsl`).
- `webgpu_vector_lib/web/index.html` shows the browser canvas harness with query-parameter demo routing and the tuner control panel.
- `scripts/demos` builds the WASM package and serves the baseline, Blasterites, and tuner routes.
- `project/evidence/EV-0002.md` through `EV-0006.md` record DP-0001 implementation verification; `EV-0007.md` records DP-0004; `EV-0008.md` records DP-0005/DP-0006 code verification; `EV-0009.md` records the browser visual-smoke capture of the demo scenes.
- `scripts/validate` is the canonical local validation command; `scripts/demos` serves the browser demos.

## Current Health
- Yellow: project identity, the browser/WebGPU baseline, the validation workflow, the Blasterites demos, and now a browser visual-smoke capture of the demo scenes (EV-0009) are all visible and merged, but the Vector CRT renderer is only partially adopted (internal presets, public API undecided), and public vector/scene API design and cross-platform architecture are not yet complete.

## Active Priorities
- Preserve the adopted DP-0001 baseline, DP-0004 validation workflow, and DP-0005 demos.
- Advance DP-0006 Vector CRT rendering: record browser/screenshot visual evidence for the Arcade Balanced target and decide whether the internal preset set becomes a public API.
- Keep design proposal lifecycle metadata and directories aligned with what is merged.

## Risks
- The Vector CRT visual model (glow falloff, presets) has landed in code but is not yet validated by recorded browser evidence; rendering quality should not be overstated until it is.
- The `VectorDisplayPreset` set is currently internal (`#[allow(dead_code)]`); treating it as a stable public API would be premature.
- Browser/WebGPU behavior may vary and should be validated explicitly before claims are made.

## Recommended Next Actions
1. Add an automatable/committed screenshot smoke check at the deterministic tester frames (EV-0009 captured them manually; a committed check would let CI guard regressions).
2. Decide whether DP-0006's internal preset set becomes a selectable public API, and visually capture the non-default presets, then drive DP-0006 to adoption.
3. Select DP-0002 (architecture split) or DP-0003 (scene/material model) as the workstream after the renderer stabilizes.
4. Define the first public vector primitive or scene API target.
