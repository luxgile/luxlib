use luxlib::prelude::*;

fn main() {
    luxlib::setup()
        .title("My app")
        .window_size(UVec2::new(800, 600))
        .target_fps(60)
        .no_init()
        .frame_loop(|frame, _| {
            let mut render = frame.render(Srgba::SILVER);
            render.text("Hello from luxlib!", 400.0, 300.0);
            Ok(render)
        })
        .start();
}
