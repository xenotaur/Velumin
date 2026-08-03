---
id: DP-0008
type: design_proposal
title: Browser-First Vector Frame API
status: proposed
owner: project maintainers
created: 2026-08-03
implementation_status: not_started
implemented_by: []
supersedes: []
superseded_by: null
scope: public rendering API, browser frame submission, vector commands
depends_on:
  - DP-0001
  - DP-0005
  - DP-0006
  - DP-0007
related:
  - DP-0002
  - DP-0003
related_design:
  - project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md
  - project/design/proposals/proposed/DP-0003-extensible-2d-scene-material-model.md
  - project/design/proposals/adopted/DP-0005-blasterites-tester-demo-and-visual-smoke.md
  - project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md
  - project/design/proposals/adopted/DP-0007-display-preset-public-api.md
---

# Browser-First Vector Frame API

## Summary

Define Velumin's first public browser-facing vector drawing API: a small immediate-frame submission surface that lets JavaScript games build and render one frame of vector commands through the existing WebGPU renderer.

The API is browser-first and vector-emulation-first. It should unlock Replication Vector, Blasterites-style scenes, and similar retro arcade games without turning Velumin into a game engine or adopting the full DP-0003 scene/material model.

## Background / Motivation

Velumin's project goal is to become a retro vector-graphics library for browser games inspired by Asteroids, Star Castle, and Space War. The current renderer already has the visual foundation: WebGPU rendering, CPU-generated thick vector primitives, a centered 4:3 viewport, additive glow, display presets, and deterministic Blasterites tester/tuner demos.

The missing product-facing piece is a public drawing API. Today, `velumin-core` has platform-neutral command types (`Vec2`, `Color`, `StrokeStyle`, `Line`, `Polyline`, `VectorCommand`), and the renderer internally consumes `&[VectorCommand]`, but browser consumers cannot submit their own vector frame data. Existing public browser entrypoints render built-in scenes (`render`, `render_blasterites_tester`, `render_blasterites_tuner`) rather than game-owned geometry.

This gap matters now because the motivating consumers are concrete: Replication Vector needs Star Castle-like ships, rings, blasters, obstacles, and explosions; Blasterites-style scenes need small low-poly spacecraft, irregular obstacles, bullets, debris, and glowy raster-line effects. Both can be represented by line/polyline vector commands before Velumin needs a retained scene graph, materials, masks, sprites, or native desktop shell.

The design follows DP-0002's best-practice split: game code owns simulation, the renderer owns GPU resources, and frontends own platform lifecycle. It also respects the project non-goal that Velumin should not implement full games beyond focused demos/examples.

## Prior Art Check

### Duplication search

- In-repo: Related implementation exists, but no duplicate public API was found. `velumin-core` already defines platform-neutral vector command data, and `webgpu_vector_lib` already renders internal `VectorCommand` slices. DP-0007 explicitly states that primitive submission remains a separate open design.
- Sibling repos: Potential consumers identified at `/Users/centaur/Workspace/JavascriptGames/javascript_games/blasterites` and `/Users/centaur/Workspace/ReplicationVector`. These are not Velumin API implementations.
- External libraries: Browser graphics/game libraries such as PixiJS, Phaser, Three.js, and engine-level options can render 2D/3D scenes, but adopting one would not satisfy Velumin's Rust/WASM/WebGPU vector-renderer goal.
- Recommendation: Proceed by exposing the existing Velumin command path through a narrow public browser API, rather than adopting an external renderer or duplicating game-specific adapters.

### Demand search

- Work items: No proposed work item found.
- Proposals: Found related demand:
  - DP-0001 leaves open whether the first public API should be immediate-mode drawing, retained scene data, or command-buffer based.
  - DP-0007 explicitly does not design primitive submission and leaves it as a separate open design.
  - DP-0003 proposes a broader future scene/material model but warns against delaying vector emulation behind wider material ambitions.
- Backlog: No separate matching backlog entry found.
- Recommendation: Link this proposal to DP-0001, DP-0003, and DP-0007. Create a companion work item for the first implementation slice if this proposal is accepted.

## Design Decisions

### Decision 1: Use immediate-frame submission for v1

Options considered:

- Immediate-frame submission: the game builds and submits all visible vector commands for each frame.
- Retained scene model: Velumin stores persistent objects and the game mutates them over time.
- Full command buffer model: the game builds a reusable command buffer or display list with explicit lifecycle.
- Full DP-0003 scene/material model: introduce layers, geometry, materials, transforms, masks, clips, blend modes, and opacity before exposing drawing.

