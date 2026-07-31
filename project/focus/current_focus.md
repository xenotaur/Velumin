---
id: FOCUS-RENDER-0001
title: Vector Renderer & Cross-Platform Architecture Focus
status: active
priority: high
owner: project maintainers
---

# Current Focus

## Active Priority
- The DP-0006 renderer follow-ups are done (the WI-SMOKE-0001 smoke check and the DP-0007 public preset API landed). Keep the adopted renderer, demos, and validation workflow stable. DP-0002 Phase 2's reusable, surface-agnostic `wgpu` renderer state is done (`WI-ARCH-0002`, PR #20); desktop-side adapter/capability negotiation (the rest of Phase 2) has no work item scoped yet.

## Why This Appears Current
- The LRH control plane is bootstrapped and `WI-BOOTSTRAP-0001` is done.
- DP-0001 (WebGPU baseline) and DP-0004 (validation workflow) are adopted and implemented.
- DP-0005 (Blasterites tester and tuner demos) is adopted and implemented (`WI-DEMO-0001`, merged across PRs #3-#6).
- DP-0006 (Vector CRT renderer migration) was adopted on 2026-07-24 and is partially implemented: the fixed 4:3 viewport, additive multi-layer glow, and a display-preset set have landed alongside the demos, validated by code inspection (EV-0008) and browser visual-smoke capture (EV-0009). Its follow-ups have landed — the `WI-SMOKE-0001` smoke check and the DP-0007 public preset API (`WI-PRESET-0001`).
- DP-0002's Phase 1 platform-neutral type extraction landed: `WI-ARCH-0001` (PR #17) converted the repository into a Cargo workspace with a `velumin-core` crate, with zero visual regression (`scripts/smoke`, 9/9 checks at MAD 0.000). Phase 2's reusable, surface-agnostic `wgpu` renderer state also landed: `WI-ARCH-0002` (PR #20) made `Renderer` host-buildable, with zero visual regression (`scripts/smoke`, 9/9 checks at MAD 0.000). Desktop-side adapter/capability negotiation, the rest of Phase 2, has no work item scoped yet.

## Priorities
1. Preserve the WebGPU browser rendering baseline, the `scripts/validate` contract, and the Blasterites demo routes.
2. Maintain the `WI-SMOKE-0001` smoke check (`scripts/smoke`, done — covers baseline/tester/tuner at 4:3/wide/tall and the four display presets).
3. Decide whether to build DP-0007's deferred public custom display-settings API (the public preset API itself is implemented, `WI-PRESET-0001`). `WI-ARCH-0002` (DP-0002 Phase 2's first slice) is implemented; desktop-side adapter/capability negotiation (Phase 2's remaining bullet) and Phase 3 (native `winit` shell) still await their own work item / selection.
4. Keep adopted design, work items, roadmap, focus, and evidence aligned with what is merged.

## Non-Goals
- Do not implement full games as part of the rendering work; the Blasterites tester is a validation harness, not a playable game.
- Do not expand the public preset surface beyond DP-0007 v1 (named presets) without a follow-up decision — in particular, keep `render_blasterites_tuner` internal and do not ship a public custom display-settings API until it is designed.
- Do not make WebGL2 fallback a milestone unless maintainers explicitly prioritize it later.
- Do not begin native `winit` desktop implementation (Phase 3) until maintainers explicitly select it as its own workstream — Phase 2's selection covers the shared, surface-agnostic renderer state only, not a native frontend.

## Exit Criteria
- DP-0006 is adopted (2026-07-24) with recorded browser/screenshot evidence that Arcade Balanced meets its acceptance target (EV-0009); its remaining follow-ups are represented as tracked work (`WI-SMOKE-0001` done; the public preset API decided by the adopted `DP-0007` and implemented in `WI-PRESET-0001`, which also captured the non-default presets), not open design questions.
- DP-0001, DP-0004, DP-0005, and DP-0006 remain represented as adopted design.
- The browser demos remain recoverable through `scripts/validate`, `scripts/demos`, and the documented Rust/WASM/Vite baseline.
- DP-0002 Phase 2 is represented as the selected active workstream (2026-07-31); DP-0003 and DP-0002 Phase 3+ remain proposed follow-up directions until separately selected.
