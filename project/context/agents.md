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
- Current implementation evidence points to Rust, WASM, `wasm-bindgen`, `wgpu`, WGSL shaders, and a Vite browser harness.
- Current rendering signal is minimal: a simple white line-like shape on a black canvas.

## Confidence / Uncertainty Notes
- High confidence: project identity, broad retro vector-graphics goal, current Rust/WASM/WebGPU implementation signals.
- Medium confidence: near-term need to stabilize build/demo/validation workflow.
- Low confidence: final API model, backend strategy, browser support, CI policy, and release packaging.

## Non-authoritative Notice
- This file is derived from `context/humans.md` and adds no independent commitments.

