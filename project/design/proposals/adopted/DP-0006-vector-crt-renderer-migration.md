---
id: DP-0006
title: Vector CRT Renderer Migration
status: adopted
adopted: 2026-07-24
implementation_status: partial
owner: project maintainers
created: 2026-05-09
implemented_by:
  - WI-DEMO-0001
evidence:
  - EV-0008
  - EV-0009
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

## Adoption and Implementation Status (2026-07-24)
Adopted as the project's vector-display rendering direction on 2026-07-24, on the strength of the EV-0009 visual-smoke capture. Adoption is `partial`: the core is implemented and validated, and the remaining items below are follow-up work under the adopted direction, not open design questions.

Partially implemented and merged alongside the DP-0005 demos. The merged renderer already provides a fixed 4:3 centered viewport (`RenderViewport::centered_4_3`, applied to both the glow and surface passes, with a unit test), multi-layer additive glow compositing, and an internal `VectorDisplayPreset` set including the `ArcadeBalanced` tuning target. Public WASM entrypoints from DP-0001 remain stable. See `project/evidence/EV-0008.md` (code inspection) and `project/evidence/EV-0009.md` (browser visual-smoke capture).

Browser visual smoke has been captured (`EV-0009`): the default Arcade-Balanced-style output renders crisp cores with soft additive bloom on a black field and no blocky glow artifacts, at deterministic tester frames. This satisfied the glow/bloom and exact-4:3 portion of the proposal's manual-inspection validation, and the maintainers adopted the direction on that basis. EV-0009 itself used a fixed 800x600 (4:3) canvas and did not exercise the non-4:3 letterbox/pillarbox path — but the subsequent `WI-SMOKE-0001` smoke check (`scripts/smoke`) now captures the scenes at wide and tall sizes as well, covering the "wide, tall, and exact 4:3" resize inspection required by the Validation Direction below (with the `centered_viewport_preserves_four_by_three_aspect` unit test as the GPU-free guard).

Follow-up under this adopted direction: the public-preset-API question is settled by the adopted `DP-0007`, and implementing it plus capturing the non-default presets is tracked in `WI-PRESET-0001`. The automatable/committed screenshot smoke check has landed (`WI-SMOKE-0001`, `scripts/smoke`).

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

