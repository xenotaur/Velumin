# Design

## Purpose
- Velumin is intended to be a retro vector-graphics library for game development.
- The current repository evidence shows a Rust/WASM/WebGPU rendering path that draws vector-style primitives through a browser canvas.

## Scope
- Current scope is the project control plane plus the browser-first Rust/WASM/WebGPU rendering implementation.
- Product scope is limited to retro vector-style rendering support until roadmap details are confirmed.

## Core Structure
- Intent layer: `principles/`, `goal/`, `roadmap/`
- Design layer: `design/`
- Execution layer: `focus/`, `work_items/`, `contributors/`
- Constraint layer: `guardrails/`
- Truth layer: `evidence/`, `status/`, `memory/`
- Derived context layer: `context/` summaries for humans and agents

## Design Proposal Lifecycle
- Design proposals live under `project/design/proposals/`.
- Proposal metadata is authoritative; directory buckets are a derived, human-facing organization.
- Current buckets:
  - `proposed/`: design proposals under consideration or not yet adopted.
  - `adopted/`: design proposals accepted as part of the project design.
- Future buckets may include `rejected/` and `superseded/` when the project needs those states.
- A proposal path should agree with its `status` frontmatter where a matching bucket exists.
- Adoption status and implementation status are separate. `status` records the design decision lifecycle; optional `implementation_status`, `implemented_by`, and `evidence` fields record delivery state and traceability to work items and evidence.

## Precedence and Interpretation Notes
- Interpret project work in this order: principles -> goal -> roadmap -> focus -> work_items -> guardrails/runtime context.
- Use `evidence/`, `status/`, and `memory/` to distinguish observed facts from assumptions.
- Do not treat derived `context/` files as authoritative commitments.

## Current Implementation Boundary
- Repository root contains `README.md`, `LICENSE`, and `webgpu_vector_lib/`.
- `webgpu_vector_lib/Cargo.toml` defines a Rust package named `webgpu_vector_lib` with `cdylib` and `rlib` crate outputs and a modern WebGPU-first `wgpu` dependency path.
- `webgpu_vector_lib/src/lib.rs` exposes a `WebGPU` type through `wasm-bindgen`, keeps browser setup and capability handling at the web boundary, owns reusable renderer state internally, and renders line/polyline commands as thick triangle geometry.
- `webgpu_vector_lib/shaders/line.wgsl` supports the crisp vector primitive pass.
- The current renderer includes a prototype offscreen glow path that composites a glow pass with crisp core geometry.
- `webgpu_vector_lib/web/index.html` provides a browser canvas harness that imports the generated WASM package, reports startup errors, and calls `render()`.

## Adopted Design
- `DP-0001 Modern WebGPU-First Rendering Path` is adopted and implemented as the current browser rendering baseline.
- DP-0001 established modern `wgpu`, browser WebGPU as the primary path, explicit browser capability handling, thick-line triangle tessellation, and a glow-pipeline spike.
- DP-0001 evidence is recorded in `project/evidence/EV-0002.md` through `project/evidence/EV-0006.md`.
- WebGL2 fallback remains optional and deferred unless maintainers explicitly prioritize it later.
- `DP-0004 Script-First Validation Workflow` is adopted and implemented as the canonical local and CI validation contract.
- DP-0004 establishes top-level validation scripts, Rust toolchain pinning, a Clippy gate with warnings denied, and a GitHub Actions workflow that calls `scripts/validate`.

## Active Design Proposals
- `DP-0002 Cross-Platform Vector Renderer Architecture` is the next architecture horizon: platform-neutral core, shared `wgpu` renderer, browser frontend, and later native `winit` frontend.
- `DP-0003 Extensible 2D Scene and Material Model` proposes a broader scene/material model beyond vector-display emulation.

## Future Extensions (Non-binding)
- A stable public API for vector primitives and scenes.
- Production rendering controls for glow, persistence, intensity, background fade, or other retro vector-display effects if they support the project goal.
- Example scenes that demonstrate the intended game aesthetic.
- Automated browser-rendered visual validation.

## Unknowns / TODO
- TODO: Confirm whether Velumin should expose a high-level scene graph, immediate-mode drawing API, retained primitives, or another API style.
- TODO: Confirm whether the `webgpu_vector_lib` crate name is temporary or intended as the long-term package identity.
- TODO: Decide whether WebGL2 fallback is worth a future compatibility workstream.
