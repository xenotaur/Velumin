# AGENTS.md

Guidance for AI coding agents working on Velumin.

## Mission
- Help develop Velumin as a Rust/WASM/WebGPU retro vector-graphics library for games.
- Preserve the evidence-backed project direction captured in the LRH-style `project/` control plane.
- Keep changes narrow, reviewable, and grounded in project evidence.

## Source of Truth
- Authoritative project intent and status live under `project/`.
- Read these first when orienting:
  1. `project/principles/principles.md`
  2. `project/goal/project_goal.md`
  3. `project/roadmap/roadmap.md`
  4. `project/focus/current_focus.md`
  5. `project/guardrails/`
  6. `project/evidence/`
  7. `project/status/current_status.md`
  8. `project/memory/decision_log.md`
- Treat `project/context/` as derived summary, not as independent authority.
- Keep design proposal metadata and directory buckets aligned.

## Validation
- Canonical local validation from the repository root:

```sh
scripts/validate
```

- For ordinary task-phase validation, prefer:

```sh
scripts/version
scripts/format --check
scripts/lint
scripts/test
```

- Run `scripts/baseline` when Rust/WASM/Vite browser build behavior may be affected.
- Do not routinely run `scripts/develop` during ordinary validation. Use it for setup/bootstrap or when explicitly debugging setup.
- If validation fails because required tools are missing, report a setup/bootstrap mismatch rather than treating it as a code regression.

## Development Rules
- Follow `STYLE.md` for style guidance.
- Follow `REVIEWS.md` when addressing PR review comments.
- Do not invent roadmap commitments, API design, browser support, or release maturity.
- Preserve uncertainty markers until maintainers resolve them.
- Keep scripts thin; prefer standard Rust, wasm-pack, npm, and Vite behavior over custom logic.
- Do not add browser visual smoke, cargo-deny, dependency review, Dependabot, or supply-chain policy unless there is an explicit work item.

## Current Technical Shape
- Rust crate: `webgpu_vector_lib/`
- Browser harness: `webgpu_vector_lib/web/`
- Canonical validation scripts: `scripts/`
- Current baseline: modern `wgpu`, browser WebGPU, thick vector primitives, and a prototype glow path.
