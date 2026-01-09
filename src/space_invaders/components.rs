use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Invader {
    pub invader_type: InvaderType,
    pub row: usize,
    pub column: usize,
    pub initial_x: f32,
    pub initial_y: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Reflect)]
pub enum InvaderType {
    A,
    B,
    C,
}

impl InvaderType {
    pub fn points(self) -> u32 {
        match self {
            InvaderType::A => 30,
            InvaderType::B => 20,
            InvaderType::C => 10,
        }
    }

    pub fn sprite_indices(self) -> [usize; 2] {
        match self {
            InvaderType::A => [0, 1],
            InvaderType::B => [2, 3],
            InvaderType::C => [4, 5],
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Laser {
    pub velocity: f32,
    pub owner: LaserOwner,
}

#[derive(Clone, Copy, PartialEq, Eq, Reflect)]
pub enum LaserOwner {
    Player,
    Invader,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SpriteAnimation {
    pub timer: Timer,
    pub frame: usize,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScoreDisplay;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LivesDisplay;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Collider {
    pub width: f32,
    pub height: f32,
}
