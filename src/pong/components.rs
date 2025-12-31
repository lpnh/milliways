use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Paddle {
    pub score: u32,
    pub speed: f32,
}

impl Default for Paddle {
    fn default() -> Self {
        Self {
            score: 0,
            speed: 300.0,
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum PaddleSide {
    Left,
    Right,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerControlled {
    pub up_key: KeyCode,
    pub down_key: KeyCode,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AiControlled {
    pub last_sleep: f32,
    pub sleeping: bool,
    pub speed: f32,
}

impl Default for AiControlled {
    fn default() -> Self {
        Self {
            last_sleep: 0.0,
            sleeping: false,
            speed: 300.0,
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Ball {
    pub velocity: Vec2,
    pub size: f32,
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            velocity: Vec2::new(300.0, 300.0),
            size: 10.0,
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayfieldBoundary;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CenterDivider;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScoreDisplay {
    pub side: PaddleSide,
}
