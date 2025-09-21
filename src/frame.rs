use crate::{
    color::Srgba,
    gpu::{Gpu, RenderQueue},
    input::Input,
    io::Io,
};

pub struct Frame<'a> {
    dt: f64,
    frame: u64,
    io: Io,
    input: &'a Input,
    gpu: &'a Gpu,
    should_close: bool,
}
impl<'a> Frame<'a> {
    pub fn new(dt: f64, frame: u64, input: &'a Input, gpu: &'a Gpu) -> Self {
        Self {
            dt,
            frame,
            io: Io,
            input,
            gpu,
            should_close: false,
        }
    }

    pub fn dt(&self) -> f32 {
        self.dt as f32
    }

    pub fn frame_number(&self) -> u64 {
        self.frame
    }

    pub fn io(&self) -> &Io {
        &self.io
    }

    pub fn gpu(&self) -> &Gpu {
        self.gpu
    }

    pub fn input(&self) -> &Input {
        self.input
    }

    pub fn render(&mut self, color: Srgba) -> RenderQueue {
        RenderQueue::new(color)
    }

    pub fn close_app(&mut self) {
        self.should_close = true;
    }
    pub fn should_close(&mut self) -> bool {
        self.should_close
    }
}
