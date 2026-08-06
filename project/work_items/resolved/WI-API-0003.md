---
id: WI-API-0003
title: Select Next Consumer-Driven Public Surface
type: evaluation
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
  - project/design/proposals/adopted/DP-0007-display-preset-public-api.md
  - project/design/proposals/proposed/DP-0003-extensible-2d-scene-material-model.md
  - project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md
depends_on:
  - WI-API-0002
blocked_by: []
blocked: false
blocked_reason: null
resolution: EV-0011 selected a narrow DP-0008 view/viewport mapping follow-up as the next consumer-driven public surface.
expected_actions:
  - create_report
  - write_docs
  - run_tests
  - create_pr
forbidden_actions:
  - force_push
  - delete_branch
  - merge_pr
  - implement_dp0008_extension
  - implement_custom_display_settings
  - implement_retained_scene_model
  - implement_native_winit
  - implement_full_game
  - modify_ci_pipeline
acceptance:
  - The evaluation compares at least DP-0007 custom display settings, a DP-0008 follow-up, DP-0003 retained scene/material work, DP-0002 Phase 3/native work, and no immediate implementation against concrete consumer evidence.
  - The result is grounded in EV-0010 and at least one fresh or rechecked Replication Vector or Blasterites-style consumer pressure point rather than the already-validated representative frame alone.
  - The evaluation records a recommended next public-surface direction, including a rationale for why the other options are deferred or selected.
  - The work does not implement a new API, retained scene model, native shell, custom display settings, or playable game.
  - lrh validate passes after the evidence/status updates.
required_evidence:
  - manual_review
  - lrh_validate
  - test_output
  - validation_output
artifacts_expected:
  - project/work_items/resolved/WI-API-0003.md
  - project/evidence/EV-0011.md
  - project/status/current_status.md
---

# WI-API-0003: Select Next Consumer-Driven Public Surface

## Summary

Evaluate the next Velumin public-surface direction after EV-0010 proved that DP-0008's typed Rust/WASM command-slice path is sufficient for Replication Vector's current representative frame.

## Problem / Context

`WI-API-0002` and EV-0010 closed the immediate Replication Vector evidence loop: downstream-owned `Vec<VectorCommand>` data can render through Velumin's browser renderer using `WebGPU::render_commands(&[VectorCommand])`, without rebuilding through JavaScript `VectorFrame`. The current control plane therefore warns against expanding DP-0008 solely for that already-validated frame, while still naming several plausible next directions: DP-0007's deferred custom display settings, a later evidence-backed DP-0008 follow-up, DP-0003 retained scene/material work, or DP-0002 Phase 3/native `winit` work. This work item should make that choice evidence-backed before implementation begins.

### Duplication search
- In-repo: Related work exists but no duplicate work item was found. `WI-API-0001` implemented DP-0008 v1, `WI-API-0002` validated Replication Vector's typed command-slice path, DP-0007 records deferred custom display settings, DP-0003 proposes a broader retained scene/material model, and DP-0002 Phase 3 remains unselected.
- Sibling repos: Related consumers exist at `ReplicationVector/replication_vector` and `JavascriptGames/javascript_games/blasterites`; neither is a Velumin planning artifact and neither replaces this selection evaluation.
- External libraries: No external library should replace this decision. The choice is about Velumin's own public API sequencing, not about adopting a renderer or game engine.
- Recommendation: Proceed with a narrow evaluation work item rather than implementing any option immediately.

### Demand search
- Work items: No proposed Velumin work item was found for this next public-surface selection before this item was created.
- Proposals: Found direct demand in DP-0008 and current status/focus: follow-up work should be driven by additional Replication Vector / Blasterites-style consumer pressure, not by speculative expansion. DP-0007, DP-0003, and DP-0002 Phase 3 define candidate directions.
- Backlog: No separate matching backlog entry was identified during the project search.
- Recommendation: Link this item to EV-0010, DP-0008, DP-0007, DP-0003, and DP-0002 Phase 3; no close/link action needed.

## Scope

- Review EV-0010 and the current Velumin control-plane recommendation after `WI-API-0002`.
- Inspect at least one concrete consumer pressure point from Replication Vector or Blasterites-style usage beyond the already-validated representative frame.
- Compare the next-direction options: DP-0007 custom display settings, a DP-0008 follow-up, DP-0003 retained scene/material work, DP-0002 Phase 3/native `winit`, and no immediate implementation.
- Record a recommendation and the evidence needed before any implementation work item is created.

## Required Changes

1. Inspect `project/evidence/EV-0010.md`, `project/status/current_status.md`, `project/focus/current_focus.md`, and `project/roadmap/roadmap.md` to capture the post-`WI-API-0002` recommendation.
2. Re-read the governing design surfaces:
   - `project/design/proposals/adopted/browser-vector-frame-api/00_proposal.md`
   - `project/design/proposals/adopted/DP-0007-display-preset-public-api.md`
   - `project/design/proposals/proposed/DP-0003-extensible-2d-scene-material-model.md`
   - `project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md`
3. Inspect at least one current consumer pressure point from Replication Vector or Blasterites-style code/evidence, and distinguish it from the already-validated EV-0010 representative frame.
4. Write a new evidence record, expected as `project/evidence/EV-0011.md`, comparing the candidate directions with pros, cons, blockers, and the evidence basis.
5. Update `project/status/current_status.md` only if the evaluation changes the recommended next action or clarifies a concrete next work item.
6. If the evidence clearly selects a next implementation slice, recommend the exact follow-up work item shape in the evidence record, but do not create or implement that follow-up in this item.

## Non-Goals

- Do not implement a DP-0008 API extension, helper, batching control, camera control, ring helper, or capacity-management feature.
- Do not implement DP-0007 custom display settings.
- Do not adopt or implement DP-0003 retained scene/material/layer APIs.
- Do not adopt or implement DP-0002 Phase 3/native `winit` work; the evaluation may still recommend that direction if fresh consumer evidence supports it.
- Do not implement Replication Vector, Blasterites, Asteroids, Star Castle, or any playable game.
- Do not modify GitHub Actions, validation infrastructure, or CI policy.
- Do not treat EV-0010's already-validated representative frame as sufficient evidence for a new API expansion by itself.

## Acceptance Criteria

- The evaluation compares at least DP-0007 custom display settings, a DP-0008 follow-up, DP-0003 retained scene/material work, DP-0002 Phase 3/native work, and no immediate implementation against concrete consumer evidence.
- The result is grounded in EV-0010 and at least one fresh or rechecked Replication Vector or Blasterites-style consumer pressure point rather than the already-validated representative frame alone.
- The evaluation records a recommended next public-surface direction, including a rationale for why the other options are deferred or selected.
- The work does not implement a new API, retained scene model, native shell, custom display settings, or playable game.
- `lrh validate` passes after the evidence/status updates.

## Validation

- `scripts/version tools`
- `lrh validate`
- `scripts/format --check --diff`
- `scripts/lint`
- `scripts/test`

## Risk Notes

- The main risk is laundering a speculative preference into the control plane as if it were consumer evidence. Mitigation: require at least one fresh or rechecked consumer pressure point beyond EV-0010's representative frame.
- DP-0007, DP-0003, and DP-0002 Phase 3 are different kinds of decisions; compare them as candidate directions, not interchangeable implementation tasks.
- The evaluation may conclude "no immediate implementation" if no pressure point is concrete enough. That is an acceptable result.
