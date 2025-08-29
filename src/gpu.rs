use std::sync::Arc;

use glam::Vec2;
use log::warn;
use winit::window::Window;

use crate::{
    LResult, LuxError,
    color::Srgba,
    material::StandardMaterial2d,
    model::{Mesh, MeshBuilder},
    shapes::DrawRect,
    vertex::Vertex2,
};

pub trait DrawCommand {
    fn render(&self, gpu: &Gpu, ctx: &mut RenderContext);
}

#[derive(Default, Clone)]
pub struct Camera2d {
    pub position: Vec2,
}

#[derive(Default)]
pub struct Update2d {
    pub camera: Camera2d,
}
impl Update2d {
    pub fn camera(&mut self, camera: Camera2d) -> &mut Self {
        self.camera = camera;
        self
    }
}
impl DrawCommand for Update2d {
    fn render(&self, _gpu: &Gpu, ctx: &mut RenderContext) {
        ctx.camera2d = self.camera.clone();
    }
}

pub struct RenderContext<'a> {
    pub render_pass: wgpu::RenderPass<'a>,
    pub camera2d: Camera2d,
}

#[derive(Default)]
pub struct RenderQueue {
    clear_color: Srgba,
    camera: Camera2d,
    commands: Vec<Box<dyn DrawCommand>>,
}
impl RenderQueue {
    pub fn new(color: Srgba) -> Self {
        Self {
            clear_color: color,
            camera: Camera2d::default(),
            commands: Vec::new(),
        }
    }

    pub fn get_clear_color(&self) -> Srgba {
        self.clear_color
    }

    pub fn get_commands(&self) -> &Vec<Box<dyn DrawCommand>> {
        &self.commands
    }

    pub fn draw<T: DrawCommand + Clone + 'static>(&mut self, command: &T) {
        self.commands.push(Box::new(command.clone()));
    }

    pub fn update_2d(&mut self, closure: impl Fn(&mut Update2d)) {
        let mut update = Update2d::default();
        closure(&mut update);
        self.commands.push(Box::new(update));
    }

    pub fn rect(&mut self, closure: impl Fn(&mut DrawRect)) {
        let mut draw = DrawRect::default();
        closure(&mut draw);
        self.commands.push(Box::new(draw));
    }
}

/// Used to hold meshes, shaders and other visual objects that are reused a lot.
#[derive(Debug)]
pub struct VisualConstants {
    quad_mesh: Mesh,
    default_2d_mat: StandardMaterial2d,
}
impl VisualConstants {
    const QUAD_VERT_BUFFER: [Vertex2; 4] = [
        Vertex2::from_xy(0.5, 0.5),
        Vertex2::from_xy(-0.5, 0.5),
        Vertex2::from_xy(-0.5, -0.5),
        Vertex2::from_xy(0.5, -0.5),
    ];
    const QUAD_IDX_BUFFER: [u16; 6] = [0, 1, 2, 0, 2, 3];

    pub fn new(gpu: &Gpu) -> Self {
        Self {
            quad_mesh: MeshBuilder {
                vertices: Self::QUAD_VERT_BUFFER.to_vec(),
                indices: Self::QUAD_IDX_BUFFER.to_vec(),
            }
            .build(gpu),
            default_2d_mat: StandardMaterial2d::new(gpu),
        }
    }

    pub fn get_quad_mesh(&self) -> &Mesh {
        &self.quad_mesh
    }

    pub fn get_default_2d_material(&self) -> &StandardMaterial2d {
        &self.default_2d_mat
    }
}

pub struct Gpu {
    main_window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_setup: bool,
    constants: Option<VisualConstants>,
}
impl Gpu {
    pub async fn new(window: Arc<Window>) -> LResult<Self> {
        let size = window.inner_size();
        let wgpu_instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });
        let surface = wgpu_instance
            .create_surface(window.clone())
            .expect("error creating surface");
        let adapter = wgpu_instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .map_err(|e| LuxError::WgpuFailedInit {
                error: e.to_string(),
            })?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .map_err(|e| LuxError::WgpuFailedInit {
                error: e.to_string(),
            })?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let mut gpu = Self {
            config,
            surface,
            device,
            queue,
            main_window: window,
            is_surface_setup: false,
            constants: None,
        };

        let constants = VisualConstants::new(&gpu);
        gpu.constants = Some(constants);

        Ok(gpu)
    }

    pub fn get_main_window(&self) -> &Window {
        &self.main_window
    }

    pub fn get_surface(&self) -> &wgpu::Surface<'static> {
        &self.surface
    }

    pub fn get_device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn get_queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn get_config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    pub fn get_constants(&self) -> &VisualConstants {
        self.constants.as_ref().unwrap()
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            warn!("tried to set window size to 0, ignoring...");
        }

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.is_surface_setup = true;
    }

    pub fn redraw(&self) {
        self.main_window.request_redraw();
    }

    pub fn render_queue(&self, render_queue: &RenderQueue) {
        self.main_window.request_redraw();
        if !self.is_surface_setup {
            return;
        }

        let output = self
            .surface
            .get_current_texture()
            .expect("error geting current texture");
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

        {
            let clear_color = render_queue.get_clear_color();
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: clear_color.r as f64,
                            g: clear_color.g as f64,
                            b: clear_color.b as f64,
                            a: clear_color.a as f64,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            let mut ctx = RenderContext {
                render_pass,
                camera2d: Camera2d::default(),
            };
            for cmd in render_queue.get_commands() {
                cmd.render(self, &mut ctx);
            }
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
