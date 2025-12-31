use bevy::prelude::*;
use rand::Rng;

use crate::pong::*;

pub fn player_paddle_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    playfield: Res<PlayfieldDimensions>,
    mut paddle_query: Query<(&PlayerControlled, &mut Transform, &Paddle)>,
    time: Res<Time>,
) {
    for (controller, mut transform, paddle) in &mut paddle_query {
        let mut movement = 0.0;

        if keyboard.pressed(controller.up_key) {
            movement += paddle.speed;
        }
        if keyboard.pressed(controller.down_key) {
            movement -= paddle.speed;
        }

        transform.translation.y += movement * time.delta_secs();

        let half_height = 50.0;
        let min_y = -(playfield.height / 2.0) + half_height;
        let max_y = (playfield.height / 2.0) - half_height;
        transform.translation.y = transform.translation.y.clamp(min_y, max_y);
    }
}

pub fn ai_paddle_movement(
    time: Res<Time>,
    playfield: Res<PlayfieldDimensions>,
    ball_query: Query<&Transform, (With<Ball>, Without<Paddle>)>,
    mut ai_query: Query<(&mut AiControlled, &mut Transform, &Paddle)>,
) {
    let Ok(ball_transform) = ball_query.single() else {
        return;
    };

    let current_time = time.elapsed_secs();

    for (mut ai, mut transform, paddle) in &mut ai_query {
        let ball_x = ball_transform.translation.x;
        let ball_y = ball_transform.translation.y;

        let dist_factor = ball_x.abs().ceil() as i32;
        let direction_factor = if ball_x > 0.0 { 1 } else { 5 };

        let glitched =
            rand::rng().random::<i32>().abs() % (dist_factor / direction_factor).max(1) == 0;

        if glitched {
            ai.last_sleep = current_time;
            ai.sleeping = true;
        } else if ai.last_sleep + 0.2 < current_time {
            ai.sleeping = false;
        }

        if !ai.sleeping {
            let paddle_center = transform.translation.y;
            let target_y = ball_y;

            if target_y < paddle_center {
                transform.translation.y -= paddle.speed * time.delta_secs();
            } else if target_y > paddle_center {
                transform.translation.y += paddle.speed * time.delta_secs();
            }

            let half_height = 50.0;
            let min_y = -(playfield.height / 2.0) + half_height;
            let max_y = (playfield.height / 2.0) - half_height;
            transform.translation.y = transform.translation.y.clamp(min_y, max_y);
        }
    }
}
