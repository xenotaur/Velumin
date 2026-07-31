---
id: WI-ARCH-0002
title: Make the wgpu Renderer Surface-Agnostic and Host-Buildable (DP-0002 Phase 2)
type: deliverable
status: proposed
priority: medium
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
  - extract_renderer_crate
  - change_rendered_output
acceptance:
  - Renderer (and its new/render/resize methods) compiles on the host target without requiring --target wasm32-unknown-unknown, and scripts/test runs cleanly there
  - Renderer's error type is no longer JsValue
  - webgpu_vector_lib's wasm-bindgen-exported WebGPU type still compiles for wasm32-unknown-unknown and its behavior (browser demos, presets) is unchanged
  - scripts/smoke reports actual per-scene captures matching committed reference signatures (MAD ~0.000), not a SKIP exit
  - scripts/test passes with no behavior change
  - lrh validate reports 0 errors
required_evidence:
  - lrh_validate
  - test_output
  - validation_output
artifacts_expected:
  - webgpu_vector_lib/src/lib.rs (updated)
  - README.md (updated, if the platform-neutral boundary description changes)
  - scripts/README.md (updated, if the platform-neutral boundary description changes)
---

# WI-ARCH-0002: Make the wgpu Renderer Surface-Agnostic and Host-Buildable (DP-0002 Phase 2)

## Summary
Refactor `webgpu_vector_lib`'s `Renderer` so it compiles and constructs on the host target from any `wgpu::Surface`/`Adapter`, not just `wasm32-unknown-unknown` behind wasm-bindgen — the concrete first slice of DP-0002 Phase 2 ("Modern Shared `wgpu` Renderer"), enabling a future native `winit` frontend (Phase 3) to reuse the same renderer state.

