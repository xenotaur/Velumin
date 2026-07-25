# Project Context (Agent-Oriented)

## Mission Summary
- Help develop Velumin as a retro vector-graphics library for retro games, while preserving the evidence-backed project direction captured in LRH artifacts.

## Read Order
1. `project/principles/principles.md`
2. `project/goal/project_goal.md`
3. `project/roadmap/roadmap.md`
4. `project/focus/current_focus.md`
5. `project/guardrails/`
6. `project/evidence/EV-0001.md`
7. `project/status/current_status.md`
8. `project/memory/decision_log.md`

## Operational Constraints
- Treat authoritative artifacts outside `context/` as the source of truth.
- Treat this file as a derived summary from `context/humans.md`.
- Keep source changes narrow, evidence-backed, and aligned with the retro vector graphics goal.
- Do not invent roadmap commitments, API design, browser support, or release maturity.
- Preserve uncertainty markers until maintainers resolve them.

## Current Evidence Summary
- Repository identity is Velumin.
- Current implementation evidence points to Rust, WASM, `wasm-bindgen`, `wgpu`, WGSL shaders, and a Vite browser harness with demo routing.
- Current rendering signal is the adopted DP-0001 browser/WebGPU baseline plus the adopted DP-0005 Blasterites tester and tuner demos: thick vector primitives on a black canvas with capability handling, additive multi-layer glow, and a fixed 4:3 viewport.
- DP-0006 (Vector CRT renderer) is adopted (2026-07-24) and partially implemented; its default output is validated by code inspection (EV-0008) and browser visual-smoke capture (EV-0009), with follow-ups tracked in WI-SMOKE-0001.
- Current validation signal is the adopted DP-0004 script-first workflow: `scripts/validate` is the canonical local validation command and GitHub Actions calls it.

## Confidence / Uncertainty Notes
- High confidence: project identity, broad retro vector-graphics goal, current Rust/WASM/WebGPU implementation signals, and the merged Blasterites demos.
- Medium confidence: the DP-0006 non-default presets, which remain internal and are not yet visually captured; the adopted default Arcade-Balanced output is validated by EV-0009 but not yet guarded by an automated check.
- Low confidence: final API model, WebGL2 fallback value, browser support matrix, CI policy, and release packaging.

## Non-authoritative Notice
- This file is derived from `context/humans.md` and adds no independent commitments.
