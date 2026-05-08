---
id: ROADMAP-CORE
title: Initial Roadmap
status: draft
owner: project maintainers
---

# Roadmap

## Status
- Draft bootstrap roadmap. The repository does not yet include detailed product milestones, so this file records conservative directions rather than commitments.

## Near-Term Direction
- Establish the LRH project control plane and keep it synchronized with observed repository state.
- Stabilize the current Rust/WASM/WebGPU rendering path enough that contributors can repeatedly build and view output.
- Expand from the current single-line rendering signal toward explicit vector primitive APIs only after validating the baseline path.

## Candidate Future Directions (Non-binding)
- Add line, polyline, shape, and scene abstractions for retro vector game visuals.
- Add examples that demonstrate Asteroids-like, Star Castle-like, or Space War-like rendering motifs without becoming full games by default.
- Add validation routines for visual output, shader behavior, and browser integration.
- Improve documentation for setup, build, and usage once commands are confirmed.

## Unknowns / TODO
- TODO: Confirm desired public API shape for vector primitives.
- TODO: Confirm target browsers and WebGPU/WebGL fallback expectations.
- TODO: Confirm preferred testing strategy for Rust, WASM, and browser rendering.
- TODO: Confirm release/package strategy.