Chosen: immediate-frame submission.

Immediate-frame submission is the best v1 fit for arcade-style games where ships, bullets, rings, debris, and explosions are inexpensive to regenerate each frame. It maps directly to the existing renderer, avoids object-lifetime ambiguity across the JS/WASM boundary, and leaves future retained or layered scene models open.

### Decision 2: Expose a JavaScript-friendly `VectorFrame` builder

Options considered:

- Pass nested Rust `VectorCommand` enums directly through wasm-bindgen.
- Accept raw flat typed arrays only.
- Expose a builder object with JS-friendly methods.
- Expose a string/JSON scene format.

Chosen: expose a `VectorFrame` builder.

The public browser API should let consumers construct frame data with simple methods:

```js
const frame = new VectorFrame();

frame.line(x1, y1, x2, y2, r, g, b, a, width, intensity);
frame.polyline(pointsFloat32Array, r, g, b, a, width, intensity);
frame.closedPolyline(pointsFloat32Array, r, g, b, a, width, intensity);

gpu.renderFrame(frame);
frame.clear();
```

Internally, `VectorFrame` owns a `Vec<VectorCommand>`. The builder validates input, converts JavaScript-friendly arrays into `Vec2` points, and keeps the Rust enum model behind the boundary.

### Decision 3: Keep v1 geometry stroke-first

Options considered:

- Lines and polylines only.
- Lines, polylines, and closed polylines.
- Add circles/rings as first-class geometry.
- Add filled polygons, masks, sprites, and materials.

Chosen: lines, polylines, and closed polylines.

Ships, bullets, Star Castle-like rings, asteroid outlines, sparks, shields, and low-poly obstacles can all be represented with stroke geometry. Closed polylines provide a compact ergonomic path for ship outlines, ring segments, and asteroid contours without adding fill, tessellated polygons, or material layering in v1.

Circle/ring helpers may be added as convenience methods later, but the stable core should remain polyline-based until real consumers prove which helpers are worth keeping.

### Decision 4: Preserve the existing logical playfield model

Options considered:

- Use raw clip-space coordinates.
- Use canvas pixel coordinates.
- Use Velumin's current centered 4:3 logical playfield.
- Add camera transforms and arbitrary virtual resolutions in v1.

Chosen: use the current centered 4:3 logical playfield.

The public browser frame API should document a center-origin, y-up, 800x600-style logical playfield consistent with the DP-0006 renderer direction. The renderer continues to fit that playfield into the canvas with letterbox/pillarbox behavior so browser resize does not distort game geometry.

Camera transforms, arbitrary virtual resolutions, and per-frame view controls are deferred. They are useful, but not required to render the first target games.

### Decision 5: Keep display settings global

Options considered:

- Per-frame display settings.
- Per-command glow/display settings.
- Global renderer display preset/settings.
- Full layer/material settings.

Chosen: keep display settings global for v1.

DP-0007 already defines global display preset selection and defers custom settings. This proposal should not combine primitive submission with a new display-tuning API. Each command carries stroke color, width, and intensity, while glow/preset behavior remains renderer-level.

### Decision 6: Validate with deterministic public-frame harnesses

Options considered:

- Only unit-test command conversion.
- Use real game integration as validation.
- Add deterministic harness frames using the public API.
- Rely only on manual browser inspection.

Chosen: add deterministic public-frame harnesses.

The implementation should include one or two deterministic browser examples that exercise the public API without becoming games:

1. Replication Vector-style frame:
   - low-poly ship;
   - Star Castle-like concentric/broken shield rings;
   - central object or turret;
   - blaster bolt;
   - spark/explosion cluster.

2. Blasterites-style frame:
   - arrow ship;
   - irregular asteroid/polyline obstacle;
   - bullet trail;
   - debris/sparks.

These harnesses should be usable by visual smoke checks and should coexist with existing built-in demo routes.

## Non-Goals

- Does not implement Replication Vector, Blasterites, Asteroids, Star Castle, or any playable game.
- Does not copy external game assets or imply affiliation with inspirational commercial games.
- Does not adopt DP-0003's full scene/material/layer/mask model.
- Does not add filled polygons, sprites, texture atlases, cel animation, watercolor materials, or pixel-art layers.
- Does not add native desktop, `winit`, Steam packaging, or Bevy integration.
- Does not add WebGL2 fallback.
- Does not expose internal preset glow-layer constants or create a public custom display-settings API.
- Does not remove or break existing browser entrypoints: `render`, `render_blasterites_tester`, `render_blasterites_tuner`, `create`, `createWithPreset`, or `setDisplayPreset`.

