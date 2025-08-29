use luxlib::prelude::*;

fn main() {
    luxlib::setup()
        .title("Luxlib Example - Mouse Input")
        .window_size(UVec2::new(800, 600))
        .target_fps(60)
        .no_init()
        .frame_loop(|frame, _| {
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

            let position = frame.input().get_mouse_position();

            let mut render = frame.render(Srgba::SILVER);
            render.rect(|r| {
                r.position(position).color(color);
            });
            Ok(render)
        })
        .start();
}
