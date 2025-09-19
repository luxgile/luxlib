use luxlib::prelude::*;

fn main() {
    luxlib::setup()
        .title("My app")
        .window_size(UVec2::new(800, 600))
        .target_fps(60)
        .no_init()
        .frame_loop(|frame, _| {
            let mut render = frame.render(Srgba::SILVER);
            render
                .text("Hello from luxlib!", 350.0, 400.0, 32.0)
                .tint(Srgba::DARK_GRAY);
            Ok(render)
        })
        .start();
}
