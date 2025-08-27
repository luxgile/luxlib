use bytemuck::Pod;

use crate::{
    bind::{BindGroup, BindGroupBuilder},
    buffer::{Buffer, BufferBuilder},
    gpu::{Gpu, RenderCommand},
    pipeline::{Pipeline, PipelineBuilder},
    vertex::{Vertex, Vertex2},
};

pub struct ModelBuilder<V: Vertex> {
    pipeline: Pipeline,
    vertices: Vec<V>,
    indices: Vec<u16>,
}
impl ModelBuilder<Vertex2> {
    const SPRITE_VERT_BUFFER: [Vertex2; 4] = [
        Vertex2::from_xy(-0.5, -0.5),
        Vertex2::from_xy(0.5, -0.5),
        Vertex2::from_xy(-0.5, -0.5),
        Vertex2::from_xy(-0.5, 0.5),
    ];
    const SPRITE_IDX_BUFFER: [u16; 6] = [0, 1, 2, 0, 2, 3];

    pub fn new_sprite(gpu: &Gpu) -> Self {
        Self {
            pipeline: PipelineBuilder::build_2d_default(gpu),
            vertices: Self::SPRITE_VERT_BUFFER.into(),
            indices: Self::SPRITE_IDX_BUFFER.into(),
        }
    }
}
impl<V: Vertex + Pod> ModelBuilder<V> {
    pub fn vertices(&mut self, vertices: Vec<V>) -> &mut Self {
        self.vertices = vertices;
        self
    }

    pub fn indices(&mut self, indices: Vec<u16>) -> &mut Self {
        self.indices = indices;
        self
    }

    pub fn build(&self, gpu: &Gpu) -> Model {
        Model {
            pipeline: self.pipeline.clone(),
            bind_group: BindGroupBuilder::default().build(gpu),
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

#[derive(Clone, Debug)]
pub struct Model {
    pipeline: Pipeline,
    bind_group: BindGroup,
    vertices: Buffer,
    indices: Buffer,
    n_indices: u32,
}
impl Model {
    pub fn new(
        pipeline: Pipeline,
        bind_group: BindGroup,
        vertices: Buffer,
        indices: Buffer,
        n_indices: u32,
    ) -> Self {
        Self {
            pipeline,
            bind_group,
            vertices,
            indices,
            n_indices,
        }
    }
}
impl RenderCommand for Model {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(self.pipeline.get_handle());
        render_pass.set_vertex_buffer(0, self.vertices.get_handle().slice(..));
        render_pass.set_bind_group(0, Some(self.bind_group.get_handle()), &[]);
        render_pass.set_index_buffer(
            self.indices.get_handle().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..self.n_indices, 0, 0..1);
    }
}
