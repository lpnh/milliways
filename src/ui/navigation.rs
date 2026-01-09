use bevy::prelude::*;

use crate::{
    asset_tracking::ResourceHandles,
    audio::{SoundEffects, sfx},
    menus::Menu,
    pong::PongGameMode,
    screens::Screen,
    ui::{menu::*, palette::*},
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MenuNavigation>();
    app.add_systems(
        Update,
        (
            auto_select_first_item,
            apply_interaction_palette,
            handle_menu_navigation,
            handle_menu_selection,
            update_menu_visuals,
        )
            .chain(),
    );
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn auto_select_first_item(
    mut nav: ResMut<MenuNavigation>,
    items: Query<Entity, (With<MenuItem>, Added<MenuItem>)>,
) {
    if let Some(first) = items.iter().next() {
        nav.selected = Some(first);
    }
}

fn apply_interaction_palette(
    mut palette_query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}

pub fn handle_menu_navigation(
    mut commands: Commands,
    sfx_handles: Res<SoundEffects>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut nav: ResMut<MenuNavigation>,
    items: Query<(Entity, &MenuItem)>,
) {
    let mut items_vec: Vec<(Entity, &MenuItem)> = items.iter().collect();
    items_vec.sort_by_key(|(_, item)| item.index);
    let items_vec: Vec<Entity> = items_vec.into_iter().map(|(entity, _)| entity).collect();

    if items_vec.is_empty() {
        return;
    }

    let Some(selected_entity) = nav.selected else {
        return;
    };

    let Some(current_pos) = items_vec.iter().position(|&e| e == selected_entity) else {
        return;
    };

    let total = items_vec.len();
    let mut new_pos = None;

    if keyboard.just_pressed(KeyCode::ArrowDown)
        || keyboard.just_pressed(KeyCode::KeyJ)
        || keyboard.just_pressed(KeyCode::KeyS)
    {
        new_pos = Some((current_pos + 1) % total);
    } else if keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::KeyK)
        || keyboard.just_pressed(KeyCode::KeyW)
    {
        new_pos = Some((current_pos + total - 1) % total);
    }

    if let Some(new_idx) = new_pos {
        commands.spawn((
            Name::new("Menu Cursor Move SFX"),
            sfx(sfx_handles.menu_move.clone()),
        ));

        nav.selected = Some(items_vec[new_idx]);
    }
}

fn update_menu_visuals(
    nav: Res<MenuNavigation>,
    items: Query<(Entity, &Children), With<MenuItem>>,
    mut arrows: Query<&mut Text, With<SelectionArrow>>,
    mut texts: Query<&mut TextColor, With<MenuText>>,
) {
    if !nav.is_changed() {
        return;
    }

    for (entity, children) in &items {
        let is_selected = nav.selected == Some(entity);

        for &child in children {
            if let Ok(mut arrow_text) = arrows.get_mut(child) {
                arrow_text.0 = if is_selected { ">" } else { " " }.to_string();
            }

            if let Ok(mut text_color) = texts.get_mut(child) {
                text_color.0 = if is_selected { TEXT } else { OVERLAY0 };
            }
        }
    }
}

pub fn handle_menu_selection(
    mut commands: Commands,
    sfx_handles: Res<SoundEffects>,
    keyboard: Res<ButtonInput<KeyCode>>,
    nav: Res<MenuNavigation>,
    items: Query<&MenuItem>,
    resource_handles: Res<ResourceHandles>,
    mut pong_mode: ResMut<PongGameMode>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: MessageWriter<AppExit>,
) {
    if !keyboard.just_pressed(KeyCode::Enter) && !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let Some(item) = nav.selected.and_then(|entity| items.get(entity).ok()) else {
        return;
    };

    commands.spawn((
        Name::new("Menu Selection SFX"),
        sfx(sfx_handles.menu_select.clone()),
    ));

    execute_action(
        &item.action,
        &resource_handles,
        &mut pong_mode,
        &mut next_screen,
        &mut next_menu,
        #[cfg(not(target_family = "wasm"))]
        &mut app_exit,
    );
}

fn execute_action(
    action: &MenuAction,
    resource_handles: &ResourceHandles,
    pong_mode: &mut ResMut<PongGameMode>,
    next_screen: &mut ResMut<NextState<Screen>>,
    next_menu: &mut ResMut<NextState<Menu>>,
    #[cfg(not(target_family = "wasm"))] app_exit: &mut MessageWriter<AppExit>,
) {
    match action {
        MenuAction::GoToGameSelection => {
            if resource_handles.is_all_done() {
                next_screen.set(Screen::GameSelection);
            } else {
                next_screen.set(Screen::Loading);
            }
        }
        MenuAction::GoToTitle => next_screen.set(Screen::Title),
        MenuAction::GoToPongGame => next_screen.set(Screen::PongGame),
        MenuAction::GoToSpaceInvadersGame => next_screen.set(Screen::SpaceInvadersGame),
        MenuAction::OpenSettings => next_menu.set(Menu::Settings),
        MenuAction::OpenCredits => next_menu.set(Menu::Credits),
        MenuAction::BackToMainMenu => next_menu.set(Menu::Main),
        MenuAction::CloseMenu => next_menu.set(Menu::None),
        MenuAction::RestartGame => {
            next_menu.set(Menu::None);
        }
        MenuAction::StartPongSinglePlayer => {
            pong_mode.mode = crate::pong::GameMode::SinglePlayer;
        }
        MenuAction::StartPongMultiplayer => {
            pong_mode.mode = crate::pong::GameMode::Multiplayer;
        }
        #[cfg(not(target_family = "wasm"))]
        MenuAction::ExitApp => {
            app_exit.write(AppExit::Success);
        }
    }
}
