use bevy::prelude::*;

use crate::space_invaders::{
    SpriteSheetHandle, components::*, resources::*, spawning::spawn_player_laser,
};

pub fn player_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cooldown: ResMut<LaserCooldown>,
    player_query: Query<&Transform, With<Player>>,
    sheet: Res<SpriteSheetHandle>,
    time: Res<Time>,
) {
    cooldown.timer.tick(time.delta());

    if keyboard.just_pressed(KeyCode::Space) {
        if cooldown.timer.is_finished() {
            if let Ok(player_transform) = player_query.single() {
                spawn_player_laser(&mut commands, player_transform.translation, &sheet);
                cooldown.timer.reset();
            }
        }
    }
}

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform), With<Player>>,
    playfield: Res<PlayfieldDimensions>,
    time: Res<Time>,
) {
    if let Ok((player, mut transform)) = player_query.single_mut() {
        let mut direction = 0.0;

        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            direction -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            direction += 1.0;
        }

        let new_x = transform.translation.x + direction * player.speed * time.delta_secs();

        let half_width = playfield.width / 2.0;
        let player_half_width = 24.0;

        transform.translation.x = new_x.clamp(
            -half_width + player_half_width,
            half_width - player_half_width,
        );
    }
}
