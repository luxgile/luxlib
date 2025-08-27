use luxlib::prelude::*;

fn main() {
    luxlib::setup()
        .title("My app")
        .window_size(UVec2::new(800, 600))
        .no_init()
        .frame_loop(|frame, _| Some(frame.render(Srgba::WHITE)))
        .start();
}
