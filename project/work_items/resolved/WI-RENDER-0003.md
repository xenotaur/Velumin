---
id: WI-RENDER-0003
title: Introduce Platform Boundary and Capability Handling
status: resolved
priority: high
owner: project maintainers
depends_on: WI-RENDER-0002---

# WI-RENDER-0003: Introduce Platform Boundary and Capability Handling

## Objective
- Separate browser-specific setup from reusable renderer state and add explicit WebGPU capability handling.

## Scope
- Keep canvas lookup, DPR calculation, browser logging, and user-facing unsupported-state reporting in the web adapter.
- Introduce renderer state that owns device, queue, surface configuration, pipelines, buffers, textures, and resize behavior.
- Detect missing `navigator.gpu`, no adapter, blocked adapter, insufficient limits/features, and unsupported texture capabilities before renderer construction.
- Add explicit resize and surface reconfiguration behavior.

## Evidence
- DP-0001: `project/design/proposals/DP-0001-modern-webgpu-rendering.md`
- DP-0002: `project/design/proposals/DP-0002-cross-platform-renderer-architecture.md`
- Current combined browser/renderer setup: `webgpu_vector_lib/src/lib.rs`
- Boundary verification: `project/evidence/EV-0004.md`

## Acceptance Criteria
- Browser setup responsibilities are separated from renderer resource ownership.
- Renderer state can be reasoned about as the future shared `wgpu` renderer boundary.
- Unsupported-browser and no-adapter paths show clear user-facing errors instead of leaving only a blank canvas.
- Resize handling updates backing-store dimensions and reconfigures the surface explicitly.
- The browser smoke demo remains visible after the boundary split.

## Status
- Done: browser setup is separated from renderer resource ownership, capability errors are reported to the page, resize reconfigures the surface explicitly, and the WASM/Vite baseline passes.
