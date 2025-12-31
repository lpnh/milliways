use bevy::prelude::*;

use crate::pong::*;

pub fn detect_scoring(
    playfield: Res<PlayfieldDimensions>,
    mut ball_query: Query<(&Ball, &mut Transform)>,
    mut paddle_query: Query<(&mut Paddle, &PaddleSide)>,
) {
    let Ok((ball, mut ball_transform)) = ball_query.single_mut() else {
        return;
    };

    let half_width = playfield.width / 2.0;
    let ball_size = ball.size;

    let scored = if ball_transform.translation.x < -half_width + ball_size / 2.0 {
        Some(PaddleSide::Right)
    } else if ball_transform.translation.x > half_width - ball_size / 2.0 {
        Some(PaddleSide::Left)
    } else {
        None
    };

    if let Some(scoring_side) = scored {
        for (mut paddle, side) in &mut paddle_query {
            if *side == scoring_side {
                paddle.score += 1;
            }
        }

        ball_transform.translation.x = 0.0;
        ball_transform.translation.y = 0.0;
    }
}

pub fn update_score_displays(
    paddle_query: Query<(&Paddle, &PaddleSide)>,
    mut display_query: Query<(&ScoreDisplay, &mut Text2d)>,
) {
    for (display, mut text) in &mut display_query {
        for (paddle, side) in &paddle_query {
            if *side == display.side {
                **text = paddle.score.to_string();
            }
        }
    }
}
