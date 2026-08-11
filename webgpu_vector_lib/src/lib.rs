#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use velumin_core::RenderViewport;
pub use velumin_core::{Color, Line, Polyline, StrokeStyle, Vec2, VectorCommand};
use velumin_core::{GlowLayer, VectorDisplaySettings, lerp_vec2, stroke, transform_points};

/// Platform-neutral error type for [`Renderer`] construction and rendering.
/// Converted to `JsValue` only at the `WebGPU` wasm-bindgen boundary (see the
/// `From<RendererError> for JsValue` impl below), so `Renderer` itself has no
/// `wasm-bindgen` dependency in its own error surface.
///
/// Each variant's [`Display`](std::fmt::Display) arm owns its *entire*
/// message text, including any debug-formatting of dynamic data — the call
/// site only ever supplies the raw value it received, never a pre-formatted
/// `String`. [`DeviceRequestFailed`](Self::DeviceRequestFailed) stores the
/// real `wgpu::RequestDeviceError` it was handed, so production formatting
/// can never drift from `Display`; the type has no public constructor, so
/// it can't be *fabricated* for a unit test, which is why the static
/// wording is factored into [`device_request_failed_message`] and tested
/// through that helper instead of through the variant directly.
#[derive(Debug)]
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
enum RendererError {
    UnsupportedSurfaceFormat,
    UnsupportedAlphaMode,
    MissingPresentMode,
    InsufficientLimits,
    DeviceRequestFailed(wgpu::RequestDeviceError),
    SurfaceTextureUnavailable,
    FrameAcquisitionFailed(wgpu::CurrentSurfaceTexture),
}

/// The static wording around a device-request failure, factored out of
/// [`RendererError`]'s `Display` impl so it can be unit tested with a
/// synthetic debug string — `wgpu::RequestDeviceError` itself has no public
/// constructor and can't be fabricated for a test.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn device_request_failed_message(debug_text: impl std::fmt::Display) -> String {
    format!(
        "Device request failed. Required WebGPU features or limits may be unavailable: {}",
        debug_text
    )
}

impl std::fmt::Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedSurfaceFormat => write!(
                f,
                "The WebGPU adapter does not report any supported surface formats."
            ),
            Self::UnsupportedAlphaMode => write!(
                f,
                "The WebGPU adapter does not report any supported alpha modes."
            ),
            Self::MissingPresentMode => write!(
                f,
                "The WebGPU adapter does not support the required FIFO presentation mode."
            ),
            Self::InsufficientLimits => write!(
                f,
                "The WebGPU adapter does not meet Velumin's required rendering limits."
            ),
            Self::DeviceRequestFailed(e) => {
                write!(f, "{}", device_request_failed_message(format!("{:?}", e)))
            }
            Self::SurfaceTextureUnavailable => write!(
                f,
                "Surface texture is temporarily unavailable; try rendering again later."
            ),
            Self::FrameAcquisitionFailed(status) => {
                write!(f, "Failed to get frame from WebGPU surface: {:?}", status)
            }
        }
    }
}

impl std::error::Error for RendererError {}

#[cfg(target_arch = "wasm32")]
impl From<RendererError> for JsValue {
    fn from(error: RendererError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

/// Log a diagnostic message from the renderer. A no-op on non-wasm32 hosts
/// (no browser console to log to); forwards to the wasm-bindgen `console.log`
/// binding on wasm32.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code, unused_variables))]
fn renderer_log(message: &str) {
    #[cfg(target_arch = "wasm32")]
    log(message);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4];

    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct GlowVertex {
    position: [f32; 2],
    color: [f32; 4],
    segment_start: [f32; 2],
    segment_end: [f32; 2],
    radius: f32,
    core_width: f32,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl GlowVertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 6] = wgpu::vertex_attr_array![
        0 => Float32x2,
        1 => Float32x4,
        2 => Float32x2,
        3 => Float32x2,
        4 => Float32,
        5 => Float32
    ];

    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

/// A named, classic-inspired vector-display look.
///
/// This is the public v1 display-preset API (DP-0007): the numeric glow/stroke
/// tuning behind each look is internal and may be re-tuned, but these variant
/// names are a stable contract. Marked `#[non_exhaustive]` so future presets can
/// be added without breaking downstream code.
///
/// This enum (and the preset -> glow-layer mapping below) stays in
/// `webgpu_vector_lib` rather than `velumin-core`: it must remain
/// wasm-bindgen-exportable (it is a parameter type on `create_with_preset` /
/// `set_display_preset`), and `velumin-core` has no `wasm-bindgen` dependency.
/// The clamped, renderer-ready `VectorDisplaySettings`/`GlowLayer` shapes it
/// maps into are the platform-neutral part and live in `velumin-core`.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum VectorDisplayPreset {
    ArcadeBalanced,
    MonochromeBeam,
    ColorQuadraScan,
    CleanNeon,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
enum VectorFrameInputError {
    InvalidPointArrayLength,
    TooFewPolylinePoints,
    TooFewClosedPolylinePoints,
    NonFiniteValue,
    InvalidColorRange,
    InvalidStrokeWidth,
    InvalidIntensity,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub enum VectorFrameViewError {
    NonFiniteValue,
    DegenerateExtents,
}

impl std::fmt::Display for VectorFrameViewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFiniteValue => write!(f, "VectorFrameView values must be finite numbers."),
            Self::DegenerateExtents => write!(
                f,
                "VectorFrameView extents must have non-zero width and height."
            ),
        }
    }
}

impl std::error::Error for VectorFrameViewError {}

