use bevy::prelude::*;

use crate::{
    menus::Menu,
    ui::{PressStart2P, Typography, menu::*, palette::*},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_menu);
}

fn spawn_menu(mut commands: Commands, font: Res<PressStart2P>, typography: Res<Typography>) {
    let mut menu = MenuBuilder::new(&font, &typography);

    commands
        .spawn((
            Name::new("Main Menu"),
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
            DespawnOnExit(Menu::Main),
        ))
        .with_children(|parent| {
            menu.item(parent, MenuAction::GoToGameSelection, "Titles");
            menu.item(parent, MenuAction::OpenSettings, "Settings");
            menu.item(parent, MenuAction::OpenCredits, "Credits");

            #[cfg(not(target_family = "wasm"))]
            menu.item(parent, MenuAction::ExitApp, "Exit");
        });
}
