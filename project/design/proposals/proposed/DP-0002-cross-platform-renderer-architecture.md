---
id: DP-0002
title: Cross-Platform Vector Renderer Architecture
status: proposed
owner: project maintainers
created: 2026-05-07
scope: velumin architecture
depends_on:
  - DP-0001
related:
  - DP-0003
  - DP-0005
  - DP-0006
  - DP-0007
---

# Cross-Platform Vector Renderer Architecture

## Summary
Build Velumin as a platform-neutral Rust vector graphics renderer with two first-class frontends:

- a browser/WASM frontend for web games;
- a native `winit` desktop shell for Steam-targeted games on Windows, Linux, and macOS.

Use `wgpu` as the shared rendering backend for both targets. Treat Bevy integration as a later consumer/plugin layer, not as the foundation of the core renderer.

## Context
Velumin's intended product shape is a retro vector-graphics library for games that look like Space War, Asteroids, and Star Castle, with crisp geometry and raster glow. The project should support browser games and a native desktop path suitable for Steam distribution.

DP-0001 covers the WebGPU-first browser rendering upgrade. This proposal extends that direction into the broader architecture needed to support desktop games without turning Velumin into a single-engine game project.

### Status as of 2026-07-25 (refresh)
This proposal predates several adopted proposals that substantially filled in the *content* this architecture is meant to carry, without changing the architecture decision itself:

- **DP-0001** (adopted) delivered the modern `wgpu`-first browser renderer this proposal builds on.
- **DP-0005** (adopted) added the deterministic Blasterites tester/tuner demos — a richer validation scene than the original white-line baseline, still browser-only.
- **DP-0006** (adopted, partial) added the fixed 4:3 viewport, additive multi-layer glow/composite passes, and the internal `VectorDisplayPreset` model — i.e. Milestone Phase 4 ("Vector Commands and Glow") is largely done.
- **DP-0007** (adopted) promoted the display-preset model to a small public API (`VectorDisplayPreset`, `WebGPU::create_with_preset`/`set_display_preset`), and `WI-SMOKE-0001` added a scripted screenshot smoke check (`scripts/smoke`) — covering the "keep pixel-level or screenshot-based validation" goal in Phase 0.

None of this reduces the scope of DP-0002: **everything above still lives inside the single `webgpu_vector_lib` crate**, fused with the browser/WASM adapter (see Current Implementation Boundary below). No crate split exists, and no native `winit` frontend exists. What DP-0002 still needs to deliver is unchanged — extracting platform-neutral boundaries and adding a native desktop target — but the Milestones below are updated to reflect that the *rendering content* for Phases 0/2/4 already exists and needs to be *relocated*, not built from scratch.

### Current Implementation Boundary (as of 2026-07-25)
- The repository is a single crate, `webgpu_vector_lib` (`webgpu_vector_lib/Cargo.toml`), not yet the `velumin-*` workspace this proposal describes.
- `webgpu_vector_lib/src/lib.rs` is a single ~1,800-line file mixing platform-neutral vector/scene logic, the shared `wgpu` renderer, and the browser/`wasm-bindgen`/`web-sys` adapter, gated throughout with `#[cfg(target_arch = "wasm32")]` / `#[cfg_attr(...)]`.
- There is no `winit` dependency and no native desktop example; the only frontend is the browser (`webgpu_vector_lib/web/`, Vite).
- A screenshot-based visual smoke check already exists (`scripts/smoke`, Playwright-driven), ahead of where Phase 0 originally expected it.

## Decision
Adopt a layered Rust architecture:

- `velumin-core`: platform-neutral vector scene data, commands, math types, colors, timing-independent render parameters, and validation.
- `velumin-renderer-wgpu`: shared `wgpu` renderer for buffers, pipelines, textures, offscreen passes, glow/composite passes, and surface rendering.
- `velumin-web`: WASM/browser adapter for canvas discovery, browser capability checks, resize/DPR behavior, animation frame scheduling, and JavaScript bindings.
- `velumin-desktop`: native `winit` adapter for window creation, event loop, input collection, resize handling, fullscreen/windowed modes, and Steam-friendly executable packaging.
- `velumin-bevy` later: optional Bevy plugin that translates Bevy world data or resources into Velumin render commands.

