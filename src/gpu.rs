use std::{
    any::Any,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use glam::{UVec2, Vec2, Vec3Swizzles};
use glyphon::{Attrs, Resolution, TextArea, TextBounds};
use log::warn;
use winit::window::Window;

use crate::{
    LuxError,
    color::Srgba,
    material::{MATERIAL2D, Material, StandardMaterial2d},
    model::{Mesh, MeshBuilder, QUAD_MESH},
    shapes::Rect,
    texture::{Texture, TextureBuilder, WHITE_TEXTURE},
    vertex::Vertex2,
};

pub trait DrawCommand: Any {
    fn prepare(&self, _gpu: &mut Gpu) {}
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
impl<'a> DrawBuilder<'a, Update2d> {
    pub fn position(&mut self, position: Vec2) -> &mut Self {
        self.camera.position = position;
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

#[derive(Debug, Clone)]
pub struct DrawRect {
    pub position: Vec2,
    pub euler_angle: f32,
    pub scale: Vec2,
    pub rect: Rect,
    pub color: Srgba,
}
impl<'a> DrawBuilder<'a, DrawRect> {
    pub fn position(&mut self, position: Vec2) -> &mut Self {
        self.position = position;
        self
    }
    pub fn scale(&mut self, scale: Vec2) -> &mut Self {
        self.scale = scale;
        self
    }
    pub fn rect(&mut self, rect: Rect) -> &mut Self {
        self.rect = rect;
        self
    }
    pub fn color(&mut self, color: Srgba) -> &mut Self {
        self.color = color;
        self
    }
    pub fn angle(&mut self, euler_angle: f32) -> &mut Self {
        self.euler_angle = euler_angle;
        self
    }
}
impl Default for DrawRect {
    fn default() -> Self {
        Self {
            position: Default::default(),
            euler_angle: 0.0,
            scale: Vec2::ONE,
            rect: Rect::from_xy(25.0, 25.0),
            color: Srgba::WHITE,
        }
    }
}
impl DrawCommand for DrawRect {
    fn render(&self, gpu: &Gpu, ctx: &mut RenderContext) {
        let mesh = Mesh::clone_quad_mesh();
        let mut material = StandardMaterial2d::clone_global();
        let window_size = gpu.get_main_window().inner_size();
        material.set_view_projection(
            ctx.camera2d.position,
            window_size.width as f32,
            window_size.height as f32,
        );
        material.set_color(self.color);
        material.set_model(
            self.position,
            self.euler_angle.to_radians(),
            self.rect.size * self.scale,
        );
        material.rebuild(gpu);

        ctx.render_pass
            .set_pipeline(material.get_pipeline().get_handle());
        ctx.render_pass
            .set_bind_group(0, Some(material.get_bind_group().get_handle()), &[]);
        ctx.render_pass
            .set_vertex_buffer(0, mesh.get_vertices().get_handle().slice(..));
        ctx.render_pass.set_index_buffer(
            mesh.get_indices().0.get_handle().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        ctx.render_pass
            .draw_indexed(0..mesh.get_indices().1, 0, 0..1);
    }
}

pub struct DrawBuilder<'a, T: DrawCommand> {
    queue: &'a mut RenderQueue,
    cmd: Option<T>,
}
impl<'a, T: DrawCommand> Deref for DrawBuilder<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.cmd.as_ref().unwrap()
    }
}
impl<'a, T: DrawCommand> DerefMut for DrawBuilder<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.cmd.as_mut().unwrap()
    }
}
impl<'a, T: DrawCommand> Drop for DrawBuilder<'a, T> {
    fn drop(&mut self) {
        self.queue.commands.push(Box::new(self.cmd.take().unwrap()));
    }
}
impl<'a, T: DrawCommand> DrawBuilder<'a, T> {
    pub fn new(queue: &'a mut RenderQueue, cmd: T) -> Self {
        Self {
            queue,
            cmd: Some(cmd),
        }
    }
}

