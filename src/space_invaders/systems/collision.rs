use bevy::prelude::*;

use crate::space_invaders::{components::*, resources::*};

fn aabb_collision(a_pos: Vec3, a_size: &Collider, b_pos: Vec3, b_size: &Collider) -> bool {
    a_pos.x < b_pos.x + b_size.width
        && a_pos.x + a_size.width > b_pos.x
        && a_pos.y < b_pos.y + b_size.height
        && a_pos.y + a_size.height > b_pos.y
}

pub fn laser_invader_collision(
    mut commands: Commands,
    laser_query: Query<(Entity, &Laser, &Transform, &Collider), With<Laser>>,
    invader_query: Query<(Entity, &Invader, &Transform, &Collider), With<Invader>>,
    mut game_state: ResMut<GameState>,
    mut formation: ResMut<InvaderFormation>,
) {
    for (laser_entity, laser, laser_transform, laser_collider) in laser_query.iter() {
        if laser.owner != LaserOwner::Player {
            continue;
        }

        for (invader_entity, invader, invader_transform, invader_collider) in invader_query.iter() {
            if aabb_collision(
                laser_transform.translation,
                laser_collider,
                invader_transform.translation,
                invader_collider,
            ) {
                commands.entity(laser_entity).despawn();
                commands.entity(invader_entity).despawn();

                game_state.score += invader.invader_type.points();

                formation.vanguard[invader.column - 1] = invader_query
                    .iter()
                    .filter(|(_, other_invader, _, _)| {
                        other_invader.column == invader.column && other_invader.row > invader.row
                    })
                    .map(|(_, other_invader, _, _)| other_invader.row)
                    .min()
                    .unwrap_or(0);

                break;
            }
        }
    }
}

pub fn laser_player_collision(
    mut commands: Commands,
    laser_query: Query<(Entity, &Laser, &Transform, &Collider), With<Laser>>,
    player_query: Query<(&Transform, &Collider), With<Player>>,
    mut game_state: ResMut<GameState>,
) {
    if let Ok((player_transform, player_collider)) = player_query.single() {
        for (laser_entity, laser, laser_transform, laser_collider) in laser_query.iter() {
            if laser.owner != LaserOwner::Invader {
                continue;
            }

            if aabb_collision(
                laser_transform.translation,
                laser_collider,
                player_transform.translation,
                player_collider,
            ) {
                commands.entity(laser_entity).despawn();

                if game_state.lives > 0 {
                    game_state.lives -= 1;
                }

                break;
            }
        }
    }
}
