---
id: FOCUS-RENDER-0001
title: Vector CRT Renderer Focus
status: active
priority: high
owner: project maintainers
---

# Current Focus

## Active Priority
- Advance the DP-0006 Vector CRT renderer to adoption while keeping the adopted DP-0001 baseline, DP-0004 validation workflow, and DP-0005 Blasterites demos stable.

## Why This Appears Current
- The LRH control plane is bootstrapped and `WI-BOOTSTRAP-0001` is done.
- DP-0001 (WebGPU baseline) and DP-0004 (validation workflow) are adopted and implemented.
- DP-0005 (Blasterites tester and tuner demos) is adopted and implemented (`WI-DEMO-0001`, merged across PRs #3-#6).
- DP-0006 (Vector CRT renderer migration) is partially implemented: the fixed 4:3 viewport, additive multi-layer glow, and an internal display-preset set have landed alongside the demos, but the visual model still needs recorded browser evidence before adoption.

## Priorities
1. Preserve the WebGPU browser rendering baseline, the `scripts/validate` contract, and the Blasterites demo routes.
2. Record browser/screenshot visual evidence for the Arcade Balanced target (`/?demo=blasterites`, `/?demo=tuner`) to close the DP-0006 evidence gap.
3. Decide whether the internal `VectorDisplayPreset` set becomes a selectable public API, then drive DP-0006 to adoption.
4. Keep adopted design, work items, roadmap, focus, and evidence aligned with what is merged.

## Non-Goals
- Do not implement full games as part of the rendering work; the Blasterites tester is a validation harness, not a playable game.
- Do not promote the internal display-preset set to a stable public API before DP-0006 has visual evidence.
- Do not make WebGL2 fallback a milestone unless maintainers explicitly prioritize it later.
- Do not begin native desktop implementation until maintainers explicitly select the DP-0002 workstream.

## Exit Criteria
- DP-0006 has recorded browser/screenshot evidence that Arcade Balanced meets its acceptance target, and is either adopted or has an explicit remaining-work list.
- DP-0001, DP-0004, and DP-0005 remain represented as adopted, implemented design.
- The browser demos remain recoverable through `scripts/validate`, `scripts/demos`, and the documented Rust/WASM/Vite baseline.
- DP-0002 and DP-0003 remain represented as proposed follow-up directions until selected.
