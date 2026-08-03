---
id: WI-API-0001
title: Expose Browser Vector Frame Submission API
type: deliverable
status: proposed
priority: high
owner: project maintainers
contributors:
  - project maintainers
assigned_agents: []
related_focus:
  - FOCUS-RENDER-0001
related_roadmap:
  - ROADMAP-CORE
related_workstreams: []
related_design:
  - project/design/proposals/proposed/browser-vector-frame-api/00_proposal.md
  - project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md
  - project/design/proposals/proposed/DP-0003-extensible-2d-scene-material-model.md
  - project/design/proposals/adopted/DP-0005-blasterites-tester-demo-and-visual-smoke.md
  - project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md
  - project/design/proposals/adopted/DP-0007-display-preset-public-api.md
depends_on: []
blocked_by: []
blocked: false
blocked_reason: null
resolution: null
expected_actions:
  - edit_file
  - run_tests
  - write_docs
  - create_pr
forbidden_actions:
  - force_push
  - delete_branch
  - merge_pr
  - implement_native_winit
  - implement_full_game
  - implement_custom_display_settings
  - request_paid_github_review_retrigger
acceptance:
  - Browser JavaScript can create a VectorFrame, append line/polyline/closed-polyline commands, and render it through WebGPU.renderFrame.
  - The implementation reuses velumin-core VectorCommand types and the existing WebGPU renderer path without replacing existing demo entrypoints.
  - A deterministic public-frame harness demonstrates Replication Vector or Blasterites-style geometry without implementing a playable game.
  - README or web-facing docs describe the public API, coordinate system, style parameters, and display-preset interaction.
  - scripts/format --check, scripts/lint, scripts/test, scripts/baseline, and lrh validate pass.
required_evidence:
  - manual_review
  - lrh_validate
  - test_output
  - validation_output
artifacts_expected:
  - webgpu_vector_lib/src/lib.rs
  - webgpu_vector_lib/web/index.html
  - README.md
  - project/work_items/proposed/WI-API-0001.md
---

# WI-API-0001: Expose Browser Vector Frame Submission API

## Summary

Implement the first public browser-facing vector frame submission API described by DP-0008, allowing JavaScript games to build a `VectorFrame` and render it through the existing WebGPU renderer.

## Problem / Context

DP-0008 proposes Velumin's first public game-facing drawing surface: a browser-first immediate-frame API for line, polyline, and closed-polyline vector geometry. The renderer already consumes `VectorCommand` slices internally, and `velumin-core` already contains the platform-neutral command and style types, but browser consumers cannot yet submit their own frame data.

This item is the first bounded implementation slice for DP-0008. It should unlock Replication Vector / Blasterites-style harness frames without adopting the full DP-0003 scene/material model, without starting native `winit` work, and without turning Velumin into a game implementation.

### Duplication search
- In-repo: Related implementation exists, but no duplicate public API was found. `velumin-core` already defines `VectorCommand` data, and `webgpu_vector_lib` internally renders command slices; DP-0008 defines the missing public browser submission API.
- Sibling repos: Potential consumers exist at `/Users/centaur/Workspace/JavascriptGames/javascript_games/blasterites` and `/Users/centaur/Workspace/ReplicationVector`, but they are not Velumin API implementations.
- External libraries: Browser/game rendering libraries such as PixiJS, Phaser, and Three.js exist, but adopting one would not satisfy Velumin's Rust/WASM/WebGPU vector-renderer direction.
- Recommendation: Proceed by exposing the existing Velumin command path through a narrow public browser API.

### Demand search
- Work items: No existing `WI-API-0001` or overlapping proposed work item found.
- Proposals: Found direct demand in `project/design/proposals/proposed/browser-vector-frame-api/00_proposal.md` (`DP-0008`), which names `WI-API-0001` as the first implementation slice.
- Backlog: No separate matching backlog entry found.
- Recommendation: Link this work item to DP-0008; no close/link action needed for an existing work item.

## Scope

- Add a browser/WASM-facing `VectorFrame` builder for immediate-frame vector command submission.
- Add a `WebGPU.renderFrame` / Rust `render_frame` entrypoint that renders a submitted frame through the existing renderer.
- Keep existing browser entrypoints and demo routes stable.
- Add at least one deterministic public-frame harness that exercises the new API with Replication Vector or Blasterites-style geometry.
- Document the API and validation path.

## Required Changes

