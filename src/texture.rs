use std::sync::Mutex;

use glam::UVec3;
use once_cell::sync::OnceCell;

use crate::{bind::BindEntry, color::Srgba, gpu::Gpu};

#[derive(Clone)]
pub enum TextureDimension {
    D1,
    D2,
    D3,
}
impl From<TextureDimension> for wgpu::TextureDimension {
    fn from(val: TextureDimension) -> Self {
        match val {
            TextureDimension::D1 => wgpu::TextureDimension::D1,
            TextureDimension::D2 => wgpu::TextureDimension::D2,
            TextureDimension::D3 => wgpu::TextureDimension::D3,
        }
    }
}

pub struct TextureBuilder {
    pub label: Option<String>,
    pub size: UVec3,
    pub mipmap: u32,
    pub dimensions: TextureDimension,
    pub samples: u32,
    pub content: Option<Vec<u8>>,
}
impl Default for TextureBuilder {
    fn default() -> Self {
        Self {
            label: None,
            size: UVec3::new(1, 1, 1),
            mipmap: 1,
            samples: 1,
            dimensions: TextureDimension::D2,
            content: None,
        }
    }
}
impl TextureBuilder {
    pub fn build_white(gpu: &Gpu) -> Texture {
        Self {
            label: Some("white 1x1".into()),
            size: UVec3::new(1, 1, 1),
            mipmap: 1,
            samples: 1,
            dimensions: TextureDimension::D2,
            content: Some(bytemuck::cast_slice(&[Srgba::WHITE]).to_vec()),
        }
        .build(gpu)
    }

    pub fn build(&self, gpu: &Gpu) -> Texture {
        let texture_size = wgpu::Extent3d {
            width: self.size.x,
            height: self.size.y,
            depth_or_array_layers: self.size.z,
        };
        let texture = gpu.get_device().create_texture(&wgpu::TextureDescriptor {
            label: self.label.as_deref(),
            size: texture_size,
            mip_level_count: self.mipmap,
            sample_count: self.samples,
            dimension: self.dimensions.clone().into(),
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let texture = Texture {
            handle: texture,
            view,
        };

        if let Some(content) = &self.content {
            texture.queue_write(gpu, content);
        }

        texture
    }
}

pub(crate) static WHITE_TEXTURE: OnceCell<Texture> = OnceCell::new();

#[derive(Clone, Debug)]
pub struct Texture {
    handle: wgpu::Texture,
    view: wgpu::TextureView,
}
impl Texture {
    pub fn clone_white_texture() -> Texture {
        WHITE_TEXTURE
            .get()
            .expect("no white texture has been set yet")
            .clone()
    }

    pub fn get_handle(&self) -> &wgpu::Texture {
        &self.handle
    }

    pub fn queue_write(&self, gpu: &Gpu, pixels: &[u8]) {
        gpu.get_queue().write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.handle,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(self.handle.size().width * 4),
                rows_per_image: Some(self.handle.size().height),
            },
            self.handle.size(),
        )
    }
}
impl BindEntry for Texture {
    fn resource(&self) -> wgpu::BindingResource<'_> {
        wgpu::BindingResource::TextureView(&self.view)
    }

    fn ty(&self) -> wgpu::BindingType {
        wgpu::BindingType::Texture {
            multisampled: false,
            view_dimension: match self.handle.dimension() {
                wgpu::TextureDimension::D1 => wgpu::TextureViewDimension::D1,
                wgpu::TextureDimension::D2 => wgpu::TextureViewDimension::D2,
                wgpu::TextureDimension::D3 => wgpu::TextureViewDimension::D3,
            },
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
        }
    }
}

#[derive(Clone, Copy)]
pub enum TextureAddressMode {
    Clamp,
    Repeat,
    Mirror,
}
impl From<TextureAddressMode> for wgpu::AddressMode {
    fn from(val: TextureAddressMode) -> Self {
        match val {
            TextureAddressMode::Clamp => wgpu::AddressMode::ClampToEdge,
            TextureAddressMode::Repeat => wgpu::AddressMode::Repeat,
            TextureAddressMode::Mirror => wgpu::AddressMode::MirrorRepeat,
        }
    }
}

#[derive(Clone, Copy)]
pub enum TextureFilter {
    Linear,
    Nearest,
}
impl From<TextureFilter> for wgpu::FilterMode {
    fn from(val: TextureFilter) -> Self {
        match val {
            TextureFilter::Linear => wgpu::FilterMode::Linear,
            TextureFilter::Nearest => wgpu::FilterMode::Nearest,
        }
    }
}

pub struct SamplerBuilder {
    label: Option<String>,
    address_mode: TextureAddressMode,
    filter: TextureFilter,
}
impl Default for SamplerBuilder {
    fn default() -> Self {
        Self {
            label: None,
            address_mode: TextureAddressMode::Clamp,
            filter: TextureFilter::Nearest,
        }
    }
}
impl SamplerBuilder {
    pub fn build(&self, gpu: &Gpu) -> Sampler {
        let sampler = gpu.get_device().create_sampler(&wgpu::SamplerDescriptor {
            label: self.label.as_deref(),
            address_mode_u: self.address_mode.into(),
            address_mode_v: self.address_mode.into(),
            address_mode_w: self.address_mode.into(),
            mag_filter: self.filter.into(),
            min_filter: self.filter.into(),
            mipmap_filter: self.filter.into(),
            ..Default::default()
        });

        Sampler { handle: sampler }
    }
}

#[derive(Debug, Clone)]
pub struct Sampler {
    handle: wgpu::Sampler,
}
impl Sampler {
    pub fn get_handle(&self) -> &wgpu::Sampler {
        &self.handle
    }
}
impl BindEntry for Sampler {
    fn resource(&self) -> wgpu::BindingResource<'_> {
        wgpu::BindingResource::Sampler(&self.handle)
    }

    fn ty(&self) -> wgpu::BindingType {
        wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
    }
}
