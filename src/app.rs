use std::sync::Arc;

use bytemuck::Pod;
use glam::UVec2;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::{
    buffer::{Buffer, BufferBuilder},
    color::Srgba,
    gpu::Gpu,
    pipeline::{Pipeline, PipelineBuilder},
    vertex::{Vertex, Vertex2},
};

pub trait RenderCommand {
    fn render(&self, render_pass: &mut wgpu::RenderPass);
}

#[derive(Default)]
pub struct RenderQueue {
    clear_color: Srgba,
    commands: Vec<Box<dyn RenderCommand>>,
}
impl RenderQueue {
    pub fn get_clear_color(&self) -> Srgba {
        self.clear_color
    }

    pub fn get_commands(&self) -> &Vec<Box<dyn RenderCommand>> {
        &self.commands
    }

    pub fn draw(&mut self, command: impl RenderCommand + 'static) {
        self.commands.push(Box::new(command));
    }
}

pub struct ModelBuilder<V: Vertex> {
    pipeline: Pipeline,
    vertices: Vec<V>,
    indices: Vec<u16>,
}
impl ModelBuilder<Vertex2> {
    const SPRITE_VERT_BUFFER: [Vertex2; 4] = [
        Vertex2::from_xy(-0.5, -0.5),
        Vertex2::from_xy(0.5, -0.5),
        Vertex2::from_xy(-0.5, -0.5),
        Vertex2::from_xy(-0.5, 0.5),
    ];
    const SPRITE_IDX_BUFFER: [u16; 6] = [0, 1, 2, 0, 2, 3];

    pub fn new_sprite(gpu: &Gpu) -> Self {
        Self {
            pipeline: PipelineBuilder::build_2d_default(gpu),
            vertices: Self::SPRITE_VERT_BUFFER.into(),
            indices: Self::SPRITE_IDX_BUFFER.into(),
        }
    }
}
impl<V: Vertex + Pod> ModelBuilder<V> {
    pub fn vertices(&mut self, vertices: Vec<V>) -> &mut Self {
        self.vertices = vertices;
        self
    }

    pub fn indices(&mut self, indices: Vec<u16>) -> &mut Self {
        self.indices = indices;
        self
    }

    pub fn build(&self, gpu: &Gpu) -> Model {
        Model {
            pipeline: self.pipeline.clone(),
            vertices: BufferBuilder::new()
                .usage(wgpu::BufferUsages::VERTEX)
                .contents(&self.vertices)
                .build(gpu),
            indices: BufferBuilder::new()
                .usage(wgpu::BufferUsages::INDEX)
                .contents(&self.indices)
                .build(gpu),
            n_indices: self.indices.len() as u32,
        }
    }
}

pub struct Model {
    pipeline: Pipeline,
    vertices: Buffer,
    indices: Buffer,
    n_indices: u32,
}
impl Model {
    pub fn new(pipeline: Pipeline, vertices: Buffer, indices: Buffer, n_indices: u32) -> Self {
        Self {
            pipeline,
            vertices,
            indices,
            n_indices,
        }
    }
}
impl RenderCommand for Model {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(self.pipeline.get_handle());
        render_pass.set_vertex_buffer(0, self.vertices.get_handle().slice(..));
        render_pass.set_index_buffer(
            self.indices.get_handle().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..self.n_indices, 0, 0..1);
    }
}

pub struct Frame<'a> {
    gpu: &'a Gpu,
}
impl<'a> Frame<'a> {
    pub fn gpu(&self) -> &Gpu {
        self.gpu
    }

    pub fn render(&mut self, color: Srgba) -> RenderQueue {
        let mut render_queue = RenderQueue::default();
        render_queue.clear_color = color;
        render_queue
    }
}

// pub trait FrameLoop {}
type FrameLoop = fn(&mut Frame) -> Option<RenderQueue>;

#[derive(Default)]
pub struct App {
    gpu: Option<Gpu>,
    frame_loop: Option<FrameLoop>,

    // The builder cannot config the app directly, as the app needs `resume` to be called first
    // after starting the app.
    config: Option<AppBuilder>,
}

impl App {
    pub fn new() -> Self {
        Self {
            gpu: None,
            frame_loop: None,
            config: None,
        }
    }
}

impl ApplicationHandler<()> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes();
        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("error creating new window"),
        );

        if let Some(config) = &self.config {
            window.set_title(&config.title);
            let _ = window.request_inner_size(LogicalSize::new(config.size.x, config.size.y));
        }

        let gpu = pollster::block_on(Gpu::new(window)).unwrap();
        self.gpu = Some(gpu);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let gpu = match &mut self.gpu {
            Some(gpu) => gpu,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => gpu.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                let gpu = self.gpu.as_ref().unwrap();
                let render_queue = if let Some(frame_loop) = &self.frame_loop {
                    let mut frame = Frame { gpu };
                    frame_loop(&mut frame).unwrap_or(RenderQueue::default())
                } else {
                    RenderQueue::default()
                };

                gpu.render_queue(&render_queue);
                // gpu.render(
                //     self.default_2d_pipeline.as_ref().unwrap(),
                //     self.vertex_buffer.as_ref().unwrap(),
                //     self.index_buffer.as_ref().unwrap(),
                //     9,
                // )
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    if let (KeyCode::Escape, true) = (code, event.state.is_pressed()) { event_loop.exit() }
                }
            }
            _ => {}
        }
    }
}

#[derive(Clone)]
pub struct AppBuilder {
    title: String,
    frame_loop: FrameLoop,
    size: UVec2,
}
impl AppBuilder {
    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.title = title.into();
        self
    }

    pub fn window_size(&mut self, size: UVec2) -> &mut Self {
        self.size = size;
        self
    }

    pub fn frame_loop(&mut self, frame_loop: fn(&mut Frame) -> Option<RenderQueue>) -> &mut Self {
        self.frame_loop = frame_loop;
        self
    }

    pub fn start_app(&mut self) {
        let event_loop = EventLoop::with_user_event()
            .build()
            .expect("cannot create event loop");
        let mut app = App::new();
        app.frame_loop = Some(self.frame_loop);
        app.config = Some(self.clone());
        event_loop.run_app(&mut app).expect("error on event loop");
    }
}
impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            title: "Luxlib Window".to_string(),
            size: UVec2::new(800, 600),
            frame_loop: empty_frame_loop,
        }
    }
}
fn empty_frame_loop(_: &mut Frame) -> Option<RenderQueue> {
    Some(RenderQueue::default())
}
