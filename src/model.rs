use std::fmt::Debug;

use bytemuck::Pod;
use once_cell::sync::OnceCell;

use crate::{
    buffer::{Buffer, BufferBuilder},
    gpu::{DrawCommand, Gpu, RenderContext},
    material::Material,
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
impl MeshBuilder<Vertex2> {
    pub fn build_circle(resolution: u32, gpu: &Gpu) -> Mesh {
        let mut temp_vertices: Vec<Vertex2> = Vec::new();
        let mut temp_indices: Vec<u16> = Vec::new();
        let resolution = resolution.max(3);

        // Center
        temp_vertices.push(Vertex2::from_xy(0.0, 0.0));

        // Perimeter
        for i in 0..resolution {
            let angle = i as f32 * 2.0 * std::f32::consts::PI / resolution as f32;
            let x = angle.cos();
            let y = angle.sin();
            temp_vertices.push(Vertex2::from_xy(x, y));
        }

        // Indices
        for i in 0..resolution {
            let center_idx = 0;
            let p1_idx = i + 1; // Current perimeter vertex
            let p2_idx = (i + 1) % resolution + 1; // Next perimeter vertex (wraps around)

            temp_indices.push(center_idx as u16);
            temp_indices.push(p1_idx as u16);
            temp_indices.push(p2_idx as u16);
        }

        MeshBuilder {
            vertices: temp_vertices,
            indices: temp_indices,
        }
        .build(gpu)
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

pub(crate) static CIRCLE_MESH_32: OnceCell<Mesh> = OnceCell::new();
pub(crate) static QUAD_MESH: OnceCell<Mesh> = OnceCell::new();
pub(crate) static QUAD_LINE_MESH: OnceCell<Mesh> = OnceCell::new();

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

    pub fn clone_quad_mesh() -> Mesh {
        QUAD_MESH
            .get()
            .expect("quad mesh has not been set yet")
            .clone()
    }

    pub fn clone_quad_line_mesh() -> Mesh {
        QUAD_LINE_MESH
            .get()
            .expect("quad line mesh has not been set yet")
            .clone()
    }

    pub fn clone_circle_mesh() -> Mesh {
        CIRCLE_MESH_32
            .get()
            .expect("circle mesh has not been set yet")
            .clone()
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
    fn render(&self, _gpu: &mut Gpu, ctx: &mut RenderContext) {
        ctx.render_pass
            .set_pipeline(self.material.get_pipeline().get_handle());
        ctx.render_pass
            .set_vertex_buffer(0, self.mesh.get_vertices().get_handle().slice(..));
        ctx.render_pass
            .set_bind_group(0, Some(self.material.get_bind_group().get_handle()), &[]);
        ctx.render_pass.set_index_buffer(
            self.mesh.get_indices().0.get_handle().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        ctx.render_pass
            .draw_indexed(0..self.mesh.get_indices().1, 0, 0..1);
    }
}