1. Update `webgpu_vector_lib/src/lib.rs` with a `wasm-bindgen`-exported `VectorFrame` type.
   - Owns a `Vec<VectorCommand>`.
   - Provides `new`, `clear`, `len`, `is_empty`, `line`, `polyline`, and `closed_polyline` methods.
   - Converts JavaScript typed-array point data into `Vec<Vec2>`.
   - Rejects malformed arrays and non-finite numeric values with clear `JsValue` errors.
   - Repeats the first point for `closed_polyline` when needed.

2. Add `WebGPU::render_frame(&mut self, frame: &VectorFrame) -> Result<(), JsValue>`.
   - Resizes the canvas using the existing browser path.
   - Calls the existing renderer with the frame command slice.
   - Uses the normal Vector CRT path rather than tester-only scanline treatment.

3. Update the browser harness in `webgpu_vector_lib/web/index.html`.
   - Preserve `/`, `?demo=blasterites`, and `?demo=tuner`.
   - Add a deterministic public-frame route, such as `?demo=frame-api`.
   - Build the demo frame from the public JS API rather than directly calling built-in Rust scene generation.

4. Add documentation.
   - Update `README.md` or equivalent web-facing docs with minimal JavaScript usage.
   - Document coordinate convention, color channel range, alpha, stroke width, intensity, and interaction with `VectorDisplayPreset`.
   - Make clear this is a v1 immediate-frame API and not a retained scene graph.

5. Add tests and validation coverage.
   - Unit-test `VectorFrame` command construction where host-compatible.
   - Test malformed typed-array length and non-finite input behavior where feasible.
   - Test closed-polyline behavior.
   - Preserve existing renderer, preset, smoke, and demo behavior.

6. Follow the review plan during execution.
   - Use a fresh independent self-review sub-agent before merge or when review-budget gates would otherwise trigger another GitHub review round.
   - Do not request extra paid GitHub review retriggers beyond the automatic first PR review.

## Non-Goals

- Do not implement Replication Vector, Blasterites, Asteroids, Star Castle, or any playable game.
- Do not add the full DP-0003 scene/material/layer/mask model.
- Do not add filled polygons, sprites, texture atlases, cel animation, watercolor materials, pixel-art layers, or a retained scene graph.
- Do not add native desktop, `winit`, Steam packaging, or Bevy integration.
- Do not add WebGL2 fallback.
- Do not expose internal preset glow-layer constants or create a public custom display-settings API.
- Do not remove or break existing browser entrypoints: `render`, `render_blasterites_tester`, `render_blasterites_tuner`, `create`, `createWithPreset`, or `setDisplayPreset`.

## Acceptance Criteria

- Browser JavaScript can create a `VectorFrame`, append line/polyline/closed-polyline commands, and render it through `WebGPU.renderFrame`.
- The implementation reuses `velumin-core` `VectorCommand` types and the existing WebGPU renderer path.
- The API supports ships, ring/obstacle outlines, blaster bolts, and spark/explosion clusters without game-specific concepts.
- Existing demo entrypoints continue to work.
- The coordinate system and style parameters are documented.
- Deterministic public-frame harness output exists for at least one Replication Vector / Blasterites-style scene.
- Local validation remains green.
- The implementation PR uses self-review with a fresh independent sub-agent instead of extra paid GitHub review retriggers.

## Validation

- `scripts/version`
- `scripts/format --check`
- `scripts/lint`
- `scripts/test`
- `scripts/baseline`
- `lrh validate`
- `scripts/smoke` on a WebGPU-capable environment if the public-frame harness is added to visual smoke coverage

## Risk Notes

- The JS API method names and parameter order become public surface; keep v1 small and avoid speculative helpers.
- Typed-array conversion and validation need careful error handling so browser consumers get actionable failures rather than panics.
- Immediate-frame submission may need batching or retained buffers later; defer until real game workloads justify that complexity.
- Public-frame harnesses must remain demos/validation scenes, not playable games.
- Browser visual validation can vary by GPU; do not overstate visual evidence beyond checks actually run.

## Related Workstream and Designs

- Focus: `project/focus/current_focus.md`
- Roadmap: `project/roadmap/roadmap.md`
- Governing proposal: `project/design/proposals/proposed/browser-vector-frame-api/00_proposal.md`
- Architecture context: `project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md`
- Future scene model context: `project/design/proposals/proposed/DP-0003-extensible-2d-scene-material-model.md`
- Renderer direction: `project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md`
- Display API context: `project/design/proposals/adopted/DP-0007-display-preset-public-api.md`
