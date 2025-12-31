use bevy::prelude::*;

#[derive(Resource, Clone, Copy, Reflect)]
#[reflect(Resource)]
pub struct PongGameMode {
    pub mode: GameMode,
}

impl Default for PongGameMode {
    fn default() -> Self {
        Self {
            mode: GameMode::SinglePlayer,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Reflect)]
pub enum GameMode {
    SinglePlayer,
    Multiplayer,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct PlayfieldDimensions {
    pub width: f32,
    pub height: f32,
}

impl Default for PlayfieldDimensions {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
        }
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct PongPhysics {
    pub ball_initial_speed_x: f32,
    pub ball_initial_speed_y: f32,
    pub paddle_speed: f32,
    pub paddle_width: f32,
    pub paddle_height: f32,
}

impl Default for PongPhysics {
    fn default() -> Self {
        Self {
            ball_initial_speed_x: 300.0,
            ball_initial_speed_y: 300.0,
            paddle_speed: 300.0,
            paddle_width: 20.0,
            paddle_height: 100.0,
        }
    }
}
