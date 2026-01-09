use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    Pause,
    menus::Menu,
    screens::Screen,
    space_invaders::{resources::*, spawning::*},
    ui::{PressStart2P, Typography},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::SpaceInvadersGame), spawn_game);
    app.add_systems(
        Update,
        (pause, spawn_pause_overlay, open_pause_menu).run_if(
            in_state(Screen::SpaceInvadersGame)
                .and(in_state(Menu::None))
                .and(input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape))),
        ),
    );
    app.add_systems(
        Update,
        close_menu.run_if(
            in_state(Screen::SpaceInvadersGame)
                .and(not(in_state(Menu::None)))
                .and(input_just_pressed(KeyCode::KeyP)),
        ),
    );
    app.add_systems(OnExit(Screen::SpaceInvadersGame), (close_menu, unpause));
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Screen::SpaceInvadersGame)),
    );
}

fn spawn_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    playfield: Res<PlayfieldDimensions>,
    font: Res<PressStart2P>,
    typography: Res<Typography>,
) {
    let texture = asset_server.load("space_invaders/sprite_sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 12), 2, 5, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    let sheet = SpriteSheetHandle {
        texture,
        layout: layout_handle,
    };

    commands.insert_resource(sheet.clone());

    spawn_player(&mut commands, &playfield, &sheet);
    spawn_invader_formation(&mut commands, &playfield, &sheet);
    spawn_ui(&mut commands, &playfield, &font, &typography);
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(false));
}

fn pause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(true));
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        ZIndex(1),
        DespawnOnExit(Pause(true)),
    ));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
