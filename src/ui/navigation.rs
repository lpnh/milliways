use bevy::prelude::*;

use crate::{
    audio::{SoundEffects, sfx},
    menus::Menu,
    screens::Screen,
    ui::{menu::*, palette::*},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            apply_interaction_palette,
            handle_menu_navigation,
            handle_menu_selection,
        ),
    );
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
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
    mut items: Query<(&mut MenuItem, &Children)>,
    mut arrows: Query<&mut Text, With<SelectionArrow>>,
    mut texts: Query<&mut TextColor, With<MenuText>>,
) {
    let mut current_index = None;
    let mut total = 0;

    for (item, _) in items.iter() {
        if item.selected {
            current_index = Some(item.index);
        }
        total += 1;
    }

    let Some(current) = current_index else {
        return;
    };

    if total == 0 {
        return;
    }

    let mut new_index = None;

    if keyboard.just_pressed(KeyCode::ArrowDown)
        || keyboard.just_pressed(KeyCode::KeyJ)
        || keyboard.just_pressed(KeyCode::KeyS)
    {
        new_index = Some((current + 1) % total);
    } else if keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::KeyK)
        || keyboard.just_pressed(KeyCode::KeyW)
    {
        new_index = Some((current + total - 1) % total);
    }

    if let Some(new_idx) = new_index {
        commands.spawn((
            Name::new("Menu Cursor Move SFX"),
            sfx(sfx_handles.menu_move.clone()),
        ));

        for (mut item, children) in &mut items {
            let is_selected = item.index == new_idx;
            item.selected = is_selected;

            for child in children.iter() {
                if let Ok(mut arrow_text) = arrows.get_mut(child) {
                    arrow_text.0 = if is_selected { ">" } else { " " }.to_string();
                }

                if let Ok(mut text_color) = texts.get_mut(child) {
                    text_color.0 = if is_selected { TEXT } else { OVERLAY0 };
                }
            }
        }
    }
}

pub fn handle_menu_selection(
    mut commands: Commands,
    sfx_handles: Res<SoundEffects>,
    keyboard: Res<ButtonInput<KeyCode>>,
    items: Query<(&MenuItem, &MenuAction)>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: MessageWriter<AppExit>,
) {
    if !keyboard.just_pressed(KeyCode::Enter) && !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    for (item, action) in &items {
        if item.selected {
            commands.spawn((
                Name::new("Menu Selection SFX"),
                sfx(sfx_handles.menu_select.clone()),
            ));

            execute_action(
                action,
                &mut next_screen,
                &mut next_menu,
                #[cfg(not(target_family = "wasm"))]
                &mut app_exit,
            );
            break;
        }
    }
}

fn execute_action(
    action: &MenuAction,
    next_screen: &mut ResMut<NextState<Screen>>,
    next_menu: &mut ResMut<NextState<Menu>>,
    #[cfg(not(target_family = "wasm"))] app_exit: &mut MessageWriter<AppExit>,
) {
    match action {
        MenuAction::GoToGameSelection => next_screen.set(Screen::GameSelection),
        MenuAction::GoToTitle => next_screen.set(Screen::Title),
        MenuAction::GoToPongGame => next_screen.set(Screen::PongGame),
        MenuAction::OpenSettings => next_menu.set(Menu::Settings),
        MenuAction::OpenCredits => next_menu.set(Menu::Credits),
        MenuAction::BackToMainMenu => next_menu.set(Menu::Main),
        MenuAction::CloseMenu => next_menu.set(Menu::None),
        MenuAction::RestartGame => {
            next_menu.set(Menu::None);
        }
        MenuAction::StartPongSinglePlayer | MenuAction::StartPongMultiplayer => {}
        #[cfg(not(target_family = "wasm"))]
        MenuAction::ExitApp => {
            app_exit.write(AppExit::Success);
        }
    }
}