## Problem / Context
DP-0002 Phase 2 calls for "a reusable renderer state that can render to any supported `wgpu::Surface`" and "explicit adapter/capability negotiation for web and desktop." Today, `Renderer` (`webgpu_vector_lib/src/lib.rs`) already takes a generic `wgpu::Surface<'static>` and `&wgpu::Adapter` as constructor parameters — but its `struct` and `impl` block are gated `#[cfg(target_arch = "wasm32")]`, and its own error type is `JsValue` (a wasm-bindgen type), with browser `console.log` calls (`log(...)`) scattered through construction and rendering. This means `Renderer` cannot compile, let alone be constructed or tested, on a host/native target today — a prerequisite gap DP-0002 Phase 3 (native `winit` shell) cannot be built against until it's closed. Phase 2 was selected as the active next workstream on 2026-07-31 (`project/memory/decision_log.md`), following Phase 1's completion in `WI-ARCH-0001` (PR #17, which extracted the platform-neutral `velumin-core` type crate but explicitly deferred the renderer/browser-adapter split to a later phase).

### Duplication search
- In-repo: No existing implementation found. `WI-RENDER-0003` (resolved) already separated browser-specific setup (canvas lookup, DPR, logging) from renderer state *within* `webgpu_vector_lib` — but that renderer state remains `wasm32`-gated with a `JsValue` error type, so no host-buildable `Renderer` exists yet. `WI-ARCH-0001` explicitly deferred the deeper renderer/browser-adapter crate split to a future phase.
- Sibling repos: None identified.
- External libraries: None identified.
- Recommendation: Proceed.

### Demand search
- Work items: `WI-ARCH-0001` (resolved) explicitly defers the renderer/browser-adapter crate split; `WI-RENDER-0003` (resolved) implemented the in-crate separation of concerns this item builds on, but did not make `Renderer` host-buildable.
- Proposals: DP-0002 (proposed) — Phase 2 "Modern Shared `wgpu` Renderer" is the direct source of this item.
- Backlog: No matching entries.
- Recommendation: No action.

## Scope
- Make `Renderer`'s own type and error surface platform-neutral (no `wasm-bindgen`/`web-sys` dependency in its own `impl` block), so it compiles on the host target.
- Keep `WebGPU` (the wasm-bindgen-exported browser adapter type) wasm32-gated as-is, now calling into the portable `Renderer`.
- Preserve all public WASM entrypoints, browser demo behavior, and rendered visual output with zero regression.

## Required Changes
1. Define a platform-neutral error type (e.g. a local `RendererError` enum implementing `std::error::Error` or similar) to replace `JsValue` as the return-error type on `Renderer::new` and `Renderer::render`. Convert to `JsValue` only at the `WebGPU` wasm-bindgen boundary (`WebGPU::create_with_preset`, `WebGPU::render`, `WebGPU::render_blasterites_tester`, `WebGPU::render_blasterites_tuner`).
2. Replace `Renderer`'s internal `log(...)` calls (currently the wasm-bindgen `console.log` extern binding) with a platform-neutral logging call (e.g. gated behind a small `#[cfg(target_arch = "wasm32")]` shim, or a no-op on host), so `Renderer`'s `impl` block itself has no `wasm_bindgen`/`web_sys` dependency.
3. Remove the `#[cfg(target_arch = "wasm32")]` gate from the `Renderer` struct and its `impl Renderer` block so both compile on the host target.
4. Keep `WebGPU`'s canvas lookup, `wgpu::SurfaceTarget::Canvas` surface creation, and `resize_canvas_to_display_size` (browser-specific, uses `web_sys::window`) wasm32-gated exactly as today; `WebGPU::create_with_preset` calls into the now-portable `Renderer::new`.
5. Update `README.md` / `scripts/README.md` if their platform-neutral/wasm32-gated boundary description needs adjustment to reflect that `Renderer` (not just `velumin-core`) now builds on the host target.

## Non-Goals
- Do not implement DP-0002 Phase 3 (native `winit` desktop shell) or any native frontend/example — that is separate future work, gated on its own selection.
- Do not extract `Renderer` into a new `velumin-renderer-wgpu` crate — the crate-boundary question stays open until Phase 3 exists to validate it, per the 2026-07-31 decision-log follow-up note on DP-0002 Phase 2's selection.
- Do not add desktop-specific adapter/capability negotiation logic or UI — there is no native host to negotiate against yet; this item only makes the existing renderer state constructible from a generic surface/adapter.
- Do not add new host-side automated render tests requiring a headless GPU — out of scope for this item; flag as a follow-up if useful.
- Do not change any rendered visual output — this is a structural/portability refactor only, validated by `scripts/smoke`.

## Acceptance Criteria
- `Renderer` (and its `new`/`render`/`resize` methods) compiles on the host target without requiring `--target wasm32-unknown-unknown`, and `scripts/test` runs cleanly there.
- `Renderer`'s error type is no longer `JsValue`.
- `webgpu_vector_lib`'s wasm-bindgen-exported `WebGPU` type still compiles for `wasm32-unknown-unknown` and its behavior (browser demos, presets) is unchanged.
- `scripts/smoke` reports actual per-scene captures matching the committed reference signatures (MAD ~0.000). A `SKIP` exit does not satisfy this criterion.
- `scripts/test` passes with no behavior change.
- `lrh validate` reports 0 errors.

## Validation
- `scripts/version tools`
- `scripts/format --check`
- `scripts/lint`
- `scripts/test`
- `scripts/baseline`
- `lrh validate`
- `scripts/smoke`

## Risk Notes
- The refactor touches every call site inside `Renderer`'s ~250-line `impl` block (many `log(...)` calls, several `JsValue::from_str` error returns) — mitigated by compiling for both host and `wasm32` targets after each change and running `scripts/smoke` before/after to catch any accidental behavior change.
- `Renderer::render` is a large function reused by all three render entrypoints (`render`, `render_blasterites_tester`, `render_blasterites_tuner`); a mistake in the log/error-type swap could silently affect all three.
- Removing the `wasm32` gate could surface latent host-only compile errors (e.g. `wgpu` backend availability assumptions) not visible today since `Renderer` has never been host-compiled.

## Related Workstream and Designs
- Design: `project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md` (Phase 2: Modern Shared `wgpu` Renderer)
- Prior work: `WI-ARCH-0001` (Phase 1, resolved, PR #17)
