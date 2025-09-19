use luxlib::{input::KeyInput, prelude::*};

struct Game {
    position: Vec2,
}

fn main() {
    luxlib::setup()
        .title("Luxlib Example - Keyboard Input")
        .window_size(UVec2::new(800, 600))
        .target_fps(60)
        .init(|_| {
            Ok(Game {
                position: [400.0, 300.0].into(),
            })
        })
        .frame_loop(|frame, game| {
            if frame.input().is_key_pressed(KeyInput::Right) {
                game.position.x += 100.0 * frame.dt();
            }
            if frame.input().is_key_pressed(KeyInput::Left) {
                game.position.x -= 100.0 * frame.dt();
            }
            if frame.input().is_key_pressed(KeyInput::Up) {
                game.position.y += 100.0 * frame.dt();
            }
            if frame.input().is_key_pressed(KeyInput::Down) {
                game.position.y -= 100.0 * frame.dt();
            }

            let mut render = frame.render(Srgba::SILVER);
            render
                .quad(game.position.x, game.position.y, 25.0, 25.0)
                .color(Srgba::DARK_GRAY);
            Ok(render)
        })
        .start();
}