#[cfg(target_arch = "wasm32")]
impl From<VectorFrameViewError> for JsValue {
    fn from(error: VectorFrameViewError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

impl std::fmt::Display for VectorFrameInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPointArrayLength => {
                write!(f, "Point arrays must contain x/y pairs.")
            }
            Self::TooFewPolylinePoints => {
                write!(f, "Polylines require at least two points.")
            }
            Self::TooFewClosedPolylinePoints => {
                write!(f, "Closed polylines require at least three points.")
            }
            Self::NonFiniteValue => {
                write!(f, "VectorFrame values must be finite numbers.")
            }
            Self::InvalidColorRange => {
                write!(
                    f,
                    "Color channels and alpha must be finite values in the 0.0..=1.0 range."
                )
            }
            Self::InvalidStrokeWidth => {
                write!(f, "Stroke width must be a finite number greater than 0.0.")
            }
            Self::InvalidIntensity => {
                write!(
                    f,
                    "Stroke intensity must be a finite number greater than or equal to 0.0."
                )
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum VectorFrameViewMapping {
    Centered4x3,
    LogicalExtents {
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    },
}

/// Public render-time coordinate mapping for immediate vector frames.
///
/// `VectorFrameView` does not own gameplay camera, wrapping, or object policy.
/// It only tells the browser renderer how submitted frame coordinates map into
/// the current canvas for this draw.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VectorFrameView {
    mapping: VectorFrameViewMapping,
}

impl Default for VectorFrameView {
    fn default() -> Self {
        Self::centered_4_3()
    }
}

impl VectorFrameView {
    pub fn centered_4_3() -> Self {
        Self {
            mapping: VectorFrameViewMapping::Centered4x3,
        }
    }

    pub fn logical_extents(
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) -> Result<Self, VectorFrameViewError> {
        validate_view_finite(&[left, bottom, right, top])?;
        if (right - left).abs() <= f32::EPSILON || (top - bottom).abs() <= f32::EPSILON {
            return Err(VectorFrameViewError::DegenerateExtents);
        }
        Ok(Self {
            mapping: VectorFrameViewMapping::LogicalExtents {
                left,
                bottom,
                right,
                top,
            },
        })
    }

    pub fn canvas_pixels(width: f32, height: f32) -> Result<Self, VectorFrameViewError> {
        Self::logical_extents(0.0, height, width, 0.0)
    }

    fn resolve(self, surface_width: u32, surface_height: u32) -> ResolvedVectorFrameView {
        match self.mapping {
            VectorFrameViewMapping::Centered4x3 => ResolvedVectorFrameView {
                viewport: RenderViewport::centered_4_3(surface_width, surface_height),
                x_scale: 1.0,
                y_scale: 1.0,
                x_offset: 0.0,
                y_offset: 0.0,
            },
            VectorFrameViewMapping::LogicalExtents {
                left,
                bottom,
                right,
                top,
            } => {
                let x_scale = 2.0 / (right - left);
                let y_scale = 2.0 / (top - bottom);
                ResolvedVectorFrameView {
                    viewport: RenderViewport {
                        x: 0,
                        y: 0,
                        width: surface_width.max(1),
                        height: surface_height.max(1),
                    },
                    x_scale,
                    y_scale,
                    x_offset: -1.0 - left * x_scale,
                    y_offset: -1.0 - bottom * y_scale,
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl VectorFrameView {
    #[wasm_bindgen(js_name = centered4x3)]
    pub fn js_centered_4_3() -> VectorFrameView {
        Self::centered_4_3()
    }

    #[wasm_bindgen(js_name = logicalExtents)]
    pub fn js_logical_extents(
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
    ) -> Result<VectorFrameView, JsValue> {
        Self::logical_extents(left, bottom, right, top).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = canvasPixels)]
    pub fn js_canvas_pixels(width: f32, height: f32) -> Result<VectorFrameView, JsValue> {
        Self::canvas_pixels(width, height).map_err(JsValue::from)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ResolvedVectorFrameView {
    viewport: RenderViewport,
    x_scale: f32,
    y_scale: f32,
    x_offset: f32,
    y_offset: f32,
}

impl ResolvedVectorFrameView {
    fn map_point(self, point: Vec2) -> Vec2 {
        Vec2 {
            x: point.x * self.x_scale + self.x_offset,
            y: point.y * self.y_scale + self.y_offset,
        }
    }

    fn map_vector(self, vector: Vec2) -> Vec2 {
        Vec2 {
            x: vector.x * self.x_scale,
            y: vector.y * self.y_scale,
        }
    }

    fn perpendicular_scale_for_tangent(self, tangent: Vec2) -> f32 {
        let mapped_tangent = self.map_vector(tangent);
        let mapped_tangent_length =
            (mapped_tangent.x * mapped_tangent.x + mapped_tangent.y * mapped_tangent.y).sqrt();
        if mapped_tangent_length <= f32::EPSILON {
            0.0
        } else {
            (self.x_scale * self.y_scale).abs() / mapped_tangent_length
        }
    }
}

impl std::error::Error for VectorFrameInputError {}

#[cfg(target_arch = "wasm32")]
impl From<VectorFrameInputError> for JsValue {
    fn from(error: VectorFrameInputError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

/// Immediate vector-command frame submitted by browser JavaScript.
///
/// `VectorFrame` is the public v1 browser drawing surface (DP-0008). Game code
/// owns simulation and rebuilds the visible vector geometry each frame; Velumin
/// owns conversion into the existing renderer path.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Default)]
pub struct VectorFrame {
    commands: Vec<VectorCommand>,
}

impl VectorFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn commands(&self) -> &[VectorCommand] {
        &self.commands
    }

    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn push_line(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
        width: f32,
        intensity: f32,
    ) -> Result<(), VectorFrameInputError> {
        let style = frame_style(red, green, blue, alpha, width, intensity)?;
        validate_finite(&[x1, y1, x2, y2])?;
        self.commands.push(VectorCommand::Line(Line {
            start: Vec2 { x: x1, y: y1 },
            end: Vec2 { x: x2, y: y2 },
            style,
        }));
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn push_polyline(
        &mut self,
        points: &[f32],
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
        width: f32,
        intensity: f32,
    ) -> Result<(), VectorFrameInputError> {
        let style = frame_style(red, green, blue, alpha, width, intensity)?;
        let points = frame_points(points, 2, VectorFrameInputError::TooFewPolylinePoints)?;
        self.commands
            .push(VectorCommand::Polyline(Polyline { points, style }));
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn push_closed_polyline(
        &mut self,
        points: &[f32],
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
        width: f32,
        intensity: f32,
    ) -> Result<(), VectorFrameInputError> {
        let style = frame_style(red, green, blue, alpha, width, intensity)?;
        let mut points =
            frame_points(points, 3, VectorFrameInputError::TooFewClosedPolylinePoints)?;
        if points.first() != points.last() {
            if let Some(first) = points.first().copied() {
                points.push(first);
            }
        }
        self.commands
            .push(VectorCommand::Polyline(Polyline { points, style }));
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl VectorFrame {
    #[wasm_bindgen(constructor)]
    pub fn js_new() -> VectorFrame {
        Self::new()
    }

    #[wasm_bindgen(js_name = clear)]
    pub fn js_clear(&mut self) {
        self.clear();
    }

    #[wasm_bindgen(js_name = len)]
    pub fn js_len(&self) -> usize {
        self.len()
    }

    #[wasm_bindgen(js_name = isEmpty)]
    pub fn js_is_empty(&self) -> bool {
        self.is_empty()
    }

    #[wasm_bindgen(js_name = line)]
    #[allow(clippy::too_many_arguments)]
    pub fn js_line(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
        width: f32,
        intensity: f32,
    ) -> Result<(), JsValue> {
        self.push_line(x1, y1, x2, y2, red, green, blue, alpha, width, intensity)
            .map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = polyline)]
    #[allow(clippy::too_many_arguments)]
    pub fn js_polyline(
        &mut self,
        points: &[f32],
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
        width: f32,
        intensity: f32,
    ) -> Result<(), JsValue> {
        self.push_polyline(points, red, green, blue, alpha, width, intensity)
            .map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = closedPolyline)]
    #[allow(clippy::too_many_arguments)]
    pub fn js_closed_polyline(
        &mut self,
        points: &[f32],
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
        width: f32,
        intensity: f32,
    ) -> Result<(), JsValue> {
        self.push_closed_polyline(points, red, green, blue, alpha, width, intensity)
            .map_err(JsValue::from)
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn frame_points(
    values: &[f32],
    minimum_points: usize,
    too_few_error: VectorFrameInputError,
) -> Result<Vec<Vec2>, VectorFrameInputError> {
    if values.len() % 2 != 0 {
        return Err(VectorFrameInputError::InvalidPointArrayLength);
    }
    validate_finite(values)?;
    let points: Vec<Vec2> = values
        .chunks_exact(2)
        .map(|xy| Vec2 { x: xy[0], y: xy[1] })
        .collect();
    if points.len() < minimum_points {
        return Err(too_few_error);
    }
    Ok(points)
}

#[allow(clippy::too_many_arguments)]
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn frame_style(
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
    width: f32,
    intensity: f32,
) -> Result<StrokeStyle, VectorFrameInputError> {
    validate_finite(&[red, green, blue, alpha, width, intensity])?;
    if [red, green, blue, alpha]
        .into_iter()
        .any(|channel| !(0.0..=1.0).contains(&channel))
    {
        return Err(VectorFrameInputError::InvalidColorRange);
    }
    if width <= 0.0 {
        return Err(VectorFrameInputError::InvalidStrokeWidth);
    }
    if intensity < 0.0 {
        return Err(VectorFrameInputError::InvalidIntensity);
    }
    Ok(stroke(
        width,
        Color {
            red,
            green,
            blue,
            alpha,
        },
        intensity,
    ))
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn validate_finite(values: &[f32]) -> Result<(), VectorFrameInputError> {
    if values.iter().all(|value| value.is_finite()) {
        Ok(())
    } else {
        Err(VectorFrameInputError::NonFiniteValue)
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn validate_view_finite(values: &[f32]) -> Result<(), VectorFrameViewError> {
    if values.iter().all(|value| value.is_finite()) {
        Ok(())
    } else {
        Err(VectorFrameViewError::NonFiniteValue)
    }
}

const ARCADE_BALANCED_GLOW: [GlowLayer; 3] = [
    GlowLayer {
        width_scale: 2.2,
        intensity_scale: 0.33,
    },
    GlowLayer {
        width_scale: 5.0,
        intensity_scale: 0.12,
    },
    GlowLayer {
        width_scale: 12.3,
        intensity_scale: 0.03,
    },
];
const MONOCHROME_BEAM_GLOW: [GlowLayer; 2] = [
    GlowLayer {
        width_scale: 2.0,
        intensity_scale: 0.22,
    },
    GlowLayer {
        width_scale: 4.5,
        intensity_scale: 0.08,
    },
];
const COLOR_QUADRA_SCAN_GLOW: [GlowLayer; 3] = [
    GlowLayer {
        width_scale: 2.6,
        intensity_scale: 0.35,
    },
    GlowLayer {
        width_scale: 6.5,
        intensity_scale: 0.16,
    },
    GlowLayer {
        width_scale: 11.0,
        intensity_scale: 0.06,
    },
];
const CLEAN_NEON_GLOW: [GlowLayer; 2] = [
    GlowLayer {
        width_scale: 3.0,
        intensity_scale: 0.18,
    },
    GlowLayer {
        width_scale: 7.0,
        intensity_scale: 0.07,
    },
];

/// Map a named preset to a clamped, renderer-ready [`VectorDisplaySettings`].
/// Replaces the former inherent `VectorDisplaySettings::from_preset` now that
/// the settings type lives in `velumin-core` (see the [`VectorDisplayPreset`]
/// doc comment for why this mapping itself stays here).
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn display_settings_from_preset(preset: VectorDisplayPreset) -> VectorDisplaySettings {
    let layers: &[GlowLayer] = match preset {
        VectorDisplayPreset::ArcadeBalanced => &ARCADE_BALANCED_GLOW,
        VectorDisplayPreset::MonochromeBeam => &MONOCHROME_BEAM_GLOW,
        VectorDisplayPreset::ColorQuadraScan => &COLOR_QUADRA_SCAN_GLOW,
        VectorDisplayPreset::CleanNeon => &CLEAN_NEON_GLOW,
    };
    let stroke_width_scale = match preset {
        VectorDisplayPreset::ArcadeBalanced => 0.35,
        _ => 1.0,
    };
    VectorDisplaySettings::from_layers(layers, stroke_width_scale)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
pub struct WebGPU {
    canvas: web_sys::HtmlCanvasElement,
    renderer: Renderer,
    frame_view: VectorFrameView,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    crisp_pipeline: wgpu::RenderPipeline,
    glow_pipeline: wgpu::RenderPipeline,
    composite_pipeline: wgpu::RenderPipeline,
    tester_composite_pipeline: wgpu::RenderPipeline,
    composite_bind_group_layout: wgpu::BindGroupLayout,
    composite_bind_group: wgpu::BindGroup,
    surface: wgpu::Surface<'static>,
    glow_texture: wgpu::Texture,
    glow_view: wgpu::TextureView,
    glow_sampler: wgpu::Sampler,
    glow_width: u32,
    glow_height: u32,
    vertex_buffer: wgpu::Buffer,
    vertex_capacity: usize,
    vertex_count: u32,
    glow_vertex_buffer: wgpu::Buffer,
    glow_vertex_capacity: usize,
    glow_vertex_count: u32,
    display_settings: VectorDisplaySettings,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
impl WebGPU {
    /// Create a renderer on the given canvas using the default display preset
    /// (`ArcadeBalanced`). To choose a different look up front, use
    /// `create_with_preset`; to change it later, use `set_display_preset`.
    #[wasm_bindgen(js_name = create)]
    pub async fn create(canvas_id: &str) -> Result<WebGPU, JsValue> {
        Self::create_with_preset(canvas_id, VectorDisplayPreset::ArcadeBalanced).await
    }

    /// Create a renderer on the given canvas with a chosen display preset.
    #[wasm_bindgen(js_name = createWithPreset)]
    pub async fn create_with_preset(
        canvas_id: &str,
        preset: VectorDisplayPreset,
    ) -> Result<WebGPU, JsValue> {
        console_error_panic_hook::set_once();
        log("Starting WebGPU setup");

        let window = web_sys::window().ok_or("No window available")?;
        let document = window.document().ok_or("No document available")?;
        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or("Canvas not found")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;

        if !browser_has_webgpu(&window) {
            return Err(JsValue::from_str(
                "This browser does not expose navigator.gpu. Velumin currently requires native browser WebGPU.",
            ));
        }

        let (width, height) = resize_canvas_to_display_size(&window, &canvas)?;

        let instance = wgpu::Instance::default();
        log("Created wgpu instance");

        let surface = instance
            .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
            .map_err(|e| {
                JsValue::from_str(&format!(
                    "Surface creation failed. WebGPU may be blocked for this canvas or browser: {:?}",
                    e
                ))
            })?;
        log("Created surface from canvas");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|e| {
                JsValue::from_str(&format!(
                    "Failed to get a WebGPU adapter. GPU access may be blocked, unsupported, or unavailable: {:?}",
                    e
                ))
            })?;
        let adapter_info = adapter.get_info();
        log(&format!(
            "Adapter found: {} ({:?})",
            adapter_info.name, adapter_info.backend
        ));

        let renderer = Renderer::new(surface, &adapter, width, height, preset).await?;

        Ok(WebGPU {
            canvas,
            renderer,
            frame_view: VectorFrameView::default(),
        })
    }

    /// Switch the active display preset at runtime. Takes effect on the next
    /// `render` / `render_blasterites_tester` call.
    #[wasm_bindgen(js_name = setDisplayPreset)]
    pub fn set_display_preset(&mut self, preset: VectorDisplayPreset) {
        self.renderer.display_settings = display_settings_from_preset(preset);
    }

    /// Set the default coordinate mapping used by `renderFrame` and the
    /// Rust/WASM `render_commands` path. Existing consumers keep centered 4:3
    /// unless they opt into a different view.
    #[wasm_bindgen(js_name = setFrameView)]
    pub fn set_frame_view(&mut self, view: &VectorFrameView) {
        self.frame_view = *view;
    }

    #[wasm_bindgen(js_name = resetFrameView)]
    pub fn reset_frame_view(&mut self) {
        self.frame_view = VectorFrameView::default();
    }

    #[wasm_bindgen]
    pub fn render(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("No window available")?;
        let (width, height) = resize_canvas_to_display_size(&window, &self.canvas)?;
        self.renderer.resize(width, height);
        self.renderer
            .render(&smoke_scene(), false, VectorFrameView::default())
            .map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&mut self, frame: &VectorFrame) -> Result<(), JsValue> {
        self.render_commands(frame.commands())
    }

    #[wasm_bindgen(js_name = renderFrameWithView)]
    pub fn render_frame_with_view(
        &mut self,
        frame: &VectorFrame,
        view: &VectorFrameView,
    ) -> Result<(), JsValue> {
        self.render_commands_with_view(frame.commands(), *view)
    }

    #[wasm_bindgen]
    pub fn render_blasterites_tester(&mut self, time_ms: f64) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("No window available")?;
        let (width, height) = resize_canvas_to_display_size(&window, &self.canvas)?;
        self.renderer.resize(width, height);
        let wrapped_time_ms = time_ms.rem_euclid(BLASTERITES_CYCLE_MS as f64) as f32;
        self.renderer
            .render(
                &blasterites_tester_scene(wrapped_time_ms),
                true,
                VectorFrameView::default(),
            )
            .map_err(JsValue::from)
    }

    #[wasm_bindgen]
    #[allow(clippy::too_many_arguments)]
    pub fn render_blasterites_tuner(
        &mut self,
        time_ms: f64,
        stroke_width_scale: f32,
        near_width_scale: f32,
        near_intensity_scale: f32,
        mid_width_scale: f32,
        mid_intensity_scale: f32,
        far_width_scale: f32,
        far_intensity_scale: f32,
    ) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("No window available")?;
        let (width, height) = resize_canvas_to_display_size(&window, &self.canvas)?;
        self.renderer.resize(width, height);
        self.renderer.display_settings = VectorDisplaySettings::from_tuner(
            stroke_width_scale,
            near_width_scale,
            near_intensity_scale,
            mid_width_scale,
            mid_intensity_scale,
            far_width_scale,
            far_intensity_scale,
        );
        let wrapped_time_ms = time_ms.rem_euclid(BLASTERITES_CYCLE_MS as f64) as f32;
        self.renderer
            .render(
                &blasterites_tester_scene(wrapped_time_ms),
                true,
                VectorFrameView::default(),
            )
            .map_err(JsValue::from)
    }
}

#[cfg(target_arch = "wasm32")]
impl WebGPU {
    /// Render already-owned Rust vector commands through the same browser
    /// renderer path as `renderFrame`, without converting them through the
    /// JavaScript `VectorFrame` builder.
    pub fn render_commands(&mut self, commands: &[VectorCommand]) -> Result<(), JsValue> {
        self.render_commands_with_view(commands, self.frame_view)
    }

    /// Render already-owned Rust vector commands with an explicit coordinate
    /// mapping, sharing the same renderer path as `renderFrameWithView`.
    pub fn render_commands_with_view(
        &mut self,
        commands: &[VectorCommand],
        view: VectorFrameView,
    ) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("No window available")?;
        let (width, height) = resize_canvas_to_display_size(&window, &self.canvas)?;
        self.renderer.resize(width, height);
        self.renderer
            .render(commands, false, view)
            .map_err(JsValue::from)
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl Renderer {
    async fn new(
        surface: wgpu::Surface<'static>,
        adapter: &wgpu::Adapter,
        width: u32,
        height: u32,
        preset: VectorDisplayPreset,
    ) -> Result<Self, RendererError> {
        let capabilities = surface.get_capabilities(adapter);
        let format = capabilities
            .formats
            .first()
            .copied()
            .ok_or(RendererError::UnsupportedSurfaceFormat)?;
        let alpha_mode = capabilities
            .alpha_modes
            .first()
            .copied()
            .ok_or(RendererError::UnsupportedAlphaMode)?;
        if !capabilities
            .present_modes
            .contains(&wgpu::PresentMode::Fifo)
        {
            return Err(RendererError::MissingPresentMode);
        }

        let required_limits = wgpu::Limits::downlevel_defaults();
        if !required_limits.check_limits(&adapter.limits()) {
            return Err(RendererError::InsufficientLimits);
        }

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("WebGPU Device"),
                required_features: wgpu::Features::empty(),
                required_limits,
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .map_err(RendererError::DeviceRequestFailed)?;
        renderer_log("Device and queue acquired");

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode,
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        renderer_log("Surface configured");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Line Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/line.wgsl").into()),
        });
        let glow_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Glow Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/glow.wgsl").into()),
        });
        renderer_log("Shader modules created");

        let crisp_pipeline = create_vector_pipeline(
            &device,
            &shader,
            config.format,
            wgpu::BlendState::REPLACE,
            "Crisp Vector Pipeline",
        );
        let glow_pipeline = create_glow_pipeline(
            &device,
            &glow_shader,
            config.format,
            additive_blend_state(),
            "Glow Bright-Pass Pipeline",
        );
        renderer_log("Vector render pipelines created");

        let composite_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Glow Composite Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/composite.wgsl").into()),
        });
        let tester_composite_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Tester Composite Shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../shaders/tester_composite.wgsl").into(),
            ),
        });

        let composite_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Glow Composite Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let composite_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Glow Composite Pipeline Layout"),
                bind_group_layouts: &[Some(&composite_bind_group_layout)],
                immediate_size: 0,
            });

        let composite_pipeline = create_composite_pipeline(
            &device,
            &composite_shader,
            config.format,
            &composite_pipeline_layout,
            "Glow Composite Pipeline",
        );
        let tester_composite_pipeline = create_composite_pipeline(
            &device,
            &tester_composite_shader,
            config.format,
            &composite_pipeline_layout,
            "Tester Composite Pipeline",
        );
        renderer_log("Glow composite pipelines created");

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vector Vertex Buffer"),
            size: 1,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let glow_vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Glow Vector Vertex Buffer"),
            size: 1,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let glow_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Glow Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let (glow_texture, glow_view, composite_bind_group, glow_width, glow_height) =
            create_glow_target(
                &device,
                config.format,
                config.width,
                config.height,
                &composite_bind_group_layout,
                &glow_sampler,
            );
        renderer_log(&format!(
            "Glow target configured at {}x{}",
            glow_width, glow_height
        ));

        Ok(Self {
            device,
            queue,
            config,
            crisp_pipeline,
            glow_pipeline,
            composite_pipeline,
            tester_composite_pipeline,
            composite_bind_group_layout,
            composite_bind_group,
            surface,
            glow_texture,
            glow_view,
            glow_sampler,
            glow_width,
            glow_height,
            vertex_buffer,
            vertex_capacity: 0,
            vertex_count: 0,
            glow_vertex_buffer,
            glow_vertex_capacity: 0,
            glow_vertex_count: 0,
            display_settings: display_settings_from_preset(preset),
        })
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width == self.config.width && height == self.config.height {
            return;
        }

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        let (glow_texture, glow_view, composite_bind_group, glow_width, glow_height) =
            create_glow_target(
                &self.device,
                self.config.format,
                width,
                height,
                &self.composite_bind_group_layout,
                &self.glow_sampler,
            );
        self.glow_texture = glow_texture;
        self.glow_view = glow_view;
        self.composite_bind_group = composite_bind_group;
        self.glow_width = glow_width;
        self.glow_height = glow_height;
        renderer_log(&format!("Surface reconfigured to {}x{}", width, height));
    }

    fn render(
        &mut self,
        commands: &[VectorCommand],
        tester_effects: bool,
        frame_view: VectorFrameView,
    ) -> Result<(), RendererError> {
        renderer_log("Starting render call");
        self.upload_vector_commands(commands, frame_view);

        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return Err(RendererError::SurfaceTextureUnavailable);
            }
            status => {
                return Err(RendererError::FrameAcquisitionFailed(status));
            }
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let surface_viewport = frame_view
            .resolve(self.config.width, self.config.height)
            .viewport;
        let glow_viewport = frame_view
            .resolve(self.glow_width, self.glow_height)
            .viewport;

        {
            let mut glow_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Glow Bright Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.glow_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            glow_pass.set_pipeline(&self.glow_pipeline);
            glow_pass.set_vertex_buffer(0, self.glow_vertex_buffer.slice(..));
            glow_pass.set_viewport(
                glow_viewport.x as f32,
                glow_viewport.y as f32,
                glow_viewport.width as f32,
                glow_viewport.height as f32,
                0.0,
                1.0,
            );
            glow_pass.set_scissor_rect(
                glow_viewport.x,
                glow_viewport.y,
                glow_viewport.width,
                glow_viewport.height,
            );
            glow_pass.draw(0..self.glow_vertex_count, 0..1);
        }

        {
            let mut surface_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Composite and Crisp Vector Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            let composite_pipeline = if tester_effects {
                &self.tester_composite_pipeline
            } else {
                &self.composite_pipeline
            };
            surface_pass.set_pipeline(composite_pipeline);
            surface_pass.set_bind_group(0, &self.composite_bind_group, &[]);
            surface_pass.draw(0..3, 0..1);

            surface_pass.set_pipeline(&self.crisp_pipeline);
            surface_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            surface_pass.set_viewport(
                surface_viewport.x as f32,
                surface_viewport.y as f32,
                surface_viewport.width as f32,
                surface_viewport.height as f32,
                0.0,
                1.0,
            );
            surface_pass.set_scissor_rect(
                surface_viewport.x,
                surface_viewport.y,
                surface_viewport.width,
                surface_viewport.height,
            );
            surface_pass.draw(0..self.vertex_count, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
        renderer_log("Frame submitted and presented");
        Ok(())
    }

    fn upload_vector_commands(&mut self, commands: &[VectorCommand], frame_view: VectorFrameView) {
        let render_view = frame_view.resolve(self.config.width, self.config.height);
        let vertices = tessellate_commands_with_view(
            commands,
            self.display_settings.stroke_width_scale(),
            1.0,
            render_view,
        );
        self.vertex_count = upload_vertices(
            &self.device,
            &self.queue,
            "Vector Vertex Buffer",
            &vertices,
            &mut self.vertex_buffer,
            &mut self.vertex_capacity,
        );

        let glow_view = frame_view.resolve(self.glow_width, self.glow_height);
        let glow_vertices =
            tessellate_glow_commands_with_view(commands, self.display_settings, glow_view);
        self.glow_vertex_count = upload_glow_vertices(
            &self.device,
            &self.queue,
            "Glow Vector Vertex Buffer",
            &glow_vertices,
            &mut self.glow_vertex_buffer,
            &mut self.glow_vertex_capacity,
        );
        renderer_log(&format!("Uploaded {} vector vertices", vertices.len()));
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn create_glow_pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
    blend: wgpu::BlendState,
    label: &str,
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Glow Pipeline Layout"),
        bind_group_layouts: &[],
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[GlowVertex::layout()],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(blend),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    })
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn create_vector_pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
    blend: wgpu::BlendState,
    label: &str,
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Vector Pipeline Layout"),
        bind_group_layouts: &[],
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[Vertex::layout()],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(blend),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    })
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn additive_blend_state() -> wgpu::BlendState {
    wgpu::BlendState {
        color: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        },
        alpha: wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        },
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn create_composite_pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
    layout: &wgpu::PipelineLayout,
    label: &str,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    })
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn create_glow_target(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    surface_width: u32,
    surface_height: u32,
    layout: &wgpu::BindGroupLayout,
    sampler: &wgpu::Sampler,
) -> (wgpu::Texture, wgpu::TextureView, wgpu::BindGroup, u32, u32) {
    let glow_width = (surface_width / 2).max(1);
    let glow_height = (surface_height / 2).max(1);
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Glow Bright-Pass Texture"),
        size: wgpu::Extent3d {
            width: glow_width,
            height: glow_height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Glow Composite Bind Group"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(sampler),
            },
        ],
    });

    (texture, view, bind_group, glow_width, glow_height)
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn upload_vertices(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: &str,
    vertices: &[Vertex],
    buffer: &mut wgpu::Buffer,
    capacity: &mut usize,
) -> u32 {
    if vertices.is_empty() {
        return 0;
    }

    let bytes = bytemuck::cast_slice(vertices);
    if vertices.len() > *capacity {
        *buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: bytes.len() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        *capacity = vertices.len();
    }

    queue.write_buffer(buffer, 0, bytes);
    vertices.len() as u32
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn upload_glow_vertices(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: &str,
    vertices: &[GlowVertex],
    buffer: &mut wgpu::Buffer,
    capacity: &mut usize,
) -> u32 {
    if vertices.is_empty() {
        return 0;
    }

    let bytes = bytemuck::cast_slice(vertices);
    if vertices.len() > *capacity {
        *buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: bytes.len() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        *capacity = vertices.len();
    }

    queue.write_buffer(buffer, 0, bytes);
    vertices.len() as u32
}

#[cfg(target_arch = "wasm32")]
fn smoke_scene() -> Vec<VectorCommand> {
    vec![VectorCommand::Line(Line {
        start: Vec2 { x: -0.75, y: 0.0 },
        end: Vec2 { x: 0.75, y: 0.0 },
        style: StrokeStyle {
            width: 0.04,
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0,
            },
            intensity: 1.0,
        },
    })]
}

