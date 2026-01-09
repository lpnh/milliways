use bevy::prelude::*;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GameState {
    pub score: u32,
    pub lives: u32,
    pub game_over: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            score: 0,
            lives: 3,
            game_over: false,
        }
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct InvaderFormation {
    pub direction: f32,
    pub speed: f32,
    pub rank: usize,
    pub vanguard: [usize; 11],
}

impl Default for InvaderFormation {
    fn default() -> Self {
        Self {
            direction: 1.0,
            speed: 54.0,
            rank: 0,
            vanguard: [5; 11],
        }
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct LaserCooldown {
    pub timer: Timer,
}

impl LaserCooldown {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }
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

#[derive(Resource, Clone)]
pub struct SpriteSheetHandle {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}
