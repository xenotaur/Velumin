---
id: DP-0005
title: Blasterites Tester Demo and Visual Smoke
status: proposed
owner: project maintainers
created: 2026-05-08
scope: browser example, vector effects validation, visual smoke
depends_on:
  - DP-0001
  - DP-0004
related:
  - DP-0002
  - DP-0003
---

# Blasterites Tester Demo and Visual Smoke

## Summary
Add a small Blasterites-inspired tester to Velumin as a browser/WASM demo plus lightweight smoke validation. The tester should show an animated vector scene with a rotating arrow ship, fired bullet, approaching asteroid, spark explosion, raster glow, scanline treatment, and subtle pulse/wobble.

The tester is not a playable game. It is a deterministic visual harness that exercises the current WebGPU vector renderer and gives future renderer work a richer scene than the single-line baseline.

## Context
Velumin's current browser baseline renders a white vector line on a black canvas through modern WebGPU. DP-0001 established thick-line tessellation and a prototype glow/composite path. DP-0004 established repository scripts as the local and CI validation contract.

That baseline is intentionally small, but it does not exercise the kinds of shapes and effects that Velumin is meant to support for Asteroids-like games. A deterministic Blasterites-style tester gives the project a compact example that validates shape composition, animation timing, glow, post-processing, and particle-like effects without expanding the project into game logic.

## Decision
Create a deterministic browser demo path selected by query parameter, for example `?demo=blasterites`, while preserving the existing `WebGPU::render()` white-line baseline.

The first tester implementation should:

- render a closed vector ship outline;
- rotate the ship based on elapsed time;
- fire a point-like bullet along the ship direction;
- render an irregular closed asteroid polyline approaching the bullet;
- switch into a spark explosion after the deterministic impact time;
- reuse the current glow pass for bright vector geometry;
- apply tester-only scanline/post-process treatment;
- add subtle time-based pulse/wobble through generated geometry and style values.

## Goals
- Provide a richer visual regression target than the single-line baseline.
- Keep the demo deterministic so tests and screenshots can target known timestamps.
- Exercise vector polylines, closed shapes, point-like effects, glow, and fullscreen composition.
- Keep the implementation inside the current browser/WASM package until DP-0002's crate split is selected.
- Make the tester useful for manual inspection now and automated visual smoke later.

## Non-Goals
- Do not implement a playable Blasterites or Asteroids clone.
- Do not copy external game assets.
- Do not require DP-0002 or DP-0003 before the first tester lands.
- Do not make the tester the only browser baseline.
- Do not block `scripts/validate` on browser automation until that path is reliable in CI.

## Implementation Direction
Extend the WASM browser API conservatively:

- keep `WebGPU::render()` rendering the current white-line baseline;
- add `WebGPU::render_blasterites_tester(time_ms: f64) -> Result<(), JsValue>`;
- update the browser harness to render the tester when the page has `?demo=blasterites`;
- keep the default page on the existing baseline path.

Generate the tester scene in Rust:

- build frame data from `time_ms`, with no random runtime state;
- represent ship, asteroid, bullet, and sparks with `VectorCommand::Line` and `VectorCommand::Polyline`;
- use normalized clip-space coordinates initially;
- approximate point effects with small crosses or short vector strokes until richer geometry exists.

Limit rendering changes to what the tester needs:

- reuse the current glow bright-pass and composite flow for bright objects;
- add a tester-only fullscreen composite shader or equivalent gated path for scanlines and mild raster treatment;
- drive pulse/wobble by varying generated geometry and intensity before upload.

## Validation Direction
The first automated validation should be CPU-side and build-level:

- Rust unit tests should verify deterministic scene output at key timestamps.
- Rust unit tests should verify the ship outline is closed.
- Rust unit tests should verify bullet commands exist before impact and spark commands exist after impact.
- Rust unit tests should verify generated commands do not produce zero-length segments.
- `scripts/baseline` should continue to prove the Rust/WASM/Vite build path.

Browser visual smoke should be added as a later hard gate after the browser automation environment is stable. The intended visual smoke target is a deterministic timestamp after impact that checks for a nonblank canvas and expected bright pixels near the explosion.

## Acceptance Criteria
- The default browser page still renders the original white-line baseline.
- `?demo=blasterites` renders the animated tester scene.
- The tester scene includes ship, bullet, asteroid, explosion sparks, glow, scanlines, and subtle pulse/wobble.
- The scene is deterministic from `time_ms`.
- Rust tests cover the deterministic scene generator.
- The Rust/WASM/Vite baseline build remains green.

## Risks
- The tester could become a game instead of a renderer validation harness.
- Fullscreen post-processing could accidentally change the baseline if not gated.
- Browser visual smoke could be flaky if added to CI before the environment is ready.
- Adding richer geometry before DP-0003 could create APIs that need to be reshaped later.

## Guardrails
- Keep the tester compact and deterministic.
- Prefer generated vector commands over new public API commitments.
- Keep the white-line baseline available.
- Add browser visual smoke as an optional script before making it part of `scripts/validate`.
