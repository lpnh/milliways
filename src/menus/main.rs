use crate::theme::catppuccin::*;
use bevy::prelude::*;

use crate::{
    arcade::menu::*, asset_tracking::ResourceHandles, menus::Menu, screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_menu);
    app.add_systems(Update, handle_selection.run_if(in_state(Menu::Main)));
}

fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let style = MenuStyle::new(&asset_server);

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
            spawn_title!(parent, style, "milliways");
            spawn_menu_item!(
                parent,
                style,
                0,
                true,
                MenuAction::GoToGameSelection,
                "Play"
            );
            spawn_menu_item!(
                parent,
                style,
                1,
                false,
                MenuAction::OpenSettings,
                "Settings"
            );
            spawn_menu_item!(parent, style, 2, false, MenuAction::OpenCredits, "Credits");

            #[cfg(not(target_family = "wasm"))]
            spawn_menu_item!(parent, style, 3, false, MenuAction::ExitApp, "Exit");
        });
}

fn handle_selection(
    keyboard: Res<ButtonInput<KeyCode>>,
    items: Query<(&MenuItem, &MenuAction)>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: MessageWriter<AppExit>,
) {
    if !keyboard.just_pressed(KeyCode::Enter) && !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    for (item, action) in &items {
        if item.selected {
            match action {
                MenuAction::GoToGameSelection => {
                    if resource_handles.is_all_done() {
                        next_screen.set(Screen::GameSelection);
                    } else {
                        next_screen.set(Screen::Loading);
                    }
                }
                MenuAction::OpenSettings => next_menu.set(Menu::Settings),
                MenuAction::OpenCredits => next_menu.set(Menu::Credits),
                #[cfg(not(target_family = "wasm"))]
                MenuAction::ExitApp => {
                    app_exit.write(AppExit::Success);
                }
                _ => {}
            }
            break;
        }
    }
}
