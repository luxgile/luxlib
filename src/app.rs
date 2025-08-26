use std::sync::Arc;

use glam::UVec2;
use winit::{application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, event_loop::EventLoop, keyboard::{self, KeyCode, PhysicalKey}, window::Window};

use crate::{gpu::Gpu, pipeline::{LayoutBuilder, Pipeline, PipelineBuilder, ShaderBuilder}};


#[derive(Default)]
pub struct App {
    gpu: Option<Gpu>,
    default_2d_pipeline: Option<Pipeline>,

    // The builder cannot config the app directly, as the app needs `resume` to be called first
    // after starting the app.
    config: Option<AppBuilder>,
}

impl App {
    pub fn new() -> Self {
        Self {
            gpu: None,
            config: None,
            default_2d_pipeline: None,
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
        self.default_2d_pipeline = Some(
            PipelineBuilder::new()
                .with_label("2d pipeline")
                .with_shader(
                    ShaderBuilder::new()
                        .with_label("2d shader")
                        .with_source(wgpu::ShaderSource::Wgsl(include_str!("2d.wgsl").into()))
                        .build(gpu.get_device()),
                )
                .with_layout(
                    LayoutBuilder::new()
                        .with_label("2d layout")
                        .build(gpu.get_device()),
                )
                .build(gpu.get_device(), gpu.get_config()),
        );
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
            WindowEvent::RedrawRequested => gpu.render(self.default_2d_pipeline.as_ref().unwrap()),
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    match (code, event.state.is_pressed()) {
                        (KeyCode::Escape, true) => event_loop.exit(),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Clone)]
pub struct AppBuilder {
    title: String,
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

    pub fn start_app(&mut self) {
        let event_loop = EventLoop::with_user_event()
            .build()
            .expect("cannot create event loop");
        let mut app = App::new();
        app.config = Some(self.clone());
        event_loop.run_app(&mut app).expect("error on event loop");
    }
}
impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            title: "Luxlib Window".to_string(),
            size: UVec2::new(800, 600),
        }
    }
}
