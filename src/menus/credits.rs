use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    menus::Menu,
    ui::{PressStart2P, Typography, menu::*, palette::*},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_credits_menu(
    mut commands: Commands,
    font: Res<PressStart2P>,
    typography: Res<Typography>,
) {
    commands
        .spawn((
            Name::new("Credits Menu"),
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
            DespawnOnExit(Menu::Credits),
        ))
        .with_children(|parent| {
            spawn_title(parent, &font, &typography, "Credits");

            parent.spawn((
                Text::new("Assets"),
                typography.body(&font.0),
                TextColor(TEXT),
            ));

            parent
                .spawn((
                    Name::new("Assets Grid"),
                    Node {
                        display: Display::Grid,
                        row_gap: Val::Px(10.0),
                        column_gap: Val::Px(30.0),
                        grid_template_columns: vec![GridTrack::auto(), GridTrack::auto()],
                        ..default()
                    },
                ))
                .with_children(|grid| {
                    let assets = vec![
                        ["Press Start 2P Font", "CC0 by CodeMan38"],
                        [
                            "Bevy logo",
                            "All rights reserved by the Bevy Foundation, permission granted for splash screen use when unmodified",
                        ],
                        ["Arcade Music Loop", "CC BY 4.0 by joshuaempyre"],
                    ];
                    for row in assets {
                        for (j, text) in row.iter().enumerate() {
                            grid.spawn((
                                Text::new(*text),
                                typography.small(&font.0),
                                TextColor(TEXT),
                                Node {
                                    justify_self: if j == 0 {
                                        JustifySelf::End
                                    } else {
                                        JustifySelf::Start
                                    },
                                    ..default()
                                },
                            ));
                        }
                    }
                });

            spawn_menu_item(
                parent,
                &font,
                &typography,
                0,
                true,
                MenuAction::BackToMainMenu,
                "Back",
            );
        });
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}
