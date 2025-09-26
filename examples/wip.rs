use luxlib::prelude::*;

fn main() {
    luxlib::setup()
        .title("My app")
        .window_size(UVec2::new(800, 600))
        .target_fps(60)
        .init(|frame| {
            let texture = frame.io().load_image("")?;
            Ok(())
        })
        .frame_loop(|frame, _| {
            let render = frame.render(Srgba::SILVER);
            // render.texture(texture, 400.0, 300.0);
            // render.text("Welcome to Luxlib! Here's your first window!", 400, 300, 18, Srgba::GRAY);
            Ok(render)
        })
        .start();
}
