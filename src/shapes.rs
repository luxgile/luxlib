use wgpu::Buffer;

use crate::{color::Srgba, gpu::RenderCommand};

#[derive(Default, Clone, Debug)]
pub struct Circle {
    pub radius: f32,
}

pub struct DrawCircle {
    circle: Circle,
    color: Srgba,
}
impl RenderCommand for DrawCircle {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        todo!()
    }
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
