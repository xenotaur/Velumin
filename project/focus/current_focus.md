---
id: FOCUS-RENDER-0001
title: Post-WebGPU Modernization Focus
status: active
priority: high
owner: project maintainers
---

# Current Focus

## Active Priority
- Keep the adopted DP-0001 browser/WebGPU baseline stable while selecting the next workstream.

## Why This Appears Current
- The LRH control plane has been bootstrapped and `WI-BOOTSTRAP-0001` is done.
- DP-0001 is adopted and implemented as the current browser/WebGPU rendering baseline.
- DP-0002 defines the next-horizon cross-platform architecture.
- DP-0004 defines the proposed script-first validation workflow for local development and CI.

## Priorities
1. Preserve the current WebGPU browser rendering baseline and documented clean-build commands.
2. Keep DP-0001 adopted design, work items, roadmap, focus, and evidence aligned.
3. Choose whether the next workstream is DP-0004 validation workflow, DP-0002 architecture split, or production glow tuning.
4. Treat glow/composite production tuning as follow-up work separate from the completed spike.

## Non-Goals
- Do not implement full games as part of the rendering modernization work.
- Do not make WebGL2 fallback a milestone unless maintainers explicitly prioritize it later.
- Do not begin native desktop implementation until maintainers explicitly select the DP-0002 workstream.
- Do not treat the glow spike as final production glow behavior.

## Exit Criteria
- DP-0001 remains represented as adopted design rather than active migration work.
- The next active workstream is selected and represented in roadmap, focus, and work items.
- The browser demo remains recoverable through the documented Rust/WASM/Vite baseline.
- DP-0002 and DP-0004 remain represented as proposed follow-up directions until selected.
