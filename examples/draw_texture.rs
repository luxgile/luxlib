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
            let screen_size = frame.gpu().get_window_size();
            render
                .texture(
                    &game.texture,
                    screen_size.x as f32 / 2.0,
                    screen_size.y as f32 / 2.0,
                )
                .uniform_scale(0.25);
            Ok(render)
        })
        .start();
}
