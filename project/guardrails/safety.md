---
id: GUARDRAILS-SAFETY
title: Safety Guardrails
status: active
owner: project maintainers
---

# Safety Guardrails

## Repository Safety
- Preserve existing source code unless a later work item explicitly authorizes implementation changes.
- Check worktree state before editing files.
- Avoid reverting user or maintainer changes unless explicitly requested.

## Runtime and Browser Safety
- Treat WebGPU availability, browser compatibility, and fallback behavior as uncertain until validated.
- Do not assume rendering correctness from successful compilation alone.
- Prefer minimal demos and reproducible validation when working on graphics behavior.

## Documentation Safety
- Mark uncertain claims as TODO or unknown.
- Keep derived context files aligned with authoritative artifacts.

