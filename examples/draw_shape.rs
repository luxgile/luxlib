use luxlib::{
    material::StandardMaterial2d,
    model::{MeshBuilder, Model, ModelBuilder},
    prelude::*,
    vertex::Vertex2,
};

struct Game {
    sprite: Model,
}

fn main() {
    luxlib::setup()
        .title("My app")
        .window_size(glam::UVec2::new(800, 600))
        .target_fps(60)
        .init(|frame| {
            let sprite = ModelBuilder {
                material: Box::new(StandardMaterial2d::new(frame.gpu())),
                mesh: MeshBuilder {
                    vertices: vec![
                        Vertex2 {
                            color: Srgba::new(1.0, 0.0, 0.0, 1.0),
                            position: Vec2::new(0.0, 0.5),
                            uv: Vec2::new(0.0, 0.0),
                        },
                        Vertex2 {
                            color: Srgba::new(0.0, 1.0, 0.0, 1.0),
                            position: Vec2::new(-0.5, 0.0),
                            uv: Vec2::new(0.0, 0.0),
                        },
                        Vertex2 {
                            color: Srgba::new(0.0, 0.0, 1.0, 1.0),
                            position: Vec2::new(-0.2, -0.4),
                            uv: Vec2::new(0.0, 0.0),
                        },
                        Vertex2 {
                            color: Srgba::new(0.0, 1.0, 0.0, 1.0),
                            position: Vec2::new(0.3, -0.3),
                            uv: Vec2::new(0.0, 0.0),
                        },
                        Vertex2 {
                            color: Srgba::new(1.0, 0.0, 0.0, 1.0),
                            position: Vec2::new(0.4, 0.2),
                            uv: Vec2::new(0.0, 0.0),
                        },
                    ],
                    indices: vec![0u16, 1, 4, 1, 2, 4, 2, 3, 4],
                },
            }
            .build(frame.gpu());

            Ok(Game { sprite })
        })
        .frame_loop(|frame, state| {
            let mut render = frame.render(Srgba::WHITE);
            render.draw(&state.sprite);
            Ok(render)
        })
        .start();
}
