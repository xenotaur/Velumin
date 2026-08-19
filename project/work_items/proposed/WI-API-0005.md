---
id: WI-API-0005
title: Dogfood VectorFrameView Against Replication Vector and Blasterites
type: evaluation
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
  - project/design/proposals/adopted/browser-vector-frame-api/00_proposal.md
  - project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md
  - project/design/proposals/adopted/DP-0007-display-preset-public-api.md
  - project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md
  - project/design/proposals/proposed/DP-0003-extensible-2d-scene-material-model.md
depends_on:
  - WI-API-0004
blocked_by: []
blocked: false
blocked_reason: null
resolution: null
expected_actions:
  - create_report
  - run_tests
  - write_docs
  - create_pr
forbidden_actions:
  - force_push
  - delete_branch
  - merge_pr
  - implement_full_game
  - implement_dp0008_extension
  - implement_custom_display_settings
  - implement_retained_scene_model
  - implement_native_winit
  - introduce_alternate_renderer
  - request_paid_github_review_retrigger
acceptance:
  - Replication Vector is tested against Velumin's landed VectorFrameView path using project-owned VectorCommand data or a documented setup-mismatch reason.
  - Blasterites-style pixel or full-window coordinate usage is tested against Velumin's landed VectorFrameView path using the actual Blasterites checkout or a newly derived adapter/harness tied to exact Blasterites source behavior.
  - The evaluation records whether the current browser renderer API is sufficient for both consumers without adding new Velumin API surface.
  - Concrete gaps are documented with file/line evidence and validation output, including camera/view controls, batching, ring helpers, wasm packaging friction, display settings, or retained scene pressure if encountered.
  - The result recommends the next Velumin public-surface direction, including no immediate expansion if the current API is sufficient.
  - lrh validate passes after the evidence/status updates.
required_evidence:
  - manual_review
  - lrh_validate
  - test_output
  - validation_output
  - visual_smoke_or_screenshot
artifacts_expected:
  - project/work_items/proposed/WI-API-0005.md
  - project/evidence/EV-0012.md
  - project/status/current_status.md
---

# WI-API-0005: Dogfood VectorFrameView Against Replication Vector and Blasterites

## Summary

Evaluate the landed `VectorFrameView` API against the real Replication Vector and Blasterites consumer shapes, and record whether Velumin's current browser renderer surface is sufficient before selecting more public API work.

## Problem / Context

`WI-API-0004` implemented the DP-0008 view/viewport mapping follow-up selected by EV-0011: JavaScript `VectorFrame` and Rust/WASM `&[VectorCommand]` submissions can now opt into centered 4:3, explicit logical extents, or canvas-pixel mapping. The next question should be answered by dogfooding rather than speculation: can Replication Vector and Blasterites use the landed API as-is, and if not, what concrete gap remains?

This item is an integration/evidence slice, not a feature slice. It should preserve the current architecture split: game code owns simulation and wrapping, the renderer owns GPU resources and render-time mapping, and frontends own browser lifecycle.

### Duplication search
- In-repo: Related resolved work exists in `WI-API-0002`, `WI-API-0003`, and `WI-API-0004`, but no proposed work item dogfoods the landed `VectorFrameView` API against both real consumers.
- Sibling repos: Replication Vector and Blasterites consumer code exists, but no checked-in `VectorFrameView` / `renderFrameWithView` / `render_commands_with_view` integration was found during the search.
- External libraries: No external library should replace this evaluation; the purpose is to test Velumin's own Rust/WASM/WebGPU API from concrete consumers.
- Recommendation: Proceed with a narrow dogfooding/evidence work item rather than adding new API surface now.

### Demand search
- Work items: No proposed Velumin work item found for post-`WI-API-0004` consumer dogfooding.
- Proposals: DP-0008 and the current focus/status require future API expansion to be grounded in Replication Vector / Blasterites-style consumer evidence.
- Backlog: No separate matching backlog entry found during the project search.
- Recommendation: Link this item to DP-0008, EV-0010, EV-0011, `WI-API-0004`, and current focus/status; no close/link action needed.

## Scope

- Test Replication Vector's project-owned `VectorCommand` scene path against the landed `VectorFrameView` mapping API.
- Test Blasterites-style full-window or canvas-pixel coordinate usage against the landed `VectorFrameView` mapping API.
- Capture concrete API, packaging, validation, or ergonomics gaps encountered during real consumer dogfooding.
- Recommend the next public-surface direction only after evidence is recorded.

## Required Changes

