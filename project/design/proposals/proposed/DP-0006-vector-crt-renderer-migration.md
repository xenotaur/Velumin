---
id: DP-0006
title: Vector CRT Renderer Migration
status: proposed
owner: project maintainers
created: 2026-05-09
scope: vector renderer, glow, viewport, display presets
depends_on:
  - DP-0001
  - DP-0004
related:
  - DP-0002
  - DP-0003
  - DP-0005
---

# Vector CRT Renderer Migration

## Summary
Migrate Velumin from the current widened-line glow spike to an internal-first Vector CRT display renderer tuned against the Blasterites tester. Preserve existing public WASM entrypoints while adding a fixed 4:3 logical viewport, real light falloff, additive glow compositing, and a small set of named classic-inspired display presets.

The first acceptance target is Arcade Balanced: crisp bright vector cores with soft diffuse bloom on a black field, no window-resize distortion, and no square or blocky glow artifacts.

## Context
Velumin's current renderer has the right foundation for classic vector-display graphics: WebGPU rendering, CPU-generated thick vector primitives, an offscreen glow target, and a deterministic Blasterites tester scene. Recent visual inspection shows three important gaps:

- glow reads as a second, wider copy of the geometry rather than diffuse emitted light;
- glow transparency is low and mostly fixed, so it does not fall off convincingly into black;
- clip-space coordinates are mapped to the full window, so resizing the browser distorts scene geometry.

These issues are directly in scope for Velumin's retro vector-graphics identity and should be handled before broadening into the full DP-0002 crate split or the full DP-0003 scene/material model.

## Decision
Select production Vector CRT rendering as the next focused renderer workstream.

The migration should:

- remain internal-first until the visual model has evidence behind it;
- keep the current browser/WASM public entrypoints stable;
- use Blasterites as the deterministic tuning harness, not as a playable game or public API commitment;
- preserve a centered 4:3 logical playfield by default;
- replace hard widened-line glow with additive emitted light, smooth falloff, and bloom-style compositing;
- define a small internal display preset model for classic-inspired looks.

## Implementation Direction

### Viewport
Use a virtual 800x600, 4:3, center-origin, y-up playfield for the browser demos. Fit that viewport centered into the canvas while preserving aspect ratio. Clear material outside the viewport to black. The initial boundary treatment is plain black letterbox or pillarbox, with no decorative frame.

Apply the same fitted viewport and scissor to the crisp vector pass and glow/emission pass so scene geometry remains stable across browser window sizes.

### Display Settings
Add internal renderer settings rather than a stable public API. The initial preset set should include:

| Preset | Purpose |
| --- | --- |
| `ArcadeBalanced` | Default tuning target: crisp core, soft colored bloom, minimal display artifacts. |
| `MonochromeBeam` | White or cyan beam with restrained color and stronger hot core. |
| `ColorQuadraScan` | More saturated colored bloom for Tempest-like scenes. |
| `CleanNeon` | Smoother modern glow with reduced CRT artifacts. |

The renderer may start by using only `ArcadeBalanced`, but the settings model should make the other looks easy to tune later.

### Glow and Composite
Replace the current single widened glow draw with an emission model:

- keep the crisp vector core as a separate final overlay;
- render glow/emission into an offscreen light target;
- use additive blending so crossings and clusters accumulate light;
- produce falloff through shader distance fields or layered emission geometry;
- composite black background, broad glow, near glow, and crisp core in that order.

A full multi-texture blur chain may land after the first migration slice. The important first step is to stop treating glow as one hard, uniformly transparent wider stroke.

## Validation Direction
Keep validation practical and evidence-backed:

- Rust tests should cover viewport fit math, deterministic Blasterites output, nonzero geometry, and valid display settings.
- `scripts/format --check`, `scripts/lint`, `scripts/test`, and `scripts/baseline` should remain green.
- Manual browser inspection remains required for this phase; do not claim CI visual validation until browser-rendered pixels are actually inspected.

Manual acceptance should inspect `?demo=blasterites` in wide, tall, and exact 4:3 browser sizes. The ship and ring must not stretch, glow should fade smoothly into black, colored strokes should bloom without washing the scene white, and the default baseline render must remain available.

## Acceptance Criteria
- Existing browser entrypoints remain available.
- Blasterites renders in a stable 4:3 centered playfield across browser resize.
- Glow no longer reads as a single square/blocky widened copy of each line.
- Display presets exist internally and include an Arcade Balanced default.
- Build-level validation remains green.
- The project does not claim production visual validation beyond the checks actually performed.

## Risks
- A too-general public API could slow down visual tuning before the look is proven.
- A full bloom chain could create more texture and pass complexity than this early renderer needs.
- Browser visual smoke may remain flaky until the automation environment is stable.
- Presets could imply exact emulation of specific commercial arcade hardware; they should remain classic-inspired tuning labels, not affiliation or accuracy claims.

## Guardrails
- Keep vector emulation as the first concrete style.
- Keep the migration narrow and browser-baseline preserving.
- Do not begin the full DP-0002 crate split as part of this workstream.
- Do not promote internal settings to public API until maintainers choose an API stabilization work item.
- Credit Asteroids, Tempest, Star Castle, and similar games only as aesthetic inspiration.

