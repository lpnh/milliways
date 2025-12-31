use crate::theme::catppuccin::*;
use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen};

pub struct MenuStyle {
    pub font: Handle<Font>,
    pub title_size: f32,
    pub item_size: f32,
    pub header_size: f32,
    pub text_size: f32,
    pub small_text_size: f32,
    pub arrow_width: f32,
    pub item_gap: f32,
}

impl MenuStyle {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            font: asset_server.load("press_start_2p/PressStart2P-Regular.ttf"),
            title_size: 48.0,
            item_size: 32.0,
            header_size: 24.0,
            text_size: 16.0,
            small_text_size: 12.0,
            arrow_width: 30.0,
            item_gap: 8.0,
        }
    }
}

macro_rules! spawn_title {
    ($parent:expr, $style:expr, $text:expr) => {
        $parent.spawn((
            Text::new($text),
            TextFont {
                font: $style.font.clone(),
                font_size: $style.title_size,
                ..default()
            },
            TextColor(TEXT),
        ))
    };
}

macro_rules! spawn_menu_item {
    ($parent:expr, $style:expr, $index:expr, $selected:expr, $action:expr, $text:expr) => {{
        let font = &$style.font;
        let item_size = $style.item_size;
        let arrow_width = $style.arrow_width;

        $parent
            .spawn((
                MenuItem {
                    index: $index,
                    selected: $selected,
                },
                $action,
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px($style.item_gap),
                    ..default()
                },
            ))
            .with_children(|item| {
                item.spawn((
                    SelectionArrow,
                    Text::new(if $selected { ">" } else { " " }),
                    TextFont {
                        font: font.clone(),
                        font_size: item_size,
                        ..default()
                    },
                    TextColor(TEXT),
                    Node {
                        width: Val::Px(arrow_width),
                        ..default()
                    },
                ));

                item.spawn((
                    MenuText,
                    Text::new($text),
                    TextFont {
                        font: font.clone(),
                        font_size: item_size,
                        ..default()
                    },
                    TextColor(if $selected {
                        TEXT
                    } else {
                        OVERLAY0
                    }),
                ));
            })
    }};
}

pub(crate) use {spawn_menu_item, spawn_title};

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

pub fn handle_menu_navigation(
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
        for (mut item, children) in &mut items {
            let is_selected = item.index == new_idx;
            item.selected = is_selected;

            for child in children.iter() {
                if let Ok(mut arrow_text) = arrows.get_mut(child) {
                    arrow_text.0 = if is_selected { ">" } else { " " }.to_string();
                }

                if let Ok(mut text_color) = texts.get_mut(child) {
                    text_color.0 = if is_selected {
                        TEXT
                    } else {
                        OVERLAY0
                    };
                }
            }
        }
    }
}

pub fn handle_menu_selection(
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
