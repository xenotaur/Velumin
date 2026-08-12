---
id: FOCUS-RENDER-0001
title: Vector Renderer & Cross-Platform Architecture Focus
status: active
priority: high
owner: project maintainers
---

# Current Focus

## Active Priority
- The DP-0006 renderer follow-ups are done (the WI-SMOKE-0001 smoke check and the DP-0007 public preset API landed), DP-0008's first public frame API slice is implemented by `WI-API-0001` (PR #27), `WI-API-0002` validated DP-0008's typed Rust/WASM command-slice path against Replication Vector's representative frame (EV-0010), `WI-API-0003` selected a narrow DP-0008 view/viewport mapping follow-up as the next consumer-driven public surface (EV-0011), and `WI-API-0004` implemented that follow-up in PR #33. Keep the adopted renderer, demos, public frame API, and validation workflow stable. DP-0002 Phase 2 is done for the browser-only shared-renderer slice: `WI-ARCH-0002` (PR #20) made `Renderer` host-buildable, and web-side adapter/capability negotiation is covered by `WebGPU::create_with_preset` plus `Renderer::new`. Desktop-side adapter/capability negotiation moved to Phase 3 and remains unselected.

## Why This Appears Current
- The LRH control plane is bootstrapped and `WI-BOOTSTRAP-0001` is done.
- DP-0001 (WebGPU baseline) and DP-0004 (validation workflow) are adopted and implemented.
- DP-0005 (Blasterites tester and tuner demos) is adopted and implemented (`WI-DEMO-0001`, merged across PRs #3-#6).
- DP-0006 (Vector CRT renderer migration) was adopted on 2026-07-24 and is partially implemented: the fixed 4:3 viewport, additive multi-layer glow, and a display-preset set have landed alongside the demos, validated by code inspection (EV-0008) and browser visual-smoke capture (EV-0009). Its follow-ups have landed — the `WI-SMOKE-0001` smoke check and the DP-0007 public preset API (`WI-PRESET-0001`).
- DP-0008 (Browser-First Vector Frame API) was adopted on 2026-08-03 and implemented by `WI-API-0001` (PR #27): browser JavaScript can submit a `VectorFrame`, Rust/WASM consumers can render owned `VectorCommand` slices, and `/?demo=frame-api` provides the deterministic public-frame harness. `WI-API-0002` validated the Rust/WASM typed command-slice path against Replication Vector's project-owned representative frame with browser runtime evidence (EV-0010), `WI-API-0003` selected view/viewport mapping as the next evidence-backed DP-0008 follow-up based on Blasterites pixel/full-window coordinate pressure (EV-0011), and `WI-API-0004` implemented public `VectorFrameView` mapping in PR #33.
- DP-0002's Phase 1 platform-neutral type extraction landed: `WI-ARCH-0001` (PR #17) converted the repository into a Cargo workspace with a `velumin-core` crate, with zero visual regression (`scripts/smoke`, 9/9 checks at MAD 0.000). Phase 2's reusable, surface-agnostic `wgpu` renderer state also landed: `WI-ARCH-0002` (PR #20) made `Renderer` host-buildable, with zero visual regression (`scripts/smoke`, 9/9 checks at MAD 0.000); its web-side adapter/capability negotiation is covered by `WebGPU::create_with_preset` and `Renderer::new`. Desktop-side adapter/capability negotiation moved to Phase 3 on 2026-08-01 and remains unselected.

## Priorities
1. Preserve the WebGPU browser rendering baseline, the `scripts/validate` contract, the Blasterites demo routes, and the DP-0008 frame-api route.
2. Maintain the `WI-SMOKE-0001` smoke check (`scripts/smoke`, done — covers baseline/tester/tuner at 4:3/wide/tall and the four display presets).
3. Treat DP-0008's current typed command-slice path plus the `WI-API-0004` `VectorFrameView` mapping surface as sufficient for the validated Replication Vector representative frame and EV-0011's Blasterites pixel/full-window coordinate pressure. DP-0007's deferred public custom display-settings API, DP-0003 retained scene/material work, and DP-0002 Phase 3/native work remain deferred until separately selected by new consumer evidence.
4. Keep adopted design, work items, roadmap, focus, and evidence aligned with what is merged.

## Non-Goals
- Do not implement full games as part of the rendering work; the Blasterites tester is a validation harness, not a playable game.
- Do not expand the public preset surface beyond DP-0007 v1 (named presets) without a follow-up decision — in particular, keep `render_blasterites_tuner` internal and do not ship a public custom display-settings API until it is designed.
- Do not expand DP-0008 into a full retained scene/material API or game implementation without a follow-up decision and downstream evidence.
- Do not make WebGL2 fallback a milestone unless maintainers explicitly prioritize it later.
- Do not begin native `winit` desktop implementation (Phase 3) until maintainers explicitly select it as its own workstream — Phase 2's selection covers the shared, surface-agnostic renderer state only, not a native frontend.

## Exit Criteria
- DP-0006 is adopted (2026-07-24) with recorded browser/screenshot evidence that Arcade Balanced meets its acceptance target (EV-0009); its remaining follow-ups are represented as tracked work (`WI-SMOKE-0001` done; the public preset API decided by the adopted `DP-0007` and implemented in `WI-PRESET-0001`, which also captured the non-default presets), not open design questions.
- DP-0001, DP-0004, DP-0005, and DP-0006 remain represented as adopted design.
- DP-0008 remains represented as adopted and implemented by `WI-API-0001`, with downstream Replication Vector typed-path validation recorded in EV-0010, view/viewport mapping selected by EV-0011, and `WI-API-0004` resolved as the scoped `VectorFrameView` follow-up.
- The browser demos remain recoverable through `scripts/validate`, `scripts/demos`, and the documented Rust/WASM/Vite baseline.
- DP-0002 Phase 2 is represented as completed for its browser-only shared-renderer scope; DP-0003 and DP-0002 Phase 3+ remain proposed follow-up directions until separately selected.
