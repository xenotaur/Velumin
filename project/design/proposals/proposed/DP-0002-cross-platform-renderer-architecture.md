---
id: DP-0002
title: Cross-Platform Vector Renderer Architecture
status: proposed
owner: project maintainers
created: 2026-05-07
scope: velumin architecture
depends_on: DP-0001
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
- expose a small JavaScript API for examples and game loops;
- preserve visual smoke tests from DP-0001;
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

### Phase 0: Preserve Current Browser Baseline
- Keep the current browser white-line smoke test passing.
- Document build/run commands.
- Keep pixel-level or screenshot-based validation for "not a black canvas."

### Phase 1: Rename and Split Boundaries
- Decide whether to rename `webgpu_vector_lib` to `velumin`.
- Extract platform-neutral command types.
- Isolate browser setup from renderer setup.
- Keep one browser example working throughout.

### Phase 2: Modern Shared `wgpu` Renderer
- Complete the DP-0001 `wgpu` upgrade.
- Introduce a reusable renderer state that can render to any supported `wgpu::Surface`.
- Add explicit adapter/capability negotiation for web and desktop.

### Phase 3: Native `winit` Shell
- Add a desktop example binary.
- Create a window with `winit`.
- Initialize `wgpu` against that native surface.
- Render the same visual smoke scene as the browser frontend.
- Validate on at least one machine per target OS before calling the phase complete.

### Phase 4: Vector Commands and Glow
- Add line/polyline command ingestion.
- Batch thick vector geometry into GPU buffers.
- Add offscreen glow/composite passes.
- Keep browser and desktop outputs visually comparable.

### Phase 5: Steam Packaging Spike
- Create a minimal desktop build artifact for Windows, Linux, and macOS.
- Document packaging requirements, including macOS notarization and Linux 64-bit expectations.
- Keep Steamworks SDK/API integration optional until a game needs Steam-specific features.

### Phase 6: Bevy Plugin Spike
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
- `wgpu` docs: https://docs.rs/crate/wgpu/latest
- `winit` docs: https://docs.rs/winit/latest/winit/
- Steam platforms documentation: https://partner.steamgames.com/doc/store/application/platforms
- Steamworks SDK documentation: https://partner.steamgames.com/doc/sdk
- Bevy project site: https://bevy.org/
