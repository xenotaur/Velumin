---
id: DP-0003
title: Extensible 2D Scene and Material Model
status: draft
owner: project maintainers
created: 2026-05-08
scope: velumin core graphics model
depends_on:
  - DP-0001
  - DP-0002
---

# Extensible 2D Scene and Material Model

## Summary
Design `velumin-core` around a general 2D scene, layer, geometry, material, mask, and compositing model rather than hardcoding vector-monitor emulation into the core API.

Vector graphics emulation remains the first implemented style. The core model should also be able to support three planned style families:

- watercolored sketches: 2D outlines with dynamic watercolor-like interior shading;
- traditional cel animation: layered image/sprite animation with timing and compositing;
- pixelated old-monitor graphics: low-resolution pixel art with CRT/arcade-style post-processing.

## Context
DP-0001 proposes a modern WebGPU-first rendering path. DP-0002 proposes a cross-platform architecture with a browser frontend, a native `winit` frontend, and later Bevy integration. This proposal addresses a separate question: what graphics model should Velumin expose so future styles can reuse the same core library?

The answer should protect Velumin's original identity as a retro vector-graphics library while preventing the core model from becoming too narrow.

## Decision
Adopt a style-extensible 2D scene model:

```text
Scene
  Layers
    DrawCommand
      Geometry
      Material
      Transform
      Mask / Clip
      BlendMode
      Opacity
      Timing / Animation Reference
```

The renderer may implement each material family with different pipelines and passes, but the platform-neutral core should describe scenes and draw intent without depending on web, `winit`, Bevy, or a single visual style.

## Goals
- Keep vector emulation as the first concrete implementation target.
- Make room for watercolor-like fills, cel animation, and pixel-monitor effects without redesigning the core.
- Support layered composition as a first-class concept.
- Support masks/clips so fills can live inside outlines or shapes.
- Support render-to-texture workflows for glow, watercolor washes, and monitor post-processing.
- Keep physically accurate paint simulation out of scope unless a later research proposal accepts that complexity.

## Non-Goals
- Do not implement all material families immediately.
- Do not turn Velumin into a complete game engine.
- Do not require Bevy, Steamworks, web APIs, or `winit` in the core graphics model.
- Do not promise physically accurate watercolor, paper, pigment diffusion, or fluid simulation.
- Do not force all styles to share one shader or one pipeline.

## Core Concepts

### Scene
A scene is an ordered collection of layers and global render settings. It should be deterministic and platform-neutral.

### Layer
A layer groups draw commands and compositing behavior. Layers may support:

- visibility;
- opacity;
- blend mode;
- transform;
- optional render-to-texture isolation;
- optional masks or clips.

Layer isolation is important because watercolor, glow, and pixel-monitor effects often need an offscreen texture before compositing.

### Geometry
Initial geometry families should include:

- line and polyline;
- closed path or polygon;
- quad/rect;
- sprite quad;
- mask shape.

Vector strokes should render as generated triangles rather than GPU line primitives.

### Material
A material describes how geometry should be shaded. Materials are style-specific but should share enough structure to be schedulable by the renderer.

Initial material families:

| Material | Purpose | Implementation Notes |
| --- | --- | --- |
| `VectorGlowMaterial` | Retro vector lines, glow, intensity, persistence | First implementation priority. Uses thick strokes and post-processing. |
| `WatercolorMaterial` | Dynamic painterly fills inside outlines or masks | Uses masks, noise/texture washes, translucent layering, and blend modes. |
| `CelMaterial` | Flat-color or sprite-based cel animation | Uses image frames, texture atlases, transforms, and ordered layers. |
| `PixelMonitorMaterial` | Pixel art with old arcade/CRT display treatment | Uses low-resolution render targets, nearest-neighbor sampling, scanlines/glow/noise/composite passes. |

### Mask / Clip
Masks and clips allow one visual element to constrain another. They are required for watercolor fills inside sketch outlines and useful for cel and pixel effects.

### Blend Mode
At minimum, the model should anticipate:

- normal/source-over;
- additive;
- multiply;
- screen or lighten;
- alpha mask composition.

The exact first implementation may be smaller, but the model should not assume only opaque drawing.

### Animation Reference
Cel animation needs timing and frame selection. The core does not need to own a full animation system at first, but draw commands should be able to reference:

- an image/frame id;
- a frame index or time;
- a transform;
- layer order.

## Feasibility Assessment

### Vector Emulation
Feasibility: high.

This remains the baseline. Thick lines, glow, and compositing are natural uses of GPU geometry, offscreen textures, and post-processing.

### Watercolored Sketches
Feasibility: medium-high.

