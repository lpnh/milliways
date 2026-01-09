use bevy::prelude::*;

use crate::{
    screens::Screen,
    space_invaders::{components::*, resources::*},
    ui::{PressStart2P, palette::TEXT, typography::Typography},
};

pub fn spawn_player(
    commands: &mut Commands,
    playfield: &PlayfieldDimensions,
    sheet: &SpriteSheetHandle,
) {
    let x = 0.0;
    let y = -(playfield.height / 2.0) + 60.0;

    commands.spawn((
        Name::new("Player"),
        Player { speed: 300.0 },
        Collider {
            width: 48.0,
            height: 36.0,
        },
        Sprite::from_atlas_image(
            sheet.texture.clone(),
            TextureAtlas {
                layout: sheet.layout.clone(),
                index: 6,
            },
        ),
        Transform {
            translation: Vec3::new(x, y, 0.0),
            scale: Vec3::splat(3.0),
            ..default()
        },
        DespawnOnExit(Screen::SpaceInvadersGame),
    ));
}

pub fn spawn_invader_formation(
    commands: &mut Commands,
    playfield: &PlayfieldDimensions,
    sheet: &SpriteSheetHandle,
) {
    let start_x = -(playfield.width / 2.0) + 130.0;
    let start_y = (playfield.height / 2.0) - 100.0;

    for row in 1..=5 {
        let invader_type = match row {
            1 => InvaderType::A,
            2..=3 => InvaderType::B,
            _ => InvaderType::C,
        };

        let sprite_indices = invader_type.sprite_indices();

        for col in 1..=11 {
            let x = start_x + ((col - 1) as f32 * 48.0);
            let y = start_y - ((row - 1) as f32 * 43.2);

            commands.spawn((
                Name::new(format!("Invader-{}-{}", row, col)),
                Invader {
                    invader_type,
                    row,
                    column: col,
                    initial_x: x,
                    initial_y: y,
                },
                Collider {
                    width: 48.0,
                    height: 36.0,
                },
                SpriteAnimation {
                    timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                    frame: 0,
                },
                Sprite::from_atlas_image(
                    sheet.texture.clone(),
                    TextureAtlas {
                        layout: sheet.layout.clone(),
                        index: sprite_indices[0],
                    },
                ),
                Transform {
                    translation: Vec3::new(x, y, 0.0),
                    scale: Vec3::splat(3.0),
                    ..default()
                },
                DespawnOnExit(Screen::SpaceInvadersGame),
            ));
        }
    }
}

pub fn spawn_ui(
    commands: &mut Commands,
    playfield: &PlayfieldDimensions,
    font: &PressStart2P,
    typography: &Typography,
) {
    commands.spawn((
        Name::new("Score Display"),
        ScoreDisplay,
        Text2d::new("00000"),
        typography.heading(&font.0),
        TextColor(TEXT),
        Transform::from_xyz(0.0, (playfield.height / 2.0) - 30.0, 0.0),
        DespawnOnExit(Screen::SpaceInvadersGame),
    ));

    commands.spawn((
        Name::new("Lives Display"),
        LivesDisplay,
        Text2d::new("3"),
        typography.body(&font.0),
        TextColor(TEXT),
        Transform::from_xyz(
            -(playfield.width / 2.0) + 50.0,
            (playfield.height / 2.0) - 30.0,
            0.0,
        ),
        DespawnOnExit(Screen::SpaceInvadersGame),
    ));
}

pub fn spawn_player_laser(commands: &mut Commands, player_pos: Vec3, _sheet: &SpriteSheetHandle) {
    commands.spawn((
        Name::new("Player Laser"),
        Laser {
            velocity: 660.0,
            owner: LaserOwner::Player,
        },
        Collider {
            width: 4.0,
            height: 10.0,
        },
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(4.0, 10.0)),
            ..default()
        },
        Transform::from_xyz(player_pos.x, player_pos.y + 20.0, 0.0),
        DespawnOnExit(Screen::SpaceInvadersGame),
    ));
}

pub fn spawn_invader_laser(commands: &mut Commands, invader_pos: Vec3, _sheet: &SpriteSheetHandle) {
    commands.spawn((
        Name::new("Invader Laser"),
        Laser {
            velocity: -300.0,
            owner: LaserOwner::Invader,
        },
        Collider {
            width: 4.0,
            height: 10.0,
        },
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(4.0, 10.0)),
            ..default()
        },
        Transform::from_xyz(invader_pos.x, invader_pos.y - 20.0, 0.0),
        DespawnOnExit(Screen::SpaceInvadersGame),
    ));
}
