use bevy::prelude::*;

use crate::ui::{PressStart2P, Typography, palette::*};

pub const MENU_ARROW_WIDTH: f32 = 30.0;
pub const MENU_ITEM_GAP: f32 = 8.0;

pub struct MenuBuilder<'a> {
    font: &'a PressStart2P,
    typography: &'a Typography,
}

impl<'a> MenuBuilder<'a> {
    pub fn new(font: &'a PressStart2P, typography: &'a Typography) -> Self {
        Self { font, typography }
    }

    pub fn title(&self, parent: &mut ChildSpawnerCommands, text: impl Into<String>) {
        spawn_title(parent, self.font, self.typography, text)
    }

    pub fn item(
        &self,
        parent: &mut ChildSpawnerCommands,
        action: MenuAction,
        text: impl Into<String>,
    ) {
        spawn_menu_item(parent, self.font, self.typography, action, text)
    }

    pub fn back_button(&self, parent: &mut ChildSpawnerCommands, action: MenuAction) {
        spawn_back_button_section(parent, self.font, self.typography, action)
    }
}

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
    action: MenuAction,
    text: impl Into<String>,
) {
    parent
        .spawn((
            MenuItem { action },
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
                Text::new(" "),
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
                TextColor(OVERLAY0),
            ));
        });
}

pub fn spawn_back_button_section(
    parent: &mut ChildSpawnerCommands,
    font: &PressStart2P,
    typography: &Typography,
    action: MenuAction,
) {
    parent
        .spawn((
            Name::new("Back Button Container"),
            Node {
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
        ))
        .with_children(|button_parent| {
            spawn_menu_item(button_parent, font, typography, action, "Back");
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
    pub action: MenuAction,
}

#[derive(Resource, Default)]
pub struct MenuNavigation {
    pub selected: Option<Entity>,
}

#[derive(Component)]
pub struct SelectionArrow;

#[derive(Component)]
pub struct MenuText;
