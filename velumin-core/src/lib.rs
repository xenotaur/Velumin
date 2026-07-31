//! Platform-neutral vector/scene/style data model for Velumin (DP-0002 Phase 1).
//!
//! This crate has no dependency on `wasm-bindgen`, `web-sys`, or `wgpu` — it
//! holds only vector command data, math types, and rendering-parameter shapes
//! that a renderer or platform adapter can consume. The `webgpu_vector_lib`
//! crate depends on this crate for these types and keeps the `wgpu` renderer,
//! the browser/`wasm-bindgen` adapter, and the public `VectorDisplayPreset`
//! selector (which must stay wasm-bindgen-exportable) on its own side.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct StrokeStyle {
    pub width: f32,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Clone, Debug)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
    pub style: StrokeStyle,
}

#[derive(Clone, Debug)]
pub struct Polyline {
    pub points: Vec<Vec2>,
    pub style: StrokeStyle,
}

#[derive(Clone, Debug)]
pub enum VectorCommand {
    Line(Line),
    Polyline(Polyline),
}

/// A single additive glow-emission band: how far a stroke's light spreads
/// (`width_scale`) and how bright that band is (`intensity_scale`).
#[derive(Clone, Copy, Debug)]
pub struct GlowLayer {
    pub width_scale: f32,
    pub intensity_scale: f32,
}

impl GlowLayer {
    pub const fn disabled() -> Self {
        Self {
            width_scale: 1.0,
            intensity_scale: 0.0,
        }
    }
}

/// The clamped, renderer-ready shape of a display look: up to three additive
/// glow layers plus a crisp-stroke width scale. Construct via [`Self::from_layers`]
/// or [`Self::from_tuner`] — both clamp inputs to safe renderer bounds.
///
/// Named presets (e.g. the public `VectorDisplayPreset` API, DP-0007) are a
/// `webgpu_vector_lib`-side concern: that crate maps a preset name to a
/// `&[GlowLayer]` table and calls [`Self::from_layers`] to build one of these.
#[derive(Clone, Copy, Debug)]
pub struct VectorDisplaySettings {
    glow_layers: [GlowLayer; 3],
    glow_layer_count: usize,
    stroke_width_scale: f32,
}

impl VectorDisplaySettings {
    pub fn from_layers(layers: &[GlowLayer], stroke_width_scale: f32) -> Self {
        let mut glow_layers = [GlowLayer::disabled(); 3];
        let glow_layer_count = layers.len().min(glow_layers.len());

        for (target, source) in glow_layers.iter_mut().zip(layers.iter()) {
            *target = GlowLayer {
                width_scale: source.width_scale.clamp(1.0, 16.0),
                intensity_scale: source.intensity_scale.clamp(0.0, 1.0),
            };
        }

        Self {
            glow_layers,
            glow_layer_count,
            stroke_width_scale: stroke_width_scale.clamp(0.25, 3.0),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_tuner(
        stroke_width_scale: f32,
        near_width_scale: f32,
        near_intensity_scale: f32,
        mid_width_scale: f32,
        mid_intensity_scale: f32,
        far_width_scale: f32,
        far_intensity_scale: f32,
    ) -> Self {
        Self::from_layers(
            &[
                GlowLayer {
                    width_scale: near_width_scale,
                    intensity_scale: near_intensity_scale,
                },
                GlowLayer {
                    width_scale: mid_width_scale,
                    intensity_scale: mid_intensity_scale,
                },
                GlowLayer {
                    width_scale: far_width_scale,
                    intensity_scale: far_intensity_scale,
                },
            ],
            stroke_width_scale,
        )
    }

    pub fn glow_layers(&self) -> &[GlowLayer] {
        &self.glow_layers[..self.glow_layer_count]
    }

    pub fn stroke_width_scale(&self) -> f32 {
        self.stroke_width_scale
    }
}

/// A fixed 4:3 viewport centered and letterboxed/pillarboxed within a larger
/// surface, so window resizing never distorts scene geometry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderViewport {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl RenderViewport {
    const TARGET_ASPECT: f32 = 4.0 / 3.0;

    pub fn centered_4_3(surface_width: u32, surface_height: u32) -> Self {
        let surface_width = surface_width.max(1);
        let surface_height = surface_height.max(1);
        let surface_aspect = surface_width as f32 / surface_height as f32;

        if surface_aspect > Self::TARGET_ASPECT {
            let width = ((surface_height as f32 * Self::TARGET_ASPECT).round() as u32).max(1);
            Self {
                x: (surface_width - width) / 2,
                y: 0,
                width,
                height: surface_height,
            }
        } else {
            let height = ((surface_width as f32 / Self::TARGET_ASPECT).round() as u32).max(1);
            Self {
                x: 0,
                y: (surface_height - height) / 2,
                width: surface_width,
                height,
            }
        }
    }
}

pub fn transform_points(points: &[Vec2], offset: Vec2, angle: f32, scale: f32) -> Vec<Vec2> {
    let sin = angle.sin();
    let cos = angle.cos();
    points
        .iter()
        .map(|point| Vec2 {
            x: offset.x + (point.x * cos - point.y * sin) * scale,
            y: offset.y + (point.x * sin + point.y * cos) * scale,
        })
        .collect()
}

pub fn lerp_vec2(start: Vec2, end: Vec2, progress: f32) -> Vec2 {
    Vec2 {
        x: start.x + (end.x - start.x) * progress,
        y: start.y + (end.y - start.y) * progress,
    }
}

pub fn stroke(width: f32, color: Color, intensity: f32) -> StrokeStyle {
    StrokeStyle {
        width,
        color,
        intensity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_near(actual: f32, expected: f32) {
        assert!((actual - expected).abs() <= 0.00001);
    }

    #[test]
    fn tuner_settings_are_clamped_to_renderer_bounds() {
        let min_settings = VectorDisplaySettings::from_tuner(0.1, 0.1, -1.0, 5.0, 0.25, 99.0, 2.0);
        let max_settings = VectorDisplaySettings::from_tuner(9.0, 0.1, -1.0, 5.0, 0.25, 99.0, 2.0);

        assert_near(min_settings.stroke_width_scale(), 0.25);
        assert_near(max_settings.stroke_width_scale(), 3.0);
        assert_eq!(min_settings.glow_layers().len(), 3);
        assert_near(min_settings.glow_layers()[0].width_scale, 1.0);
        assert_near(min_settings.glow_layers()[0].intensity_scale, 0.0);
        assert_near(min_settings.glow_layers()[1].width_scale, 5.0);
        assert_near(min_settings.glow_layers()[1].intensity_scale, 0.25);
        assert_near(min_settings.glow_layers()[2].width_scale, 16.0);
        assert_near(min_settings.glow_layers()[2].intensity_scale, 1.0);
    }

    #[test]
    fn centered_viewport_preserves_four_by_three_aspect() {
        assert_eq!(
            RenderViewport::centered_4_3(1600, 600),
            RenderViewport {
                x: 400,
                y: 0,
                width: 800,
                height: 600,
            }
        );
        assert_eq!(
            RenderViewport::centered_4_3(800, 1000),
            RenderViewport {
                x: 0,
                y: 200,
                width: 800,
                height: 600,
            }
        );
        assert_eq!(
            RenderViewport::centered_4_3(800, 600),
            RenderViewport {
                x: 0,
                y: 0,
                width: 800,
                height: 600,
            }
        );
    }
}
