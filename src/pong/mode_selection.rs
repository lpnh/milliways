use bevy::prelude::*;

use crate::{
    pong::{spawning::*, *},
    screens::Screen,
    ui::{PressStart2P, Typography, menu::*, palette::*},
};

#[derive(Component)]
pub struct ModeSelectionMenu;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::PongGame), spawn_menu);
    app.add_systems(
        Update,
        start_game_on_mode_change.run_if(in_state(Screen::PongGame)),
    );
}

fn spawn_menu(mut commands: Commands, font: Res<PressStart2P>, typography: Res<Typography>) {
    let mut menu = MenuBuilder::new(&font, &typography);

    commands
        .spawn((
            Name::new("Mode Selection"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(40.0),
                ..default()
            },
            BackgroundColor(MANTLE),
            ModeSelectionMenu,
            DespawnOnExit(Screen::PongGame),
        ))
        .with_children(|parent| {
            menu.title(parent, "Pong");
            menu.item(parent, MenuAction::StartPongSinglePlayer, "1 Player");
            menu.item(parent, MenuAction::StartPongMultiplayer, "2 Players");
            menu.back_button(parent, MenuAction::GoToGameSelection);
        });
}

fn start_game_on_mode_change(
    mode: Res<PongGameMode>,
    menu: Query<Entity, With<ModeSelectionMenu>>,
    mut commands: Commands,
    playfield: Res<PlayfieldDimensions>,
    font: Res<PressStart2P>,
    typography: Res<Typography>,
) {
    if !mode.is_changed() {
        return;
    }

    if let Ok(entity) = menu.single() {
        commands.entity(entity).despawn();
    }

    spawn_pong_level(&mut commands, &mode, &playfield, &font, &typography);
}

fn spawn_pong_level(
    commands: &mut Commands,
    game_mode: &PongGameMode,
    playfield: &PlayfieldDimensions,
    font: &PressStart2P,
    typography: &Typography,
) {
    spawn_ball(commands);

    match game_mode.mode {
        GameMode::SinglePlayer => {
            spawn_player_paddle(commands, PaddleSide::Left, KeyCode::KeyW, KeyCode::KeyS);
            spawn_ai_paddle(commands, PaddleSide::Right);
        }
        GameMode::Multiplayer => {
            spawn_player_paddle(commands, PaddleSide::Left, KeyCode::KeyW, KeyCode::KeyS);
            spawn_player_paddle(
                commands,
                PaddleSide::Right,
                KeyCode::ArrowUp,
                KeyCode::ArrowDown,
            );
        }
    }

    spawn_playfield(commands, playfield);
    spawn_score_displays(commands, playfield, font, typography);
}
