mod components;
pub mod resources;
pub mod spawning;
mod systems;

pub use components::*;
pub use resources::*;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>()
        .register_type::<Invader>()
        .register_type::<InvaderType>()
        .register_type::<Laser>()
        .register_type::<LaserOwner>()
        .register_type::<SpriteAnimation>()
        .register_type::<ScoreDisplay>()
        .register_type::<LivesDisplay>()
        .register_type::<Collider>();

    app.register_type::<GameState>()
        .register_type::<InvaderFormation>()
        .register_type::<LaserCooldown>()
        .register_type::<PlayfieldDimensions>();

    app.init_resource::<GameState>()
        .init_resource::<InvaderFormation>()
        .init_resource::<LaserCooldown>()
        .init_resource::<PlayfieldDimensions>();

    app.add_plugins(systems::plugin);
}
