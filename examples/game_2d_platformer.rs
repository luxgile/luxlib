use luxlib::prelude::*;

pub struct Obstacle {
    position: Vec2,
    rect: Rect,
}

const PLAYER_SIZE: f32 = 25.0;
const PLAYER_SPEED: f32 = 10.0;
const PLAYER_JUMP: f32 = 15.0;
const PLAYER_COLOR: Srgba = Srgba::RED;
#[derive(Default)]
struct Player {
    position: Vec2,
    velocity: Vec2,
    on_ground: bool,
}
impl Player {
    fn update(&mut self, frame: &mut Frame, obstacles: &[Obstacle]) {
        if frame.input().is_key_pressed(KeyInput::Right) {
            self.velocity.x += PLAYER_SPEED;
        }
        if frame.input().is_key_pressed(KeyInput::Left) {
            self.velocity.x -= PLAYER_SPEED;
        }
        if frame.input().is_key_just_pressed(KeyInput::Space) && self.on_ground {
            self.velocity.y += PLAYER_JUMP;
            self.on_ground = false;
        }

        for obstacle in obstacles {
            if let Some(hit) = obstacle.rect.check_circle(
                obstacle.position,
                Circle {
                    radius: PLAYER_SIZE,
                },
                self.position,
            ) {
                if hit.normal.y > 0.0 {
                    self.on_ground = true;
                    self.velocity.y = 0.0;
                } else if f32::abs(hit.normal.x) > f32::EPSILON {
                    self.velocity.x = 0.0;
                }
                break;
            }
        }

        self.position += self.velocity * frame.dt();
    }

    fn render(&self, render: &mut RenderQueue) {
        render
            .circle(self.position.x, self.position.y, PLAYER_SIZE)
            .color(PLAYER_COLOR);
    }
}

#[derive(Default)]
struct Game {
    obstacles: Vec<Obstacle>,
    player: Player,
}

fn main() {
    luxlib::setup()
        .init(game_init)
        .frame_loop(game_loop)
        .start();
}

fn game_init(_frame: &mut Frame) -> anyhow::Result<Game> {
    Ok(Game::default())
}

fn game_loop(frame: &mut Frame, game: &mut Game) -> anyhow::Result<RenderQueue> {
    let player = &mut game.player;

    player.update(frame, &[]);

    let mut render = frame.render(Srgba::WHITE);
    player.render(&mut render);
    Ok(render)
}
