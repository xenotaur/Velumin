---
id: GUARDRAILS-APPROVALS
title: Approval Guardrails
status: active
owner: project maintainers
---

# Approval Guardrails

## Default Approval Posture
- Keep bootstrap and routine development changes narrow and reviewable.
- Prefer additions and focused edits over broad rewrites.
- Do not change project direction, public API commitments, or backend strategy without maintainer confirmation.

## Changes Requiring Explicit Maintainer Approval
- Destructive operations such as deleting files, renaming major directories, or resetting history.
- Large architectural rewrites of the Rust/WASM/WebGPU rendering path.
- New external services, paid infrastructure, or telemetry.
- Public claims about production readiness, browser support, or release status.

## Bootstrap Constraint
- This bootstrap may add files only under `project/`.

