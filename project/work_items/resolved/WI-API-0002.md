---
id: WI-API-0002
title: Validate DP-0008 Against Replication Vector Consumer
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
  - project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md
  - project/design/proposals/proposed/DP-0003-extensible-2d-scene-material-model.md
  - project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md
  - project/design/proposals/adopted/DP-0007-display-preset-public-api.md
depends_on:
  - WI-API-0001
blocked_by: []
blocked: false
blocked_reason: null
resolution: Validated in PR #29 — EV-0010 proves Replication Vector-owned VectorCommand data renders through Velumin's Rust/WASM WebGPU::render_commands path in a browser runtime; no immediate DP-0008 API expansion is justified by the representative frame.
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
acceptance:
  - Replication Vector's existing project-owned VectorCommand scene is tested against the Velumin browser renderer path without rebuilding it through JavaScript VectorFrame.
  - Runtime browser render evidence is captured for that representative frame, unless a WebGPU-capable browser is unavailable and the evaluation records the exact setup mismatch.
  - The evaluation records whether WebGPU::render_commands(&[VectorCommand]) is sufficient for one representative Replication Vector frame.
  - Any concrete gaps are documented with evidence, including camera or view controls, batching or capacity reuse, ring helpers, display settings, wasm packaging, or browser harness friction.
  - The work does not implement Replication Vector gameplay or expand Velumin's public API.
  - The result recommends the next public-surface direction using evidence, not speculation.
required_evidence:
  - manual_review
  - lrh_validate
  - test_output
  - validation_output
artifacts_expected:
  - project/work_items/proposed/WI-API-0002.md
  - project/evidence/EV-0010.md
  - project/status/current_status.md
---

# WI-API-0002: Validate DP-0008 Against Replication Vector Consumer

## Summary

Evaluate whether DP-0008's newly implemented browser frame API is sufficient for a real downstream Replication Vector consumer that already owns Velumin `VectorCommand` scene data.

## Problem / Context

DP-0008 and `WI-API-0001` exposed `VectorFrame` for JavaScript consumers and `WebGPU::render_commands(&[VectorCommand])` for Rust/WASM consumers that already own typed Velumin command data. Replication Vector is the motivating downstream case: it already constructs a parent probe outline, asteroid outline, shield arc, and projectile line as Velumin `VectorCommand` data, and its prior evidence recorded that direct browser rendering of downstream-owned scene commands was not yet proven.

This item should close that evidence loop before Velumin expands DP-0008, designs DP-0007 custom display settings, or adopts a broader DP-0003 retained scene/material model.

### Duplication search
- In-repo: Related implementation exists in `WI-API-0001`, `README.md`, and `webgpu_vector_lib/src/lib.rs`, but no existing work item evaluates the API against the real Replication Vector consumer.
- Sibling repos: Replication Vector has `first_replication_vector_scene() -> Vec<VectorCommand>` and downstream evidence at `ReplicationVector/replication_vector/project/evidence/EV-0004.md` documenting the previous upstream browser-rendering gap.
- External libraries: No external library should replace this evaluation; the purpose is to dogfood Velumin's own Rust/WASM/WebGPU API from a downstream repo.
- Recommendation: Proceed with a narrow downstream validation work item.

### Demand search
- Work items: No proposed Velumin work item found for this downstream validation.
- Proposals: DP-0008 explicitly says follow-up work should be driven by Replication Vector / Blasterites-style consumer evidence.
- Backlog: No separate matching backlog entry found.
- Recommendation: Link this item to DP-0008 and current focus; no close/link action needed.

## Scope

- Use Replication Vector's existing project-owned `VectorCommand` scene as the representative consumer frame.
- Exercise the Rust/WASM typed command-slice boundary through Velumin's browser renderer path.
- Record whether the current API is sufficient and document actual gaps encountered.
- Recommend the next public-surface direction based on the evidence.

## Required Changes

1. Inspect Replication Vector's current scene and validation state.
2. Attempt a minimal downstream browser-rendering integration using the existing Velumin API.
3. Prefer the typed Rust/WASM `WebGPU::render_commands(&[VectorCommand])` path for the Replication Vector-owned commands.
4. Capture runtime browser render evidence for the representative frame, such as a smoke run, screenshot, or pixel-validation artifact that would catch a blank or failed frame.
5. Run the relevant Velumin and Replication Vector validation commands available in the session.
6. Add a Velumin evidence record documenting the result and any concrete gaps.
7. Update Velumin current status only if the evaluation changes the project's recommended next action.

## Non-Goals

- Do not implement Replication Vector gameplay, input, physics, mining, enemies, child-probe construction, or progression.
- Do not add a new Velumin public API in this item.
- Do not implement DP-0007 custom display settings.
- Do not adopt or implement DP-0003 retained scene/material/layer APIs.
- Do not introduce an alternate rendering stack in Replication Vector.
- Do not add native `winit`, Bevy integration, or WebGL2 fallback.

## Acceptance Criteria

- Replication Vector's existing `VectorCommand` scene is exercised against Velumin's browser renderer path in a browser runtime.
- The evidence includes a non-skipped browser render or capture result for the representative frame, or leaves the evaluation unresolved with an exact unavailable-browser/setup reason.
- The evaluation records whether the typed Rust/WASM command-slice path is sufficient for one representative frame.
- Any API or packaging gaps are documented with exact file/line evidence and validation output.
- A recommendation is recorded for the next Velumin API direction: DP-0008 follow-up, DP-0007 custom settings, DP-0003 retained scene/material, or no immediate API expansion.
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
- Replication Vector `scripts/baseline` or a documented setup-mismatch note if its Velumin checkout/browser build is unavailable
- Browser runtime render/capture validation for the Replication Vector command frame, or an explicit unresolved result when no WebGPU-capable browser is available

## Risk Notes

- Replication Vector may need a writable checkout or updated `.deps/velumin`; if unavailable, report a setup/bootstrap mismatch rather than turning the item into speculative design.
- Browser rendering evidence may depend on WebGPU-capable local Chromium; if unavailable, record exact compile/test evidence, the missing browser-validation condition, and leave the browser sufficiency question unresolved rather than treating build-only validation as acceptance.
- A single representative frame can identify API friction but should not be overclaimed as proof that all future game rendering needs are covered.
- The evaluation may find that no Velumin API change is needed yet; that is a valid outcome.

## Related Workstream and Designs

- Focus: `project/focus/current_focus.md`
- Roadmap: `project/roadmap/roadmap.md`
- Governing proposal: `project/design/proposals/adopted/browser-vector-frame-api/00_proposal.md`
- Architecture context: `project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md`
- Future scene model context: `project/design/proposals/proposed/DP-0003-extensible-2d-scene-material-model.md`
- Display API context: `project/design/proposals/adopted/DP-0007-display-preset-public-api.md`
- Downstream evidence: `ReplicationVector/replication_vector/project/evidence/EV-0004.md`
