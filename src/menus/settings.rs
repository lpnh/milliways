use crate::theme::catppuccin::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{arcade::menu::*, menus::Menu};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_settings_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let style = MenuStyle::new(&asset_server);

    commands
        .spawn((
            Name::new("Settings Menu"),
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
            DespawnOnExit(Menu::Settings),
        ))
        .with_children(|parent| {
            spawn_title!(parent, style, "Settings");

            parent.spawn((
                Text::new("Settings coming soon..."),
                TextFont {
                    font: style.font.clone(),
                    font_size: style.text_size,
                    ..default()
                },
                TextColor(TEXT),
            ));

            spawn_menu_item!(parent, style, 0, true, MenuAction::BackToMainMenu, "Back");
        });
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}
