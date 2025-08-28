use bytemuck::{Pod, Zeroable};
use glam::Vec2;

use crate::color::Srgba;

pub trait Vertex {
    fn get_layout() -> wgpu::VertexBufferLayout<'static>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex2 {
    pub color: Srgba,
    pub position: Vec2,
    pub uv: Vec2,
}
impl Vertex2 {
    const ATTRIBUTES: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x2, 2 => Float32x2];

    pub const fn new() -> Self {
        Self {
            position: Vec2 { x: 0.0, y: 0.0 },
            color: Srgba::WHITE,
            uv: Vec2::new(0.0, 0.0),
        }
    }
    pub const fn from_xy(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            color: Srgba::WHITE,
            uv: Vec2::new(0.0, 0.0),
        }
    }
    pub const fn from_position(pos: Vec2) -> Self {
        Self {
            position: pos,
            color: Srgba::WHITE,
            uv: Vec2::new(0.0, 0.0),
        }
    }
}
unsafe impl Pod for Vertex2 {}
unsafe impl Zeroable for Vertex2 {}
impl Vertex for Vertex2 {
    fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex2>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
