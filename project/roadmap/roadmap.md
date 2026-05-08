---
id: ROADMAP-CORE
title: Staged Rendering Modernization Roadmap
status: active
owner: project maintainers
---

# Roadmap

## Status
- LRH bootstrap is complete.
- DP-0001 is adopted and implemented as the current browser/WebGPU rendering baseline.
- DP-0002 remains the next architectural horizon after the browser/WebGPU path, while DP-0004 defines the proposed validation-workflow workstream.

## Completed Direction: DP-0001 Modern WebGPU-First Rendering
- The browser white-line smoke demo has been preserved as the baseline.
- The web rendering path has been upgraded to modern WebGPU-first `wgpu`.
- Browser setup and capability handling are separated from renderer resource ownership.
- Core vector rendering uses CPU-generated thick-line triangles rather than GPU line primitives.
- A glow-pipeline spike exists as proof of rendering flow, not final production tuning.
- WebGL2 fallback remains deferred unless maintainers later make it an explicit requirement.

## Next Horizon: DP-0002 Cross-Platform Renderer Architecture
- After the DP-0001 browser/WebGPU baseline, split Velumin toward a platform-neutral core, shared `wgpu` renderer, browser frontend, and later native desktop frontend.
- Keep the same vector command and renderer model usable by browser and desktop targets where possible.
- Treat native `winit` work as a staged architecture milestone, not as a blocker for browser work.

## Workflow Horizon: DP-0004 Script-First Validation
- Add repository-owned validation scripts so local development, CI, and agent environments use the same command contract.
- Keep GitHub Actions focused on setup, caching, and calling repository scripts.
- Add visual/browser smoke validation after the script layer is stable.

## Later Directions
- Mature glow and compositing after modern WebGPU rendering and triangle-based vector primitives are reliable.
- Add a native `winit` shell that renders the same smoke scenes through the shared renderer.
- Explore optional Bevy integration only after the core and renderer boundaries are stable.
- Add richer examples that demonstrate Asteroids-like, Star Castle-like, or Space War-like visuals without turning Velumin into a full game project.

## Settled Defaults
- WebGPU-first is the default rendering strategy.
- Chrome and Edge desktop are the first browser validation targets.
- WebGL2 compatibility is optional and deferred.
- Rendering work must include a visible smoke check or pixel/screenshot validation path.
- DP-0002 and DP-0004 are proposed follow-up directions after the adopted DP-0001 baseline.