const BLASTERITES_IMPACT_MS: f32 = 3000.0;
const BLASTERITES_CYCLE_MS: f32 = 5600.0;

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn blasterites_tester_scene(time_ms: f32) -> Vec<VectorCommand> {
    let wrapped_time = time_ms.rem_euclid(BLASTERITES_CYCLE_MS);
    let pulse = 0.85 + 0.15 * (wrapped_time * 0.006).sin();
    let white = Color {
        red: 0.92,
        green: 0.96,
        blue: 1.0,
        alpha: 1.0,
    };
    let amber = Color {
        red: 1.0,
        green: 0.68,
        blue: 0.18,
        alpha: 1.0,
    };
    let blue = Color {
        red: 0.55,
        green: 0.8,
        blue: 1.0,
        alpha: 1.0,
    };

    let mut commands = Vec::new();

    commands.push(VectorCommand::Polyline(Polyline {
        points: blasterites_ship_outline(wrapped_time),
        style: stroke(0.018, white, 1.0 + pulse * 0.25),
    }));

    if let Some(points) = blasterites_bullet_points(wrapped_time) {
        commands.extend(points.into_iter().map(|(start, end)| {
            VectorCommand::Line(Line {
                start,
                end,
                style: stroke(0.012, blue, 1.35),
            })
        }));
    }

    if wrapped_time < BLASTERITES_IMPACT_MS {
        commands.push(VectorCommand::Polyline(Polyline {
            points: blasterites_asteroid_outline(wrapped_time),
            style: stroke(0.014, white, 0.95 + pulse * 0.15),
        }));
    } else {
        commands.extend(blasterites_spark_lines(wrapped_time).into_iter().map(
            |(start, end, intensity)| {
                VectorCommand::Line(Line {
                    start,
                    end,
                    style: stroke(0.01, amber, intensity),
                })
            },
        ));
    }

    commands
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn blasterites_ship_outline(time_ms: f32) -> Vec<Vec2> {
    let base = [
        Vec2 { x: 0.15, y: 0.0 },
        Vec2 { x: -0.13, y: 0.1 },
        Vec2 { x: -0.06, y: 0.0 },
        Vec2 { x: -0.13, y: -0.1 },
        Vec2 { x: 0.15, y: 0.0 },
    ];
    let angle = -0.2 + time_ms * 0.00125;
    let wobble = 1.0 + 0.045 * (time_ms * 0.008).sin();
    transform_points(&base, Vec2 { x: -0.45, y: -0.05 }, angle, wobble)
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn blasterites_bullet_points(time_ms: f32) -> Option<Vec<(Vec2, Vec2)>> {
    let bullet_start_ms = 1150.0;
    if !(bullet_start_ms..BLASTERITES_IMPACT_MS).contains(&time_ms) {
        return None;
    }

    let progress =
        ((time_ms - bullet_start_ms) / (BLASTERITES_IMPACT_MS - bullet_start_ms)).clamp(0.0, 1.0);
    let start = Vec2 { x: -0.3, y: -0.03 };
    let end = Vec2 { x: 0.23, y: 0.03 };
    let center = lerp_vec2(start, end, progress);
    let radius = 0.025 + 0.006 * (time_ms * 0.02).sin().abs();

    Some(vec![
        (
            Vec2 {
                x: center.x - radius,
                y: center.y,
            },
            Vec2 {
                x: center.x + radius,
                y: center.y,
            },
        ),
        (
            Vec2 {
                x: center.x,
                y: center.y - radius,
            },
            Vec2 {
                x: center.x,
                y: center.y + radius,
            },
        ),
    ])
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn blasterites_asteroid_outline(time_ms: f32) -> Vec<Vec2> {
    let progress = (time_ms / BLASTERITES_IMPACT_MS).clamp(0.0, 1.0);
    let center = lerp_vec2(
        Vec2 { x: 0.82, y: 0.2 },
        Vec2 { x: 0.23, y: 0.03 },
        progress,
    );
    let angle = time_ms * -0.00055;
    let wobble = 1.0 + 0.04 * (time_ms * 0.005).cos();
    let base = [
        Vec2 { x: 0.0, y: -0.18 },
        Vec2 { x: 0.13, y: -0.12 },
        Vec2 { x: 0.17, y: 0.02 },
        Vec2 { x: 0.1, y: 0.16 },
        Vec2 { x: -0.05, y: 0.18 },
        Vec2 { x: -0.17, y: 0.08 },
        Vec2 { x: -0.14, y: -0.08 },
        Vec2 { x: 0.0, y: -0.18 },
    ];
    transform_points(&base, center, angle, wobble)
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn blasterites_spark_lines(time_ms: f32) -> Vec<(Vec2, Vec2, f32)> {
    let elapsed = time_ms - BLASTERITES_IMPACT_MS;
    let life = 1300.0;
    if !(0.0..life).contains(&elapsed) {
        return Vec::new();
    }

    let center = Vec2 { x: 0.23, y: 0.03 };
    let fade = 1.0 - elapsed / life;
    let speed = 0.00034 * elapsed;
    (0..18)
        .map(|index| {
            let angle = index as f32 * 1.91986 + 0.35 * (elapsed * 0.006).sin();
            let spread = speed * (0.7 + (index % 5) as f32 * 0.12);
            let tail = 0.035 + 0.012 * (index % 3) as f32;
            let direction = Vec2 {
                x: angle.cos(),
                y: angle.sin(),
            };
            let end = Vec2 {
                x: center.x + direction.x * spread,
                y: center.y + direction.y * spread,
            };
            let start = Vec2 {
                x: end.x - direction.x * tail * fade,
                y: end.y - direction.y * tail * fade,
            };
            (start, end, 0.4 + fade * 1.3)
        })
        .collect()
}

#[allow(dead_code)]
fn tessellate_commands(commands: &[VectorCommand]) -> Vec<Vertex> {
    tessellate_commands_with_style_scale(commands, 1.0, 1.0)
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn tessellate_commands_with_style_scale(
    commands: &[VectorCommand],
    width_scale: f32,
    intensity_scale: f32,
) -> Vec<Vertex> {
    tessellate_commands_with_view(
        commands,
        width_scale,
        intensity_scale,
        VectorFrameView::default().resolve(1, 1),
    )
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn tessellate_commands_with_view(
    commands: &[VectorCommand],
    width_scale: f32,
    intensity_scale: f32,
    view: ResolvedVectorFrameView,
) -> Vec<Vertex> {
    let mut vertices = Vec::new();

    for command in commands {
        match command {
            VectorCommand::Line(line) => {
                push_line_vertices_with_view(
                    &mut vertices,
                    line.start,
                    line.end,
                    scaled_style(line.style, width_scale, intensity_scale),
                    view,
                );
            }
            VectorCommand::Polyline(polyline) => {
                for points in polyline.points.windows(2) {
                    push_line_vertices_with_view(
                        &mut vertices,
                        points[0],
                        points[1],
                        scaled_style(polyline.style, width_scale, intensity_scale),
                        view,
                    );
                }
            }
        }
    }

    vertices
}

#[allow(dead_code)]
fn tessellate_glow_commands(
    commands: &[VectorCommand],
    settings: VectorDisplaySettings,
) -> Vec<GlowVertex> {
    tessellate_glow_commands_with_view(commands, settings, VectorFrameView::default().resolve(1, 1))
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn tessellate_glow_commands_with_view(
    commands: &[VectorCommand],
    settings: VectorDisplaySettings,
    view: ResolvedVectorFrameView,
) -> Vec<GlowVertex> {
    let mut vertices = Vec::new();
    let stroke_width_scale = settings.stroke_width_scale();

    for layer in settings.glow_layers() {
        for command in commands {
            match command {
                VectorCommand::Line(line) => {
                    push_glow_line_vertices_with_view(
                        &mut vertices,
                        line.start,
                        line.end,
                        scaled_style(line.style, stroke_width_scale, 1.0),
                        *layer,
                        view,
                    );
                }
                VectorCommand::Polyline(polyline) => {
                    let style = scaled_style(polyline.style, stroke_width_scale, 1.0);
                    for points in polyline.points.windows(2) {
                        push_glow_line_vertices_with_view(
                            &mut vertices,
                            points[0],
                            points[1],
                            style,
                            *layer,
                            view,
                        );
                    }
                }
            }
        }
    }

    vertices
}

fn scaled_style(style: StrokeStyle, width_scale: f32, intensity_scale: f32) -> StrokeStyle {
    StrokeStyle {
        width: style.width * width_scale,
        intensity: style.intensity * intensity_scale,
        ..style
    }
}

#[derive(Clone, Copy)]
struct LineBasis {
    tangent: Vec2,
    normal_unit: Vec2,
    normal: Vec2,
}

fn line_basis(start: Vec2, end: Vec2, width: f32) -> Option<LineBasis> {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let length = (dx * dx + dy * dy).sqrt();
    if length <= f32::EPSILON || width <= 0.0 {
        return None;
    }

    let tangent = Vec2 {
        x: dx / length,
        y: dy / length,
    };
    let normal_unit = Vec2 {
        x: -tangent.y,
        y: tangent.x,
    };
    let half_width = width * 0.5;

    Some(LineBasis {
        tangent,
        normal_unit,
        normal: Vec2 {
            x: normal_unit.x * half_width,
            y: normal_unit.y * half_width,
        },
    })
}

fn mapped_line_basis(start: Vec2, end: Vec2) -> Option<LineBasis> {
    line_basis(start, end, 1.0)
}

fn vertex_color(style: StrokeStyle) -> [f32; 4] {
    [
        style.color.red * style.color.alpha * style.intensity,
        style.color.green * style.color.alpha * style.intensity,
        style.color.blue * style.color.alpha * style.intensity,
        style.color.alpha,
    ]
}

fn glow_color(style: StrokeStyle, layer: GlowLayer) -> [f32; 4] {
    [
        style.color.red * style.color.alpha * style.intensity * layer.intensity_scale,
        style.color.green * style.color.alpha * style.intensity * layer.intensity_scale,
        style.color.blue * style.color.alpha * style.intensity * layer.intensity_scale,
        style.color.alpha,
    ]
}

fn point_array(point: Vec2) -> [f32; 2] {
    [point.x, point.y]
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn push_line_vertices_with_view(
    vertices: &mut Vec<Vertex>,
    start: Vec2,
    end: Vec2,
    style: StrokeStyle,
    view: ResolvedVectorFrameView,
) {
    let Some(basis) = line_basis(start, end, style.width) else {
        return;
    };
    let color = vertex_color(style);
    let mapped_start = view.map_point(start);
    let mapped_end = view.map_point(end);
    let Some(mapped_basis) = mapped_line_basis(mapped_start, mapped_end) else {
        return;
    };
    let normal_width = style.width * view.perpendicular_scale_for_tangent(basis.tangent) * 0.5;
    let normal = Vec2 {
        x: mapped_basis.normal_unit.x * normal_width,
        y: mapped_basis.normal_unit.y * normal_width,
    };

    let a = Vertex {
        position: point_array(Vec2 {
            x: mapped_start.x - normal.x,
            y: mapped_start.y - normal.y,
        }),
        color,
    };
    let b = Vertex {
        position: point_array(Vec2 {
            x: mapped_end.x - normal.x,
            y: mapped_end.y - normal.y,
        }),
        color,
    };
    let c = Vertex {
        position: point_array(Vec2 {
            x: mapped_end.x + normal.x,
            y: mapped_end.y + normal.y,
        }),
        color,
    };
    let d = Vertex {
        position: point_array(Vec2 {
            x: mapped_start.x + normal.x,
            y: mapped_start.y + normal.y,
        }),
        color,
    };

    vertices.extend_from_slice(&[a, b, c, a, c, d]);
}

#[allow(dead_code)]
fn push_line_vertices(vertices: &mut Vec<Vertex>, start: Vec2, end: Vec2, style: StrokeStyle) {
    let Some(basis) = line_basis(start, end, style.width) else {
        return;
    };
    let color = vertex_color(style);

    let a = Vertex {
        position: [start.x - basis.normal.x, start.y - basis.normal.y],
        color,
    };
    let b = Vertex {
        position: [end.x - basis.normal.x, end.y - basis.normal.y],
        color,
    };
    let c = Vertex {
        position: [end.x + basis.normal.x, end.y + basis.normal.y],
        color,
    };
    let d = Vertex {
        position: [start.x + basis.normal.x, start.y + basis.normal.y],
        color,
    };

    vertices.extend_from_slice(&[a, b, c, a, c, d]);
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn push_glow_line_vertices_with_view(
    vertices: &mut Vec<GlowVertex>,
    start: Vec2,
    end: Vec2,
    style: StrokeStyle,
    layer: GlowLayer,
    view: ResolvedVectorFrameView,
) {
    let Some(basis) = line_basis(start, end, style.width) else {
        return;
    };
    let radius = style.width * layer.width_scale * 0.5;
    let normal_scale = view.perpendicular_scale_for_tangent(basis.tangent);
    let radius_clip = radius * normal_scale;
    let core_width_clip = style.width * normal_scale;
    let color = glow_color(style, layer);
    let mapped_start = view.map_point(start);
    let mapped_end = view.map_point(end);
    let Some(mapped_basis) = mapped_line_basis(mapped_start, mapped_end) else {
        return;
    };
    let normal = Vec2 {
        x: mapped_basis.normal_unit.x * radius_clip,
        y: mapped_basis.normal_unit.y * radius_clip,
    };
    let start_cap = Vec2 {
        x: mapped_start.x - mapped_basis.tangent.x * radius_clip,
        y: mapped_start.y - mapped_basis.tangent.y * radius_clip,
    };
    let end_cap = Vec2 {
        x: mapped_end.x + mapped_basis.tangent.x * radius_clip,
        y: mapped_end.y + mapped_basis.tangent.y * radius_clip,
    };

    let a = glow_vertex(
        Vec2 {
            x: start_cap.x - normal.x,
            y: start_cap.y - normal.y,
        },
        mapped_start,
        mapped_end,
        color,
        radius_clip,
        core_width_clip,
    );
    let b = glow_vertex(
        Vec2 {
            x: end_cap.x - normal.x,
            y: end_cap.y - normal.y,
        },
        mapped_start,
        mapped_end,
        color,
        radius_clip,
        core_width_clip,
    );
    let c = glow_vertex(
        Vec2 {
            x: end_cap.x + normal.x,
            y: end_cap.y + normal.y,
        },
        mapped_start,
        mapped_end,
        color,
        radius_clip,
        core_width_clip,
    );
    let d = glow_vertex(
        Vec2 {
            x: start_cap.x + normal.x,
            y: start_cap.y + normal.y,
        },
        mapped_start,
        mapped_end,
        color,
        radius_clip,
        core_width_clip,
    );

    vertices.extend_from_slice(&[a, b, c, a, c, d]);
}

#[allow(dead_code)]
fn push_glow_line_vertices(
    vertices: &mut Vec<GlowVertex>,
    start: Vec2,
    end: Vec2,
    style: StrokeStyle,
    layer: GlowLayer,
) {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let length = (dx * dx + dy * dy).sqrt();
    if length <= f32::EPSILON || style.width <= 0.0 {
        return;
    }

    let radius = style.width * layer.width_scale * 0.5;
    let core_width = style.width;
    let tangent_x = dx / length;
    let tangent_y = dy / length;
    let normal_x = -tangent_y * radius;
    let normal_y = tangent_x * radius;
    let start_cap = Vec2 {
        x: start.x - tangent_x * radius,
        y: start.y - tangent_y * radius,
    };
    let end_cap = Vec2 {
        x: end.x + tangent_x * radius,
        y: end.y + tangent_y * radius,
    };
    let color = [
        style.color.red * style.color.alpha * style.intensity * layer.intensity_scale,
        style.color.green * style.color.alpha * style.intensity * layer.intensity_scale,
        style.color.blue * style.color.alpha * style.intensity * layer.intensity_scale,
        style.color.alpha,
    ];

    let a = glow_vertex(
        Vec2 {
            x: start_cap.x - normal_x,
            y: start_cap.y - normal_y,
        },
        start,
        end,
        color,
        radius,
        core_width,
    );
    let b = glow_vertex(
        Vec2 {
            x: end_cap.x - normal_x,
            y: end_cap.y - normal_y,
        },
        start,
        end,
        color,
        radius,
        core_width,
    );
    let c = glow_vertex(
        Vec2 {
            x: end_cap.x + normal_x,
            y: end_cap.y + normal_y,
        },
        start,
        end,
        color,
        radius,
        core_width,
    );
    let d = glow_vertex(
        Vec2 {
            x: start_cap.x + normal_x,
            y: start_cap.y + normal_y,
        },
        start,
        end,
        color,
        radius,
        core_width,
    );

    vertices.extend_from_slice(&[a, b, c, a, c, d]);
}

fn glow_vertex(
    position: Vec2,
    start: Vec2,
    end: Vec2,
    color: [f32; 4],
    radius: f32,
    core_width: f32,
) -> GlowVertex {
    GlowVertex {
        position: [position.x, position.y],
        color,
        segment_start: [start.x, start.y],
        segment_end: [end.x, end.y],
        radius,
        core_width,
    }
}

#[cfg(target_arch = "wasm32")]
fn browser_has_webgpu(window: &web_sys::Window) -> bool {
    js_sys::Reflect::has(window.navigator().as_ref(), &JsValue::from_str("gpu")).unwrap_or(false)
}

#[cfg(target_arch = "wasm32")]
fn resize_canvas_to_display_size(
    window: &web_sys::Window,
    canvas: &web_sys::HtmlCanvasElement,
) -> Result<(u32, u32), JsValue> {
    let device_pixel_ratio = window.device_pixel_ratio();
    let width = ((canvas.client_width() as f64 * device_pixel_ratio).round() as u32).max(1);
    let height = ((canvas.client_height() as f64 * device_pixel_ratio).round() as u32).max(1);

    canvas.set_width(width);
    canvas.set_height(height);

    Ok((width, height))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renderer_error_display_text_matches_exactly() {
        assert_eq!(
            RendererError::UnsupportedSurfaceFormat.to_string(),
            "The WebGPU adapter does not report any supported surface formats."
        );
        assert_eq!(
            RendererError::UnsupportedAlphaMode.to_string(),
            "The WebGPU adapter does not report any supported alpha modes."
        );
        assert_eq!(
            RendererError::MissingPresentMode.to_string(),
            "The WebGPU adapter does not support the required FIFO presentation mode."
        );
        assert_eq!(
            RendererError::InsufficientLimits.to_string(),
            "The WebGPU adapter does not meet Velumin's required rendering limits."
        );
        assert_eq!(
            RendererError::SurfaceTextureUnavailable.to_string(),
            "Surface texture is temporarily unavailable; try rendering again later."
        );
        assert_eq!(
            RendererError::FrameAcquisitionFailed(wgpu::CurrentSurfaceTexture::Lost).to_string(),
            "Failed to get frame from WebGPU surface: Lost"
        );
    }

    #[test]
    fn device_request_failed_message_wraps_the_debug_text() {
        // wgpu::RequestDeviceError has no public constructor, so it can't be
        // fabricated to test RendererError::DeviceRequestFailed's Display arm
        // directly; this tests the same static wording via the helper that
        // arm delegates to, with a synthetic debug string standing in for a
        // real error's {:?} output.
        assert_eq!(
            device_request_failed_message("some debug text"),
            "Device request failed. Required WebGPU features or limits may be unavailable: some debug text"
        );
    }

    fn white_style(width: f32) -> StrokeStyle {
        StrokeStyle {
            width,
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0,
            },
            intensity: 1.0,
        }
    }

    #[test]
    fn line_tessellates_to_two_triangles() {
        let vertices = tessellate_commands(&[VectorCommand::Line(Line {
            start: Vec2 { x: -0.75, y: 0.0 },
            end: Vec2 { x: 0.75, y: 0.0 },
            style: white_style(0.04),
        })]);

        assert_eq!(vertices.len(), 6);
        assert_eq!(vertices[0].position, [-0.75, -0.02]);
        assert_eq!(vertices[2].position, [0.75, 0.02]);
        assert_eq!(vertices[5].position, [-0.75, 0.02]);
    }

    #[test]
    fn polyline_tessellates_each_segment() {
        let vertices = tessellate_commands(&[VectorCommand::Polyline(Polyline {
            points: vec![
                Vec2 { x: -0.5, y: 0.0 },
                Vec2 { x: 0.0, y: 0.0 },
                Vec2 { x: 0.5, y: 0.0 },
            ],
            style: white_style(0.04),
        })]);

        assert_eq!(vertices.len(), 12);
    }

    #[test]
    fn zero_length_lines_are_skipped() {
        let vertices = tessellate_commands(&[VectorCommand::Line(Line {
            start: Vec2 { x: 0.0, y: 0.0 },
            end: Vec2 { x: 0.0, y: 0.0 },
            style: white_style(0.04),
        })]);

        assert!(vertices.is_empty());
    }

    #[test]
    fn layered_glow_tessellates_multiple_soft_emission_bands() {
        let settings = display_settings_from_preset(VectorDisplayPreset::ArcadeBalanced);
        let vertices = tessellate_glow_commands(
            &[VectorCommand::Line(Line {
                start: Vec2 { x: -0.75, y: 0.0 },
                end: Vec2 { x: 0.75, y: 0.0 },
                style: white_style(0.04),
            })],
            settings,
        );

        assert_eq!(vertices.len(), settings.glow_layers().len() * 6);
        assert_vec2_near(vertices[0].position, [-0.7654, -0.0154]);
        assert_vec2_near(vertices[2].position, [0.7654, 0.0154]);
        assert_color_near(vertices[0].color, [0.33, 0.33, 0.33, 1.0]);
        assert_vec2_near(vertices[12].position, [-0.8361, -0.0861]);
        assert_vec2_near(vertices[14].position, [0.8361, 0.0861]);
        assert_color_near(vertices[12].color, [0.03, 0.03, 0.03, 1.0]);
        assert_eq!(vertices[0].segment_start, [-0.75, 0.0]);
        assert_eq!(vertices[0].segment_end, [0.75, 0.0]);
        assert_near(vertices[0].radius, 0.0154);
        assert_near(vertices[12].radius, 0.0861);
    }

    #[test]
    fn transparent_strokes_emit_no_rgb_for_crisp_or_glow() {
        let transparent_style = StrokeStyle {
            width: 0.04,
            color: Color {
                red: 0.8,
                green: 0.4,
                blue: 0.2,
                alpha: 0.0,
            },
            intensity: 2.0,
        };
        let commands = [VectorCommand::Line(Line {
            start: Vec2 { x: -0.75, y: 0.0 },
            end: Vec2 { x: 0.75, y: 0.0 },
            style: transparent_style,
        })];
        let settings = display_settings_from_preset(VectorDisplayPreset::ArcadeBalanced);

        let crisp_vertices = tessellate_commands(&commands);
        let glow_vertices = tessellate_glow_commands(&commands, settings);

        assert!(!crisp_vertices.is_empty());
        assert!(!glow_vertices.is_empty());
        assert_color_near(crisp_vertices[0].color, [0.0, 0.0, 0.0, 0.0]);
        assert_color_near(glow_vertices[0].color, [0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn arcade_balanced_preset_encodes_tuned_defaults() {
        let settings = display_settings_from_preset(VectorDisplayPreset::ArcadeBalanced);

        assert_near(settings.stroke_width_scale(), 0.35);
        assert_eq!(settings.glow_layers().len(), 3);
        assert_near(settings.glow_layers()[0].width_scale, 2.2);
        assert_near(settings.glow_layers()[0].intensity_scale, 0.33);
        assert_near(settings.glow_layers()[1].width_scale, 5.0);
        assert_near(settings.glow_layers()[1].intensity_scale, 0.12);
        assert_near(settings.glow_layers()[2].width_scale, 12.3);
        assert_near(settings.glow_layers()[2].intensity_scale, 0.03);
    }

    #[test]
    fn display_presets_have_valid_glow_layers() {
        for preset in [
            VectorDisplayPreset::ArcadeBalanced,
            VectorDisplayPreset::MonochromeBeam,
            VectorDisplayPreset::ColorQuadraScan,
            VectorDisplayPreset::CleanNeon,
        ] {
            let settings = display_settings_from_preset(preset);

            assert!(!settings.glow_layers().is_empty());
            for layer in settings.glow_layers() {
                assert!(layer.width_scale.is_finite());
                assert!(layer.intensity_scale.is_finite());
                assert!(layer.width_scale > 1.0);
                assert!(layer.intensity_scale > 0.0);
            }
        }
    }

    #[test]
    fn tuner_line_width_scales_crisp_and_glow_geometry() {
        let settings = VectorDisplaySettings::from_tuner(2.0, 2.2, 0.28, 5.0, 0.11, 9.0, 0.045);
        let commands = [VectorCommand::Line(Line {
            start: Vec2 { x: -0.75, y: 0.0 },
            end: Vec2 { x: 0.75, y: 0.0 },
            style: white_style(0.04),
        })];
        let crisp_vertices =
            tessellate_commands_with_style_scale(&commands, settings.stroke_width_scale(), 1.0);
        let glow_vertices = tessellate_glow_commands(&commands, settings);

        assert_vec2_near(crisp_vertices[0].position, [-0.75, -0.04]);
        assert_near(glow_vertices[0].radius, 0.088);
        assert_near(glow_vertices[0].core_width, 0.08);
    }

    #[test]
    fn blasterites_tester_scene_is_nonempty_at_key_times() {
        for time_ms in [0.0, 1500.0, 3200.0] {
            let commands = blasterites_tester_scene(time_ms);
            assert!(!commands.is_empty());
            assert!(!tessellate_commands(&commands).is_empty());
        }
    }

    #[test]
    fn blasterites_ship_outline_is_closed() {
        let points = blasterites_ship_outline(1400.0);

        assert_eq!(points.first(), points.last());
    }

    #[test]
    fn blasterites_bullet_exists_before_impact_and_sparks_after_impact() {
        assert!(blasterites_bullet_points(1800.0).is_some());
        assert!(blasterites_bullet_points(3400.0).is_none());
        assert!(blasterites_spark_lines(1800.0).is_empty());
        assert!(!blasterites_spark_lines(3400.0).is_empty());
    }

    #[test]
    fn blasterites_scene_commands_avoid_zero_length_segments() {
        for time_ms in [0.0, 1800.0, 3200.0, 4200.0] {
            for command in blasterites_tester_scene(time_ms) {
                match command {
                    VectorCommand::Line(line) => {
                        assert!(line_length(line.start, line.end) > f32::EPSILON);
                    }
                    VectorCommand::Polyline(polyline) => {
                        for points in polyline.points.windows(2) {
                            assert!(line_length(points[0], points[1]) > f32::EPSILON);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn blasterites_tester_is_deterministic_and_animates() {
        // Compare a deterministic byte representation of the WHOLE tessellated
        // tester scene (ship, bullet, asteroid, sparks, and scene assembly), not
        // just a command count or a single helper — the invariant the browser
        // screenshot smoke check (WI-SMOKE-0001) relies on to target known frames.
        let scene_bytes = |time_ms: f32| -> Vec<u8> {
            let vertices = tessellate_commands(&blasterites_tester_scene(time_ms));
            bytemuck::cast_slice::<Vertex, u8>(&vertices).to_vec()
        };

        // Deterministic: the same elapsed time yields byte-identical geometry.
        let pre = scene_bytes(2000.0);
        assert!(!pre.is_empty());
        assert_eq!(pre, scene_bytes(2000.0));

        // Animates: the deterministic pre-impact (t=2000ms) and post-impact
        // (t=4000ms) frames differ across the whole scene, so a frozen or
        // static render is caught.
        assert_ne!(pre, scene_bytes(4000.0));
    }

    #[test]
    fn vector_frame_builds_line_and_reports_length() {
        let mut frame = VectorFrame::new();

        frame
            .push_line(-0.5, 0.0, 0.5, 0.0, 0.2, 0.8, 1.0, 1.0, 0.02, 1.25)
            .unwrap();

        assert_eq!(frame.len(), 1);
        assert!(!frame.is_empty());
        match &frame.commands()[0] {
            VectorCommand::Line(line) => {
                assert_eq!(line.start, Vec2 { x: -0.5, y: 0.0 });
                assert_eq!(line.end, Vec2 { x: 0.5, y: 0.0 });
                assert_near(line.style.width, 0.02);
                assert_near(line.style.intensity, 1.25);
                assert_near(line.style.color.green, 0.8);
            }
            VectorCommand::Polyline(_) => panic!("expected line command"),
        }
    }

    #[test]
    fn vector_frame_clear_removes_commands() {
        let mut frame = VectorFrame::new();
        frame
            .push_line(-0.5, 0.0, 0.5, 0.0, 1.0, 1.0, 1.0, 1.0, 0.02, 1.0)
            .unwrap();

        frame.clear();

        assert!(frame.is_empty());
        assert_eq!(frame.len(), 0);
    }

    #[test]
    fn vector_frame_builds_polyline_from_flat_point_pairs() {
        let mut frame = VectorFrame::new();

        frame
            .push_polyline(
                &[-0.4, -0.1, 0.0, 0.2, 0.4, -0.1],
                1.0,
                0.5,
                0.2,
                1.0,
                0.015,
                0.9,
            )
            .unwrap();

        match &frame.commands()[0] {
            VectorCommand::Polyline(polyline) => {
                assert_eq!(
                    polyline.points,
                    vec![
                        Vec2 { x: -0.4, y: -0.1 },
                        Vec2 { x: 0.0, y: 0.2 },
                        Vec2 { x: 0.4, y: -0.1 },
                    ]
                );
                assert_near(polyline.style.color.red, 1.0);
                assert_near(polyline.style.width, 0.015);
            }
            VectorCommand::Line(_) => panic!("expected polyline command"),
        }
    }

    #[test]
    fn vector_frame_closed_polyline_repeats_open_first_point() {
        let mut frame = VectorFrame::new();

        frame
            .push_closed_polyline(
                &[-0.1, -0.1, 0.1, -0.1, 0.0, 0.1],
                1.0,
                1.0,
                1.0,
                1.0,
                0.02,
                1.0,
            )
            .unwrap();

        match &frame.commands()[0] {
            VectorCommand::Polyline(polyline) => {
                assert_eq!(polyline.points.len(), 4);
                assert_eq!(polyline.points.first(), polyline.points.last());
            }
            VectorCommand::Line(_) => panic!("expected polyline command"),
        }
    }

    #[test]
    fn vector_frame_closed_polyline_does_not_duplicate_already_closed_points() {
        let mut frame = VectorFrame::new();

        frame
            .push_closed_polyline(
                &[-0.1, -0.1, 0.1, -0.1, 0.0, 0.1, -0.1, -0.1],
                1.0,
                1.0,
                1.0,
                1.0,
                0.02,
                1.0,
            )
            .unwrap();

        match &frame.commands()[0] {
            VectorCommand::Polyline(polyline) => assert_eq!(polyline.points.len(), 4),
            VectorCommand::Line(_) => panic!("expected polyline command"),
        }
    }

    #[test]
    fn vector_frame_rejects_malformed_and_non_finite_input() {
        let mut frame = VectorFrame::new();

        assert_eq!(
            frame.push_polyline(&[0.0, 0.0, 0.5], 1.0, 1.0, 1.0, 1.0, 0.02, 1.0),
            Err(VectorFrameInputError::InvalidPointArrayLength)
        );
        assert_eq!(
            frame.push_polyline(&[0.0, 0.0], 1.0, 1.0, 1.0, 1.0, 0.02, 1.0),
            Err(VectorFrameInputError::TooFewPolylinePoints)
        );
        assert_eq!(
            frame.push_closed_polyline(&[0.0, 0.0, 0.5, 0.0], 1.0, 1.0, 1.0, 1.0, 0.02, 1.0),
            Err(VectorFrameInputError::TooFewClosedPolylinePoints)
        );
        assert_eq!(
            frame.push_line(0.0, 0.0, f32::NAN, 0.0, 1.0, 1.0, 1.0, 1.0, 0.02, 1.0),
            Err(VectorFrameInputError::NonFiniteValue)
        );
        assert_eq!(
            frame.push_line(0.0, 0.0, 0.1, 0.1, 1.2, 1.0, 1.0, 1.0, 0.02, 1.0),
            Err(VectorFrameInputError::InvalidColorRange)
        );
        assert_eq!(
            frame.push_line(0.0, 0.0, 0.1, 0.1, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0),
            Err(VectorFrameInputError::InvalidStrokeWidth)
        );
        assert_eq!(
            frame.push_line(0.0, 0.0, 0.1, 0.1, 1.0, 1.0, 1.0, 1.0, 0.02, -0.1),
            Err(VectorFrameInputError::InvalidIntensity)
        );
        assert!(frame.is_empty());
    }

    #[test]
    fn vector_frame_view_rejects_invalid_extents() {
        assert_eq!(
            VectorFrameView::logical_extents(0.0, 0.0, 0.0, 1.0),
            Err(VectorFrameViewError::DegenerateExtents)
        );
        assert_eq!(
            VectorFrameView::logical_extents(0.0, f32::NAN, 1.0, 1.0),
            Err(VectorFrameViewError::NonFiniteValue)
        );
    }

    #[test]
    fn default_frame_view_keeps_centered_four_by_three_viewport() {
        assert_eq!(
            VectorFrameView::default().resolve(1600, 600).viewport,
            RenderViewport {
                x: 400,
                y: 0,
                width: 800,
                height: 600,
            }
        );
    }

    #[test]
    fn canvas_pixel_frame_view_maps_top_left_pixels_to_full_viewport_clip_space() {
        let view = VectorFrameView::canvas_pixels(800.0, 600.0)
            .unwrap()
            .resolve(1280, 480);

        assert_eq!(
            view.viewport,
            RenderViewport {
                x: 0,
                y: 0,
                width: 1280,
                height: 480,
            }
        );
        let top_left = view.map_point(Vec2 { x: 0.0, y: 0.0 });
        let bottom_right = view.map_point(Vec2 { x: 800.0, y: 600.0 });

        assert_vec2_near([top_left.x, top_left.y], [-1.0, 1.0]);
        assert_vec2_near([bottom_right.x, bottom_right.y], [1.0, -1.0]);
    }

    #[test]
    fn frame_view_mapping_shared_by_vector_frame_and_typed_commands() {
        let style = stroke(
            8.0,
            Color {
                red: 0.45,
                green: 0.9,
                blue: 1.0,
                alpha: 1.0,
            },
            1.25,
        );
        let typed_commands = [VectorCommand::Line(Line {
            start: Vec2 { x: 0.0, y: 0.0 },
            end: Vec2 { x: 800.0, y: 0.0 },
            style,
        })];
        let mut frame = VectorFrame::new();
        frame
            .push_line(0.0, 0.0, 800.0, 0.0, 0.45, 0.9, 1.0, 1.0, 8.0, 1.25)
            .unwrap();
        let view = VectorFrameView::canvas_pixels(800.0, 600.0)
            .unwrap()
            .resolve(800, 600);

        let frame_vertices = tessellate_commands_with_view(frame.commands(), 1.0, 1.0, view);
        let typed_vertices = tessellate_commands_with_view(&typed_commands, 1.0, 1.0, view);

        assert_eq!(frame_vertices.len(), 6);
        assert_eq!(typed_vertices.len(), 6);
        assert_eq!(
            bytemuck::cast_slice::<Vertex, u8>(&frame_vertices),
            bytemuck::cast_slice::<Vertex, u8>(&typed_vertices)
        );
    }

    #[test]
    fn canvas_pixel_frame_view_preserves_horizontal_and_vertical_stroke_widths() {
        let view = VectorFrameView::canvas_pixels(1280.0, 480.0)
            .unwrap()
            .resolve(1280, 480);
        let style = white_style(10.0);
        let horizontal = tessellate_commands_with_view(
            &[VectorCommand::Line(Line {
                start: Vec2 { x: 0.0, y: 120.0 },
                end: Vec2 {
                    x: 1280.0,
                    y: 120.0,
                },
                style,
            })],
            1.0,
            1.0,
            view,
        );
        let vertical = tessellate_commands_with_view(
            &[VectorCommand::Line(Line {
                start: Vec2 { x: 320.0, y: 0.0 },
                end: Vec2 { x: 320.0, y: 480.0 },
                style,
            })],
            1.0,
            1.0,
            view,
        );

        let horizontal_clip_width = horizontal[2].position[1] - horizontal[0].position[1];
        let vertical_clip_width = vertical[2].position[0] - vertical[0].position[0];
        let horizontal_pixels = horizontal_clip_width.abs() * view.viewport.height as f32 * 0.5;
        let vertical_pixels = vertical_clip_width.abs() * view.viewport.width as f32 * 0.5;

        assert_near(horizontal_pixels, 10.0);
        assert_near(vertical_pixels, 10.0);
    }

    #[test]
    fn anisotropic_frame_view_glow_quad_matches_shader_distance_for_diagonal_lines() {
        let view = VectorFrameView::canvas_pixels(1280.0, 480.0)
            .unwrap()
            .resolve(1280, 480);
        let settings = VectorDisplaySettings::from_layers(
            &[GlowLayer {
                width_scale: 2.0,
                intensity_scale: 0.5,
            }],
            1.0,
        );
        let vertices = tessellate_glow_commands_with_view(
            &[VectorCommand::Line(Line {
                start: Vec2 { x: 120.0, y: 100.0 },
                end: Vec2 { x: 940.0, y: 360.0 },
                style: white_style(12.0),
            })],
            settings,
            view,
        );

        assert_eq!(vertices.len(), 6);
        let first = vertices[0];
        let start = Vec2 {
            x: first.segment_start[0],
            y: first.segment_start[1],
        };
        let end = Vec2 {
            x: first.segment_end[0],
            y: first.segment_end[1],
        };
        let position = Vec2 {
            x: first.position[0],
            y: first.position[1],
        };
        let distance = point_line_distance(position, start, end);

        assert_near(distance, first.radius);
    }

    #[test]
    fn typed_vector_command_scene_reaches_renderer_geometry_path() {
        let commands = vec![
            VectorCommand::Polyline(Polyline {
                points: vec![
                    Vec2 { x: -0.7, y: 0.0 },
                    Vec2 { x: -0.84, y: 0.08 },
                    Vec2 { x: -0.78, y: 0.0 },
                    Vec2 { x: -0.84, y: -0.08 },
                    Vec2 { x: -0.7, y: 0.0 },
                ],
                style: white_style(0.018),
            }),
            VectorCommand::Polyline(Polyline {
                points: vec![
                    Vec2 { x: 0.06, y: -0.24 },
                    Vec2 { x: 0.28, y: -0.34 },
                    Vec2 { x: 0.5, y: -0.22 },
                    Vec2 { x: 0.56, y: 0.02 },
                    Vec2 { x: 0.42, y: 0.24 },
                    Vec2 { x: 0.16, y: 0.28 },
                    Vec2 { x: -0.04, y: 0.1 },
                    Vec2 { x: 0.06, y: -0.24 },
                ],
                style: white_style(0.014),
            }),
            VectorCommand::Line(Line {
                start: Vec2 { x: -0.62, y: 0.0 },
                end: Vec2 { x: 0.02, y: 0.02 },
                style: stroke(
                    0.012,
                    Color {
                        red: 0.45,
                        green: 0.9,
                        blue: 1.0,
                        alpha: 1.0,
                    },
                    1.45,
                ),
            }),
        ];
        let settings = display_settings_from_preset(VectorDisplayPreset::ArcadeBalanced);

        let crisp_vertices = tessellate_commands(&commands);
        let glow_vertices = tessellate_glow_commands(&commands, settings);

        assert!(!crisp_vertices.is_empty());
        assert!(!glow_vertices.is_empty());
        assert!(crisp_vertices.len() >= 18);
        assert_eq!(glow_vertices.len() % settings.glow_layers().len(), 0);
    }

    fn line_length(start: Vec2, end: Vec2) -> f32 {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn point_line_distance(point: Vec2, start: Vec2, end: Vec2) -> f32 {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let length = (dx * dx + dy * dy).sqrt();
        ((point.x - start.x) * dy - (point.y - start.y) * dx).abs() / length
    }

    fn assert_vec2_near(actual: [f32; 2], expected: [f32; 2]) {
        assert_near(actual[0], expected[0]);
        assert_near(actual[1], expected[1]);
    }

    fn assert_color_near(actual: [f32; 4], expected: [f32; 4]) {
        for index in 0..4 {
            assert_near(actual[index], expected[index]);
        }
    }

    fn assert_near(actual: f32, expected: f32) {
        assert!((actual - expected).abs() <= 0.00001);
    }
}
