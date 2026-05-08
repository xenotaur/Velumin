---
id: DP-0001
title: Modern WebGPU-First Rendering Path
status: adopted
owner: project maintainers
created: 2026-05-07
adopted: 2026-05-08
scope: webgpu_vector_lib
---

# Modern WebGPU-First Rendering Path

## Summary
Upgrade Velumin from the previous `wgpu 0.16` + WebGL compatibility path to a modern `wgpu` WebGPU-first renderer. The goal is to keep the repaired browser smoke test as a baseline, then migrate the rendering stack toward native browser WebGPU for predictable shaders, render-to-texture passes, and glow/compositing effects.

## Adoption Status
Adopted on 2026-05-08 after completion of the DP-0001 workstream:

- `WI-RENDER-0001`: preserve browser rendering baseline;
- `WI-RENDER-0002`: upgrade to modern WebGPU-first `wgpu`;
- `WI-RENDER-0003`: introduce platform boundary and capability handling;
- `WI-RENDER-0004`: add minimal vector primitive API and thick-line tessellation;
- `WI-RENDER-0005`: spike glow pipeline after modern baseline.

Implementation evidence is recorded in `project/evidence/EV-0002.md` through `project/evidence/EV-0006.md`. Production glow tuning and broader architecture splitting remain follow-up work under later design proposals.

## Context
Velumin is intended to be a retro vector-graphics library suitable for Space War, Asteroids, and Star Castle-like visuals. That implies:

- crisp vector primitives;
- controllable line thickness and intensity;
- raster glow and bloom-like compositing;
- predictable browser behavior;
- eventual room for native/standalone builds.

At proposal time, the implementation was a minimal WASM browser harness using `wgpu = "0.16"` with the `webgl` feature. As of this proposal, docs.rs listed `wgpu 29.0.1` as the latest crate line and showed `0.16.3` as a July 2023 release. Current `wgpu` feature metadata included `webgpu` as a default feature and `webgl` as a separate compatibility feature.

## Problem
The existing path works only as a fragile prototype:

- The original Rust logging binding imported a global JS `log()` function that the page did not define.
- `PrimitiveTopology::LineList` was a poor browser smoke test on the old WebGL compatibility backend.
- The canvas did not have stable CSS dimensions, making DPR backing-store resizing interact badly with layout.
- Older WGSL/Naga validation rejected shader patterns that should be easy to express on a modern stack.
- The project currently has no documented backend strategy.

These issues are manageable for a smoke test, but they are a warning sign for glow-heavy vector rendering.

## Goals
- Make native browser WebGPU the primary web rendering backend.
- Preserve a known-good visual smoke test before the migration starts.
- Define rendering layers that can later support native apps without duplicating vector logic.
- Establish enough validation that future backend changes do not silently return to a black canvas.
- Keep WebGL2 compatibility optional and explicitly lower priority.

## Non-Goals
- Do not design the full public Velumin game API in this proposal.
- Do not implement Asteroids, Star Castle, or Space War as complete games.
- Do not require WebGL fallback in the first upgrade milestone.
- Do not add glow/bloom before a modern baseline can draw reliable geometry.

## Proposed Direction
Adopt a WebGPU-first architecture with a small platform boundary:

- `core`: vector commands, colors, line widths, transforms, scene/frame data.
- `renderer`: `wgpu` resource management, pipelines, render passes, buffers, and textures.
- `web`: WASM/browser canvas setup, event loop, resize handling, and demo harness.
- `native` later: window/surface setup for desktop apps using the same core and renderer where possible.

The first rendering primitive should be a thick line generated as triangles. Avoid GPU line primitives as a core abstraction; browsers and backends differ too much for retro-vector aesthetics.

## Browser Compatibility Strategy
Use explicit support tiers so WebGPU-first does not accidentally imply universal browser support.

| Tier | Target | Expected Support | Notes |
| --- | --- | --- | --- |
| Tier 1 | Chrome and Edge desktop on Windows and macOS | Primary supported web target | Best first target for the migration and visual smoke tests. |
| Tier 1 | Chrome and Edge on ChromeOS | Primary supported web target | Validate if hardware is available. |
| Tier 2 | Chrome on Android 12+ with supported Qualcomm/ARM GPUs | Supported after device testing | Treat mobile GPU and driver coverage as empirical, not assumed. |
| Tier 2 | Safari on current macOS, iOS, and iPadOS releases with WebGPU enabled by default | Supported after Apple-device testing | Track exact OS/browser minimums in release notes once confirmed. |
| Tier 2 | Firefox on currently supported WebGPU platforms | Supported after platform testing | Track platform gaps explicitly; do not assume parity with Chromium. |
| Tier 3 | Linux browsers, older mobile devices, older Safari/Firefox versions, and browser/device combinations with blocked adapters | Best-effort or unsupported | Show a clear unsupported/limited-GPU message unless a fallback backend is intentionally added. |

