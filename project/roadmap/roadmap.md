---
id: ROADMAP-CORE
title: Staged Rendering Modernization Roadmap
status: active
owner: project maintainers
---

# Roadmap

## Status
- LRH bootstrap is complete.
- The active roadmap now follows DP-0001, with DP-0002 recorded as the next architectural horizon after the browser/WebGPU path is stable.

## Active Direction: DP-0001 Modern WebGPU-First Rendering
- Preserve the current browser white-line smoke demo as the baseline before making renderer changes.
- Upgrade from the current `wgpu 0.16` + WebGL compatibility path to a modern WebGPU-first `wgpu` path.
- Keep Chrome and Edge desktop as the first Tier 1 validation targets.
- Defer WebGL2 fallback work unless maintainers later make it an explicit requirement.
- Require visual validation so browser rendering regressions do not silently become black-canvas failures.
- Introduce a platform boundary that keeps browser setup in the web adapter and shared GPU resource ownership in renderer state.
- Move core vector rendering away from GPU line primitives by generating thick-line triangles.

## Next Horizon: DP-0002 Cross-Platform Renderer Architecture
- After DP-0001 establishes a stable browser/WebGPU baseline, split Velumin toward a platform-neutral core, shared `wgpu` renderer, browser frontend, and later native desktop frontend.
- Keep the same vector command and renderer model usable by browser and desktop targets where possible.
- Treat native `winit` work as the next staged architecture milestone, not as a blocker for the DP-0001 browser modernization.

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
- DP-0002 is staged after DP-0001 rather than replacing the browser-first modernization focus.
