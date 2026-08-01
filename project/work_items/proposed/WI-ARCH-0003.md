---
id: WI-ARCH-0003
title: Structure Renderer's Capability-Negotiation Failures as Typed Errors (DP-0002 Phase 2 prep, does not close the negotiation bullet)
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

# WI-ARCH-0003: Structure Renderer's Capability-Negotiation Failures as Typed Errors (DP-0002 Phase 2 prep, does not close the negotiation bullet)

## Summary
Replace `RendererError`'s bare `String` payload with a small enum of specific capability-negotiation failure variants (unsupported surface format, unsupported alpha mode, missing required present mode, insufficient device limits, device request failure), plus a catch-all variant for `Renderer::render`'s runtime errors — so a future native caller can match on which specific negotiation check failed, not just parse a message string. Preserves exact JS-facing error text at the `WebGPU` boundary.

## Problem / Context
**This item does not close DP-0002 Phase 2's "explicit adapter/capability negotiation for web and desktop" bullet.** `WebGPU::create_with_preset` acquires the `wgpu::Adapter` itself (via `instance.request_adapter(...)`) before ever calling `Renderer::new` — that acquisition step is entirely browser-specific (`web_sys::window`, canvas-derived surface) and has no native equivalent today. `WI-ARCH-0002` (PR #20) made the format/alpha-mode/present-mode/limits *checks* inside `Renderer::new` platform-neutral, so those specific checks would already apply to a future native surface with no changes needed — but native adapter selection and failure handling (the actual desktop-side negotiation) remain entirely untracked, and stay that way until DP-0002 Phase 3 provides a real native entry point. This item's Non-Goals explicitly exclude that entry point, so it cannot be read as satisfying the Phase 2 bullet; it should be understood as a standalone error-model refactor, not Phase 2 progress in itself.

What this item *does* do: `Renderer::new`'s existing capability-check failures are all currently represented as a single `RendererError(String)` (`webgpu_vector_lib/src/lib.rs:16`) — fine for the web's JS-facing error messages, but not structured enough for a future native (non-JS) caller to distinguish failure kinds programmatically (e.g. to retry with relaxed limits vs. report a hard incompatibility). This item restructures that error type only, as low-risk prep work that a future Phase 3 native caller could benefit from — independent of whether or when Phase 3 happens.

### Duplication search
- In-repo: No existing implementation found. `WI-ARCH-0002` introduced `RendererError` as a plain `String` wrapper; this item refines that same type.
- Sibling repos: None identified.
- External libraries: None identified.
- Recommendation: Proceed.

### Demand search
- Work items: `WI-ARCH-0002` (resolved) introduced `RendererError` but explicitly scoped out desktop-specific capability negotiation as a Non-Goal, since no native host exists yet. This item is a standalone error-model refactor, not a continuation that closes that deferred scope — the desktop-side negotiation bullet itself remains untracked and open, pending DP-0002 Phase 3.
- Proposals: DP-0002 (proposed) — Phase 2 "Modern Shared `wgpu` Renderer." The "adapter/capability negotiation for web and desktop" bullet remains open after this item; do not mark it done on this item's account.
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
5. Add unit tests asserting each variant's `Display` output equals the current literal message text exactly, including the dynamic `Other`/device-request/frame-failure variants that interpolate a `{:?}`-formatted upstream error or dimensions — construct those variants directly with a representative payload and assert on the formatted string, since no existing test exercises these failure paths and neither `scripts/smoke` (which only drives successful WebGPU scenes) nor plain compilation can catch a wording or punctuation drift.

## Non-Goals
- Do not implement DP-0002 Phase 3 (native `winit` desktop shell) or any native entry point — that is separate future work, gated on its own selection.
- Do not change any web-facing (JS-visible) error message text — this is a structural refactor of the error *type*, not its wording.
- Do not add new capability checks beyond the 5 that already exist in `Renderer::new`.
- Do not change any rendered visual output.

## Acceptance Criteria
- `RendererError` is an enum with distinct variants for each capability-negotiation failure, not a bare `String` wrapper.
- Every JS-facing error message produced via the `From<RendererError> for JsValue` impl is byte-identical to the current text, **enforced by a unit test per variant** (not just manual inspection) — `scripts/test` alone does not satisfy this criterion unless it includes these assertions.
- `lrh validate` reports 0 errors.
- `scripts/smoke` reports actual per-scene captures matching the committed reference signatures (MAD ~0.000). A `SKIP` exit does not satisfy this criterion.
- This item does not close DP-0002 Phase 2's "adapter/capability negotiation for web and desktop" bullet — that remains open pending Phase 3; do not update `project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md` to mark it done on this item's account.

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
