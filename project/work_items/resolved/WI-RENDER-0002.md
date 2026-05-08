---
id: WI-RENDER-0002
title: Upgrade to Modern WebGPU-First wgpu
status: resolved
priority: high
owner: project maintainers
depends_on: WI-RENDER-0001---

# WI-RENDER-0002: Upgrade to Modern WebGPU-First `wgpu`

## Objective
- Upgrade Velumin away from `wgpu = "0.16"` with WebGL compatibility as the primary path and onto a modern WebGPU-first `wgpu` dependency line.

## Scope
- Update `wgpu` and related WASM/browser dependencies only as required by the modern API.
- Prefer native browser WebGPU features over the `webgl` compatibility feature.
- Update surface creation, device descriptors, limits, shader module setup, render pass descriptors, and presentation code as needed.
- Keep the smoke demo visually equivalent to the preserved baseline.

## Evidence
- DP-0001: `project/design/proposals/DP-0001-modern-webgpu-rendering.md`
- Current dependency declaration: `webgpu_vector_lib/Cargo.toml`
- Current renderer setup: `webgpu_vector_lib/src/lib.rs`
- Baseline work item: `project/work_items/WI-RENDER-0001.md`
- Upgrade evidence: `project/evidence/EV-0003.md`

## Acceptance Criteria
- `Cargo.toml` and `Cargo.lock` reflect the upgraded dependency set.
- The primary web path no longer depends on `wgpu 0.16` + `webgl` as the main rendering strategy.
- Clean Rust/WASM/Vite build commands from `WI-RENDER-0001` still pass.
- The browser smoke demo remains visually equivalent to the preserved white-line baseline.
- Browser console output reaches setup, pipeline creation, render call, and frame presentation without errors.

## Status
- Done: `wgpu` is upgraded to `29.0.3`, the `webgl` feature is no longer used as the primary path, the WASM/Vite baseline passes, and browser logs show the `BrowserWebGpu` backend.