Development builds may run on `localhost`, but deployed browser demos require secure contexts because WebGPU is only exposed in secure browser contexts. The web adapter should detect unsupported browsers, missing adapters, blocked adapters, and insufficient limits/features before constructing the renderer.

## Migration Plan

### Phase 0: Baseline
- Keep the current white-line smoke test passing in the browser.
- Ensure `cargo check --target wasm32-unknown-unknown`, `wasm-pack build --target web`, and `npx vite build` pass.
- Document the local demo command.
- Add a tiny visual verification note or script that checks for non-black/white pixels.

### Phase 1: Dependency Upgrade
- Upgrade `wgpu` from `0.16` to the current modern line.
- Prefer native browser WebGPU features over the `webgl` feature.
- Update `web-sys`, `wasm-bindgen`, and related features only as required by the new `wgpu` API.
- Keep the smoke test visually identical after the migration.

### Phase 2: Platform Boundary
- Move browser-specific setup out of the renderer core.
- Introduce a small renderer state type that owns device, queue, surface config, pipelines, and resize behavior.
- Keep canvas lookup, DPR calculation, and browser logging in the web adapter.
- Add startup capability negotiation in the web adapter:
  - detect missing `navigator.gpu`;
  - handle `requestAdapter()` returning no adapter;
  - choose the preferred canvas format through the platform API;
  - check required features, limits, and texture capabilities before creating pipelines;
  - report a user-facing unsupported or degraded-capability state rather than panicking or showing a black canvas.
- Add explicit resize/reconfigure behavior.

### Phase 3: Vector Primitive API
- Introduce a minimal command representation, for example:
  - `Line { start, end, width, color, intensity }`
  - `Polyline { points, width, color, intensity }`
- Generate triangles for thick vector strokes.
- Batch primitives into buffers rather than hardcoding geometry in WGSL.
- Keep shader-generated geometry only for smoke tests or fullscreen passes.

### Phase 4: Glow Pipeline
- Render bright vector geometry into an offscreen texture.
- Add blur/downsample passes for glow.
- Composite glow plus crisp core lines to the surface.
- Expose a small set of retro-display controls such as glow radius, intensity, persistence, and background fade.

### Phase 5: Optional Compatibility Work
- Decide whether a WebGL2 compatibility backend is worth maintaining.
- If yes, treat it as a lower-fidelity fallback with its own validation expectations.
- If no, document browser requirements clearly.

## Acceptance Criteria
- Browser demo shows a visible white line on black after a clean rebuild.
- Browser console reaches setup, pipeline creation, render call, and frame presentation without errors.
- The project can run a documented build path from a clean checkout.
- Rendered output is verified with a screenshot or pixel-level smoke check.
- The renderer no longer depends on WebGL line primitives for core line rendering.
- The upgraded dependency set is reflected in `Cargo.toml` and `Cargo.lock`.
- Tier 1 browser targets are verified before the migration is considered complete.
- Tier 2 browser targets are either verified or explicitly listed as unverified with known follow-up work.
- Unsupported-browser and no-adapter paths show a clear message and do not leave a blank canvas as the only signal.
- Visual validation records browser, OS, GPU/adapter name when available, and whether the backend is core WebGPU, compatibility mode, or fallback.

## Risks
- Modern `wgpu` APIs may require nontrivial changes to surface creation, device descriptors, lifetime handling, and WASM features.
- Native browser WebGPU availability may limit supported browsers compared with WebGL2.
- Some users may have WebGPU disabled by browser policy, GPU blocklists, driver issues, insecure contexts, or unsupported hardware even when their browser family nominally supports WebGPU.
- Maintaining both WebGPU and WebGL2 backends could slow development.
- Glow passes will require more validation than the current single-pass smoke test.

## Open Questions
- Which browsers should Velumin officially support first?
- Should the crate remain named `webgpu_vector_lib`, or should it become `velumin` before API work begins?
- Should visual verification use Playwright/browser automation, a local screenshot script, or a checked-in example harness?
- Should the first public API be immediate-mode drawing, retained scene data, or command-buffer based?
- Is WebGL2 fallback a real project requirement or only a nice-to-have?

## References
- Current project evidence: `webgpu_vector_lib/Cargo.toml`, `webgpu_vector_lib/src/lib.rs`, `webgpu_vector_lib/shaders/line.wgsl`, `webgpu_vector_lib/web/index.html`.
- `wgpu` latest docs: https://docs.rs/crate/wgpu/latest
- `wgpu` feature flags: https://docs.rs/crate/wgpu/latest/features
- `wgpu 0.16.3` docs: https://docs.rs/crate/wgpu/0.16.3
