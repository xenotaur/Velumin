---
id: WI-ARCH-0001
title: Extract velumin-core platform-neutral crate (DP-0002 Phase 1)
type: deliverable
status: resolved
priority: medium
owner: project maintainers
assigned_agents: []
related_focus: []
related_roadmap:
  - ROADMAP-CORE
related_workstreams: []
related_design:
  - project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md
depends_on: []
blocked_by: []
blocked: false
blocked_reason: null
resolution: Implemented in PR #17 (https://github.com/xenotaur/Velumin/pull/17) — extracted velumin-core (zero wasm-bindgen/web-sys/wgpu dependency) and converted the repository into a Cargo workspace; webgpu_vector_lib depends on and re-exports from velumin-core. scripts/smoke reported 9/9 checks at MAD 0.000 (zero visual regression). VectorDisplayPreset stayed in webgpu_vector_lib (wasm-bindgen-exportability constraint), an approved deviation from the item's literal Required Changes. Resolution will be updated with the merge commit SHA at closeout.
expected_actions:
  - create_file
  - edit_file
  - run_tests
forbidden_actions:
  - force_push
  - delete_branch
  - implement_phase_2_renderer
  - implement_phase_3_desktop
  - rename_crate
  - change_rendered_output
acceptance:
  - A Cargo workspace exists with at least members webgpu_vector_lib and velumin-core
  - velumin-core's Cargo.toml has no wasm-bindgen, web-sys, or wgpu dependency
  - velumin-core builds and its tests pass on the host target (no wasm32 target required)
  - webgpu_vector_lib still builds for wasm32-unknown-unknown and depends on velumin-core
  - The public WASM API surface is unchanged from a JS consumer's perspective
  - scripts/test passes with no behavior change
  - scripts/smoke reports actual per-scene captures matching the committed reference signatures (MAD ~0.000), not a SKIP exit
  - lrh validate reports 0 errors
required_evidence:
  - lrh_validate
  - test_output
  - validation_output
artifacts_expected:
  - Cargo.toml (new workspace root)
  - velumin-core/Cargo.toml
  - velumin-core/src/lib.rs
  - webgpu_vector_lib/Cargo.toml (updated)
  - webgpu_vector_lib/src/lib.rs (updated)
---

# WI-ARCH-0001: Extract velumin-core Platform-Neutral Crate (DP-0002 Phase 1)

## Summary
Extract a new `velumin-core` crate holding Velumin's platform-neutral vector/scene/style types, with zero dependency on `wasm-bindgen`, `web-sys`, or `wgpu` — the first concrete step of DP-0002 Phase 1 ("Rename and Split Boundaries"). `webgpu_vector_lib` becomes a Cargo workspace and depends on `velumin-core` for these types, but keeps the renderer and browser adapter for now.

## Problem / Context
DP-0002 (proposed) calls for a layered architecture (`velumin-core` / `velumin-renderer-wgpu` / `velumin-web` / `velumin-desktop`) so the same renderer can serve both browser and native Steam-targeted desktop games. Today everything — vector types, the `wgpu` renderer, and the browser/`wasm-bindgen` adapter — lives fused in one ~1,800-line file (`webgpu_vector_lib/src/lib.rs`), gated throughout with `#[cfg(target_arch = "wasm32")]`. Phase 1 is the enabling prerequisite for Phase 3 (native `winit` shell): a native frontend cannot exist until the vector/scene model is decoupled from the browser and no longer requires the wasm target to compile. This work item scopes only the type extraction — not the deeper renderer/browser split, which is safer to defer until Phase 3 exists to validate the right boundary (see Non-Goals).

**Scoping decision (2026-07-30, recorded in `project/memory/decision_log.md`):** this work item does not rename `webgpu_vector_lib`. This decides only this item's scope — it does **not** resolve DP-0002's own open question ("Decide whether to rename `webgpu_vector_lib` to `velumin`", still `undecided`) or the `project/design/design.md` TODO on package identity; both remain open for a separate future decision.

### Duplication search
- In-repo: No existing implementation found; no prior crate-split attempt exists.
- Sibling repos: None identified.
- External libraries: None identified.
- Recommendation: Proceed.

### Demand search
- Work items: None found.
- Proposals: DP-0002 (proposed) — Phase 1 "Rename and Split Boundaries" is the direct source of this item; not itself satisfied by any existing work.
- Backlog: No matching entries.
- Recommendation: No action.

## Scope
- Convert the repository to a Cargo workspace.
- Extract a new `velumin-core` crate containing platform-neutral vector/scene/style data types and pure CPU-side geometry helpers, with zero `wasm-bindgen`/`web-sys`/`wgpu` dependency.
- Keep the existing `webgpu_vector_lib` crate (rename deferred) as the renderer + browser adapter, now depending on `velumin-core`.
- Preserve all public WASM entrypoints and browser demo behavior with zero visual regression.

