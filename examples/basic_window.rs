use luxlib::prelude::*;

fn main() {
    luxlib::init()
        .title("My app")
        .window_size(UVec2::new(800, 600))
        .frame_loop(|frame| Some(frame.render(Srgba::WHITE)))
        .start_app();
}
