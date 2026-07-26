---
id: FOCUS-RENDER-0001
title: Vector CRT Renderer Focus
status: active
priority: high
owner: project maintainers
---

# Current Focus

## Active Priority
- The DP-0006 renderer follow-ups are done (the WI-SMOKE-0001 smoke check and the DP-0007 public preset API landed). Keep the adopted renderer, demos, and validation workflow stable, and select the next workstream (DP-0007's deferred custom-settings API, or DP-0002 / DP-0003).

## Why This Appears Current
- The LRH control plane is bootstrapped and `WI-BOOTSTRAP-0001` is done.
- DP-0001 (WebGPU baseline) and DP-0004 (validation workflow) are adopted and implemented.
- DP-0005 (Blasterites tester and tuner demos) is adopted and implemented (`WI-DEMO-0001`, merged across PRs #3-#6).
- DP-0006 (Vector CRT renderer migration) was adopted on 2026-07-24 and is partially implemented: the fixed 4:3 viewport, additive multi-layer glow, and a display-preset set have landed alongside the demos, validated by code inspection (EV-0008) and browser visual-smoke capture (EV-0009). Its follow-ups have landed — the `WI-SMOKE-0001` smoke check and the DP-0007 public preset API (`WI-PRESET-0001`).

## Priorities
1. Preserve the WebGPU browser rendering baseline, the `scripts/validate` contract, and the Blasterites demo routes.
2. Maintain the `WI-SMOKE-0001` smoke check (`scripts/smoke`, done — covers baseline/tester/tuner at 4:3/wide/tall and the four display presets).
3. Decide whether to build DP-0007's deferred public custom display-settings API, then select the next workstream (DP-0002 or DP-0003). The public preset API itself is implemented (`WI-PRESET-0001`).
4. Keep adopted design, work items, roadmap, focus, and evidence aligned with what is merged.

## Non-Goals
- Do not implement full games as part of the rendering work; the Blasterites tester is a validation harness, not a playable game.
- Do not expand the public preset surface beyond DP-0007 v1 (named presets) without a follow-up decision — in particular, keep `render_blasterites_tuner` internal and do not ship a public custom display-settings API until it is designed.
- Do not make WebGL2 fallback a milestone unless maintainers explicitly prioritize it later.
- Do not begin native desktop implementation until maintainers explicitly select the DP-0002 workstream.

## Exit Criteria
- DP-0006 is adopted (2026-07-24) with recorded browser/screenshot evidence that Arcade Balanced meets its acceptance target (EV-0009); its remaining follow-ups are represented as tracked work (`WI-SMOKE-0001` done; the public preset API decided by the adopted `DP-0007` and implemented in `WI-PRESET-0001`, which also captured the non-default presets), not open design questions.
- DP-0001, DP-0004, DP-0005, and DP-0006 remain represented as adopted design.
- The browser demos remain recoverable through `scripts/validate`, `scripts/demos`, and the documented Rust/WASM/Vite baseline.
- DP-0002 and DP-0003 remain represented as proposed follow-up directions until selected.
