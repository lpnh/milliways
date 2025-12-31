pub mod catppuccin;
pub mod interaction;
pub mod widget;

pub mod prelude {
    pub use super::widget;
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}
