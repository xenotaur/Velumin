# Design

## Purpose
- Velumin is intended to be a retro vector-graphics library for game development.
- The current repository evidence shows an early Rust/WASM/WebGPU rendering path that draws through a browser canvas.

## Scope
- Current scope is the project control plane plus the observed graphics-library implementation skeleton.
- Product scope is limited to retro vector-style rendering support until roadmap details are confirmed.

## Core Structure
- Intent layer: `principles/`, `goal/`, `roadmap/`
- Execution layer: `focus/`, `work_items/`, `contributors/`
- Constraint layer: `guardrails/`
- Truth layer: `evidence/`, `status/`, `memory/`
- Derived context layer: `context/` summaries for humans and agents

## Precedence and Interpretation Notes
- Interpret project work in this order: principles -> goal -> roadmap -> focus -> work_items -> guardrails/runtime context.
- Use `evidence/`, `status/`, and `memory/` to distinguish observed facts from assumptions.
- Do not treat derived `context/` files as authoritative commitments.

## Current Implementation Boundary
- Repository root contains `README.md`, `LICENSE`, and `webgpu_vector_lib/`.
- `webgpu_vector_lib/Cargo.toml` defines a Rust package named `webgpu_vector_lib` with `cdylib` and `rlib` crate outputs.
- `webgpu_vector_lib/src/lib.rs` exposes a `WebGPU` type through `wasm-bindgen`, initializes a `wgpu` surface from an HTML canvas, and renders through a pipeline.
- `webgpu_vector_lib/shaders/line.wgsl` defines a simple white rectangular line-like primitive.
- `webgpu_vector_lib/web/index.html` provides a browser canvas harness that imports the generated WASM package and calls `render()`.

## Future Extensions (Non-binding)
- A stable public API for vector primitives and scenes.
- Rendering options for glow, persistence, intensity, or other retro vector-display effects if they support the project goal.
- Example scenes that demonstrate the intended game aesthetic.
- Automated validation of builds, shaders, and browser-rendered output.

## Unknowns / TODO
- TODO: Confirm whether Velumin should expose a high-level scene graph, immediate-mode drawing API, retained primitives, or another API style.
- TODO: Confirm whether WebGPU is the only intended backend.
- TODO: Confirm whether the `webgpu_vector_lib` crate name is temporary or intended as the long-term package identity.

