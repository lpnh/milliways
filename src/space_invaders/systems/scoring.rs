use bevy::prelude::*;

use crate::space_invaders::{components::*, resources::*};

pub fn update_score_display(
    game_state: Res<GameState>,
    mut score_query: Query<&mut Text2d, With<ScoreDisplay>>,
) {
    if game_state.is_changed() {
        if let Ok(mut text) = score_query.single_mut() {
            **text = format!("{:05}", game_state.score);
        }
    }
}

pub fn update_lives_display(
    game_state: Res<GameState>,
    mut lives_query: Query<&mut Text2d, With<LivesDisplay>>,
) {
    if game_state.is_changed() {
        if let Ok(mut text) = lives_query.single_mut() {
            **text = format!("{}", game_state.lives);
        }
    }
}

pub fn check_game_over(game_state: Res<GameState>, invader_query: Query<&Invader>) {
    if game_state.lives == 0 {
    } else if invader_query.iter().count() == 0 {
    }
}