pub struct DrawText {
    pub position: Vec2,
    pub euler_angle: f32,
    pub scale: Vec2,
    pub tint: Srgba,
    pub text: String,
    pub font_size: f32,
}
impl<'a> DrawBuilder<'a, DrawText> {
    pub fn position(&mut self, position: Vec2) -> &mut Self {
        self.position = position;
        self
    }
    pub fn angle(&mut self, euler_angle: f32) -> &mut Self {
        self.euler_angle = euler_angle;
        self
    }
    pub fn scale(&mut self, scale: Vec2) -> &mut Self {
        self.scale = scale;
        self
    }
    pub fn text(&mut self, text: impl Into<String>) -> &mut Self {
        self.text = text.into();
        self
    }
    pub fn font_size(&mut self, font_size: f32) -> &mut Self {
        self.font_size = font_size;
        self
    }
    pub fn tint(&mut self, tint: Srgba) -> &mut Self {
        self.tint = tint;
        self
    }
}
impl Default for DrawText {
    fn default() -> Self {
        Self {
            position: Default::default(),
            euler_angle: Default::default(),
            scale: Vec2::ONE,
            text: String::new(),
            font_size: 16.0,
            tint: Srgba::BLACK,
        }
    }
}
impl DrawCommand for DrawText {
    fn prepare(&self, gpu: &mut Gpu) {
        let screen_size = gpu.main_window.inner_size();
        gpu.viewport.update(
            &gpu.queue,
            Resolution {
                width: screen_size.width,
                height: screen_size.height,
            },
        );

        //TODO: This makes text invisible
        // gpu.text_buffer.set_size(
        //     &mut gpu.font_system,
        //     Some(self.font_size),
        //     Some(self.font_size * 1.5),
        // );
        gpu.text_buffer.set_text(
            &mut gpu.font_system,
            &self.text,
            &Attrs::new().family(glyphon::Family::Monospace),
            glyphon::Shaping::Advanced,
        );
        gpu.text_buffer
            .shape_until_scroll(&mut gpu.font_system, false);

        gpu.text_renderer
            .prepare(
                &gpu.device,
                &gpu.queue,
                &mut gpu.font_system,
                &mut gpu.atlas,
                &gpu.viewport,
                [TextArea {
                    buffer: &gpu.text_buffer,
                    left: self.position.x,
                    top: screen_size.height as f32 - self.position.y,
                    scale: 1.0,
                    bounds: TextBounds {
                        left: 0,
                        top: 0,
                        right: screen_size.width as i32,
                        bottom: screen_size.height as i32,
                    },
                    default_color: self.tint.as_rgba8().into(),
                    custom_glyphs: &[],
                }],
                &mut gpu.swash_cache,
            )
            .expect("issue preparing text renderer");
    }

    fn render(&self, gpu: &Gpu, ctx: &mut RenderContext) {
        gpu.text_renderer
            .render(&gpu.atlas, &gpu.viewport, &mut ctx.render_pass)
            .unwrap();
    }
}

pub struct DrawTexture {
    pub position: Vec2,
    pub euler_angle: f32,
    pub scale: Vec2,
    pub texture: Texture,
    pub tint: Srgba,
}
impl<'a> DrawBuilder<'a, DrawTexture> {
    pub fn position(&mut self, position: Vec2) -> &mut Self {
        self.position = position;
        self
    }
    pub fn angle(&mut self, euler_angle: f32) -> &mut Self {
        self.euler_angle = euler_angle;
        self
    }
    pub fn uniform_scale(&mut self, scale: f32) -> &mut Self {
        self.scale = Vec2::ONE * scale;
        self
    }
    pub fn scale(&mut self, scale: Vec2) -> &mut Self {
        self.scale = scale;
        self
    }
    pub fn texture(&mut self, texture: &Texture) -> &mut Self {
        self.texture = texture.clone();
        self
    }
    pub fn tint(&mut self, tint: Srgba) -> &mut Self {
        self.tint = tint;
        self
    }
}
impl Default for DrawTexture {
    fn default() -> Self {
        Self {
            position: Default::default(),
            euler_angle: Default::default(),
            scale: Vec2::ONE,
            texture: Texture::clone_white_texture(),
            tint: Srgba::WHITE,
        }
    }
}
impl DrawCommand for DrawTexture {
    fn render(&self, gpu: &Gpu, ctx: &mut RenderContext) {
        let mesh = Mesh::clone_quad_mesh();
        let mut material = StandardMaterial2d::clone_global();
        let window_size = gpu.get_main_window().inner_size();
        material.set_view_projection(
            ctx.camera2d.position,
            window_size.width as f32,
            window_size.height as f32,
        );
        material.set_color(self.tint);
        material.set_model(
            self.position,
            self.euler_angle.to_radians(),
            self.scale * self.texture.get_size().xy().as_vec2(),
        );
        material.set_texture(self.texture.clone());
        material.rebuild(gpu);

        ctx.render_pass
            .set_pipeline(material.get_pipeline().get_handle());
        ctx.render_pass
            .set_bind_group(0, Some(material.get_bind_group().get_handle()), &[]);
        ctx.render_pass
            .set_vertex_buffer(0, mesh.get_vertices().get_handle().slice(..));
        ctx.render_pass.set_index_buffer(
            mesh.get_indices().0.get_handle().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        ctx.render_pass
            .draw_indexed(0..mesh.get_indices().1, 0, 0..1);
    }
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

    pub fn update_camera_2d(&mut self) -> DrawBuilder<'_, Update2d> {
        DrawBuilder::new(
            self,
            Update2d {
                camera: self.camera.clone(),
            },
        )
    }

    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) -> DrawBuilder<'_, DrawRect> {
        let mut draw = DrawBuilder::new(self, DrawRect::default());
        draw.rect(Rect::from_xy(width, height));
        draw.position(Vec2::new(x, y));
        draw
    }

    pub fn texture(&mut self, texture: &Texture, x: f32, y: f32) -> DrawBuilder<'_, DrawTexture> {
        let mut dt = DrawBuilder::new(self, DrawTexture::default());
        dt.texture(texture);
        dt.position(Vec2::new(x, y));
        dt
    }

    pub fn text(&mut self, text: impl Into<String>, x: f32, y: f32) -> DrawBuilder<'_, DrawText> {
        let mut dt = DrawBuilder::new(self, DrawText::default());
        dt.text(text.into());
        dt.position(Vec2::new(x, y));
        dt
    }
}

