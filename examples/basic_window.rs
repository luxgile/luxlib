use luxlib::prelude::*;

fn main() {
    luxlib::setup()
        .title("My app")
        .window_size(UVec2::new(800, 600))
        .target_fps(60)
        .no_init()
        .frame_loop(|frame, _| {
            let mut render = frame.render(Srgba::SILVER);
            render.text("LAKC", |dt| {
                dt.position(Vec2::new(400.0, 300.0))
                    .font_size(48.0)
                    .tint(Srgba::BLACK);
            });
            Ok(render)
        })
        .start();
}
