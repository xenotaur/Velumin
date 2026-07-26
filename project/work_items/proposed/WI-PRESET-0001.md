---
id: WI-PRESET-0001
title: Public Display Preset API
type: deliverable
status: proposed
priority: medium
owner: project maintainers
depends_on:
  - WI-SMOKE-0001
related_design:
  - project/design/proposals/adopted/DP-0007-display-preset-public-api.md
  - project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md
blocked: false
blocked_reason: null
resolution: null
---

# WI-PRESET-0001: Public Display Preset API

## Objective
- Implement the v1 public display-preset API adopted in DP-0007: expose `VectorDisplayPreset` as a public, selectable API (named presets only) in both the Rust crate and the WASM/JS surface, per the DP-0007 Adopted Decisions.

## Scope
- Make `VectorDisplayPreset` a public `pub enum`, marked `#[non_exhaustive]`, deriving `Clone, Copy, Debug, PartialEq, Eq`; remove its `#[allow(dead_code)]`.
- Add a runtime selector `WebGPU::set_display_preset(preset)` and a creation-time option that lets callers choose any preset up front (e.g. a `create_with_preset(canvas, preset)` factory or an optional preset argument), while the existing `create` entrypoint stays stable and defaults to `ArcadeBalanced`. Keep the `render` / demo entrypoints stable.
- Export `VectorDisplayPreset` and the setter through `wasm-bindgen` (enum export, not string keys).
- Add a browser demo route or example that switches presets at runtime.
- Extend the `WI-SMOKE-0001` smoke check (`scripts/smoke`) to capture the non-default presets, so each advertised look has visual evidence.
- Document the public API names (README / crate docs) and mark the numeric glow/stroke tuning as internal and non-contract.

## Non-Goals (per DP-0007 v1)
- No public custom-settings API (arbitrary glow/stroke settings) — deferred to a follow-up.
- Do not promote `render_blasterites_tuner` to public API; it stays an internal demo/testing entrypoint.
- No per-scene/per-layer settings (that is DP-0003 scene/material-model territory).
- Do not change the 4:3 viewport/letterbox policy; a preset covers glow/stroke style only.

## Required Changes
- `webgpu_vector_lib/src/lib.rs`: make `VectorDisplayPreset` public + `#[non_exhaustive]`; add `set_display_preset` (runtime) and a creation-time preset option — a factory or optional argument that selects any preset, with the existing `create` defaulting to `ArcadeBalanced`; wire the selected preset into `display_settings`.
- `webgpu_vector_lib/src/lib.rs` (wasm-bindgen exports): export the enum and setter for JS.
- `webgpu_vector_lib/web/index.html`: a demo route/control that switches presets at runtime.
- `webgpu_vector_lib/web/smoke.mjs` + `scripts/smoke`: capture each non-default preset; commit reference signatures for them.
- `README.md` / crate docs: document the public preset API.

## Acceptance Criteria
- A public Rust and WASM entrypoint selects each of the four named presets and visibly changes the look, at creation and at runtime.
- `VectorDisplayPreset` is public, `#[non_exhaustive]`, and no longer `#[allow(dead_code)]`.
- A demo route/example switches presets at runtime.
- Each preset variant has recorded visual evidence via the extended smoke check before it is advertised as supported.
- The public API names are documented; the numeric tuning is documented as internal/non-contract.
- No public custom-settings API is added; `render_blasterites_tuner` remains internal.

## Validation
- `scripts/version`
- `scripts/format --check`
- `scripts/lint`
- `scripts/test`
- `lrh validate`
- `scripts/smoke` on a WebGPU-capable environment (now covering the non-default presets)

## Evidence
- DP-0007 (adopted): `project/design/proposals/adopted/DP-0007-display-preset-public-api.md`
- Internal preset model: `webgpu_vector_lib/src/lib.rs` (`VectorDisplayPreset`, `VectorDisplaySettings::from_preset`)
- Smoke check to extend: `webgpu_vector_lib/web/smoke.mjs`, `scripts/smoke`, `project/work_items/resolved/WI-SMOKE-0001.md`
