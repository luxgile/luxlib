use crate::{
    color::Srgba,
    gpu::{Gpu, RenderQueue},
    input::Input,
};

pub struct Frame<'a> {
    dt: f64,
    input: &'a Input,
    gpu: &'a Gpu,
}
impl<'a> Frame<'a> {
    pub fn new(dt: f64, input: &'a Input, gpu: &'a Gpu) -> Self {
        Self { dt, input, gpu }
    }

    pub fn dt(&self) -> f32 {
        self.dt as f32
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
}
