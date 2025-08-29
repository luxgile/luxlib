use luxlib::prelude::*;

struct Game {
    pub texture: Texture,
}

fn main() {
    luxlib::setup()
        .title("My app")
        .window_size(UVec2::new(800, 600))
        .target_fps(60)
        .init(|frame| {
            Ok(Game {
                texture: frame
                    .io()
                    .load_image("examples/texture.png")?
                    .create_texture(frame.gpu()),
            })
        })
        .frame_loop(|frame, game| {
            let mut render = frame.render(Srgba::WHITE);
            render.texture(|t| {
                t.texture(&game.texture)
                    .position(Vec2::new(400.0, 300.0))
                    .scale(Vec2::ONE * 0.25);
            });
            Ok(render)
        })
        .start();
}