1. Inspect the current Velumin `VectorFrameView`, `renderFrameWithView`, and typed `render_commands_with_view` implementation and docs.
2. Inspect Replication Vector's current Velumin dependency and representative `VectorCommand` scene.
3. Attempt to render the Replication Vector-owned command data through Velumin using the landed view-mapping path, preferably without rebuilding typed Rust commands through JavaScript.
4. Inspect Blasterites' current coordinate and rendering model, especially canvas-pixel/full-window simulation and drawing.
5. Attempt a minimal Blasterites-style frame or adapter using `VectorFrameView.canvasPixels(...)` or explicit logical extents, without implementing game behavior; this must use the actual Blasterites checkout or derive the adapter from exact Blasterites source behavior rather than reusing Velumin's existing generic `/?demo=frame-api` harness as evidence.
6. Capture browser runtime evidence, screenshot/pixel evidence, or an exact setup-mismatch note if a WebGPU-capable runtime is unavailable.
7. Add a Velumin evidence record, expected as `project/evidence/EV-0012.md`, documenting results, gaps, and recommendation.
8. Update `project/status/current_status.md` only if the evidence changes or sharpens the recommended next action.

## Non-Goals

- Do not implement Replication Vector, Blasterites, Asteroids, Star Castle, or any playable game.
- Do not add new Velumin public API surface in this item.
- Do not implement DP-0007 custom display settings.
- Do not adopt or implement DP-0003 retained scene/material/layer APIs.
- Do not implement DP-0002 Phase 3/native `winit` work.
- Do not introduce an alternate renderer or game engine.
- Do not convert downstream consumer repositories into permanent Velumin examples unless a separate downstream work item explicitly scopes that change.
- Do not satisfy the Blasterites dogfooding criterion by rerunning Velumin's existing generic `/?demo=frame-api` harness alone.
- Do not request paid GitHub review-agent retriggers; use self-review with a fresh independent sub-agent when review is needed.

## Acceptance Criteria

- Replication Vector is tested against Velumin's landed `VectorFrameView` path using project-owned `VectorCommand` data, or the evaluation records the exact setup mismatch that prevents this.
- Blasterites-style pixel/full-window coordinate usage is tested against Velumin's landed `VectorFrameView` path, either through the real consumer checkout or a newly derived adapter/harness tied to exact Blasterites source behavior.
- Browser runtime evidence proves visible rendering, or the evaluation remains explicitly unresolved with a concrete WebGPU/browser setup reason.
- The evidence records whether the current browser renderer API is sufficient for each consumer without adding new Velumin API surface.
- Any concrete gaps are documented with file/line evidence and validation output.
- The result recommends the next Velumin public-surface direction: no immediate expansion, DP-0008 follow-up, DP-0007 custom display settings, DP-0003 retained scene/material, or DP-0002 Phase 3/native work.
- `lrh validate` passes after the evidence/status updates.

## Validation

- `scripts/version tools`
- `lrh validate`
- `scripts/format --check --diff`
- `scripts/lint`
- `scripts/test`
- `scripts/baseline`
- Replication Vector `scripts/version tools`
- Replication Vector `scripts/test`
- Replication Vector `scripts/baseline` or a documented setup-mismatch note
- Blasterites available validation or smoke command, a browser capture from a newly derived Blasterites-source-tied adapter/harness, or a documented setup-mismatch note
- Browser runtime render/capture validation for the dogfooded frame(s), or an explicit unresolved result when no WebGPU-capable browser is available

## Risk Notes

- Dogfooding can drift into building consumer features. Keep the deliverable to evidence and minimal validation harnessing.
- Temporary integration friction in sibling repos should be distinguished from Velumin API gaps.
- A single frame from each consumer can expose API friction, but should not be overclaimed as proof that all future gameplay rendering needs are covered.
- If the current API is sufficient, "no immediate Velumin API expansion" is a valid and valuable result.

## Related Workstream and Designs

- Focus: `project/focus/current_focus.md`
- Roadmap: `project/roadmap/roadmap.md`
- Prior Replication Vector evidence: `project/evidence/EV-0010.md`
- Next-surface selection evidence: `project/evidence/EV-0011.md`
- Implemented prerequisite: `project/work_items/resolved/WI-API-0004.md`
- Governing proposal: `project/design/proposals/adopted/browser-vector-frame-api/00_proposal.md`
- Renderer direction: `project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md`
- Display API context: `project/design/proposals/adopted/DP-0007-display-preset-public-api.md`
- Architecture context: `project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md`
- Future scene model context: `project/design/proposals/proposed/DP-0003-extensible-2d-scene-material-model.md`
