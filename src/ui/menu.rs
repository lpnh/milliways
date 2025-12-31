use bevy::prelude::*;

use crate::ui::{PressStart2P, Typography, palette::*};

pub const MENU_ARROW_WIDTH: f32 = 30.0;
pub const MENU_ITEM_GAP: f32 = 8.0;

pub fn spawn_title(
    parent: &mut ChildSpawnerCommands,
    font: &PressStart2P,
    typography: &Typography,
    text: impl Into<String>,
) {
    parent.spawn((Text::new(text), typography.title(&font.0), TextColor(TEXT)));
}

pub fn spawn_menu_item(
    parent: &mut ChildSpawnerCommands,
    font: &PressStart2P,
    typography: &Typography,
    index: usize,
    selected: bool,
    action: MenuAction,
    text: impl Into<String>,
) {
    parent
        .spawn((
            MenuItem { index, selected },
            action,
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(MENU_ITEM_GAP),
                ..default()
            },
        ))
        .with_children(|item| {
            item.spawn((
                SelectionArrow,
                Text::new(if selected { ">" } else { " " }),
                typography.heading(&font.0),
                TextColor(TEXT),
                Node {
                    width: Val::Px(MENU_ARROW_WIDTH),
                    ..default()
                },
            ));

            item.spawn((
                MenuText,
                Text::new(text),
                typography.heading(&font.0),
                TextColor(if selected { TEXT } else { OVERLAY0 }),
            ));
        });
}

#[derive(Component, Clone, Debug)]
pub enum MenuAction {
    GoToGameSelection,
    GoToTitle,
    GoToPongGame,
    OpenSettings,
    OpenCredits,
    BackToMainMenu,
    CloseMenu,
    RestartGame,
    StartPongSinglePlayer,
    StartPongMultiplayer,
    #[cfg(not(target_family = "wasm"))]
    ExitApp,
}

#[derive(Component)]
pub struct MenuItem {
    pub index: usize,
    pub selected: bool,
}

#[derive(Component)]
pub struct SelectionArrow;

#[derive(Component)]
pub struct MenuText;
