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
- DP-0006 (Vector CRT renderer) is adopted (2026-07-24) and partially implemented; its remaining follow-ups are tracked, not blocking.

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

## Adopted Direction: DP-0006 Vector CRT Renderer Migration
- Adopted 2026-07-24 as the project's vector-display rendering direction; `implementation_status: partial`.
- Landed: a fixed 4:3 centered viewport, additive multi-layer glow compositing, and an internal `VectorDisplayPreset` set with `ArcadeBalanced` as the tuning target — validated by code inspection (`EV-0008`) and browser visual-smoke capture (`EV-0009`). The non-4:3 resize/letterbox path is now captured by the WI-SMOKE-0001 smoke check at wide and tall sizes.
- Follow-up work under the adopted direction: the automatable/committed screenshot smoke check with wide/tall resize capture (`WI-SMOKE-0001`, done), and the public preset API — decided by the adopted `DP-0007` and implemented in `WI-PRESET-0001`, which also captured the non-default presets. DP-0007's public custom display-settings API remains a deferred follow-up.

## Next Horizon: DP-0002 Cross-Platform Renderer Architecture
- After the Vector CRT renderer stabilizes, split Velumin toward a platform-neutral core, shared `wgpu` renderer, browser frontend, and later native desktop frontend.
- Keep the same vector command and renderer model usable by browser and desktop targets where possible.
- Treat native `winit` work as a staged architecture milestone, not as a blocker for browser work.
- Phase 1's platform-neutral type extraction is done: the repository is now a Cargo workspace with a `velumin-core` crate (`WI-ARCH-0001`, PR #17). Phase 2 (a reusable, surface-agnostic `wgpu` renderer state, plus desktop-side adapter/capability negotiation) was selected as the active next workstream on 2026-07-31 (`project/memory/decision_log.md`); no work item is scoped yet. Phase 3 (native `winit` shell) remains future work behind Phase 2.

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
