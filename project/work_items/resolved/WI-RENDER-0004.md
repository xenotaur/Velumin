---
id: WI-RENDER-0004
title: Add Minimal Vector Primitive API and Thick-Line Tessellation
status: resolved
priority: high
owner: project maintainers
depends_on: WI-RENDER-0003---

# WI-RENDER-0004: Add Minimal Vector Primitive API and Thick-Line Tessellation

## Objective
- Introduce the first minimal vector primitive representation and render thick vector strokes as triangles rather than GPU line primitives.

## Scope
- Add a minimal command representation for `Line` and `Polyline`-style primitives with start/end or point data, width, color, and intensity.
- Generate thick-line geometry on the CPU as triangles suitable for predictable browser rendering.
- Batch generated primitive geometry into GPU buffers instead of hardcoding core geometry in WGSL.
- Keep shader-generated geometry limited to smoke tests or fullscreen/pass-specific work.

## Evidence
- DP-0001: `project/design/proposals/adopted/DP-0001-modern-webgpu-rendering.md`
- DP-0002: `project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md`
- Current shader-generated smoke geometry: `webgpu_vector_lib/shaders/line.wgsl`
- Primitive verification: `project/evidence/EV-0005.md`

## Acceptance Criteria
- A minimal line/polyline command path exists for renderer input.
- Thick vector strokes are produced as triangle geometry.
- Core vector rendering does not rely on GPU line primitives.
- Primitive geometry is batched into buffers before rendering.
- The browser demo can render the preserved smoke line through the primitive path.

## Status
- Done: line/polyline commands exist, thick lines tessellate to triangles on the CPU, geometry is uploaded through a vertex buffer, and the browser smoke scene renders through the primitive path.
