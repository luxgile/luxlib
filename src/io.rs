use glam::UVec3;
use image::GenericImageView;

use crate::{Error, LuxError, gpu::Gpu, prelude::Texture, texture::TextureBuilder};

pub struct Image {
    image: image::DynamicImage,
}
impl Image {
    pub fn handle(&self) -> &image::DynamicImage {
        &self.image
    }

    pub fn create_texture(&self, gpu: &Gpu) -> Texture {
        TextureBuilder {
            size: UVec3::new(self.image.dimensions().0, self.image.dimensions().1, 1),
            content: Some(self.handle().to_rgba8().to_vec()),
            ..Default::default()
        }
        .build(gpu)
    }
}

pub struct Io;
impl Io {
    pub fn load_image(&self, path: impl Into<String>) -> Result<Image, LuxError> {
        let image = image::ImageReader::open(path.into())
            .map_err(|e| LuxError::IoError(e.to_string()))?
            .decode()
            .map_err(|e| LuxError::IoError(e.to_string()))?;

        Ok(Image { image })
    }
}
