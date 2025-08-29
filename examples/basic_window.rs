use luxlib::prelude::*;

fn main() {
    luxlib::setup()
        .title("My app")
        .window_size(UVec2::new(800, 600))
        .target_fps(60)
        .no_init()
        .frame_loop(|frame, _| Ok(frame.render(Srgba::WHITE)))
        .start();
}
