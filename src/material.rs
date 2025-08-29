use std::fmt::Debug;

use bytemuck::{Pod, Zeroable};
use glam::{Mat3, Mat4, Vec2};

use crate::{
    bind::{BindGroup, BindGroupBuilder, ShaderStage},
    color::Srgba,
    gpu::Gpu,
    pipeline::{Pipeline, PipelineBuilder},
    texture::{Sampler, SamplerBuilder, Texture, TextureBuilder},
    uniform::Uniform,
};

pub trait Material {
    fn get_pipeline(&self) -> &Pipeline;
    fn get_bind_group(&self) -> &BindGroup;
    fn box_clone(&self) -> Box<dyn Material>;
    fn as_debug(&self) -> &dyn Debug;
}

#[derive(Debug, Clone)]
pub struct StandardMaterial2d {
    main_texture: Texture,
    main_sampler: Sampler,
    color: Uniform<Srgba>,
    model: Uniform<glam::Mat4>,
    view_proj: Uniform<glam::Mat4>,

    dirty: bool,
    cached_pipeline: Pipeline,
    cached_bind_group: BindGroup,
}

impl StandardMaterial2d {
    pub fn new(gpu: &Gpu) -> Self {
        Self {
            main_texture: TextureBuilder::build_white(gpu),
            main_sampler: SamplerBuilder::default().build(gpu),
            color: Uniform::new(Srgba::WHITE),
            model: Uniform::new(glam::Mat4::IDENTITY),
            view_proj: Uniform::new(glam::Mat4::IDENTITY),
            dirty: true,
            cached_pipeline: PipelineBuilder::default().build(gpu, None),
            cached_bind_group: BindGroupBuilder::default().build(gpu),
        }
    }

    pub fn set_texture(&mut self, texture: Texture) {
        self.main_texture = texture;
        self.dirty = true;
    }

    pub fn set_sampler(&mut self, sampler: Sampler) {
        self.main_sampler = sampler;
        self.dirty = true;
    }

    pub fn set_color(&mut self, color: Srgba) {
        self.color.set(color);
        self.dirty = true;
    }

    pub fn set_model(&mut self, position: Vec2, angle_radians: f32, scale: Vec2) {
        self.model.set(
            Mat4::from_translation(position.extend(0.0))
                * Mat4::from_rotation_z(angle_radians)
                * Mat4::from_scale(scale.extend(0.0)),
        );
        self.dirty = true;
    }

    pub fn set_view_projection(&mut self, position: Vec2, width: f32, height: f32) {
        let view = Mat4::from_translation(-position.extend(0.0));
        let projection = Mat4::orthographic_rh(0.0, width, 0.0, height, 0.0, 1.0);
        self.view_proj.set(projection * view);
        self.dirty = true;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn rebuild(&mut self, gpu: &Gpu) {
        self.dirty = false;
        self.color.rebuild(gpu);
        self.model.rebuild(gpu);
        self.view_proj.rebuild(gpu);
        self.cached_bind_group = self.build_bind_group(gpu);
        self.cached_pipeline = self.build_pipeline(gpu);
    }

    fn build_pipeline(&self, gpu: &Gpu) -> Pipeline {
        PipelineBuilder::build_2d_default(gpu, &self.cached_bind_group)
    }

    fn build_bind_group(&self, gpu: &Gpu) -> BindGroup {
        BindGroupBuilder::default()
            .entry(ShaderStage::VERTEX, &self.model)
            .entry(ShaderStage::VERTEX, &self.view_proj)
            .entry(ShaderStage::FRAGMENT, &self.main_texture)
            .entry(ShaderStage::FRAGMENT, &self.main_sampler)
            .entry(ShaderStage::FRAGMENT, &self.color)
            .build(gpu)
    }
}

impl Material for StandardMaterial2d {
    fn get_pipeline(&self) -> &Pipeline {
        &self.cached_pipeline
    }

    fn get_bind_group(&self) -> &BindGroup {
        &self.cached_bind_group
    }

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }

    fn as_debug(&self) -> &dyn Debug {
        self
    }
}
