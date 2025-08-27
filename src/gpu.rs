use std::sync::Arc;

use log::warn;
use winit::window::Window;

use crate::{LResult, LuxError, buffer::Buffer, color::Srgba, pipeline::Pipeline};

pub trait RenderCommand {
    fn render(&self, render_pass: &mut wgpu::RenderPass);
}

#[derive(Default)]
pub struct RenderQueue {
    clear_color: Srgba,
    commands: Vec<Box<dyn RenderCommand>>,
}
impl RenderQueue {
    pub fn new(color: Srgba) -> Self {
        Self {
            clear_color: color,
            commands: Vec::new(),
        }
    }

    pub fn get_clear_color(&self) -> Srgba {
        self.clear_color
    }

    pub fn get_commands(&self) -> &Vec<Box<dyn RenderCommand>> {
        &self.commands
    }

    pub fn draw<T: RenderCommand + Clone + 'static>(&mut self, command: &T) {
        self.commands.push(Box::new(command.clone()));
    }
}

pub struct Gpu {
    main_window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_setup: bool,
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

        Ok(Self {
            config,
            surface,
            device,
            queue,
            main_window: window,
            is_surface_setup: false,
        })
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

            for cmd in render_queue.get_commands() {
                cmd.render(&mut render_pass);
            }
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
