use cosmic_text::Metrics;
use glam::UVec3;

use crate::{
    color::{Rgba8, Srgba},
    gpu::{DrawCommand, Gpu},
    prelude::Texture,
    texture::TextureBuilder,
};

pub struct Text {
    pub text: String,
    pub font_size: f32,
    pub width: f32,

    pub buffer: cosmic_text::Buffer,
    // texture: Texture,
    dirty: bool,
}
impl Text {
    pub fn new(text: impl Into<String>, font_size: f32) -> Self {
        Self {
            text: text.into(),
            font_size,
            width: 128.0,
            dirty: true,
            buffer: cosmic_text::Buffer::new_empty(Metrics::new(16.0, 16.0 * 1.2)),
            // texture: Texture::clone_white_texture(),
        }
    }

    // pub fn get_texture(&self) -> &Texture {
    //     &self.texture
    // }



    pub fn rebuild(
        &mut self,
        gpu: &Gpu,
        font_system: &mut cosmic_text::FontSystem,
        swash: &mut cosmic_text::SwashCache,
    ) {
        self.dirty = false;
        let line_height = self.font_size * 1.2;
        let metrics = Metrics::new(self.font_size, line_height);
        self.buffer.set_metrics(font_system, metrics);
        let mut buffer = self.buffer.borrow_with(font_system);

        let attrs = cosmic_text::Attrs::new();
        buffer.set_text(&self.text, &attrs, cosmic_text::Shaping::Advanced);
        buffer.set_size(Some(self.width), None);
        buffer.shape_until_scroll(true);

        // let height = line_height * buffer.layout_runs().count() as f32;
        // let mut texture_pixels =
        //     vec![vec![Srgba::CLEAR.as_rgba8(); self.width as usize]; height as usize];
        // buffer.draw(
        //     swash,
        //     cosmic_text::Color::rgb(0, 0, 0),
        //     |x, y, w, h, color| {
        //         let a = color.a();
        //         if a == 0
        //             || x < 0
        //             || x >= self.width as i32
        //             || y < 0
        //             || y >= height as i32
        //             || w != 1
        //             || h != 1
        //         {
        //             // Ignore alphas of 0, or invalid x, y coordinates, or unimplemented sizes
        //             return;
        //         }
        //
        //         // Scale by alpha (mimics blending with black)
        //         let scale = |c: u8| (c as i32 * a as i32 / 255).clamp(0, 255) as u8;
        //
        //         let r = scale(color.r());
        //         let g = scale(color.g());
        //         let b = scale(color.b());
        //         texture_pixels[y as usize][x as usize] = Rgba8::new(r, g, b, 255);
        //     },
        // );

        // self.texture = TextureBuilder {
        //     size: UVec3::new(self.width as u32, height as u32, 1),
        //     content: Some(bytemuck::cast_vec(
        //         texture_pixels.into_iter().flatten().collect(),
        //     )),
        //     ..Default::default()
        // }
        // .build(gpu)
    }
}
