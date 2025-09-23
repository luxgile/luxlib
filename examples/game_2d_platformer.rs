use luxlib::{gpu::Camera2d, prelude::*};

// Represent quad obstacle
pub struct Obstacle {
    position: Vec2,
    rect: Rect,
}
impl Obstacle {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            rect: Rect::new(Vec2::new(w, h)),
        }
    }
}

// Player and its config
const PLAYER_SIZE: f32 = 25.0;
const PLAYER_SPEED: f32 = 300.0;
const PLAYER_JUMP: f32 = 450.0;
const PLAYER_GRAVITY: f32 = 600.0;
const PLAYER_COLOR: Srgba = Srgba::RED;
#[derive(Default)]
struct Player {
    position: Vec2,
    velocity: Vec2,
    on_ground: bool,
}
impl Player {
    fn update(&mut self, frame: &mut Frame, obstacles: &[Obstacle]) {
        // Movement
        if frame.input().is_key_pressed(KeyInput::Right) {
            self.velocity.x = PLAYER_SPEED;
        } else if frame.input().is_key_pressed(KeyInput::Left) {
            self.velocity.x = -PLAYER_SPEED;
        } else {
            self.velocity.x = 0.0;
        }

        // Obstacle detection
        let mut has_collision = false;
        for obstacle in obstacles {
            if let Some(hit) = obstacle.rect.check_circle(
                obstacle.position,
                Circle {
                    radius: PLAYER_SIZE,
                },
                self.position,
            ) {
                has_collision = true;
                if hit.normal.y > 0.0 {
                    // Ground
                    self.on_ground = true;
                    self.velocity.y = 0.0;
                    self.position.y =
                        PLAYER_SIZE + obstacle.position.y + obstacle.rect.size.y / 2.0;
                } else if f32::abs(hit.normal.x) > f32::EPSILON {
                    // Walls
                    if Vec2::dot(hit.normal, self.velocity) < 0.0 {
                        self.velocity.x = 0.0;
                    }
                } else {
                    // Ceiling
                    self.velocity.y = self.velocity.y.min(0.0);
                }
            }
        }

        // Apply gravity
        if !self.on_ground {
            self.velocity.y -= PLAYER_GRAVITY * frame.dt();
        } else {
            self.velocity.y = 0.0;
        }

        // Jump
        if frame.input().is_key_just_pressed(KeyInput::Space) && self.on_ground {
            self.velocity.y = PLAYER_JUMP;
            self.on_ground = false;
        }

        // Sudden fall from platform
        if !has_collision && self.on_ground {
            self.on_ground = false;
        }

        self.position += self.velocity * frame.dt();
    }

    fn render(&self, render: &mut RenderQueue) {
        render
            .circle(self.position.x, self.position.y, PLAYER_SIZE)
            .color(PLAYER_COLOR);
    }
}

// Game state
const CAMERA_SMOOTHNESS: f32 = 0.9;
#[derive(Default)]
struct Game {
    obstacles: Vec<Obstacle>,
    player: Player,
    camera_pos: Vec2,
}

fn main() {
    luxlib::setup()
        .target_fps(60)
        .init(game_init)
        .frame_loop(game_loop)
        .start();
}

fn game_init(_frame: &mut Frame) -> anyhow::Result<Game> {
    Ok(Game {
        obstacles: vec![
            Obstacle::new(0.0, -200.0, 550.0, 250.0),
            Obstacle::new(250.0, 10.0, 150.0, 20.0),
            Obstacle::new(-250.0, 10.0, 150.0, 20.0),
            Obstacle::new(75.0, 0.0, 30.0, 300.0),
        ],
        ..Default::default()
    })
}

fn game_loop(frame: &mut Frame, game: &mut Game) -> anyhow::Result<RenderQueue> {
    // Game update
    let player = &mut game.player;
    player.update(frame, &game.obstacles);

    game.camera_pos = player.position.lerp(game.camera_pos, CAMERA_SMOOTHNESS);

    // Game render
    let mut render = frame.render(Srgba::SILVER);
    render.update_camera_2d().position(game.camera_pos);
    player.render(&mut render);
    for obstacle in &game.obstacles {
        render
            .quad(
                obstacle.position.x,
                obstacle.position.y,
                obstacle.rect.size.x,
                obstacle.rect.size.y,
            )
            .color(Srgba::DARK_GRAY);
    }
    render
        .text("Use the arrows to move and 'Space' to jump!", 10.0, 10.0, 24.0)
        .tint(Srgba::DARK_GRAY);
    Ok(render)
}
