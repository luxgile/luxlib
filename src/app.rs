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
    gpu::{Gpu, RenderQueue},
    pipeline::{Pipeline, PipelineBuilder},
    vertex::{Vertex, Vertex2},
};


pub struct Frame<'a> {
    gpu: &'a Gpu,
}
impl<'a> Frame<'a> {
    pub fn gpu(&self) -> &Gpu {
        self.gpu
    }

    pub fn render(&mut self, color: Srgba) -> RenderQueue {
        let mut render_queue = RenderQueue::new(color);
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
