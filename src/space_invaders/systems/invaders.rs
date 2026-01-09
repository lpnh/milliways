use bevy::prelude::*;
use rand::Rng;

use crate::space_invaders::{
    SpriteSheetHandle, components::*, resources::*, spawning::spawn_invader_laser,
};

pub fn update_formation_movement(
    mut formation: ResMut<InvaderFormation>,
    invader_query: Query<&Transform, With<Invader>>,
    playfield: Res<PlayfieldDimensions>,
) {
    let mut leftmost = f32::MAX;
    let mut rightmost = f32::MIN;

    for transform in invader_query.iter() {
        leftmost = leftmost.min(transform.translation.x);
        rightmost = rightmost.max(transform.translation.x);
    }

    let half_width = playfield.width / 2.0;
    let margin = 50.0;

    if formation.direction > 0.0 && rightmost + 24.0 > half_width - margin {
        formation.direction = -1.0;
        formation.speed *= 1.1;
        formation.rank += 1;
    } else if formation.direction < 0.0 && leftmost - 24.0 < -half_width + margin {
        formation.direction = 1.0;
        formation.speed *= 1.1;
        formation.rank += 1;
    }
}

pub fn invader_movement(
    formation: Res<InvaderFormation>,
    mut invader_query: Query<&mut Transform, With<Invader>>,
    time: Res<Time>,
) {
    let movement = formation.direction * formation.speed * time.delta_secs();

    for mut transform in invader_query.iter_mut() {
        transform.translation.x += movement;
    }
}

pub fn spawn_invader_lasers(
    mut commands: Commands,
    invader_query: Query<(&Invader, &Transform), With<Invader>>,
    formation: Res<InvaderFormation>,
    sheet: Res<SpriteSheetHandle>,
) {
    let mut rng = rand::rng();

    for (invader, transform) in invader_query.iter() {
        if invader.row == formation.vanguard[invader.column - 1] {
            if rng.random::<f32>() < 0.001 {
                spawn_invader_laser(&mut commands, transform.translation, &sheet);
            }
        }
    }
}

pub fn update_sprite_animation(
    mut invader_query: Query<(&Invader, &mut SpriteAnimation, &mut Sprite)>,
    time: Res<Time>,
) {
    for (invader, mut animation, mut sprite) in invader_query.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            animation.frame = 1 - animation.frame;

            let sprite_indices = invader.invader_type.sprite_indices();
            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index = sprite_indices[animation.frame];
            }
        }
    }
}
