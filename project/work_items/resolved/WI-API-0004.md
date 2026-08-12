---
id: WI-API-0004
title: Expose Browser Frame View Mapping
type: deliverable
status: resolved
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
  - project/design/proposals/adopted/browser-vector-frame-api/00_proposal.md
  - project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md
  - project/design/proposals/adopted/DP-0007-display-preset-public-api.md
  - project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md
  - project/design/proposals/proposed/DP-0003-extensible-2d-scene-material-model.md
depends_on:
  - WI-API-0003
blocked_by: []
blocked: false
blocked_reason: null
resolution: "Implemented and merged in PR #33 (commit 7f8cedc63af79292682b10042918e5b8b47e57f7): public VectorFrameView mapping for centered 4:3, logical extents, and canvas-pixel frame rendering."
expected_actions:
  - edit_file
  - run_tests
  - write_docs
  - create_pr
forbidden_actions:
  - force_push
  - delete_branch
  - merge_pr
  - implement_full_game
  - implement_game_wrapping_policy
  - implement_custom_display_settings
  - implement_retained_scene_model
  - implement_native_winit
  - introduce_alternate_renderer
  - request_paid_github_review_retrigger
acceptance:
  - Browser JavaScript can choose a public view or coordinate mapping for VectorFrame submission without rewriting game simulation coordinates into Velumin's default playfield by hand.
  - Rust/WASM consumers rendering owned &[VectorCommand] data can use the same view or coordinate mapping path as VectorFrame submission.
  - The implementation preserves the existing centered 4:3 default behavior for existing demos and public entrypoints unless a consumer opts into the new mapping.
  - A deterministic browser harness proves a Blasterites-style or Replication Vector-style frame can render from a non-default coordinate model, such as full-canvas pixel-like coordinates or explicit logical extents.
  - README or equivalent docs describe the default coordinate model, the new mapping options, and the separation between view mapping and gameplay policy.
  - scripts/format --check, scripts/lint, scripts/test, scripts/baseline, lrh validate, and relevant browser smoke or screenshot evidence pass.
required_evidence:
  - manual_review
  - lrh_validate
  - test_output
  - validation_output
  - visual_smoke_or_screenshot
artifacts_expected:
  - webgpu_vector_lib/src/lib.rs
  - webgpu_vector_lib/web/index.html
  - README.md
  - project/work_items/resolved/WI-API-0004.md
---

# WI-API-0004: Expose Browser Frame View Mapping

## Summary

Implement the narrow DP-0008 follow-up selected by EV-0011: a public browser frame view/coordinate mapping surface that lets games submit `VectorFrame` or typed `&[VectorCommand]` data from non-default coordinate systems without ad hoc per-game conversion.

## Problem / Context

`WI-API-0001` exposed DP-0008's first immediate-frame API, and `WI-API-0002` validated that Replication Vector's current typed `VectorCommand` frame can render through Velumin without a new API. `WI-API-0003` then recorded EV-0011, which found the next concrete consumer pressure in Blasterites-style browser games: existing simulation and drawing often live in full-window or canvas-pixel coordinates, while DP-0008 v1 documents Velumin's fixed centered 4:3 logical playfield. This item should implement that selected DP-0008 extension while preserving DP-0002's split: game code owns simulation and wrapping, the renderer owns GPU resources and render-time mapping, and the frontend owns browser lifecycle.

### Duplication search
- In-repo: Related implementation exists in `WI-API-0001`, `WebGPU::render_commands(&[VectorCommand])`, `VectorFrame`, `RenderViewport::centered_4_3`, and the current smoke/demo harnesses, but no existing proposed work item implements public frame view or coordinate mapping.
- Sibling repos: Related consumer pressure exists in `JavascriptGames/javascript_games` Blasterites and Replication Vector-style scenes, but those are consumers, not Velumin API implementations.
- External libraries: Game engines and graphics libraries provide cameras/viewports, but adopting one would not satisfy Velumin's Rust/WASM/WebGPU vector-renderer public API direction.
- Recommendation: Proceed with a narrow Velumin DP-0008 extension rather than adopting an external renderer or implementing game-specific adapters.

### Demand search
- Work items: No proposed `WI-API-0004` or overlapping proposed work item found.
- Proposals: DP-0008 explicitly deferred camera transforms, arbitrary virtual resolutions, and per-frame view controls; EV-0011 selects that deferred area as the next consumer-driven public surface.
- Backlog: No separate matching backlog entry found during the project search.
- Recommendation: Link this item to DP-0008, EV-0011, and current focus/status; no close/link action needed.

