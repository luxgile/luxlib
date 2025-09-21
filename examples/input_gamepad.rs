use gilrs::{Axis, Button};
use luxlib::prelude::*;

struct Game {
    gamepad_id: usize,
}

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
                    20.0,
                    20.0,
                    24.0,
                );

                let stick_radius = 50.0;
                
                // Left stick
                let left_stick_x = gamepad.value(Axis::LeftStickX);
                let left_stick_y = gamepad.value(Axis::LeftStickY);
                render.text("Left stick", 315.0, 130.0, 12.0);
                render.circle_line(350.0, 150.0, stick_radius);
                render.circle(
                    350.0 + left_stick_x * stick_radius,
                    150.0 - left_stick_y * stick_radius,
                    5.0,
                );

                // Right stick
                let right_stick_x = gamepad.value(Axis::RightStickX);
                let right_stick_y = gamepad.value(Axis::RightStickY);
                render.text("Right stick", 510.0, 130.0, 12.0);
                render.circle_line(550.0, 150.0, stick_radius);
                render.circle(
                    550.0 + right_stick_x * stick_radius,
                    150.0 - right_stick_y * stick_radius,
                    5.0,
                );
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
