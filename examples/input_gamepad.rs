use gilrs::Axis;
use luxlib::prelude::*;

struct Game {
    gamepad_id: usize,
}

const STICK_DEADZONE: f32 = 0.1;
const TRIGGER_DEADZONE: f32 = -0.9;

fn main() {
    luxlib::setup()
        .title("Luxlib Example - Gamepad Input")
        .window_size(UVec2::new(800, 600))
        .target_fps(60)
        .init(|_| Ok(Game { gamepad_id: 0 }))
        .frame_loop(|frame, game| {
            let mut render = frame.render(Srgba::SILVER);

            // Change gamepad using arrow keys
            if frame.input().is_key_just_pressed(KeyInput::Left) && game.gamepad_id > 0 {
                game.gamepad_id -= 1;
            }
            if frame.input().is_key_just_pressed(KeyInput::Right) {
                game.gamepad_id += 1;
            }

            // Display gamepad info
            if let Some(gamepad) = frame.input().gamepad(game.gamepad_id) {
                render.text(
                    format!("Gamepad [{}]: {}", gamepad.id(), gamepad.name()),
                    10.0,
                    frame.gpu().get_window_size().y as f32 - 20.0,
                    24.0,
                );

                let left_stick_x = gamepad.axis_data(Axis::LeftStickX).unwrap();
                let left_stick_y = gamepad.axis_data(Axis::LeftStickY).unwrap();
                let right_stick_x = gamepad.axis_data(Axis::RightStickX).unwrap();
                let right_stick_y = gamepad.axis_data(Axis::RightStickY).unwrap();
                // TODO: Need a way to draw circles
            } else {
                // If no gamepad is found:
                render.text(
                    format!("No gamepad [{}] found.", game.gamepad_id),
                    10.0,
                    frame.gpu().get_window_size().y as f32 - 20.0,
                    24.0,
                );
            }
            Ok(render)
        })
        .start();
}