## Scope

- Add a small public browser/WASM API for selecting how submitted frame coordinates map into the render viewport.
- Support both JavaScript `VectorFrame` submission and Rust/WASM typed `&[VectorCommand]` submission through the same internal render path.
- Preserve the existing default centered 4:3 behavior for current demos and consumers.
- Validate with a deterministic harness frame that uses a non-default coordinate model.

## Required Changes

1. Inspect the current `RenderViewport`, `Renderer::render`, `WebGPU::render_frame`, and `WebGPU::render_commands` paths to identify the smallest shared mapping boundary.
2. Design and implement a public mapping API that covers at least one explicit non-default consumer mode, such as full-canvas pixel-like coordinates or explicit logical extents, while retaining the default centered 4:3 mapping.
3. Ensure JavaScript `VectorFrame` rendering and Rust/WASM `&[VectorCommand]` rendering can both use the new mapping without duplicating conversion logic.
4. Preserve existing public entrypoints and demo routes: `/`, `/?demo=blasterites`, `/?demo=tuner`, and `/?demo=frame-api`.
5. Add or update deterministic browser harness coverage proving the same Blasterites-style or Replication Vector-style vector frame can render correctly from a non-default coordinate model.
6. Update README or equivalent public docs with the default coordinate model, the new mapping API, examples, and the boundary between render-time mapping and game-owned simulation policy.
7. Follow the review plan during execution: use fresh independent self-review before merge and avoid paid GitHub review retriggers beyond the automatic first PR review.

## Non-Goals

- Do not implement Replication Vector, Blasterites, Asteroids, Star Castle, or any playable game.
- Do not implement screen wrapping, camera following, input, collision, physics, gameplay lifecycle, or object ownership.
- Do not add DP-0007 public custom display settings or expose numeric glow-layer constants.
- Do not adopt or implement DP-0003 retained scene/material/layer/mask APIs.
- Do not add filled polygons, sprites, texture atlases, pixel-art layers, or non-vector materials.
- Do not add native desktop, `winit`, Steam packaging, Bevy integration, or WebGL2 fallback.
- Do not break existing centered 4:3 behavior or existing public browser entrypoints.

## Acceptance Criteria

- Browser JavaScript can opt into a public view or coordinate mapping for `VectorFrame` submission without rewriting simulation coordinates into Velumin's default centered playfield by hand.
- Rust/WASM consumers rendering owned `&[VectorCommand]` data can use the same view or coordinate mapping path.
- Existing default rendering behavior remains stable for current demos and consumers.
- A deterministic harness demonstrates non-default coordinate mapping with representative vector geometry: ship/probe outline, asteroid or obstacle outline, shield/ring arc or polyline, projectile line, and spark/debris marks.
- Public docs explain the default coordinate model, the mapping options, and why wrapping/camera-follow/gameplay remain consumer-owned.
- Local validation remains green.
- Browser visual smoke or explicit screenshot/pixel evidence proves the new harness renders visible geometry rather than only constructing commands.
- The implementation PR uses self-review with a fresh independent sub-agent instead of extra paid GitHub review retriggers.

## Validation

- `scripts/version tools`
- `scripts/format --check --diff`
- `scripts/lint`
- `scripts/test`
- `scripts/baseline`
- `lrh validate`
- `scripts/smoke` on a WebGPU-capable environment covering the new non-default mapping harness, or an explicit manual screenshot/pixel evidence artifact when smoke automation is unavailable

## Risk Notes

- Coordinate mapping can easily drift into game-camera or wrapping policy. Keep the API about render-time coordinate transforms only.
- A public mapping type or method becomes API surface; keep the first version small, named, and compatible with both JS and Rust/WASM paths.
- The implementation must not silently change the existing 4:3 default, because current smoke references and consumers rely on it.
- Browser visual evidence may vary by GPU; record exactly which smoke or screenshot validation ran.

## Related Workstream and Designs

- Focus: `project/focus/current_focus.md`
- Roadmap: `project/roadmap/roadmap.md`
- Evidence selecting this work: `project/evidence/EV-0011.md`
- Governing proposal: `project/design/proposals/adopted/browser-vector-frame-api/00_proposal.md`
- Renderer direction: `project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md`
- Display API context: `project/design/proposals/adopted/DP-0007-display-preset-public-api.md`
- Architecture context: `project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md`
- Future scene model context: `project/design/proposals/proposed/DP-0003-extensible-2d-scene-material-model.md`
