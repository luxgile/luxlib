use luxlib::{model::ModelBuilder, prelude::*, vertex::Vertex2};

fn main() {
    luxlib::init()
        .title("My app")
        .window_size(glam::UVec2::new(800, 600))
        .frame_loop(|frame| {
            let mut render = frame.render(Srgba::WHITE);
            let sprite = ModelBuilder::new_sprite(frame.gpu())
                .vertices(vec![
                    Vertex2 {
                        color: Srgba::new(1.0, 0.0, 0.0, 1.0),
                        position: Vec2::new(0.0, 0.5),
                    },
                    Vertex2 {
                        color: Srgba::new(0.0, 1.0, 0.0, 1.0),
                        position: Vec2::new(-0.5, 0.0),
                    },
                    Vertex2 {
                        color: Srgba::new(0.0, 0.0, 1.0, 1.0),
                        position: Vec2::new(-0.2, -0.4),
                    },
                    Vertex2 {
                        color: Srgba::new(0.0, 1.0, 0.0, 1.0),
                        position: Vec2::new(0.3, -0.3),
                    },
                    Vertex2 {
                        color: Srgba::new(1.0, 0.0, 0.0, 1.0),
                        position: Vec2::new(0.4, 0.2),
                    },
                ])
                .indices(vec![0u16, 1, 4, 1, 2, 4, 2, 3, 4])
                .build(frame.gpu());
            render.draw(sprite);
            Some(render)
        })
        .start_app();
}
