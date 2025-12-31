use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    menus::Menu,
    ui::{PressStart2P, Typography, menu::*},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_menu(mut commands: Commands, font: Res<PressStart2P>, typography: Res<Typography>) {
    let menu = MenuBuilder::new(&font, &typography);

    commands
        .spawn((
            Name::new("Pause Menu"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(40.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            GlobalZIndex(2),
            DespawnOnExit(Menu::Pause),
        ))
        .with_children(|parent| {
            menu.title(parent, "Game Paused");
            menu.item(parent, MenuAction::CloseMenu, "Resume");
            menu.item(parent, MenuAction::RestartGame, "Restart");
            menu.item(parent, MenuAction::GoToTitle, "Quit to Title");
        });
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
