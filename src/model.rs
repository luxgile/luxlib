use std::fmt::Debug;

use bytemuck::Pod;
use glam::UVec3;

use crate::{
    buffer::{Buffer, BufferBuilder},
    color::Srgba,
    gpu::{Gpu, DrawCommand},
    material::{Material, StandardMaterial2d},
    texture::{SamplerBuilder, TextureBuilder},
    vertex::{Vertex, Vertex2},
};

pub struct MeshBuilder<V: Vertex> {
    pub vertices: Vec<V>,
    pub indices: Vec<u16>,
}
impl<V: Vertex> Default for MeshBuilder<V> {
    fn default() -> Self {
        Self {
            vertices: Default::default(),
            indices: Default::default(),
        }
    }
}
impl<V: Vertex + Pod> MeshBuilder<V> {
    pub fn build(&self, gpu: &Gpu) -> Mesh {
        Mesh {
            vertices: BufferBuilder::new()
                .usage(wgpu::BufferUsages::VERTEX)
                .contents(&self.vertices)
                .build(gpu),
            indices: BufferBuilder::new()
                .usage(wgpu::BufferUsages::INDEX)
                .contents(&self.indices)
                .build(gpu),
            n_indices: self.indices.len() as u32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    vertices: Buffer,
    indices: Buffer,
    n_indices: u32,
}
impl Mesh {
    pub fn get_vertices(&self) -> &Buffer {
        &self.vertices
    }

    pub fn get_indices(&self) -> (&Buffer, u32) {
        (&self.indices, self.n_indices)
    }
}

pub struct ModelBuilder<V: Vertex> {
    pub material: Box<dyn Material>,
    pub mesh: MeshBuilder<V>,
}
impl<V: Vertex + Pod> ModelBuilder<V> {
    pub fn build(&self, gpu: &Gpu) -> Model {
        Model {
            material: self.material.box_clone(),
            mesh: self.mesh.build(gpu),
        }
    }
}

pub struct Model {
    material: Box<dyn Material>,
    mesh: Mesh,
}
impl Model {}
impl Clone for Model {
    fn clone(&self) -> Self {
        Self {
            material: self.material.box_clone(),
            mesh: self.mesh.clone(),
        }
    }
}
impl Debug for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Model")
            .field("material", &self.material.as_debug())
            .field("mesh", &self.mesh)
            .finish()
    }
}
impl DrawCommand for Model {
    fn render(&self, gpu: &Gpu, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(self.material.get_pipeline().get_handle());
        render_pass.set_vertex_buffer(0, self.mesh.get_vertices().get_handle().slice(..));
        render_pass.set_bind_group(0, Some(self.material.get_bind_group().get_handle()), &[]);
        render_pass.set_index_buffer(
            self.mesh.get_indices().0.get_handle().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..self.mesh.get_indices().1, 0, 0..1);
    }
}
