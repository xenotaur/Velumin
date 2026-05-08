---
id: WI-RENDER-0005
title: Spike Glow Pipeline After Modern Baseline
status: planned
priority: medium
owner: project maintainers
depends_on: WI-RENDER-0004
---

# WI-RENDER-0005: Spike Glow Pipeline After Modern Baseline

## Objective
- Explore a glow/composite rendering pipeline after modern WebGPU rendering and thick-line primitives are stable.

## Scope
- Render bright vector geometry into an offscreen texture.
- Explore blur and downsample passes for retro vector glow.
- Composite glow with crisp core lines onto the surface.
- Explore retro-display controls such as glow radius, intensity, persistence, and background fade.
- Treat this as a spike, not a production renderer commitment.

## Evidence
- DP-0001: `project/design/proposals/DP-0001-modern-webgpu-rendering.md`
- DP-0002: `project/design/proposals/DP-0002-cross-platform-renderer-architecture.md`
- Primitive work item: `project/work_items/WI-RENDER-0004.md`

## Acceptance Criteria
- A prototype offscreen bright-pass path is demonstrated or clearly documented.
- Blur/downsample and composite approaches are compared enough to choose a production direction later.
- Spike output preserves crisp core geometry while adding visible glow.
- Browser validation records backend, browser, OS, and adapter information where available.
- Follow-up production work is identified separately instead of expanding this spike without review.

## Status
- Planned after `WI-RENDER-0004`.

