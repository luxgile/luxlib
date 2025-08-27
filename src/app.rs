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
    pub fn new(gpu: &'a Gpu) -> Self {
        Self { gpu }
    }

    pub fn gpu(&self) -> &Gpu {
        self.gpu
    }

    pub fn render(&mut self, color: Srgba) -> RenderQueue {
        let mut render_queue = RenderQueue::new(color);
        render_queue
    }
}

// pub trait FrameLoop {}
pub type InitFn<T> = fn(&mut Frame) -> T;
pub type FrameLoop<T> = fn(&mut Frame, &mut T) -> Option<RenderQueue>;

pub struct AppDesc {
    title: String,
    size: UVec2,
}

pub struct App<T> {
    state: Option<T>,
    gpu: Option<Gpu>,
    init_loop: InitFn<T>,
    frame_loop: FrameLoop<T>,
    config: Option<AppDesc>,
}
impl<T> App<T> {
    pub fn new(init_loop: InitFn<T>, frame_loop: FrameLoop<T>) -> Self {
        Self {
            state: None,
            init_loop,
            gpu: None,
            frame_loop,
            config: None,
        }
    }
}

impl<T> ApplicationHandler<()> for App<T> {
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

        self.state = Some((self.init_loop)(&mut Frame::new(&gpu)));

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
                let render_queue = {
                    let mut state = self.state.as_mut().unwrap();
                    let mut frame = Frame::new(self.gpu.as_ref().unwrap());
                    (self.frame_loop)(&mut frame, &mut state).unwrap_or(RenderQueue::default())
                };

                gpu.render_queue(&render_queue);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    if let (KeyCode::Escape, true) = (code, event.state.is_pressed()) {
                        event_loop.exit()
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Clone)]
pub struct AppBuilderStage1 {
    title: String,
    size: UVec2,
}
impl AppBuilderStage1 {
    fn get_desc(&self) -> AppDesc {
        AppDesc {
            title: self.title.clone(),
            size: self.size,
        }
    }

    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.title = title.into();
        self
    }

    pub fn window_size(&mut self, size: UVec2) -> &mut Self {
        self.size = size;
        self
    }

    pub fn no_init(&mut self) -> AppBuilderStage2<()> {
        AppBuilderStage2::new(self.clone(), |_| ())
    }

    pub fn init<T>(&mut self, init_fn: InitFn<T>) -> AppBuilderStage2<T> {
        AppBuilderStage2::new(self.clone(), init_fn)
    }
}
impl Default for AppBuilderStage1 {
    fn default() -> Self {
        Self {
            title: "Luxlib Window".to_string(),
            size: [800, 600].into(),
        }
    }
}

pub struct AppBuilderStage2<T> {
    stage1: AppBuilderStage1,
    frame_loop: FrameLoop<T>,
    init_fn: InitFn<T>,
}
impl<T> AppBuilderStage2<T> {
    fn new(stage1: AppBuilderStage1, init_fn: InitFn<T>) -> Self {
        Self {
            stage1,
            frame_loop: |_, _| None,
            init_fn,
        }
    }

    pub fn frame_loop(
        &mut self,
        frame_loop: fn(&mut Frame, &mut T) -> Option<RenderQueue>,
    ) -> &mut Self {
        self.frame_loop = frame_loop;
        self
    }

    pub fn start(&mut self) {
        let event_loop = EventLoop::with_user_event()
            .build()
            .expect("cannot create event loop");

        let mut app = App::new(self.init_fn, self.frame_loop);
        app.config = Some(self.stage1.get_desc());
        event_loop.run_app(&mut app).expect("error on event loop");
    }
}
