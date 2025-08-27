use luxlib::{
    model::{Model, ModelBuilder},
    prelude::*,
    vertex::Vertex2,
};

struct Game {
    sprite: Option<Model>,
}

fn main() {
    luxlib::setup()
        .title("My app")
        .window_size(glam::UVec2::new(800, 600))
        .init(|frame| {
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
            Game {
                sprite: Some(sprite),
            }
        })
        .frame_loop(|frame, state| {
            let mut render = frame.render(Srgba::WHITE);
            render.draw(state.sprite.as_ref().unwrap());
            Some(render)
        })
        .start();
}
