use crate::theme::catppuccin::*;
use bevy::prelude::*;

use crate::{pong::*, screens::Screen};

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
        Name::new(format!("Paddle {:?} (Player)", side)),
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
        Name::new(format!("Paddle {:?} (AI)", side)),
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

pub fn spawn_score_displays(commands: &mut Commands) {
    commands.spawn((
        Name::new("Score Left"),
        ScoreDisplay {
            side: PaddleSide::Left,
        },
        Text::new("0"),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(TEXT),
        Transform::from_xyz(-100.0, 250.0, 0.0),
        DespawnOnExit(Screen::PongGame),
    ));

    commands.spawn((
        Name::new("Score Right"),
        ScoreDisplay {
            side: PaddleSide::Right,
        },
        Text::new("0"),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(TEXT),
        Transform::from_xyz(100.0, 250.0, 0.0),
        DespawnOnExit(Screen::PongGame),
    ));
}

pub fn spawn_center_divider(commands: &mut Commands) {
    for i in 0..20 {
        let y = -290.0 + (i as f32 * 30.0);
        commands.spawn((
            Name::new("Center Divider Dot"),
            CenterDivider,
            Sprite {
                color: OVERLAY0,
                custom_size: Some(Vec2::new(5.0, 20.0)),
                ..default()
            },
            Transform::from_xyz(0.0, y, 0.0),
            DespawnOnExit(Screen::PongGame),
        ));
    }
}