The feasible version is stylized watercolor, not physical paint simulation. A practical implementation can combine:

- outline geometry;
- closed-shape masks;
- procedural noise or texture samples;
- translucent layered fills;
- edge darkening or bleeding;
- multiply/overlay-like blending.

This depends on masks, blend modes, and offscreen render targets. Those are compatible with the DP-0001/DP-0002 renderer plan.

### Traditional Cel Animation
Feasibility: high.

Cel animation follows naturally from layers, sprites, transforms, timing, and compositing. It is not free, because Velumin still needs texture assets, frame references, atlas support, and animation timing conventions. It is, however, a low-risk extension once layers and textured quads exist.

### Pixelated Old-Monitor Graphics
Feasibility: very high.

This style aligns strongly with vector emulation because both rely on display simulation. The likely approach:

- render pixel art or low-resolution scene layers into a virtual framebuffer;
- upscale using nearest-neighbor sampling;
- apply scanlines, glow, color bleed, shadow-mask/phosphor effects, curvature if desired, noise, and persistence.

This should reuse much of the post-processing architecture built for vector glow.

## Implementation Staging

### Phase 0: Protect Vector Baseline
- Keep the current visible vector-line smoke test passing.
- Do not expand scope until baseline browser and desktop rendering paths are stable enough to verify.

### Phase 1: Core Scene Model
- Define `Scene`, `Layer`, `DrawCommand`, `Geometry`, `Material`, `BlendMode`, and `Mask` types.
- Keep these types independent of platform frontends.
- Implement only the minimum needed for vector strokes.

### Phase 2: Vector Material
- Implement `VectorGlowMaterial`.
- Support thick line/polyline rendering.
- Add glow/composite passes.
- Validate output in browser and native desktop paths from DP-0002.

### Phase 3: Texture and Layer Infrastructure
- Add image/texture handles.
- Add sprite quad support.
- Add texture atlas or frame-region support.
- Add layer opacity and ordering.

### Phase 4: Pixel-Monitor Material
- Add virtual-resolution render targets.
- Add nearest-neighbor upscale path.
- Add scanline/glow/noise monitor effects.
- Validate with a simple Centipede/Donkey Kong-like pixel scene, without implementing a full game.

### Phase 5: Cel Material
- Add frame references and timing helpers.
- Add sprite/cel layer examples.
- Validate layered animation with transparent backgrounds and ordered composition.

### Phase 6: Watercolor Material
- Add closed-shape masks and fill regions.
- Add procedural or texture-based wash shading.
- Add translucent multi-layer blending.
- Validate with a sketch outline and animated/persistent watercolor-like fill.

## Acceptance Criteria
- `velumin-core` can represent vector stroke commands without depending on any frontend.
- The model can represent at least one closed filled shape with an outline and masked interior material.
- The model can represent a sprite/cel layer with frame identity and transform.
- The model can represent a pixel-art layer rendered at a virtual resolution and upscaled.
- The renderer can choose different pipelines based on material family without changing the public scene structure.
- Vector emulation remains the first working style and is not delayed by later material families.

## Risks
- Scope creep could turn Velumin from a focused renderer into an unfinished general-purpose engine.
- Watercolor can become expensive or vague if "dynamic" is not constrained to shader-driven stylization.
- Cel animation may pull in asset-management requirements that belong outside the core renderer.
- Pixel-monitor effects may overlap with vector glow but still need separate tuning and validation.
- A too-general material model could become abstract before any style works well.

## Guardrails
- Implement one visual style at a time.
- Keep vector emulation as the proof path.
- Prefer concrete examples over abstract material machinery.
- Treat new styles as optional renderer modules until stable.
- Keep game simulation, audio, input, Steam integration, and editor tooling outside DP-0003.

## Open Questions
- Should materials be an enum in early versions, or trait/plugin-based from the beginning?
- How much animation timing belongs in `velumin-core` versus examples or engine integrations?
- Should watercolor use procedural noise only at first, or support imported paper/pigment textures from the start?
- Should pixel-monitor effects be a material, a layer post-process, or a full-scene post-process?
- What asset handle model should bridge browser, desktop, and later Bevy integration?

## References
- DP-0001: `project/design/proposals/DP-0001-modern-webgpu-rendering.md`
- DP-0002: `project/design/proposals/DP-0002-cross-platform-renderer-architecture.md`
- W3C Compositing and Blending: https://www.w3.org/TR/compositing-1/
- W3C CSS Masking: https://www.w3.org/TR/css-masking-1/
- `wgpu` texture docs: https://docs.rs/wgpu/latest/wgpu/struct.Texture.html
- MDN `GPUDevice.createSampler`: https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createSampler
