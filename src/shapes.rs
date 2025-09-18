use glam::Vec2;

use crate::{
    color::Srgba,
    gpu::{DrawCommand, Gpu, RenderContext},
};

#[derive(Default, Debug, Clone)]
pub struct Rect {
    pub size: Vec2,
}
impl Rect {
    pub fn new(size: Vec2) -> Self {
        Self { size }
    }

    pub fn from_xy(width: f32, height: f32) -> Self {
        Self::new(Vec2::new(width, height))
    }
}


#[derive(Default, Clone, Debug)]
pub struct Circle {
    pub radius: f32,
}

pub struct DrawCircle {
    circle: Circle,
    color: Srgba,
}
impl DrawCommand for DrawCircle {
    fn render(&self, _gpu: &Gpu, _ctx: &mut RenderContext) {}
}

#[derive(Default, Clone, Debug)]
pub struct CircleSegment {
    pub radius: f32,
    pub start_angle: f32,
    pub end_angle: f32,
}

// pub struct RenderCircleSegment {
//     vertices: Buffer,
//     indices: Buffer,
// }
// impl RenderCircleSegment {
//     pub fn new(circle: CircleSegment, segments: u32) -> RenderCircleSegment {
//        let vertices: Vec::new();
//
//     }
// }
// impl RenderCommand for RenderCircleSegment {
//     fn render(&self, render_pass: &mut wgpu::RenderPass) {}
// }
