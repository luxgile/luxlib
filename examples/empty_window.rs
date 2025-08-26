fn main() {
    luxlib::init()
        .title("My app")
        .window_size(glam::UVec2::new(800, 600))
        .start_app();
}
