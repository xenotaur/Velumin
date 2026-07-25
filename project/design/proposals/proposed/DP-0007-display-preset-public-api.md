---
id: DP-0007
title: Display Preset Public API
status: proposed
owner: project maintainers
created: 2026-07-25
scope: public rendering API, display presets, custom display settings
depends_on:
  - DP-0006
related:
  - DP-0003
  - DP-0005
---

# Display Preset Public API

## Summary
Promote the internal Vector CRT display-preset model introduced by DP-0006 to a small, stable **public API** so that Velumin consumers can select a classic-inspired display look by name, and optionally supply custom display settings, without depending on the internal renderer.

The recommended shape is: a public `VectorDisplayPreset` enum (the four named looks), a stable way to apply a preset to the renderer, and a lower-level custom-settings escape hatch for advanced tuning. Preset **names** become a public contract; the numeric glow/stroke tuning behind each preset stays internal and re-tunable.

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

## Proposed API Shape (for review)
This is a starting point for the decision, not a locked contract.

### Rust
- Make `VectorDisplayPreset` public (`pub enum`), deriving `Clone, Copy, Debug, PartialEq, Eq`. Mark it `#[non_exhaustive]` so adding presets later is not a breaking change (this is what makes the "adding presets stays backward-compatible" goal true — a plain public enum would let downstream code match all variants exhaustively and break on a new one). The alternative is to declare the four-variant set closed; `#[non_exhaustive]` is recommended.
- Add a renderer method, e.g. `WebGPU::set_display_preset(&mut self, preset: VectorDisplayPreset)`, and/or a creation option so a preset can be chosen up front.
- Add a public custom-settings path, e.g. a `DisplaySettings` builder or `WebGPU::set_display_settings(&mut self, settings: DisplaySettings)`, wrapping the existing clamped `from_layers` semantics.

### WASM / JS
- Prefer a `wasm-bindgen` enum export for `VectorDisplayPreset`, or a string-keyed `set_display_preset(name: &str)` accepting `"arcade-balanced" | "monochrome-beam" | "color-quadra-scan" | "clean-neon"` (a string key is often simpler to consume from JS; the choice is an open question).
- Expose a custom-settings setter mirroring the Rust one, replacing the ad-hoc `render_blasterites_tuner` parameter list for real consumers.

## Acceptance Criteria
- A stable public entrypoint (Rust and WASM) selects each of the four named presets and visibly changes the rendered look.
- A custom-settings public entrypoint applies arbitrary (clamped) glow/stroke settings.
- A browser demo route or example demonstrates switching presets at runtime.
- `VectorDisplayPreset` is public and no longer `#[allow(dead_code)]`.
- Each preset variant has recorded visual evidence (extends `WI-SMOKE-0001`'s capture to the non-default presets).
- Public API names are documented (README / crate docs); the numeric tuning is documented as internal and non-contract.

## Risks
- **Premature name/look commitment.** Once preset names are public, renaming is breaking. Mitigate by keeping the set small (the current four) and additive.
- **Unvalidated presets.** Only `ArcadeBalanced` has visual evidence (`EV-0009`). Advertising `MonochromeBeam`, `ColorQuadraScan`, `CleanNeon` before capturing them risks shipping looks that do not meet the DP-0006 quality bar. Adoption of this proposal should follow (or bundle) capturing the non-default presets.
- **API-surface creep.** A custom-settings API can grow toward the full material model (DP-0003); keep this proposal's custom path deliberately minimal (glow layers + stroke scale only).
- **JS ergonomics vs. Rust type safety.** Enum-over-`wasm-bindgen` vs. string keys is a real trade-off (see Open Questions).

## Open Questions
- Enum export vs. string keys for the WASM/JS surface?
- Is preset selection set at construction, mutable at runtime, or both?
- Does v1 include fully custom presets (arbitrary `GlowLayer` arrays), or defer custom settings to a follow-up and ship named presets first?
- Should `render_blasterites_tuner` be deprecated/renamed once a public custom-settings API exists?
- Are display settings global to the renderer, or eventually per-scene/per-layer (DP-0003)?
- Should preset selection also carry the viewport/letterbox policy, or stay strictly about glow/stroke style?

## Implementation Staging (once adopted)
1. Capture visual evidence for the three non-default presets (a `WI-SMOKE-*` follow-up), so all advertised looks meet the bar.
2. Make `VectorDisplayPreset` public and add the `set_display_preset` selector (Rust + WASM), keeping existing entrypoints stable.
3. Add the custom display-settings entry point; migrate the tuner demo onto it.
4. Add a demo route/example and documentation; extend the smoke check to cover preset switching.

## References
- `project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md`
- `project/evidence/EV-0009.md`
- `project/work_items/resolved/WI-SMOKE-0001.md`
- `webgpu_vector_lib/src/lib.rs` (`VectorDisplayPreset`, `VectorDisplaySettings`, `render_blasterites_tuner`)
