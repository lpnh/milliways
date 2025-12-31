use bevy::prelude::*;

use crate::{
    pong::{spawning::*, *},
    screens::Screen,
    ui::{PressStart2P, Typography, menu::*, palette::*},
};

#[derive(Component)]
struct ModeSelectionMenu;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::PongGame), spawn_menu);
    app.add_systems(Update, handle_selection.run_if(in_state(Screen::PongGame)));
}

fn spawn_menu(mut commands: Commands, font: Res<PressStart2P>, typography: Res<Typography>) {
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
            spawn_title(parent, &font, &typography, "Pong");
            spawn_menu_item(
                parent,
                &font,
                &typography,
                0,
                true,
                MenuAction::StartPongSinglePlayer,
                "1 Player",
            );
            spawn_menu_item(
                parent,
                &font,
                &typography,
                1,
                false,
                MenuAction::StartPongMultiplayer,
                "2 Players",
            );
            spawn_menu_item(
                parent,
                &font,
                &typography,
                2,
                false,
                MenuAction::GoToGameSelection,
                "Back",
            );
        });
}

fn handle_selection(
    keyboard: Res<ButtonInput<KeyCode>>,
    items: Query<(&MenuItem, &MenuAction)>,
    menu: Query<Entity, With<ModeSelectionMenu>>,
    mut commands: Commands,
    mut mode: ResMut<PongGameMode>,
    mut next_screen: ResMut<NextState<Screen>>,
    playfield: Res<PlayfieldDimensions>,
    font: Res<PressStart2P>,
    typography: Res<Typography>,
) {
    if !keyboard.just_pressed(KeyCode::Enter) && !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    for (item, action) in &items {
        if item.selected {
            match action {
                MenuAction::StartPongSinglePlayer => {
                    mode.mode = GameMode::SinglePlayer;
                    if let Ok(entity) = menu.single() {
                        commands.entity(entity).despawn();
                    }
                    spawn_pong_level(&mut commands, &mode, &playfield, &font, &typography);
                }
                MenuAction::StartPongMultiplayer => {
                    mode.mode = GameMode::Multiplayer;
                    if let Ok(entity) = menu.single() {
                        commands.entity(entity).despawn();
                    }
                    spawn_pong_level(&mut commands, &mode, &playfield, &font, &typography);
                }
                MenuAction::GoToGameSelection => next_screen.set(Screen::GameSelection),
                _ => {}
            }
            break;
        }
    }
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
