---
id: DP-0007
title: Display Preset Public API
status: adopted
owner: project maintainers
created: 2026-07-25
adopted: 2026-07-25
implementation_status: implemented
implemented_by:
  - WI-PRESET-0001
scope: public rendering API, display presets, custom display settings
depends_on:
  - DP-0006
related:
  - DP-0003
  - DP-0005
---

# Display Preset Public API

## Summary
Promote the internal Vector CRT display-preset model introduced by DP-0006 to a small, stable **public API** so that Velumin consumers can select a classic-inspired display look by name, without depending on the internal renderer.

The adopted shape (v1) is: a public `VectorDisplayPreset` enum (the four named looks), selectable both at renderer creation and at runtime. Preset **names** become a public contract; the numeric glow/stroke tuning behind each preset stays internal and re-tunable. A public custom-settings escape hatch is deferred to a follow-up.

## Adopted Decisions (2026-07-25)
The proposal is adopted with these decisions resolving its open questions; the v1 API is implemented by `WI-PRESET-0001` (`implementation_status: implemented`; the work item moves to `resolved` at closeout):

- **Q1 — JS surface:** export `VectorDisplayPreset` as a `wasm-bindgen` enum (type-checked at the boundary), not string keys.
- **Q2 — Selection timing:** both — a default preset chosen at `WebGPU::create()` and a runtime `set_display_preset(...)` setter.
- **Q3 — v1 scope:** ship the named presets and selector only; **defer** a public custom-settings API to a follow-up (the internal tuner still covers experimentation).
- **Q4 — Tuner entrypoint:** keep `render_blasterites_tuner` as an internal demo/testing entrypoint; do not advertise it as public.
- **Q5 — Scope of settings:** display settings are global to the renderer for v1; per-scene/per-layer settings remain DP-0003 territory.
- **Q6 — Preset vs viewport:** a preset covers glow/stroke style only; the 4:3 viewport/letterbox policy stays separate (`centered_4_3`).
- **Enum stability:** `VectorDisplayPreset` is `#[non_exhaustive]` so adding presets later is not a breaking change.
- **Visual evidence:** `WI-PRESET-0001` extended the `scripts/smoke` check to capture the three non-default presets on the deterministic tester frame (with a cross-preset distinctness assertion), so all four looks now have recorded visual evidence.

## Context
DP-0006 is adopted and partially implemented. The renderer already carries a `VectorDisplaySettings` value (`glow_layers` + `stroke_width_scale`) and constructs it three ways in `webgpu_vector_lib/src/lib.rs`:

- `VectorDisplaySettings::from_preset(VectorDisplayPreset)` — maps each of `ArcadeBalanced`, `MonochromeBeam`, `ColorQuadraScan`, `CleanNeon` to a fixed glow-layer table.
- `VectorDisplaySettings::from_tuner(..)` — seven explicit glow/stroke parameters, used by the `render_blasterites_tuner` demo entrypoint.
- `VectorDisplaySettings::from_layers(..)` — the clamping constructor both of the above funnel through.

The renderer is initialised to `from_preset(ArcadeBalanced)` at creation, and every scene (`render`, `render_blasterites_tester`) draws through whatever `display_settings` holds. But `VectorDisplayPreset` is `#[allow(dead_code)]`: **no public entrypoint selects a preset by name.** The only public style control today is `render_blasterites_tuner`, which is a testing/tuning harness, not a game-facing API.

DP-0006 deliberately kept presets internal "until the visual model has evidence." That evidence now exists for the default look: `EV-0009` recorded the manual browser visual-smoke capture, and `WI-SMOKE-0001` automated it into `scripts/smoke`. This makes promoting a preset selector to the public API the natural next step — while the other three presets still need visual capture before they are advertised (see Risks).

## Decision
Expose a minimal, additive public API for **display style selection**, in both the Rust crate and the WASM/JS surface:

1. A public `VectorDisplayPreset` enum with the four named variants.
2. A stable way to apply a preset to a renderer instance (a mutator and/or a creation-time option).
3. A lower-level "custom display settings" entry point (generalising `from_tuner`) for consumers who want to tune beyond the presets.

Keep existing public WASM entrypoints (`render`, and the Blasterites demo/tuner entrypoints) stable. This proposal covers **only how the display look is chosen**; it does not design the primitive-submission (drawing) API or the scene model.

## Goals
- Give consumers a one-call way to pick a recognisable classic look by name.
- Make preset variant names a stable, documented contract; adding new presets stays backward-compatible.
- Keep numeric tuning (glow-layer widths/intensities, stroke scale) internal and re-tunable without an API break.
- Provide an escape hatch for custom display settings so the named presets are ergonomic defaults, not a ceiling.
- Remove the `#[allow(dead_code)]` on `VectorDisplayPreset` by giving it a real public consumer.

