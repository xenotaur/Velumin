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
- DP-0001, DP-0004, DP-0005, DP-0006, and DP-0008 are adopted. DP-0005 shipped the deterministic Blasterites tester and live tuner browser demos (merged across PRs #3-#6). DP-0008's first implementation slice (`WI-API-0001`, PR #27) exposes the public browser `VectorFrame` API plus a Rust/WASM typed `VectorCommand` slice path; `WI-API-0002` validated that typed path against Replication Vector's project-owned command scene with browser runtime evidence (EV-0010). `WI-API-0003` selected a narrow DP-0008 view/viewport mapping follow-up as the next consumer-driven public surface, based on EV-0010 plus fresh Blasterites pixel/full-window coordinate pressure recorded in EV-0011; proposed `WI-API-0004` now scopes that follow-up.
- DP-0006 (Vector CRT renderer migration) was adopted on 2026-07-24 and is partially implemented: a fixed 4:3 viewport, additive multi-layer glow, and an internal display-preset set have landed alongside the demos, validated by code inspection (EV-0008) and browser visual-smoke capture (EV-0009). The WI-SMOKE-0001 smoke check (`scripts/smoke`) now captures the baseline, tester (pre/post), and tuner at 4:3, wide, and tall, so the non-4:3 letterbox/pillarbox path is covered. The public preset API (adopted DP-0007) is implemented in WI-PRESET-0001 — a public `#[non_exhaustive]` `VectorDisplayPreset` selectable at creation and runtime — and the non-default presets are now captured in the smoke check.
- DP-0002 and DP-0003 remain proposed (not adopted) design directions. DP-0002's Phase 1 platform-neutral type extraction is done: `WI-ARCH-0001` (PR #17) converted the repository into a Cargo workspace with a `velumin-core` crate holding vector/scene/style types, zero `wasm-bindgen`/`web-sys`/`wgpu` dependency. DP-0002 Phase 2 is done for the browser-only shared-renderer slice: `WI-ARCH-0002` (PR #20) made `Renderer` host-buildable, and web-side adapter/capability negotiation is covered by `WebGPU::create_with_preset` plus `Renderer::new`. DP-0008 now uses that split for browser frame submission; desktop-side adapter/capability negotiation moved to Phase 3 on 2026-08-01 and remains unselected. DP-0003 remains unselected.

## Evidence Basis
- `README.md` identifies Velumin as a retro vector-graphics library and documents the `/`, `/?demo=blasterites`, `/?demo=tuner`, and `/?demo=frame-api` demo routes plus the browser `VectorFrame` and Rust/WASM `VectorCommand` frame-submission boundaries.
- `webgpu_vector_lib/src/lib.rs` shows WebGPU canvas setup, capability handling, renderer state, vector primitive tessellation, additive multi-layer glow, a centered 4:3 viewport (`RenderViewport::centered_4_3`), the public `VectorDisplayPreset` set, the `VectorFrame` builder, the Rust/WASM `render_commands(&[VectorCommand])` path, and the `render`, `renderFrame`, `render_blasterites_tester`, and `render_blasterites_tuner` entrypoints exposed through `wasm-bindgen`.
- `webgpu_vector_lib/shaders/` shows the crisp line pass (`line.wgsl`), glow and composite passes (`glow.wgsl`, `composite.wgsl`), and the tester-only scanline composite (`tester_composite.wgsl`).
- `webgpu_vector_lib/web/index.html` shows the browser canvas harness with query-parameter demo routing and the tuner control panel.
- `scripts/demos` builds the WASM package and serves the baseline, Blasterites, and tuner routes.
- `project/evidence/EV-0002.md` through `EV-0006.md` record DP-0001 implementation verification; `EV-0007.md` records DP-0004; `EV-0008.md` records DP-0005/DP-0006 code verification; `EV-0009.md` records the browser visual-smoke capture of the demo scenes; `EV-0010.md` records browser runtime validation of Replication Vector-owned `VectorCommand` data through Velumin's typed Rust/WASM command-slice path; `EV-0011.md` selects DP-0008 view/viewport mapping as the next consumer-driven public surface.
- `scripts/validate` is the canonical local validation command; `scripts/demos` serves the browser demos.

## Current Health
- Yellow: project identity, the browser/WebGPU baseline, the validation workflow, the Blasterites demos, the adopted Vector CRT renderer (DP-0006, with visual-smoke evidence EV-0009), an automatable smoke check (WI-SMOKE-0001, `scripts/smoke`), the DP-0007 named display-preset API, and the DP-0008/WI-API-0001 public frame API are visible and merged. Replication Vector's representative command-slice consumer frame is validated in EV-0010, EV-0011 selects DP-0008 view/viewport mapping as the next consumer-driven public surface, and proposed `WI-API-0004` now captures that implementation slice. A broader retained scene/material API, native desktop frontend, and DP-0007's deferred public custom display-settings API are not yet built.

## Active Priorities
- Preserve the adopted DP-0001 baseline, DP-0004 validation workflow, DP-0005 demos, and DP-0006 renderer.
- Preserve the adopted DP-0006 renderer, the WI-SMOKE-0001 smoke check, and the DP-0007 public preset API (implemented in WI-PRESET-0001).
- Do not expand DP-0008 solely for the current Replication Vector representative frame; EV-0010 found the typed command-slice path sufficient. EV-0011 adds fresh Blasterites evidence and selects a narrow DP-0008 view/viewport mapping follow-up as the next consumer-driven public surface; proposed `WI-API-0004` is the scoped follow-up to ready and execute next. DP-0007's deferred public custom display-settings API, DP-0003 retained scene/material work, and DP-0002 Phase 3/native work remain deferred until separately selected.
- Keep design proposal lifecycle metadata and directories aligned with what is merged.

## Risks
- The Vector CRT visual model (glow falloff, presets) has landed in code. The default look is validated by browser visual-smoke capture (EV-0009), and the WI-SMOKE-0001 automated check (`scripts/smoke`, covering 4:3, wide, tall, and the non-default presets; it skips on GPU-less CI) keeps the captured preset set from drifting.
- DP-0008's immediate-frame API is intentionally stroke-first and browser-first; treating it as a full retained scene/material API would be premature.
- Browser/WebGPU behavior may vary and should be validated explicitly before claims are made.

## Recommended Next Actions
1. Ready and execute proposed `WI-API-0004`, the focused DP-0008 view/viewport mapping follow-up for browser games with full-window or arbitrary logical coordinate systems that need to submit `VectorFrame` / `&[VectorCommand]` data without ad hoc per-game coordinate conversion.
2. Select and scope DP-0002 Phase 3 before pursuing desktop-side adapter/capability negotiation or a native `winit` shell.
3. If more API work is proposed, ground it in another concrete Replication Vector or Blasterites consumer pressure point rather than the already-validated EV-0010 representative frame.