## Required Changes
1. Create `velumin-core/Cargo.toml` (new workspace member; `edition = "2024"`, `rust-version = "1.87"`). No dependencies beyond `std`/`core`: none of the extracted types (`Vec2`, `Color`, `StrokeStyle`, `Line`, `Polyline`, `VectorCommand`, the display-preset/settings types, `RenderViewport`) currently derive `bytemuck::Pod`/`Zeroable` — only `Vertex`/`GlowVertex`, which stay in `webgpu_vector_lib`, use `bytemuck`. Add it to `velumin-core` only if a moved type is later shown to need it.
2. Move to `velumin-core/src/lib.rs`: `Vec2`, `Color`, `StrokeStyle`, `Line`, `Polyline`, `VectorCommand` (currently `webgpu_vector_lib/src/lib.rs:260-306`), `VectorDisplaySettings`/`VectorDisplayPreset`/`GlowLayer` (`webgpu_vector_lib/src/lib.rs:62-221`), `RenderViewport` (`webgpu_vector_lib/src/lib.rs:223-250`), and the pure geometry helpers `transform_points`, `lerp_vec2`, `stroke` (`webgpu_vector_lib/src/lib.rs:1287-1316`).
3. Add a root workspace `Cargo.toml` declaring members `["webgpu_vector_lib", "velumin-core"]`.
4. Update `webgpu_vector_lib/Cargo.toml` to depend on `velumin-core` (path dependency); update `webgpu_vector_lib/src/lib.rs` imports so the existing public WASM API (`create`, `createWithPreset`, `setDisplayPreset`, the preset enum, vector command types) is unchanged for JS/Rust consumers.
5. Keep `Vertex`/`GlowVertex` (wgpu vertex-layout structs), the tessellation functions that produce them, and the Blasterites demo-scene functions in `webgpu_vector_lib` for this phase (see Non-Goals).
6. Update `scripts/*` (`baseline`, `test`, `lint`, `smoke`, `demos`) and `.github/workflows/` as needed so they resolve correctly against the new workspace layout.
7. Update `README.md` / `scripts/README.md` crate-layout references if they describe a single crate.

## Non-Goals
- Do not extract a separate `velumin-renderer-wgpu` crate in this item — `Vertex`/`GlowVertex` and the tessellation functions that produce them stay in `webgpu_vector_lib`. That boundary is Phase 2's "reusable renderer state for any `wgpu::Surface`" and is safer to validate once Phase 3 (native `winit`) exists to prove the abstraction, rather than guessed at now.
- Do not extract a separate `velumin-web` crate; the browser/`wasm-bindgen` adapter (`WebGPU` struct, `browser_has_webgpu`, `resize_canvas_to_display_size`, `log`) stays in `webgpu_vector_lib`.
- Do not rename `webgpu_vector_lib` to `velumin` in this item (see the 2026-07-30 scoping decision in `project/memory/decision_log.md`); DP-0002's own rename question stays `undecided` for a future, separate decision.
- Do not implement DP-0002 Phase 3 (native `winit` desktop), or any Phase 5 (Steam) / Phase 6 (Bevy) work.
- Do not move the Blasterites demo-scene functions out of `webgpu_vector_lib` — they remain demo content, not extracted API surface.
- Do not change any rendered visual output; this is a structural refactor only.

## Acceptance Criteria
- A Cargo workspace exists with (at least) members `webgpu_vector_lib` and `velumin-core`.
- `velumin-core`'s `Cargo.toml` has no `wasm-bindgen`, `web-sys`, or `wgpu` dependency.
- `velumin-core` builds and its tests pass on the host target (no `wasm32-unknown-unknown` required).
- `webgpu_vector_lib` still builds for `wasm32-unknown-unknown` and depends on `velumin-core`.
- The public WASM API surface is unchanged from a JS consumer's perspective.
- `scripts/test` passes with no behavior change.
- `scripts/smoke` reports actual per-scene captures matching the committed reference signatures (MAD ~0.000). A `SKIP` exit (no WebGPU adapter available) does not satisfy this criterion — zero visual regression must be demonstrated by a real capture, not inferred from a successful-but-skipped run.
- `lrh validate` reports 0 errors.

## Validation
- `scripts/version tools`
- `scripts/format --check`
- `scripts/lint`
- `scripts/test`
- `scripts/baseline` (the wasm-target `cargo check` and Vite production build; `scripts/smoke` alone does not exercise the production build path)
- `lrh validate`
- `scripts/smoke` (must report actual captured-frame results, not a `SKIP`; see Acceptance Criteria)

## Risk Notes
- Splitting a large, wasm-gated file risks subtle `cfg` mistakes — mitigated by keeping `velumin-core` deliberately minimal and host-buildable from the start.
- Workspace changes can affect `wasm-pack build` paths and CI; must verify `scripts/baseline`/`scripts/demos`/`scripts/smoke` still resolve correctly post-split.
- The core/renderer line for `Vertex`/tessellation is a genuine judgment call; deferring it (Non-Goals) avoids picking a boundary before Phase 3 can validate it, but a second, harder split is still ahead.

## Related Workstream and Designs
- Design: `project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md` (Phase 1: Rename and Split Boundaries)
