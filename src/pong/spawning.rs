use bevy::prelude::*;

use crate::{
    pong::*,
    screens::Screen,
    ui::{PressStart2P, Typography, palette::*},
};

pub fn spawn_player_paddle(
    commands: &mut Commands,
    side: PaddleSide,
    up_key: KeyCode,
    down_key: KeyCode,
) {
    let x = match side {
        PaddleSide::Left => -380.0,
        PaddleSide::Right => 380.0,
    };

    commands.spawn((
        Paddle::default(),
        side,
        PlayerControlled { up_key, down_key },
        Sprite {
            color: TEXT,
            custom_size: Some(Vec2::new(20.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(x, 0.0, 0.0),
        DespawnOnExit(Screen::PongGame),
    ));
}

pub fn spawn_ai_paddle(commands: &mut Commands, side: PaddleSide) {
    let x = match side {
        PaddleSide::Left => -380.0,
        PaddleSide::Right => 380.0,
    };

    commands.spawn((
        Paddle::default(),
        side,
        AiControlled::default(),
        Sprite {
            color: TEXT,
            custom_size: Some(Vec2::new(20.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(x, 0.0, 0.0),
        DespawnOnExit(Screen::PongGame),
    ));
}

pub fn spawn_ball(commands: &mut Commands) {
    commands.spawn((
        Name::new("Ball"),
        Ball::default(),
        Sprite {
            color: TEXT,
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        DespawnOnExit(Screen::PongGame),
    ));
}

pub fn spawn_score_displays(
    commands: &mut Commands,
    playfield: &PlayfieldDimensions,
    font: &PressStart2P,
    typography: &Typography,
) {
    let half_height = playfield.height / 2.0;
    let score_y = half_height - 40.0;
    let score_spacing = 100.0;

    commands.spawn((
        Name::new("Left Score"),
        ScoreDisplay {
            side: PaddleSide::Left,
        },
        Text2d::new("0"),
        typography.title(&font.0),
        TextColor(OVERLAY0),
        Transform::from_xyz(-score_spacing, score_y, 0.0),
        DespawnOnExit(Screen::PongGame),
    ));

    commands.spawn((
        Name::new("Right Score"),
        ScoreDisplay {
            side: PaddleSide::Right,
        },
        Text2d::new("0"),
        typography.title(&font.0),
        TextColor(OVERLAY0),
        Transform::from_xyz(score_spacing, score_y, 0.0),
        DespawnOnExit(Screen::PongGame),
    ));
}

pub fn spawn_playfield(commands: &mut Commands, playfield: &PlayfieldDimensions) {
    let half_height = playfield.height / 2.0;

    commands
        .spawn((
            Name::new("Playfield"),
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(Screen::PongGame),
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite {
                    color: TEXT,
                    custom_size: Some(Vec2::new(playfield.width, 2.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, half_height, 0.0),
            ));

            parent.spawn((
                Sprite {
                    color: TEXT,
                    custom_size: Some(Vec2::new(playfield.width, 2.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, -half_height, 0.0),
            ));

            for i in 0..20 {
                let y = -290.0 + (i as f32 * 30.0);
                parent.spawn((
                    CenterDivider,
                    Sprite {
                        color: OVERLAY0,
                        custom_size: Some(Vec2::new(5.0, 20.0)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, y, 0.0),
                ));
            }
        });
}
