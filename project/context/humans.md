# Project Context (Human-Oriented)

## One-line Description
- Velumin is an early retro vector-graphics library for creating retro games with a visual style inspired by classic vector-display arcade and space combat games.

## Overview
- The repository currently contains a concise README, a Rust crate under `webgpu_vector_lib/`, a WGSL line shader, and a Vite browser harness.
- The observed implementation exposes a `WebGPU` type through `wasm-bindgen`, initializes a `wgpu` rendering surface from an HTML canvas, and renders a simple white line-like shape on a black canvas.
- This context is derived from authoritative LRH artifacts and observed repository structure; it is not itself authoritative.

## Goals and Direction
- Goal: create Velumin as a reusable retro vector-graphics library for game developers.
- Near-term focus: establish the LRH project control plane and keep future work grounded in evidence.
- The likely next technical direction is to move from the current rendering prototype toward explicit vector primitive APIs, but the exact API model is not yet confirmed.

## Design Snapshot
- Authoritative intent lives in `principles/`, `goal/`, and `roadmap/`.
- Execution state lives in `focus/`, `work_items/`, and `contributors/`.
- Constraints live in `guardrails/`.
- Evidence, status, and decisions live in `evidence/`, `status/`, and `memory/`.
- The current code boundary is Rust/WASM/WebGPU plus a browser canvas harness.

## Current Status Snapshot
- Health: yellow.
- Velumin has a visible identity and minimal rendering path, but documentation, validation commands, API design, roadmap detail, and browser support expectations remain incomplete.

## Known Unknowns
- Exact public API shape for vector drawing.
- Whether WebGPU is the only intended backend.
- Supported browsers and fallback expectations.
- Build, test, demo, and CI validation workflow.
- Named ownership and review expectations.
- Release or packaging strategy.

## Notes
- Derived summary only (non-authoritative).

