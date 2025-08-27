use bytemuck::NoUninit;
use wgpu::{BufferUsages, util::DeviceExt};

use crate::gpu::Gpu;

pub struct BufferBuilder {
    label: Option<String>,
    contents: Vec<u8>,
    usage: BufferUsages,
}
impl Default for BufferBuilder {
    fn default() -> Self {
        Self {
            label: Default::default(),
            contents: Default::default(),
            usage: BufferUsages::empty(),
        }
    }
}
impl BufferBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(&mut self, label: impl Into<String>) -> &mut Self {
        self.label = Some(label.into());
        self
    }

    pub fn contents(&mut self, contents: &[impl NoUninit]) -> &mut Self {
        self.contents = bytemuck::cast_slice(contents).to_vec();
        self
    }

    pub fn usage(&mut self, usage: BufferUsages) -> &mut Self {
        self.usage = usage;
        self
    }

    pub fn build(&self, gpu: &Gpu) -> Buffer {
        let buffer = gpu.get_device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: self.label.as_deref(),
            contents: &self.contents,
            usage: self.usage,
        });
        Buffer { handle: buffer }
    }
}

#[derive(Clone, Debug)]
pub struct Buffer {
    handle: wgpu::Buffer,
}
impl Buffer {
    pub fn get_handle(&self) -> &wgpu::Buffer {
        &self.handle
    }
}
