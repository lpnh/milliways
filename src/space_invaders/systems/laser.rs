use bevy::prelude::*;

use crate::space_invaders::{components::*, resources::*};

pub fn laser_movement(
    mut laser_query: Query<(&Laser, &mut Transform), With<Laser>>,
    time: Res<Time>,
) {
    for (laser, mut transform) in laser_query.iter_mut() {
        transform.translation.y += laser.velocity * time.delta_secs();
    }
}

pub fn despawn_offscreen_lasers(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
    playfield: Res<PlayfieldDimensions>,
) {
    let half_height = playfield.height / 2.0;

    for (entity, transform) in laser_query.iter() {
        if transform.translation.y > half_height + 20.0
            || transform.translation.y < -half_height - 20.0
        {
            commands.entity(entity).despawn();
        }
    }
}
