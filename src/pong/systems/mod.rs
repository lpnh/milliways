pub mod ball;
pub mod paddle;
pub mod scoring;

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            paddle::player_paddle_movement,
            paddle::ai_paddle_movement,
            ball::ball_movement,
            ball::ball_wall_collision,
            ball::ball_paddle_collision,
            scoring::detect_scoring,
            scoring::update_score_displays,
        )
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::PongGame)),
    );
}