The core rendering model should expose commands or frame data rather than require a specific game loop, ECS, or engine. Frontends own platform lifecycle; the renderer owns GPU resources; game code owns simulation.

## Architecture

```text
Game / Example
  |
  v
velumin-core
  - vector commands
  - scene/frame description
  - style, intensity, glow parameters
  - validation and CPU-side tessellation helpers
  |
  v
velumin-renderer-wgpu
  - device/surface-independent renderer state
  - pipelines, buffers, textures
  - thick-line triangle generation path
  - glow and composite passes
  |
  +--------------------------+
  |                          |
  v                          v
velumin-web              velumin-desktop
  - wasm-bindgen          - winit event loop
  - canvas setup          - native windows
  - browser WebGPU        - Steam-friendly app shell
  - requestAnimationFrame - desktop input/fullscreen
```

## Target Frontends

### Browser/WASM
The browser frontend should remain the fastest way to inspect and share demos. It should:

- run through native browser WebGPU when available;
- detect missing/blocked WebGPU adapters before renderer creation;
- expose a small JavaScript API for examples and game loops (a first slice of this now exists: `WebGPU.create`, `WebGPU.createWithPreset`, and instance method `gpu.setDisplayPreset`, DP-0007);
- preserve visual smoke tests from DP-0001 (now scripted via `scripts/smoke`, WI-SMOKE-0001);
- keep WebGL2 fallback optional and explicitly out of scope until chosen.

### Native Desktop With `winit`
The desktop frontend should be the Steam path. It should:

- create native windows on Windows, Linux, and macOS;
- initialize the same `wgpu` renderer used by the web frontend;
- support resize, fullscreen/windowed mode, high-DPI scaling, keyboard/gamepad-ready input plumbing, and close/suspend/resume behavior;
- produce ordinary platform executables that can be packaged for Steam;
- keep Steamworks integration separate from the graphics library.

### Bevy Integration Later
The Bevy layer should be optional. It should:

- depend on Velumin, not the other way around;
- map Bevy components/resources into Velumin frame commands;
- use Bevy's scheduling and app lifecycle where appropriate;
- avoid forcing ECS or Bevy render-graph concepts into `velumin-core`.

## Crate and Package Shape
The exact names may change if the project renames `webgpu_vector_lib` to `velumin`, but the boundary should be clear:

| Package | Purpose | Depends On |
| --- | --- | --- |
| `velumin-core` | Public vector command and scene model | Rust std/core math dependencies only |
| `velumin-renderer-wgpu` | Shared GPU renderer | `velumin-core`, `wgpu`, `bytemuck` |
| `velumin-web` | Browser/WASM frontend | `velumin-core`, `velumin-renderer-wgpu`, `wasm-bindgen`, `web-sys` |
| `velumin-desktop` | Native desktop shell and examples | `velumin-core`, `velumin-renderer-wgpu`, `winit` |
| `velumin-bevy` | Optional Bevy plugin | `velumin-core`, Bevy, possibly `velumin-renderer-wgpu` depending on integration depth |

## Best-Practice Rationale
- Keep platform lifecycle at the edges. Web canvas setup and native window/event-loop setup should not leak into core vector APIs.
- Share renderer code where the graphics API is shared. `wgpu` gives a common abstraction over browser WebGPU and native APIs.
- Avoid GPU line primitives as a core dependency. Generate thick vector strokes as triangles for predictable appearance.
- Treat glow as a renderer feature, not a frontend feature. Browser and desktop should use the same offscreen/composite pipeline when possible.
- Keep Steam concerns outside Velumin's core. Steam packaging, Steamworks APIs, achievements, and overlay support belong in game/application layers or examples.
- Make Bevy optional. A Bevy plugin is valuable, but making Bevy the core would turn Velumin into an engine-specific renderer.

## Milestones

