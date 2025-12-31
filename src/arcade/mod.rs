pub mod menu;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (menu::handle_menu_navigation, menu::handle_menu_selection),
    );
}
