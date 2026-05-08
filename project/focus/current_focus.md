---
id: FOCUS-RENDER-0001
title: WebGPU Rendering Modernization Focus
status: active
priority: high
owner: project maintainers
---

# Current Focus

## Active Priority
- Execute the DP-0001 WebGPU-first rendering modernization plan while preserving the browser smoke baseline.

## Why This Appears Current
- The LRH control plane has been bootstrapped and `WI-BOOTSTRAP-0001` is done.
- DP-0001 defines the active path from the current `wgpu 0.16` + WebGL compatibility prototype toward a modern WebGPU-first renderer.
- DP-0002 defines the next-horizon cross-platform architecture, but it depends on stabilizing the browser/WebGPU foundation first.

## Priorities
1. Preserve the current white-line browser rendering baseline with documented clean-build commands.
2. Upgrade the web rendering path to modern WebGPU-first `wgpu` while keeping the smoke output visually equivalent.
3. Introduce a small platform boundary between browser canvas setup and reusable renderer state.
4. Add startup capability handling for unsupported browsers, missing adapters, blocked adapters, and insufficient limits.
5. Introduce minimal line/polyline vector commands with CPU-side thick-line triangle generation and GPU buffer batching.
6. Defer glow/composite work until the modern baseline and primitive path are stable.

## Non-Goals
- Do not implement full games as part of the rendering modernization work.
- Do not make WebGL2 fallback a milestone unless maintainers explicitly prioritize it later.
- Do not begin native desktop implementation until the DP-0001 browser baseline work is complete.
- Do not add glow/bloom as production renderer behavior before the modern WebGPU baseline and thick-line primitive path are reliable.

## Exit Criteria
- Clean checkout build commands are documented and verified for Rust/WASM/Vite development.
- The browser demo shows a visible white line on black after a clean rebuild.
- Browser startup, pipeline creation, render call, and frame presentation have observable validation.
- Unsupported-browser and no-adapter paths show clear user-facing messaging rather than only a blank canvas.
- Core vector rendering no longer depends on GPU line primitives.
- DP-0002 remains represented as the next staged architecture horizon after DP-0001.
