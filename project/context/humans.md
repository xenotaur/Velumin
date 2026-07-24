# Project Context (Human-Oriented)

## One-line Description
- Velumin is an early retro vector-graphics library for creating retro games with a visual style inspired by classic vector-display arcade and space combat games.

## Overview
- The repository currently contains a concise README, a Rust crate under `webgpu_vector_lib/`, WGSL shaders, and a Vite browser harness with query-parameter demo routing.
- The observed implementation exposes a `WebGPU` type through `wasm-bindgen`, initializes a `wgpu` rendering surface from an HTML canvas, handles browser WebGPU capability errors, renders thick line/polyline primitives through triangle geometry, composites additive multi-layer glow, and applies a fixed 4:3 centered viewport. It renders a baseline white-line scene plus deterministic Blasterites tester and tuner demos.
- This context is derived from authoritative LRH artifacts and observed repository structure; it is not itself authoritative.

## Goals and Direction
- Goal: create Velumin as a reusable retro vector-graphics library for game developers.
- Near-term focus: advance the DP-0006 Vector CRT renderer toward adoption while keeping the adopted DP-0001 baseline, DP-0004 validation workflow, and DP-0005 Blasterites demos stable.
- The immediate next step is recorded browser visual evidence for the renderer; later directions are DP-0002 architecture splitting and DP-0003 scene/material modeling.

## Design Snapshot
- Authoritative intent lives in `principles/`, `goal/`, and `roadmap/`.
- Execution state lives in `focus/`, `work_items/`, and `contributors/`.
- Constraints live in `guardrails/`.
- Evidence, status, and decisions live in `evidence/`, `status/`, and `memory/`.
- The current code boundary is Rust/WASM/WebGPU plus a browser canvas harness.

## Current Status Snapshot
- Health: yellow.
- Velumin has a visible identity, a working browser/WebGPU baseline, a script-first validation workflow, and merged Blasterites tester and tuner demos, but the Vector CRT renderer visual model is not yet backed by recorded browser evidence, and public API shape and broader architecture boundaries remain incomplete.

## Known Unknowns
- Exact public API shape for vector drawing.
- Supported browsers and fallback expectations.
- Browser visual validation workflow.
- Named ownership and review expectations.
- Release or packaging strategy.
- Whether WebGL2 fallback is worth a future compatibility workstream.

## Notes
- Derived summary only (non-authoritative).
