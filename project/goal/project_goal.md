---
id: GOAL-CORE
title: Retro Vector Graphics Library for Games
status: active
owner: project maintainers
time_horizon: long
---

# Project Goal

## Objective

Create Velumin as a retro vector-graphics library for building retro games with a visual look inspired by Asteroids, Star Castle, and Space War.

## Intended Outcome
- A reusable graphics library that can render crisp vector-style primitives suitable for retro games.
- A Rust/WASM/WebGPU implementation path that can be exercised from a browser canvas.
- Examples or harnesses that make library behavior visible and testable.
- Development artifacts that allow LRH-guided agents and humans to interpret project intent safely.

## Intended Users / Stakeholders
- Game developers building retro vector-style browser games.
- Contributors maintaining Velumin's Rust, WebGPU, shader, and web harness code.
- Agents or reviewers using LRH artifacts to reason about project direction and validation.

## In Scope
- Vector-style rendering primitives and supporting shader/render pipeline work.
- Rust library code compiled for WebAssembly.
- WebGPU-backed rendering through `wgpu`.
- Browser-based demonstration or validation harnesses.
- Documentation and project-control artifacts that clarify intent, safety, and current status.

## Out of Scope (Initial)
- Full game implementations beyond focused demos or examples.
- Unrelated rendering styles that weaken the retro vector-display identity.
- Native platform support unless explicitly prioritized later.
- Large architectural rewrites without evidence-backed need.

## Success Direction
- Velumin can be used to create recognizable retro vector-graphics scenes in a browser.
- Rendering APIs become more expressive while remaining easy to validate.
- The repository maintains a clear relationship between project goals, current work, evidence, and status.