## Non-Goals
- Do not expose the internal glow numeric constants (the per-preset `GlowLayer` tables) as a stable contract.
- Do not add per-primitive style overrides in this proposal (a future extension; relates to DP-0003's material model).
- Do not turn the `render_blasterites_tuner` demo entrypoint into the public tuning API; a clean custom-settings API supersedes it.
- Do not commit to a stable primitive-submission/drawing API here — that remains an open, separate design.
- Do not design cross-platform (native) API parity beyond keeping the model platform-neutral (DP-0002 territory).

## Options Considered
1. **Named presets only.** Simplest and most ergonomic, but offers no customization; advanced users are stuck with four looks.
2. **Custom settings only.** Expose the low-level glow-layer/stroke settings; keep presets internal. Flexible but has no ergonomic defaults and pushes CRT-tuning knowledge onto every consumer.
3. **Named presets + custom escape hatch (recommended).** Named presets are the ergonomic default; custom settings serve advanced users. Matches what the code already supports (`from_preset` + `from_tuner` funnel through `from_layers`).
4. **Do nothing / keep internal.** Rejected: DP-0006 is adopted and the default look has visual evidence (`EV-0009`), so there is no longer a reason to withhold a selector.

## Adopted API Shape
The v1 contract, per the Adopted Decisions above. Method names below are indicative; exact signatures are settled during implementation (`WI-PRESET-0001`).

### Rust
- Make `VectorDisplayPreset` public (`pub enum`), `#[non_exhaustive]`, deriving `Clone, Copy, Debug, PartialEq, Eq`.
- Add `WebGPU::set_display_preset(&mut self, preset: VectorDisplayPreset)` (runtime) **and** a creation option so a preset can be chosen up front (default `ArcadeBalanced`).
- No public custom-settings path in v1 — deferred to a follow-up (the internal `from_layers`/`from_tuner` machinery stays internal).

### WASM / JS
- Export `VectorDisplayPreset` as a `wasm-bindgen` enum and a `set_display_preset(preset)` setter accepting it. No string-key API and no public custom-settings setter in v1.
- `render_blasterites_tuner` remains an internal demo/testing entrypoint, not public API.

## Acceptance Criteria (for implementation, WI-PRESET-0001)
- A stable public entrypoint (Rust and WASM) selects each of the four named presets and visibly changes the rendered look, both at creation and at runtime.
- `VectorDisplayPreset` is public, `#[non_exhaustive]`, and no longer `#[allow(dead_code)]`.
- A browser demo route or example demonstrates switching presets at runtime.
- Each preset variant has recorded visual evidence (extends the `WI-SMOKE-0001` smoke check to the non-default presets) before it is advertised as supported.
- Public API names are documented (README / crate docs); the numeric tuning is documented as internal and non-contract.
- No public custom-settings API in v1; `render_blasterites_tuner` stays internal.

## Risks
- **Premature name/look commitment.** Once preset names are public, renaming is breaking. Mitigate by keeping the set small (the current four) and additive.
- **Preset quality bar.** `MonochromeBeam`, `ColorQuadraScan`, and `CleanNeon` are now captured by the smoke check (`WI-PRESET-0001`), so all four looks have visual evidence; keep re-checking that captures meet the DP-0006 quality bar when the presets are re-tuned.
- **API-surface creep.** Deferring the custom-settings API (Q3) keeps v1 minimal; a future custom path must stay small (glow layers + stroke scale) and not drift toward the full material model (DP-0003).

## Resolved / Deferred Questions
All open questions from the proposal are resolved by the Adopted Decisions above (Q1–Q6 plus enum stability). Two items are explicitly **deferred** rather than answered here:

- A **public custom-settings API** (arbitrary glow/stroke settings) — deferred to a follow-up proposal/work item after named presets ship.
- **Per-scene/per-layer** display settings — belongs to the DP-0003 scene/material model, not this proposal.

## Implementation Staging
1. Make `VectorDisplayPreset` public and `#[non_exhaustive]` (remove `#[allow(dead_code)]`); add the `set_display_preset` selector plus a creation-time default (Rust + WASM), keeping existing entrypoints stable.
2. Add a browser demo route/example that switches presets at runtime.
3. Capture visual evidence for the three non-default presets (extend the `WI-SMOKE-0001` smoke check), so all advertised looks meet the DP-0006 bar.
4. Document the public API names (README / crate docs); mark the numeric tuning internal and non-contract.

Tracked in `WI-PRESET-0001`.

## References
- `project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md`
- `project/evidence/EV-0009.md`
- `project/work_items/resolved/WI-SMOKE-0001.md`
- `webgpu_vector_lib/src/lib.rs` (`VectorDisplayPreset`, `VectorDisplaySettings`, `render_blasterites_tuner`)