## Implementation Plan

A first implementation should be tracked by a single work item, tentatively:

- `WI-API-0001`: Expose browser vector frame submission API.

Expected implementation stages:

1. Add a `VectorFrame` wasm-bindgen class in `webgpu_vector_lib`.
   - Owns a `Vec<VectorCommand>`.
   - Provides `new`, `clear`, `len`, `is_empty`, `line`, `polyline`, and `closed_polyline` methods.
   - Converts JavaScript typed-array point data into `Vec<Vec2>`.
   - Validates malformed arrays and non-finite values with clear JS errors.

2. Add `WebGPU::render_frame(&mut self, frame: &VectorFrame) -> Result<(), JsValue>`.
   - Resizes the canvas using the existing browser path.
   - Calls the existing renderer with the frame's command slice.
   - Uses the normal Vector CRT path rather than tester-only scanline treatment unless a later decision adds public post-processing controls.

3. Add browser examples or routes that use the public API.
   - Preserve the existing baseline and Blasterites routes.
   - Add deterministic Replication Vector-style and/or public-frame demo paths.
   - Keep the demos as harnesses, not playable games.

4. Add validation.
   - Rust/unit tests for frame command construction and closed-polyline behavior.
   - Tests for invalid typed-array lengths and non-finite inputs.
   - Existing `scripts/format --check`, `scripts/lint`, `scripts/test`, and `scripts/baseline` remain green.
   - Extend screenshot/visual smoke only if the browser automation path remains reliable.

5. Document the public API.
   - README or web demo docs show minimal JavaScript usage.
   - Document coordinate system, color/intensity ranges, width semantics, and display preset interaction.
   - Mark the API as v1 and additive; future scene/layer/material APIs may build on it.

## Acceptance Criteria

- Browser JavaScript can create a `VectorFrame`, append line/polyline/closed-polyline commands, and render it through `WebGPU.renderFrame`.
- The implementation reuses `velumin-core` command types and the existing WebGPU renderer path.
- The API supports ships, ring/obstacle outlines, blaster bolts, and spark/explosion clusters without game-specific concepts.
- Existing demo entrypoints continue to work.
- The coordinate system and style parameters are documented.
- Deterministic public-frame harness output exists for at least one Replication Vector / Blasterites-style scene.
- Local validation remains green.
- The project still does not claim to implement a full game or a general-purpose scene/material engine.

## Risks

- Premature API stability: once JavaScript method names and parameter order are public, changing them is breaking. Mitigate by keeping v1 small and documenting only the stable minimum.
- JS/WASM ergonomics: raw Rust enums are not pleasant across wasm-bindgen. Mitigate with the `VectorFrame` builder.
- Performance: rebuilding and uploading frame commands every frame may become expensive for complex levels. Mitigate later with batching, capacity reuse, or retained buffers once real workloads justify them.
- Scope creep: adding cameras, layers, fills, sprites, and materials here would duplicate DP-0003 too early. Keep v1 stroke-first.
- Visual claims: deterministic harnesses are evidence for rendering behavior, not proof of full game readiness.

## Open Questions

- Should v1 expose camelCase JavaScript names only (`closedPolyline`, `renderFrame`) while Rust uses snake_case, or should generated names be explicitly annotated for all public methods?
- Should `polyline` accept only `Float32Array`, or also plain JavaScript arrays for convenience?
- Should `VectorFrame` expose a capacity reservation method in v1, or defer performance controls until real consumer pressure appears?
- Should the first deterministic public-frame harness replace any existing demo route, or add a new route such as `?demo=frame-api`?

## Cross-References

- `project/goal/project_goal.md`
- `project/focus/current_focus.md`
- `project/status/current_status.md`
- `project/design/proposals/adopted/DP-0001-modern-webgpu-rendering.md`
- `project/design/proposals/adopted/DP-0005-blasterites-tester-demo-and-visual-smoke.md`
- `project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md`
- `project/design/proposals/adopted/DP-0007-display-preset-public-api.md`
- `project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md`
- `project/design/proposals/proposed/DP-0003-extensible-2d-scene-material-model.md`
