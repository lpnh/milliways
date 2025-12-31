use bevy::prelude::*;

use crate::{
    screens::Screen,
    ui::{PressStart2P, Typography, menu::*, palette::*},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::GameSelection), spawn_menu);
}

fn spawn_menu(mut commands: Commands, font: Res<PressStart2P>, typography: Res<Typography>) {
    commands
        .spawn((
            Name::new("Game Selection"),
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
            DespawnOnExit(Screen::GameSelection),
        ))
        .with_children(|parent| {
            spawn_title(parent, &font, &typography, "Select Game");
            spawn_menu_item(
                parent,
                &font,
                &typography,
                0,
                true,
                MenuAction::GoToPongGame,
                "Pong",
            );
            spawn_menu_item(
                parent,
                &font,
                &typography,
                1,
                false,
                MenuAction::GoToTitle,
                "Back",
            );
        });
}
