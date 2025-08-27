use crate::texture::Texture;

pub trait Material {
    fn build_pipeline();
    fn build_bind_group();
}

pub struct Material2d {
    main_texture: Texture,
}
