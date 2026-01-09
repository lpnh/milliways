mod game_selection;
mod loading;
mod pong_game;
mod space_invaders_game;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        game_selection::plugin,
        loading::plugin,
        pong_game::plugin,
        space_invaders_game::plugin,
        splash::plugin,
        title::plugin,
    ));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    MilliwaysSplash,
    Title,
    Loading,
    GameSelection,
    PongGame,
    SpaceInvadersGame,
}