pub struct Gpu {
    main_window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    font_system: glyphon::FontSystem,
    swash_cache: glyphon::SwashCache,
    viewport: glyphon::Viewport,
    atlas: glyphon::TextAtlas,
    text_renderer: glyphon::TextRenderer,
    text_buffer: glyphon::Buffer,
    is_surface_setup: bool,
    are_constants_setup: bool,
}
impl Gpu {
    const QUAD_VERTS: [Vertex2; 4] = [
        Vertex2 {
            position: Vec2::new(0.5, 0.5),
            uv: Vec2::new(1.0, 1.0),
            color: Srgba::WHITE,
        },
        Vertex2 {
            position: Vec2::new(-0.5, 0.5),
            uv: Vec2::new(0.0, 1.0),
            color: Srgba::WHITE,
        },
        Vertex2 {
            position: Vec2::new(-0.5, -0.5),
            uv: Vec2::new(0.0, 0.0),
            color: Srgba::WHITE,
        },
        Vertex2 {
            position: Vec2::new(0.5, -0.5),
            uv: Vec2::new(1.0, 0.0),
            color: Srgba::WHITE,
        },
    ];
    const QUAD_IDX: [u16; 6] = [0, 1, 2, 0, 2, 3];

    pub async fn new(window: Arc<Window>) -> Result<Self, LuxError> {
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

        let mut font_system = glyphon::FontSystem::new();
        let swash_cache = glyphon::SwashCache::new();
        let cache = glyphon::Cache::new(&device);
        let viewport = glyphon::Viewport::new(&device, &cache);
        let mut atlas = glyphon::TextAtlas::new(&device, &queue, &cache, surface_format);
        let text_renderer = glyphon::TextRenderer::new(
            &mut atlas,
            &device,
            wgpu::MultisampleState::default(),
            None,
        );
        let mut text_buffer =
            glyphon::Buffer::new(&mut font_system, glyphon::Metrics::new(30.0, 42.0));

        let physical_size = window.inner_size();
        let scale_factor = window.scale_factor();
        text_buffer.set_size(
            &mut font_system,
            Some((physical_size.width as f64 * scale_factor) as f32),
            Some((physical_size.height as f64 * scale_factor) as f32),
        );
        text_buffer.set_text(&mut font_system, "Hello world! 👋\nThis is rendered with 🦅 glyphon 🦁\nThe text below should be partially clipped.\na b c d e f g h i j k l m n o p q r s t u v w x y z", &Attrs::new().family(glyphon::Family::SansSerif), glyphon::Shaping::Advanced);
        text_buffer.shape_until_scroll(&mut font_system, false);

        let gpu = Self {
            config,
            surface,
            device,
            queue,
            main_window: window,
            font_system,
            swash_cache,
            viewport,
            atlas,
            text_renderer,
            text_buffer,
            is_surface_setup: false,
            are_constants_setup: false,
        };

        Ok(gpu)
    }

    pub fn setup_constants(&mut self) {
        if self.are_constants_setup {
            warn!("constants have already been setup, ignoring...");
            return;
        }
        self.are_constants_setup = true;

        WHITE_TEXTURE
            .set(TextureBuilder::build_white(self))
            .unwrap();
        QUAD_MESH
            .set(
                MeshBuilder {
                    vertices: Self::QUAD_VERTS.to_vec(),
                    indices: Self::QUAD_IDX.to_vec(),
                }
                .build(self),
            )
            .unwrap();

        MATERIAL2D.set(StandardMaterial2d::new(self)).unwrap();
    }

    pub fn get_main_window(&self) -> &Window {
        &self.main_window
    }

    pub fn get_window_size(&self) -> UVec2 {
        let size = self.get_main_window().inner_size();
        UVec2::new(size.width, size.height)
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

    pub fn render_queue(&mut self, render_queue: &RenderQueue) {
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
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
            for cmd in render_queue.get_commands().iter().rev() {
                cmd.prepare(self);
                cmd.render(self, &mut ctx);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        self.atlas.trim();
    }
}