Status tags below reflect the 2026-07-25 refresh plus the 2026-07-30 Phase 1 update (WI-ARCH-0001, PR #17), which converted the repository into a Cargo workspace (`velumin-core` + `webgpu_vector_lib`). "Done (browser-only)" means the capability exists but only inside `webgpu_vector_lib` — not yet behind the remaining platform-neutral boundaries this proposal defines (the renderer/browser-adapter split, in particular).

### Phase 0: Preserve Current Browser Baseline — Done (browser-only)
- Keep the current browser white-line smoke test passing. — done; preserved through DP-0001/DP-0005/DP-0006 and covered by `scripts/smoke`.
- Document build/run commands. — done (`README.md`, `scripts/README.md`).
- Keep pixel-level or screenshot-based validation for "not a black canvas." — done and exceeded: `scripts/smoke` (Playwright) asserts structural pixel properties for the baseline, tester (pre/post-impact), tuner, and all four display presets at 4:3, and additionally exercises wide and tall (non-4:3) viewports for the Blasterites tester specifically. The baseline, tuner, and non-default presets are not yet captured at wide/tall.

### Phase 1: Rename and Split Boundaries — Partially done
- Decide whether to rename `webgpu_vector_lib` to `velumin`. — still undecided; explicitly deferred by [WI-ARCH-0001](../../../work_items/resolved/WI-ARCH-0001.md)'s scoping decision (`project/memory/decision_log.md`, 2026-07-30).
- Extract platform-neutral command types. — done: `WI-ARCH-0001` (PR #17) extracted `velumin-core` (zero `wasm-bindgen`/`web-sys`/`wgpu` dependency) holding `Vec2`, `Color`, `StrokeStyle`, `Line`, `Polyline`, `VectorCommand`, `VectorDisplaySettings`, `GlowLayer`, `RenderViewport`, and the pure geometry helpers. The repository is now a Cargo workspace of `velumin-core` and `webgpu_vector_lib`. `VectorDisplayPreset` stays in `webgpu_vector_lib` since it must remain `wasm-bindgen`-exportable.
- Isolate browser setup from renderer setup. — not started at the crate level; `velumin-renderer-wgpu` / `velumin-web` remain unsplit inside `webgpu_vector_lib` (deferred to a future phase per WI-ARCH-0001's Non-Goals, safer to validate once Phase 3 exists).
- Keep one browser example working throughout. — done; `scripts/smoke` reported 9/9 checks at MAD 0.000 (zero visual regression) after the split.

The command-type extraction is done; the renderer/browser-adapter split remains the concrete prerequisite for Phase 3.

### Phase 2: Modern Shared `wgpu` Renderer — Partially done (browser-only)
- Complete the DP-0001 `wgpu` upgrade. — done (DP-0001 adopted).
- Introduce a reusable renderer state that can render to any supported `wgpu::Surface`. — not done: `Renderer`/`WebGPU::create` in `webgpu_vector_lib/src/lib.rs` is constructed from a browser `HtmlCanvasElement` and is not surface-agnostic.
- Add explicit adapter/capability negotiation for web and desktop. — web-side capability checks exist (missing/blocked WebGPU adapter handling); desktop-side negotiation does not exist (no native surface path at all).

### Phase 3: Native `winit` Shell — Not started
- No `winit` dependency exists in `webgpu_vector_lib/Cargo.toml`, and no desktop example or binary exists in the repository. All bullets in this phase remain as originally scoped.

### Phase 4: Vector Commands and Glow — Partially done (browser rendering complete; cross-platform requirement outstanding)
- Add line/polyline command ingestion. — done (`VectorCommand::Line`/`Polyline`).
- Batch thick vector geometry into GPU buffers. — done (CPU-tessellated thick-line triangles, DP-0001).
- Add offscreen glow/composite passes. — done (additive multi-layer glow + composite, DP-0006), now with a public preset selector (DP-0007) and a smoke check covering all four presets.
- Keep browser and desktop outputs visually comparable. — not done and not yet assessable: there is no desktop output to compare against (Phase 3 has not started). This phase cannot be marked fully done until Phase 3 exists and comparability is validated.

### Phase 5: Steam Packaging Spike — Not started
- Create a minimal desktop build artifact for Windows, Linux, and macOS.
- Document packaging requirements, including macOS notarization and Linux 64-bit expectations.
- Keep Steamworks SDK/API integration optional until a game needs Steam-specific features.

### Phase 6: Bevy Plugin Spike — Not started
- Add an experimental `velumin-bevy` integration.
- Render Velumin commands from a Bevy app.
- Keep this behind a feature flag or separate package until stable.

## Acceptance Criteria
- Browser frontend renders a smoke scene through the shared renderer.
- Native `winit` frontend renders the same smoke scene through the shared renderer.
- The core vector API has no dependency on `wasm-bindgen`, `web-sys`, `winit`, Steamworks, or Bevy.
- The renderer does not require WebGL line primitives for core vector rendering.
- Desktop build instructions exist for Windows, Linux, and macOS.
- Unsupported-adapter paths report a clear error state instead of showing only a blank window or canvas.
- Bevy integration remains optional and does not constrain the core API.

## Tradeoffs

### Advantages
- One rendering model can serve both browser and Steam-style desktop games.
- Rust code remains central across all targets.
- `wgpu` reduces backend-specific rendering work.
- `winit` provides a direct native desktop path without embedding a browser runtime.
- Bevy can be supported later without dictating the core design.

### Costs
- Velumin must own more infrastructure than a pure Bevy plugin or web-only library.
- Desktop examples require platform testing and packaging work.
- Native input, audio, assets, save data, and Steamworks remain outside the renderer and must be chosen separately.
- `wgpu` upgrades may require active maintenance as APIs evolve.

## Alternatives Considered

### Webview App Wrapper
Package the browser build in Electron or Tauri.

- Pros: maximum web-code reuse and fast path to desktop-like packaging.
- Cons: less direct control over GPU behavior, larger/runtime-dependent packaging tradeoffs, and weaker proof that Velumin is a native-capable graphics library.

### Bevy-First Design
Build Velumin primarily as a Bevy plugin.

- Pros: faster access to game systems and cross-platform game scaffolding.
- Cons: couples Velumin to Bevy's ECS/render lifecycle and makes non-Bevy consumers secondary.

### Commercial Engine Integration
Integrate Velumin-like visuals into Unity, Godot, or Unreal.

- Pros: mature game/export tooling.
- Cons: does not build on the existing Rust/WASM/`wgpu` direction and weakens Velumin's identity as a Rust graphics library.

## Risks
- Desktop platform parity may take longer than browser parity.
- Linux graphics/windowing differences may require more validation than Windows/macOS.
- macOS Steam distribution adds signing/notarization requirements outside renderer code.
- Steam Deck expectations may imply controller, performance, resolution, and Proton/native decisions not covered by the renderer itself.
- Bevy integration may diverge from direct `wgpu` integration if attempted too early.

## Open Questions
- Should Velumin provide only rendering, or also a tiny game-loop helper for examples?
- Which input/gamepad library should desktop examples use?
- Should audio be deliberately out of scope, or should examples pick a minimal audio crate?
- Should the native desktop target prioritize native Linux builds, Windows builds under Proton, or both for Steam Deck?
- What level of visual equivalence is required between browser and desktop outputs?

## References
- DP-0001: `project/design/proposals/adopted/DP-0001-modern-webgpu-rendering.md`
- DP-0005: `project/design/proposals/adopted/DP-0005-blasterites-tester-demo-and-visual-smoke.md`
- DP-0006: `project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md`
- DP-0007: `project/design/proposals/adopted/DP-0007-display-preset-public-api.md`
- Current single-crate implementation: `webgpu_vector_lib/src/lib.rs`, `webgpu_vector_lib/Cargo.toml`
- `wgpu` docs: https://docs.rs/crate/wgpu/latest
- `winit` docs: https://docs.rs/winit/latest/winit/
- Steam platforms documentation: https://partner.steamgames.com/doc/store/application/platforms
- Steamworks SDK documentation: https://partner.steamgames.com/doc/sdk
- Bevy project site: https://bevy.org/
