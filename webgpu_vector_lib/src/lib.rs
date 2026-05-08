use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct WebGPU {
    canvas: web_sys::HtmlCanvasElement,
    renderer: Renderer,
}

struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    surface: wgpu::Surface<'static>,
}

#[wasm_bindgen]
impl WebGPU {
    #[wasm_bindgen(constructor)]
    pub async fn new(canvas_id: &str) -> Result<WebGPU, JsValue> {
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
        self.renderer.render()
    }
}

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
        log("Shader module created");

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Line Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
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
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });
        log("Render pipeline created");

        Ok(Self {
            device,
            queue,
            config,
            render_pipeline,
            surface,
        })
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width == self.config.width && height == self.config.height {
            return;
        }

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        log(&format!("Surface reconfigured to {}x{}", width, height));
    }

    fn render(&self) -> Result<(), JsValue> {
        log("Starting render call");
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

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
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

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_viewport(
                0.0,
                0.0,
                self.config.width as f32,
                self.config.height as f32,
                0.0,
                1.0,
            );
            render_pass.set_scissor_rect(0, 0, self.config.width, self.config.height);
            render_pass.draw(0..6, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
        log("Frame submitted and presented");
        Ok(())
    }
}

fn browser_has_webgpu(window: &web_sys::Window) -> bool {
    js_sys::Reflect::has(window.navigator().as_ref(), &JsValue::from_str("gpu")).unwrap_or(false)
}

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
