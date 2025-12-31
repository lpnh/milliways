pub mod menu;
pub mod navigation;
pub mod palette;
pub mod typography;
pub mod widget;

pub use typography::{PressStart2P, Typography};

pub mod prelude {
    pub use super::widget;
}

use bevy::prelude::*;

fn load_typography(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(typography::PressStart2P::new(&asset_server));
    commands.insert_resource(Typography::default());
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(navigation::plugin)
        .add_systems(Startup, load_typography);
}
