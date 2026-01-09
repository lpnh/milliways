mod collision;
mod invaders;
mod laser;
mod player;
mod scoring;

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            player::player_input,
            player::player_movement,
            invaders::update_formation_movement,
            invaders::invader_movement,
            invaders::spawn_invader_lasers,
            invaders::update_sprite_animation,
            laser::laser_movement,
            laser::despawn_offscreen_lasers,
            collision::laser_invader_collision,
            collision::laser_player_collision,
            scoring::update_score_display,
            scoring::update_lives_display,
            scoring::check_game_over,
        )
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::SpaceInvadersGame)),
    );
}
