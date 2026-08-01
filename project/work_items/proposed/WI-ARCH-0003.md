---
id: WI-ARCH-0003
title: Structure Renderer's Capability-Negotiation Failures as Typed Errors (DP-0002 Phase 2)
type: deliverable
status: proposed
priority: low
owner: project maintainers
assigned_agents: []
related_focus:
  - FOCUS-RENDER-0001
related_roadmap:
  - ROADMAP-CORE
related_workstreams: []
related_design:
  - project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md
depends_on: []
blocked_by: []
blocked: false
blocked_reason: null
resolution: null
expected_actions:
  - edit_file
  - run_tests
forbidden_actions:
  - force_push
  - delete_branch
  - implement_phase_3_desktop
  - change_rendered_output
  - change_web_facing_error_text
acceptance:
  - RendererError is an enum with distinct variants for each capability-negotiation failure, not a bare String wrapper
  - Every JS-facing error message produced via the From<RendererError> for JsValue impl is byte-identical to the current text
  - lrh validate reports 0 errors
  - scripts/smoke reports actual per-scene captures matching committed reference signatures (MAD ~0.000), not a SKIP exit
required_evidence:
  - lrh_validate
  - test_output
  - validation_output
artifacts_expected:
  - webgpu_vector_lib/src/lib.rs (updated)
---

# WI-ARCH-0003: Structure Renderer's Capability-Negotiation Failures as Typed Errors (DP-0002 Phase 2)

## Summary
Replace `RendererError`'s bare `String` payload with a small enum of specific capability-negotiation failure variants (unsupported surface format, unsupported alpha mode, missing required present mode, insufficient device limits, device request failure), plus a catch-all variant for `Renderer::render`'s runtime errors — so a future native caller can match on which specific negotiation check failed, not just parse a message string. Preserves exact JS-facing error text at the `WebGPU` boundary.

## Problem / Context
DP-0002 Phase 2's "explicit adapter/capability negotiation for web and desktop" bullet is substantially satisfied already: `WI-ARCH-0002` (PR #20) made the format/alpha-mode/present-mode/limits checks in `Renderer::new` platform-neutral, so they already apply to any future native surface with no changes needed — `Renderer::new` takes a generic `wgpu::Surface`/`Adapter`, and `wgpu`'s own `request_adapter`/`request_device` APIs are already platform-agnostic. The remaining gap is narrow: these failures are all currently represented as a single `RendererError(String)` (`webgpu_vector_lib/src/lib.rs:16`) — fine for the web's JS-facing error messages, but not structured enough for a future native (non-JS) caller to distinguish failure kinds programmatically (e.g. to retry with relaxed limits vs. report a hard incompatibility).

A genuine native entry point (winit window → surface → adapter → call `Renderer::new`) remains DP-0002 Phase 3's job, not this item's — this item only prepares the error shape both consumers will eventually share.

### Duplication search
- In-repo: No existing implementation found. `WI-ARCH-0002` introduced `RendererError` as a plain `String` wrapper; this item refines that same type.
- Sibling repos: None identified.
- External libraries: None identified.
- Recommendation: Proceed.

### Demand search
- Work items: `WI-ARCH-0002` (resolved) introduced `RendererError` but explicitly scoped out desktop-specific capability negotiation as a Non-Goal, since no native host exists yet. This item is the narrow, implementable-now residue of that deferred scope, chosen over deferring the whole DP-0002 Phase 2 bullet until Phase 3 exists.
- Proposals: DP-0002 (proposed) — Phase 2 "Modern Shared `wgpu` Renderer," the "adapter/capability negotiation for web and desktop" bullet.
- Backlog: No matching entries.
- Recommendation: No action.

## Scope
- Turn `RendererError` into an enum with one variant per capability-negotiation failure site in `Renderer::new`, plus a catch-all for `Renderer::render`'s runtime errors.
- Preserve exact current JS-facing error text — this is a structural change to the error type, not a wording change.
- Do not add a native entry point or any desktop-specific negotiation logic beyond what already exists in `Renderer::new`.

## Required Changes
1. Replace `struct RendererError(String)` (`webgpu_vector_lib/src/lib.rs:16`) with an enum covering the 5 capability-negotiation failure sites in `Renderer::new` (`:375` unsupported surface format, `:378` unsupported alpha mode, `:384` missing FIFO present mode, `:391` insufficient limits, `:407` device request failed) plus one `Other(String)` variant covering `Renderer::render`'s 2 runtime error sites (`:608` surface texture unavailable, `:613` frame failure).
2. Implement `Display` for the enum, one arm per variant, producing the exact current message text byte-for-byte (verify against the current literals before changing anything).
3. Update all 7 `RendererError::new(...)` construction call sites to construct the appropriate variant instead.
4. Keep the `From<RendererError> for JsValue` impl producing identical `JsValue::from_str(&format!("{}", err))` output — no change to its own logic, just to what type it now converts from.

## Non-Goals
- Do not implement DP-0002 Phase 3 (native `winit` desktop shell) or any native entry point — that is separate future work, gated on its own selection.
- Do not change any web-facing (JS-visible) error message text — this is a structural refactor of the error *type*, not its wording.
- Do not add new capability checks beyond the 5 that already exist in `Renderer::new`.
- Do not change any rendered visual output.

## Acceptance Criteria
- `RendererError` is an enum with distinct variants for each capability-negotiation failure, not a bare `String` wrapper.
- Every JS-facing error message produced via the `From<RendererError> for JsValue` impl is byte-identical to the current text.
- `lrh validate` reports 0 errors.
- `scripts/smoke` reports actual per-scene captures matching the committed reference signatures (MAD ~0.000). A `SKIP` exit does not satisfy this criterion.

## Validation
- `scripts/version tools`
- `scripts/format --check`
- `scripts/lint`
- `scripts/test`
- `scripts/baseline`
- `lrh validate`
- `scripts/smoke`

## Risk Notes
- The error-text-preservation acceptance criterion is easy to accidentally violate when rewriting message construction as `Display` arms — mitigated by diffing each variant's `Display` output against the current literal string before considering the change complete.
- This item's value is speculative until a native caller actually exists (Phase 3) to consume the typed variants; if Phase 3 is never selected, this remains unused but harmless structural polish.

## Related Workstream and Designs
- Design: `project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md` (Phase 2: Modern Shared `wgpu` Renderer)
- Prior work: `WI-ARCH-0002` (PR #20, resolved) — introduced `RendererError` as a plain `String` wrapper
