use glam::{Mat3, Vec2};
use wgpu::Buffer;

use crate::{
    color::Srgba,
    gpu::{DrawCommand, Gpu, RenderContext},
    material::Material,
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

#[derive(Debug, Clone)]
pub struct DrawRect {
    pub position: Vec2,
    pub euler_angle: f32,
    pub rect: Rect,
    pub color: Srgba,
}
impl DrawRect {
    pub fn position(&mut self, position: Vec2) -> &mut Self {
        self.position = position;
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
            rect: Rect::from_xy(25.0, 25.0),
            color: Srgba::WHITE,
        }
    }
}
impl DrawCommand for DrawRect {
    fn render(&self, gpu: &Gpu, ctx: &mut RenderContext) {
        let mesh = gpu.get_constants().get_quad_mesh();
        let mut material = gpu.get_constants().get_default_2d_material().clone();
        let window_size = gpu.get_main_window().inner_size();
        material.set_view_projection(
            ctx.camera2d.position,
            window_size.width as f32,
            window_size.height as f32,
        );
        material.set_color(self.color);
        material.set_model(self.position, self.euler_angle.to_radians(), self.rect.size);
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

#[derive(Default, Clone, Debug)]
pub struct Circle {
    pub radius: f32,
}

pub struct DrawCircle {
    circle: Circle,
    color: Srgba,
}
impl DrawCommand for DrawCircle {
    fn render(&self, gpu: &Gpu, ctx: &mut RenderContext) {}
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
