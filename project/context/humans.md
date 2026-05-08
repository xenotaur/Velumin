# Project Context (Human-Oriented)

## One-line Description
- Velumin is an early retro vector-graphics library for creating retro games with a visual style inspired by classic vector-display arcade and space combat games.

## Overview
- The repository currently contains a concise README, a Rust crate under `webgpu_vector_lib/`, a WGSL line shader, and a Vite browser harness.
- The observed implementation exposes a `WebGPU` type through `wasm-bindgen`, initializes a `wgpu` rendering surface from an HTML canvas, handles browser WebGPU capability errors, renders thick line/polyline primitives through triangle geometry, and includes a prototype glow pass.
- This context is derived from authoritative LRH artifacts and observed repository structure; it is not itself authoritative.

## Goals and Direction
- Goal: create Velumin as a reusable retro vector-graphics library for game developers.
- Near-term focus: keep the adopted DP-0001 WebGPU baseline and DP-0004 validation workflow stable while selecting the next workstream.
- Likely next technical directions are DP-0002 architecture splitting, DP-0003 scene/material modeling, browser visual validation, or production glow tuning.

## Design Snapshot
- Authoritative intent lives in `principles/`, `goal/`, and `roadmap/`.
- Execution state lives in `focus/`, `work_items/`, and `contributors/`.
- Constraints live in `guardrails/`.
- Evidence, status, and decisions live in `evidence/`, `status/`, and `memory/`.
- The current code boundary is Rust/WASM/WebGPU plus a browser canvas harness.

## Current Status Snapshot
- Health: yellow.
- Velumin has a visible identity, a working browser/WebGPU baseline, and a script-first validation workflow, but public API shape, production glow behavior, browser visual validation, and broader architecture boundaries remain incomplete.

## Known Unknowns
- Exact public API shape for vector drawing.
- Supported browsers and fallback expectations.
- Browser visual validation workflow.
- Named ownership and review expectations.
- Release or packaging strategy.
- Whether WebGL2 fallback is worth a future compatibility workstream.

## Notes
- Derived summary only (non-authoritative).
