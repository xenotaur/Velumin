---
id: WI-DEMO-0001
title: Blasterites Tester and Tuner Demos
type: deliverable
status: resolved
priority: high
owner: project maintainers
depends_on:
  - WI-RENDER-0005
related_design:
  - project/design/proposals/adopted/DP-0005-blasterites-tester-demo-and-visual-smoke.md
  - project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md
blocked: false
blocked_reason: null
resolution: Implemented and merged the deterministic Blasterites tester and live tuner browser demos (PRs #3-#6); verified against the merged renderer code in EV-0008.
---

# WI-DEMO-0001: Blasterites Tester and Tuner Demos

## Objective
- Add a deterministic Blasterites-inspired tester and a live tuner as browser/WASM demos on top of the DP-0001 WebGPU baseline, per DP-0005.

## Scope
- Add query-parameter demo routing (`?demo=blasterites`, `?demo=tuner`) while preserving the `WebGPU::render()` white-line baseline.
- Render a deterministic animated vector scene: rotating ship, fired bullet, approaching asteroid, spark explosion, glow, scanline/composite treatment, and subtle pulse/wobble driven by elapsed time.
- Expose a live tuner UI that drives glow-layer and stroke-width parameters through a `render_blasterites_tuner` entrypoint.
- Add `scripts/demos` to build the WASM package and serve the demo routes through Vite.

## Evidence
- DP-0005: `project/design/proposals/adopted/DP-0005-blasterites-tester-demo-and-visual-smoke.md`
- DP-0006: `project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md`
- Demo entrypoints: `webgpu_vector_lib/src/lib.rs` (`render_blasterites_tester`, `render_blasterites_tuner`)
- Browser harness and routing: `webgpu_vector_lib/web/index.html`
- Demo launch script: `scripts/demos`
- Verification: `project/evidence/EV-0008.md` (code inspection), `project/evidence/EV-0009.md` (browser visual-smoke capture)

## Acceptance Criteria
- The baseline `/` route still renders the preserved white-line smoke scene.
- `/?demo=blasterites` renders the deterministic tester scene.
- `/?demo=tuner` renders the tester with live glow/stroke controls.
- The tester is deterministic from elapsed time so future smoke checks can target known animation moments.
- `scripts/demos` builds the WASM package and starts the Vite demo server.

## Status
- Done: the tester and tuner demos, query-parameter routing, and `scripts/demos` are implemented and merged across PRs #3-#6; the demo entrypoints and routing are present in the merged renderer and harness. Browser visual-smoke evidence for the demo scenes was subsequently captured in EV-0009 (baseline, tester at deterministic pre-/post-impact frames, and tuner).
