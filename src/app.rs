use std::{
    sync::Arc,
    time::{self, Duration, Instant},
};

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
    color::Srgba,
    frame::Frame,
    gpu::{Gpu, RenderQueue},
    input::Input,
};

// pub trait FrameLoop {}
pub type InitFn<T> = fn(&mut Frame) -> T;
pub type FrameLoop<T> = fn(&mut Frame, &mut T) -> Option<RenderQueue>;

#[derive(Clone)]
pub struct AppDesc {
    title: String,
    size: UVec2,
    frame_time: Duration,
}

pub struct App<T> {
    state: Option<T>,
    input: Input,
    gpu: Option<Gpu>,
    init_loop: InitFn<T>,
    frame_loop: FrameLoop<T>,
    desc: Option<AppDesc>,

    frames: u64,
    last_frame_time: Instant,
}
impl<T> App<T> {
    pub fn new(init_loop: InitFn<T>, frame_loop: FrameLoop<T>) -> Self {
        Self {
            last_frame_time: Instant::now(),
            frames: 0,
            input: Input::default(),
            state: None,
            init_loop,
            gpu: None,
            frame_loop,
            desc: None,
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

        if let Some(config) = &self.desc {
            window.set_title(&config.title);
            let _ = window.request_inner_size(LogicalSize::new(config.size.x, config.size.y));
        }

        let mut gpu = pollster::block_on(Gpu::new(window)).unwrap();
        gpu.setup_constants();

        self.state = Some((self.init_loop)(&mut Frame::new(0.0, 0, &self.input, &gpu)));

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

        let desc = self.desc.as_ref().unwrap();

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => gpu.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                gpu.redraw();

                if self.last_frame_time.elapsed() < desc.frame_time {
                    return;
                }

                let dt = self.last_frame_time.elapsed().as_secs_f64();
                self.last_frame_time = Instant::now();

                let render_queue = {
                    let state = self.state.as_mut().unwrap();
                    let mut frame = Frame::new(dt, self.frames, &self.input, gpu);
                    (self.frame_loop)(&mut frame, state).unwrap_or_default()
                };

                self.frames += 1;
                self.input.advance();

                gpu.render_queue(&render_queue);
            }
            WindowEvent::KeyboardInput {
                event,
                device_id,
                is_synthetic,
            } => {
                self.input
                    .handle_keyboard_input(&event, &device_id, is_synthetic);

                if let PhysicalKey::Code(code) = event.physical_key
                    && let (KeyCode::Escape, true) = (code, event.state.is_pressed())
                {
                    event_loop.exit()
                }
            }
            _ => {}
        }
    }
}

#[derive(Clone)]
pub struct AppBuilderStage1 {
    desc: AppDesc,
}
impl AppBuilderStage1 {
    fn get_desc(&self) -> &AppDesc {
        &self.desc
    }

    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.desc.title = title.into();
        self
    }

    pub fn target_fps(&mut self, fps: u64) -> &mut Self {
        self.desc.frame_time = Duration::from_micros(1_000_000 / fps);
        self
    }

    pub fn window_size(&mut self, size: UVec2) -> &mut Self {
        self.desc.size = size;
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
            desc: AppDesc {
                title: "Luxlib Window".to_string(),
                size: [800, 600].into(),
                frame_time: Duration::from_micros(0),
            },
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
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        let mut app = App::new(self.init_fn, self.frame_loop);
        app.desc = Some(self.stage1.get_desc().clone());
        event_loop.run_app(&mut app).expect("error on event loop");
    }
}
