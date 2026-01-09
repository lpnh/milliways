use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    menus::Menu,
    ui::{PressStart2P, Typography, menu::*, palette::*},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_settings_menu(
    mut commands: Commands,
    font: Res<PressStart2P>,
    typography: Res<Typography>,
) {
    let mut menu = MenuBuilder::new(&font, &typography);

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
            parent.spawn((
                Text::new("Settings coming soon..."),
                typography.small(&font.0),
                TextColor(TEXT),
            ));

            menu.back_button(parent, MenuAction::BackToMainMenu);
        });
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}
