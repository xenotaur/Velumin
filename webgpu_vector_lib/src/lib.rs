#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

impl Vertex {
    #[cfg(target_arch = "wasm32")]
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4];

    #[cfg(target_arch = "wasm32")]
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

impl GlowVertex {
    #[cfg(target_arch = "wasm32")]
    const ATTRIBUTES: [wgpu::VertexAttribute; 6] = wgpu::vertex_attr_array![
        0 => Float32x2,
        1 => Float32x4,
        2 => Float32x2,
        3 => Float32x2,
        4 => Float32,
        5 => Float32
    ];

    #[cfg(target_arch = "wasm32")]
    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct VectorDisplaySettings {
    glow_layers: &'static [GlowLayer],
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl VectorDisplaySettings {
    const ARCADE_BALANCED_GLOW: [GlowLayer; 3] = [
        GlowLayer {
            width_scale: 2.2,
            intensity_scale: 0.28,
        },
        GlowLayer {
            width_scale: 5.0,
            intensity_scale: 0.11,
        },
        GlowLayer {
            width_scale: 9.0,
            intensity_scale: 0.045,
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

    fn from_preset(preset: VectorDisplayPreset) -> Self {
        match preset {
            VectorDisplayPreset::ArcadeBalanced => Self {
                glow_layers: &Self::ARCADE_BALANCED_GLOW,
            },
            VectorDisplayPreset::MonochromeBeam => Self {
                glow_layers: &Self::MONOCHROME_BEAM_GLOW,
            },
            VectorDisplayPreset::ColorQuadraScan => Self {
                glow_layers: &Self::COLOR_QUADRA_SCAN_GLOW,
            },
            VectorDisplayPreset::CleanNeon => Self {
                glow_layers: &Self::CLEAN_NEON_GLOW,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
enum VectorDisplayPreset {
    ArcadeBalanced,
    MonochromeBeam,
    ColorQuadraScan,
    CleanNeon,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct GlowLayer {
    width_scale: f32,
    intensity_scale: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct RenderViewport {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl RenderViewport {
    const TARGET_ASPECT: f32 = 4.0 / 3.0;

    fn centered_4_3(surface_width: u32, surface_height: u32) -> Self {
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
}

#[cfg(target_arch = "wasm32")]
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
    #[wasm_bindgen(js_name = create)]
    pub async fn create(canvas_id: &str) -> Result<WebGPU, JsValue> {
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

        let renderer = Renderer::new(surface, &adapter, width, height).await?;

        Ok(WebGPU { canvas, renderer })
    }

    #[wasm_bindgen]
    pub fn render(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("No window available")?;
        let (width, height) = resize_canvas_to_display_size(&window, &self.canvas)?;
        self.renderer.resize(width, height);
        self.renderer.render(&smoke_scene(), false)
    }

    #[wasm_bindgen]
    pub fn render_blasterites_tester(&mut self, time_ms: f64) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("No window available")?;
        let (width, height) = resize_canvas_to_display_size(&window, &self.canvas)?;
        self.renderer.resize(width, height);
        let wrapped_time_ms = time_ms.rem_euclid(BLASTERITES_CYCLE_MS as f64) as f32;
        self.renderer
            .render(&blasterites_tester_scene(wrapped_time_ms), true)
    }
}

#[cfg(target_arch = "wasm32")]
impl Renderer {
    async fn new(
        surface: wgpu::Surface<'static>,
        adapter: &wgpu::Adapter,
        width: u32,
        height: u32,
    ) -> Result<Self, JsValue> {
        let capabilities = surface.get_capabilities(adapter);
        let format = capabilities.formats.first().copied().ok_or_else(|| {
            JsValue::from_str("The WebGPU adapter does not report any supported surface formats.")
        })?;
        let alpha_mode = capabilities.alpha_modes.first().copied().ok_or_else(|| {
            JsValue::from_str("The WebGPU adapter does not report any supported alpha modes.")
        })?;
        if !capabilities
            .present_modes
            .contains(&wgpu::PresentMode::Fifo)
        {
            return Err(JsValue::from_str(
                "The WebGPU adapter does not support the required FIFO presentation mode.",
            ));
        }

        let required_limits = wgpu::Limits::downlevel_defaults();
        if !required_limits.check_limits(&adapter.limits()) {
            return Err(JsValue::from_str(
                "The WebGPU adapter does not meet Velumin's required rendering limits.",
            ));
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
            .map_err(|e| {
                JsValue::from_str(&format!(
                    "Device request failed. Required WebGPU features or limits may be unavailable: {:?}",
                    e
                ))
            })?;
        log("Device and queue acquired");

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
        log("Surface configured");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Line Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/line.wgsl").into()),
        });
        let glow_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Glow Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/glow.wgsl").into()),
        });
        log("Shader modules created");

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
        log("Vector render pipelines created");

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
        log("Glow composite pipelines created");

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
        log(&format!(
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
            display_settings: VectorDisplaySettings::from_preset(
                VectorDisplayPreset::ArcadeBalanced,
            ),
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
        log(&format!("Surface reconfigured to {}x{}", width, height));
    }

    fn render(&mut self, commands: &[VectorCommand], tester_effects: bool) -> Result<(), JsValue> {
        log("Starting render call");
        self.upload_vector_commands(commands);

        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return Err(JsValue::from_str(
                    "Surface texture is temporarily unavailable; try rendering again later.",
                ));
            }
            status => {
                return Err(JsValue::from_str(&format!(
                    "Failed to get frame from WebGPU surface: {:?}",
                    status
                )));
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

        let surface_viewport = RenderViewport::centered_4_3(self.config.width, self.config.height);
        let glow_viewport = RenderViewport::centered_4_3(self.glow_width, self.glow_height);

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
        log("Frame submitted and presented");
        Ok(())
    }

    fn upload_vector_commands(&mut self, commands: &[VectorCommand]) {
        let vertices = tessellate_commands(commands);
        self.vertex_count = upload_vertices(
            &self.device,
            &self.queue,
            "Vector Vertex Buffer",
            &vertices,
            &mut self.vertex_buffer,
            &mut self.vertex_capacity,
        );

        let glow_vertices = tessellate_glow_commands(commands, self.display_settings);
        self.glow_vertex_count = upload_glow_vertices(
            &self.device,
            &self.queue,
            "Glow Vector Vertex Buffer",
            &glow_vertices,
            &mut self.glow_vertex_buffer,
            &mut self.glow_vertex_capacity,
        );
        log(&format!("Uploaded {} vector vertices", vertices.len()));
    }
}

#[cfg(target_arch = "wasm32")]
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

#[cfg(target_arch = "wasm32")]
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

#[cfg(target_arch = "wasm32")]
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

#[cfg(target_arch = "wasm32")]
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

#[cfg(target_arch = "wasm32")]
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

#[cfg(target_arch = "wasm32")]
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

#[cfg(target_arch = "wasm32")]
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

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn transform_points(points: &[Vec2], offset: Vec2, angle: f32, scale: f32) -> Vec<Vec2> {
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

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn lerp_vec2(start: Vec2, end: Vec2, progress: f32) -> Vec2 {
    Vec2 {
        x: start.x + (end.x - start.x) * progress,
        y: start.y + (end.y - start.y) * progress,
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn stroke(width: f32, color: Color, intensity: f32) -> StrokeStyle {
    StrokeStyle {
        width,
        color,
        intensity,
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn tessellate_commands(commands: &[VectorCommand]) -> Vec<Vertex> {
    tessellate_commands_with_style_scale(commands, 1.0, 1.0)
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn tessellate_commands_with_style_scale(
    commands: &[VectorCommand],
    width_scale: f32,
    intensity_scale: f32,
) -> Vec<Vertex> {
    let mut vertices = Vec::new();

    for command in commands {
        match command {
            VectorCommand::Line(line) => {
                push_line_vertices(
                    &mut vertices,
                    line.start,
                    line.end,
                    scaled_style(line.style, width_scale, intensity_scale),
                );
            }
            VectorCommand::Polyline(polyline) => {
                for points in polyline.points.windows(2) {
                    push_line_vertices(
                        &mut vertices,
                        points[0],
                        points[1],
                        scaled_style(polyline.style, width_scale, intensity_scale),
                    );
                }
            }
        }
    }

    vertices
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn tessellate_glow_commands(
    commands: &[VectorCommand],
    settings: VectorDisplaySettings,
) -> Vec<GlowVertex> {
    let mut vertices = Vec::new();

    for layer in settings.glow_layers {
        for command in commands {
            match command {
                VectorCommand::Line(line) => {
                    push_glow_line_vertices(
                        &mut vertices,
                        line.start,
                        line.end,
                        line.style,
                        *layer,
                    );
                }
                VectorCommand::Polyline(polyline) => {
                    for points in polyline.points.windows(2) {
                        push_glow_line_vertices(
                            &mut vertices,
                            points[0],
                            points[1],
                            polyline.style,
                            *layer,
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

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn push_line_vertices(vertices: &mut Vec<Vertex>, start: Vec2, end: Vec2, style: StrokeStyle) {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let length = (dx * dx + dy * dy).sqrt();
    if length <= f32::EPSILON || style.width <= 0.0 {
        return;
    }

    let half_width = style.width * 0.5;
    let normal_x = -dy / length * half_width;
    let normal_y = dx / length * half_width;
    let color = [
        style.color.red * style.intensity,
        style.color.green * style.intensity,
        style.color.blue * style.intensity,
        style.color.alpha,
    ];

    let a = Vertex {
        position: [start.x - normal_x, start.y - normal_y],
        color,
    };
    let b = Vertex {
        position: [end.x - normal_x, end.y - normal_y],
        color,
    };
    let c = Vertex {
        position: [end.x + normal_x, end.y + normal_y],
        color,
    };
    let d = Vertex {
        position: [start.x + normal_x, start.y + normal_y],
        color,
    };

    vertices.extend_from_slice(&[a, b, c, a, c, d]);
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
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
        style.color.red * style.intensity * layer.intensity_scale,
        style.color.green * style.intensity * layer.intensity_scale,
        style.color.blue * style.intensity * layer.intensity_scale,
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
        style.width,
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
        style.width,
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
        style.width,
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
        style.width,
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
        let settings = VectorDisplaySettings::from_preset(VectorDisplayPreset::ArcadeBalanced);
        let vertices = tessellate_glow_commands(
            &[VectorCommand::Line(Line {
                start: Vec2 { x: -0.75, y: 0.0 },
                end: Vec2 { x: 0.75, y: 0.0 },
                style: white_style(0.04),
            })],
            settings,
        );

        assert_eq!(vertices.len(), settings.glow_layers.len() * 6);
        assert_vec2_near(vertices[0].position, [-0.794, -0.044]);
        assert_vec2_near(vertices[2].position, [0.794, 0.044]);
        assert_color_near(vertices[0].color, [0.28, 0.28, 0.28, 1.0]);
        assert_vec2_near(vertices[12].position, [-0.93, -0.18]);
        assert_vec2_near(vertices[14].position, [0.93, 0.18]);
        assert_color_near(vertices[12].color, [0.045, 0.045, 0.045, 1.0]);
        assert_eq!(vertices[0].segment_start, [-0.75, 0.0]);
        assert_eq!(vertices[0].segment_end, [0.75, 0.0]);
        assert_near(vertices[0].radius, 0.044);
        assert_near(vertices[12].radius, 0.18);
    }

    #[test]
    fn display_presets_have_valid_glow_layers() {
        for preset in [
            VectorDisplayPreset::ArcadeBalanced,
            VectorDisplayPreset::MonochromeBeam,
            VectorDisplayPreset::ColorQuadraScan,
            VectorDisplayPreset::CleanNeon,
        ] {
            let settings = VectorDisplaySettings::from_preset(preset);

            assert!(!settings.glow_layers.is_empty());
            for layer in settings.glow_layers {
                assert!(layer.width_scale.is_finite());
                assert!(layer.intensity_scale.is_finite());
                assert!(layer.width_scale > 1.0);
                assert!(layer.intensity_scale > 0.0);
            }
        }
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

    fn line_length(start: Vec2, end: Vec2) -> f32 {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        (dx * dx + dy * dy).sqrt()
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
