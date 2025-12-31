use crate::theme::catppuccin::*;
use bevy::prelude::*;

use crate::{arcade::menu::*, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::GameSelection), spawn_menu);
}

fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let style = MenuStyle::new(&asset_server);

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
            spawn_title!(parent, style, "Select Game");
            spawn_menu_item!(parent, style, 0, true, MenuAction::GoToPongGame, "Pong");
            spawn_menu_item!(parent, style, 1, false, MenuAction::GoToTitle, "Back");
        });
}
