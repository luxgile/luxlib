use bytemuck::Pod;

use crate::{
    bind::BindEntry,
    buffer::{Buffer, BufferBuilder},
    gpu::Gpu,
};

#[derive(Debug, Clone)]
pub struct Uniform<T> {
    value: T,
    buffer: Option<Buffer>,
}
impl<T: Pod> Uniform<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            buffer: None,
        }
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    pub fn rebuild(&mut self, gpu: &Gpu) {
        self.buffer = Some(
            BufferBuilder {
                contents: bytemuck::cast_slice(&[self.value]).to_vec(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                ..Default::default()
            }
            .build(gpu),
        )
    }
}
impl<T> BindEntry for Uniform<T> {
    fn resource(&self) -> wgpu::BindingResource<'_> {
        self.buffer
            .as_ref()
            .unwrap()
            .get_handle()
            .as_entire_binding()
    }

    fn ty(&self) -> wgpu::BindingType {
        wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        }
    }
}
