mod components;
pub mod mode_selection;
mod resources;
mod spawning;
mod systems;

pub use components::*;
pub use resources::*;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Paddle>()
        .register_type::<PaddleSide>()
        .register_type::<PlayerControlled>()
        .register_type::<AiControlled>()
        .register_type::<Ball>()
        .register_type::<PlayfieldBoundary>()
        .register_type::<CenterDivider>()
        .register_type::<ScoreDisplay>();

    app.register_type::<PongGameMode>()
        .register_type::<GameMode>()
        .register_type::<PlayfieldDimensions>()
        .register_type::<PongPhysics>();

    app.init_resource::<PongGameMode>()
        .init_resource::<PlayfieldDimensions>()
        .init_resource::<PongPhysics>();

    app.add_plugins((mode_selection::plugin, systems::plugin));
}
