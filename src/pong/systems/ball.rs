use bevy::prelude::*;

use crate::{
    audio::{SoundEffects, sfx},
    pong::*,
};

pub fn ball_movement(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Transform)>) {
    for (ball, mut transform) in &mut ball_query {
        transform.translation.x += ball.velocity.x * time.delta_secs();
        transform.translation.y += ball.velocity.y * time.delta_secs();
    }
}

pub fn ball_wall_collision(
    mut commands: Commands,
    sfx_handles: Res<SoundEffects>,
    playfield: Res<PlayfieldDimensions>,
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
) {
    for (mut ball, mut transform) in &mut ball_query {
        let half_size = ball.size / 2.0;
        let min_y = -(playfield.height / 2.0) + half_size;
        let max_y = (playfield.height / 2.0) - half_size;

        if transform.translation.y <= min_y {
            transform.translation.y = min_y;
            ball.velocity.y = ball.velocity.y.abs();

            commands.spawn((
                Name::new("Ball Wall Collision SFX"),
                sfx(sfx_handles.ball_collision.clone()),
            ));
        } else if transform.translation.y >= max_y {
            transform.translation.y = max_y;
            ball.velocity.y = -ball.velocity.y.abs();

            commands.spawn((
                Name::new("Ball Wall Collision SFX"),
                sfx(sfx_handles.ball_collision.clone()),
            ));
        }
    }
}

pub fn ball_paddle_collision(
    mut commands: Commands,
    sfx_handles: Res<SoundEffects>,
    mut ball_query: Query<(&mut Ball, &mut Transform), Without<Paddle>>,
    paddle_query: Query<(&Transform, &PaddleSide), With<Paddle>>,
) {
    let Ok((mut ball, mut ball_transform)) = ball_query.single_mut() else {
        return;
    };

    let paddle_width = 20.0;
    let paddle_height = 100.0;
    let ball_size = ball.size;

    for (paddle_transform, side) in &paddle_query {
        let ball_pos = ball_transform.translation;
        let paddle_pos = paddle_transform.translation;

        let collision = match side {
            PaddleSide::Left => {
                ball_pos.x - ball_size / 2.0 < paddle_pos.x + paddle_width / 2.0
                    && ball_pos.x + ball_size / 2.0 > paddle_pos.x
                    && ball_pos.y + ball_size / 2.0 >= paddle_pos.y - paddle_height / 2.0
                    && ball_pos.y - ball_size / 2.0 <= paddle_pos.y + paddle_height / 2.0
                    && ball.velocity.x < 0.0
            }
            PaddleSide::Right => {
                ball_pos.x + ball_size / 2.0 > paddle_pos.x - paddle_width / 2.0
                    && ball_pos.x - ball_size / 2.0 < paddle_pos.x
                    && ball_pos.y + ball_size / 2.0 >= paddle_pos.y - paddle_height / 2.0
                    && ball_pos.y - ball_size / 2.0 <= paddle_pos.y + paddle_height / 2.0
                    && ball.velocity.x > 0.0
            }
        };

        if collision {
            ball.velocity.x = -ball.velocity.x;

            commands.spawn((
                Name::new("Ball Paddle Collision SFX"),
                sfx(sfx_handles.ball_collision.clone()),
            ));

            match side {
                PaddleSide::Left => {
                    ball_transform.translation.x =
                        paddle_pos.x + paddle_width / 2.0 + ball_size / 2.0;
                }
                PaddleSide::Right => {
                    ball_transform.translation.x =
                        paddle_pos.x - paddle_width / 2.0 - ball_size / 2.0;
                }
            }
        }
    }
}
