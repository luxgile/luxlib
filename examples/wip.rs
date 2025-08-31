use cosmic_text::{Attrs, Buffer, Color, FontSystem, Metrics, Shaping, SwashCache};
use luxlib::{prelude::*, texture::TextureBuilder};

fn main() {
    luxlib::setup()
        .title("My app")
        .window_size(UVec2::new(800, 600))
        .target_fps(60)
        .init(|frame| {
            // A FontSystem provides access to detected system fonts, create one per application
            let mut font_system = FontSystem::new();

            // A SwashCache stores rasterized glyphs, create one per application
            let mut swash_cache = SwashCache::new();

            // Text metrics indicate the font size and line height of a buffer
            const FONT_SIZE: f32 = 24.0;
            const LINE_HEIGHT: f32 = FONT_SIZE * 1.2;
            let metrics = Metrics::new(FONT_SIZE, LINE_HEIGHT);

            // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
            let mut buffer = Buffer::new(&mut font_system, metrics);

            let mut buffer = buffer.borrow_with(&mut font_system);

            // Set a size for the text buffer, in pixels
            let width = 150.0;
            // The height is unbounded
            buffer.set_size(Some(width), None);

            // Attributes indicate what font to choose
            let attrs = Attrs::new();

            // Add some text!
            let text = std::env::args()
                .nth(1)
                .unwrap_or(" Hi, Rust! 🦀 ".to_string());
            buffer.set_text(&text, &attrs, Shaping::Advanced);

            // Perform shaping as desired
            buffer.shape_until_scroll(true);

            // Default text color (0xFF, 0xFF, 0xFF is white)
            const TEXT_COLOR: Color = Color::rgb(0x00, 0x00, 0x00);

            // Set up the canvas
            let height = LINE_HEIGHT * buffer.layout_runs().count() as f32;
            let mut texture_pixels =
                vec![vec![Srgba::CLEAR.as_rgba8(); width as usize]; height as usize];

            // Draw to the canvas
            buffer.draw(&mut swash_cache, TEXT_COLOR, |x, y, w, h, color| {
                let a = color.a();
                if a == 0
                    || x < 0
                    || x >= width as i32
                    || y < 0
                    || y >= height as i32
                    || w != 1
                    || h != 1
                {
                    // Ignore alphas of 0, or invalid x, y coordinates, or unimplemented sizes
                    return;
                }

                // Scale by alpha (mimics blending with black)
                let scale = |c: u8| (c as i32 * a as i32 / 255).clamp(0, 255) as u8;

                let r = scale(color.r());
                let g = scale(color.g());
                let b = scale(color.b());
                texture_pixels[y as usize][x as usize] = Rgba8::new(r, g, b, 255);
            });

            let texture = TextureBuilder {
                size: UVec3::new(width as u32, height as u32, 1),
                content: Some(bytemuck::cast_vec(
                    texture_pixels.into_iter().flatten().collect(),
                )),
                ..Default::default()
            }
            .build(frame.gpu());

            Ok(texture)
        })
        .frame_loop(|frame, texture| {
            let mut render = frame.render(Srgba::SILVER);
            render.texture(texture, |t| {
                t.position(Vec2::new(400.0, 300.0)).scale(Vec2::ONE);
            });
            // render.text("Welcome to Luxlib! Here's your first window!", 400, 300, 18, Srgba::GRAY);
            Ok(render)
        })
        .start();
}
