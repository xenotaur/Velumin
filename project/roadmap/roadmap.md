---
id: ROADMAP-CORE
title: Staged Rendering Modernization Roadmap
status: active
owner: project maintainers
---

# Roadmap

## Status
- LRH bootstrap is complete.
- DP-0001 is adopted and implemented as the browser/WebGPU rendering baseline.
- DP-0004 is adopted and implemented as the core local/CI validation workflow.
- DP-0005 is adopted and implemented as the Blasterites tester and tuner browser demos.
- DP-0006 (Vector CRT renderer) is the active renderer workstream and is partially implemented.

## Completed Direction: DP-0001 Modern WebGPU-First Rendering
- The browser white-line smoke demo is preserved as the baseline.
- The web rendering path uses modern WebGPU-first `wgpu`.
- Browser setup and capability handling are separated from renderer resource ownership.
- Core vector rendering uses CPU-generated thick-line triangles rather than GPU line primitives.
- A glow-pipeline spike proved the rendering flow; production tuning is folded into DP-0006.
- WebGL2 fallback remains deferred unless maintainers later make it an explicit requirement.

## Completed Workflow: DP-0004 Script-First Validation
- Repository-owned validation scripts define the shared local, CI, and agent command contract.
- GitHub Actions focuses on setup, caching, and calling repository scripts.

## Completed Direction: DP-0005 Blasterites Tester and Tuner Demos
- A deterministic Blasterites-inspired tester renders a rotating ship, bullet, approaching asteroid, spark explosion, glow, scanline treatment, and subtle pulse/wobble, keyed to elapsed time.
- A live tuner exposes glow-layer and stroke-width controls over the same scene.
- Demo routing (`/?demo=blasterites`, `/?demo=tuner`) preserves the baseline `/` white-line smoke scene.
- `scripts/demos` builds the WASM package and serves the demo routes.
- Delivered by `WI-DEMO-0001`; verified against merged code in `EV-0008`.

## Active Horizon: DP-0006 Vector CRT Renderer Migration
- Migrate the widened-line glow spike to an internal-first Vector CRT display renderer tuned against the Blasterites tester, preserving the existing public WASM entrypoints.
- Already landed: a fixed 4:3 centered viewport (no window-resize distortion), additive multi-layer glow compositing, and an internal `VectorDisplayPreset` set with `ArcadeBalanced` as the tuning target.
- Remaining before adoption: recorded browser/screenshot visual evidence that Arcade Balanced meets its acceptance target, and a decision on whether the internal preset set becomes a public API.

## Next Horizon: DP-0002 Cross-Platform Renderer Architecture
- After the Vector CRT renderer stabilizes, split Velumin toward a platform-neutral core, shared `wgpu` renderer, browser frontend, and later native desktop frontend.
- Keep the same vector command and renderer model usable by browser and desktop targets where possible.
- Treat native `winit` work as a staged architecture milestone, not as a blocker for browser work.

## Later Directions
- Add a native `winit` shell that renders the same smoke and tester scenes through the shared renderer.
- Explore optional Bevy integration only after the core and renderer boundaries are stable.
- Add richer examples that demonstrate Asteroids-like, Star Castle-like, or Space War-like visuals without turning Velumin into a full game project.

## Settled Defaults
- WebGPU-first is the default rendering strategy.
- Chrome and Edge desktop are the first browser validation targets.
- WebGL2 compatibility is optional and deferred.
- Rendering work must include a visible smoke check or pixel/screenshot validation path.
- `scripts/validate` is the canonical local validation command; `scripts/demos` serves the browser demos.
- DP-0002 and DP-0003 are proposed follow-up directions after the adopted DP-0001, DP-0004, and DP-0005 baselines.
