use luxlib::prelude::*;

struct Game {
    cursor_size: f32,
}

fn main() {
    luxlib::setup()
        .title("Luxlib Example - Mouse Input")
        .window_size(UVec2::new(800, 600))
        .target_fps(60)
        .init(|_| Ok(Game { cursor_size: 50.0 }))
        .frame_loop(|frame, game| {
            let mut color = Srgba::DARK_GRAY;
            if frame.input().is_mouse_pressed(MouseInput::Left) {
                color = Srgba::RED;
            } else if frame.input().is_mouse_pressed(MouseInput::Middle) {
                color = Srgba::BLUE;
            } else if frame.input().is_mouse_pressed(MouseInput::Right) {
                color = Srgba::PURPLE;
            } else if frame.input().is_mouse_pressed(MouseInput::Back) {
                color = Srgba::GREEN;
            } else if frame.input().is_mouse_pressed(MouseInput::Forward) {
                color = Srgba::ORANGE;
            }

            game.cursor_size += frame.input().get_wheel_delta() * 10.0;
            game.cursor_size = game.cursor_size.clamp(10.0, 150.0);

            let position = frame.input().get_mouse_position();

            let mut render = frame.render(Srgba::SILVER);
            render
                .text("buttons: change cursor color", 10.0, 80.0, 24.0)
                .tint(Srgba::DARK_GRAY);
            render
                .text("scroll: change cursor size", 10.0, 50.0, 24.0)
                .tint(Srgba::DARK_GRAY);
            render
                .rect(position.x, position.y, game.cursor_size, game.cursor_size)
                .color(color);
            Ok(render)
        })
        .start();
}
