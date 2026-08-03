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
- DP-0001, DP-0004, DP-0005, and DP-0006 are adopted. DP-0005 shipped the deterministic Blasterites tester and live tuner browser demos (merged across PRs #3-#6).
- DP-0006 (Vector CRT renderer migration) was adopted on 2026-07-24 and is partially implemented: a fixed 4:3 viewport, additive multi-layer glow, and an internal display-preset set have landed alongside the demos, validated by code inspection (EV-0008) and browser visual-smoke capture (EV-0009). The WI-SMOKE-0001 smoke check (`scripts/smoke`) now captures the baseline, tester (pre/post), and tuner at 4:3, wide, and tall, so the non-4:3 letterbox/pillarbox path is covered. The public preset API (adopted DP-0007) is implemented in WI-PRESET-0001 — a public `#[non_exhaustive]` `VectorDisplayPreset` selectable at creation and runtime — and the non-default presets are now captured in the smoke check.
- DP-0002 and DP-0003 remain proposed (not adopted) design directions. DP-0002's Phase 1 platform-neutral type extraction is done: `WI-ARCH-0001` (PR #17) converted the repository into a Cargo workspace with a `velumin-core` crate holding vector/scene/style types, zero `wasm-bindgen`/`web-sys`/`wgpu` dependency. DP-0002 Phase 2 is done for the browser-only shared-renderer slice: `WI-ARCH-0002` (PR #20) made `Renderer` host-buildable, and web-side adapter/capability negotiation is covered by `WebGPU::create_with_preset` plus `Renderer::new`. Desktop-side adapter/capability negotiation moved to Phase 3 on 2026-08-01 and remains unselected. DP-0003 remains unselected.

## Evidence Basis
- `README.md` identifies Velumin as a retro vector-graphics library and documents the `/`, `/?demo=blasterites`, and `/?demo=tuner` demo routes.
- `webgpu_vector_lib/src/lib.rs` shows WebGPU canvas setup, capability handling, renderer state, vector primitive tessellation, additive multi-layer glow, a centered 4:3 viewport (`RenderViewport::centered_4_3`), an internal `VectorDisplayPreset` set, and the `render`, `render_blasterites_tester`, and `render_blasterites_tuner` entrypoints exposed through `wasm-bindgen`.
- `webgpu_vector_lib/shaders/` shows the crisp line pass (`line.wgsl`), glow and composite passes (`glow.wgsl`, `composite.wgsl`), and the tester-only scanline composite (`tester_composite.wgsl`).
- `webgpu_vector_lib/web/index.html` shows the browser canvas harness with query-parameter demo routing and the tuner control panel.
- `scripts/demos` builds the WASM package and serves the baseline, Blasterites, and tuner routes.
- `project/evidence/EV-0002.md` through `EV-0006.md` record DP-0001 implementation verification; `EV-0007.md` records DP-0004; `EV-0008.md` records DP-0005/DP-0006 code verification; `EV-0009.md` records the browser visual-smoke capture of the demo scenes.
- `scripts/validate` is the canonical local validation command; `scripts/demos` serves the browser demos.

## Current Health
- Yellow: project identity, the browser/WebGPU baseline, the validation workflow, the Blasterites demos, the adopted Vector CRT renderer (DP-0006, with visual-smoke evidence EV-0009), and an automatable smoke check (WI-SMOKE-0001, `scripts/smoke`) are all visible and merged, but a broader public vector/scene drawing API and cross-platform architecture are not yet designed, and DP-0007's deferred public custom display-settings API is not yet built.

## Active Priorities
- Preserve the adopted DP-0001 baseline, DP-0004 validation workflow, DP-0005 demos, and DP-0006 renderer.
- Preserve the adopted DP-0006 renderer, the WI-SMOKE-0001 smoke check, and the DP-0007 public preset API (implemented in WI-PRESET-0001).
- Decide whether/when to build DP-0007's deferred public custom display-settings API. DP-0002 Phase 2 is complete for its browser-only shared-renderer scope; desktop-side adapter/capability negotiation is now a Phase 3 bullet and still awaits Phase 3's own workstream selection.
- Keep design proposal lifecycle metadata and directories aligned with what is merged.

## Risks
- The Vector CRT visual model (glow falloff, presets) has landed in code and its default look is validated by browser visual-smoke capture (EV-0009) and the WI-SMOKE-0001 automated check (`scripts/smoke`, covering 4:3, wide, and tall; it skips on GPU-less CI). The non-default presets are not yet visually captured and their quality should not be overstated.
- The `VectorDisplayPreset` set is currently internal (`#[allow(dead_code)]`); treating it as a stable public API would be premature.
- Browser/WebGPU behavior may vary and should be validated explicitly before claims are made.

## Recommended Next Actions
1. Decide whether to build DP-0007's deferred public custom display-settings API (a follow-up proposal/work item), or leave named presets as the v1 public surface.
2. Select and scope DP-0002 Phase 3 before pursuing desktop-side adapter/capability negotiation or a native `winit` shell.
3. Define the first public vector primitive or scene API target.
