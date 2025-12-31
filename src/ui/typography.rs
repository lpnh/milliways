use bevy::prelude::*;

#[derive(Resource)]
pub struct PressStart2P(pub Handle<Font>);

impl PressStart2P {
    pub(super) fn new(asset_server: &AssetServer) -> Self {
        Self(asset_server.load("press_start_2p/PressStart2P-Regular.ttf"))
    }
}

#[derive(Resource, Clone)]
pub struct Typography {
    pub title_size: f32,
    pub heading_size: f32,
    pub body_size: f32,
    pub small_size: f32,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            title_size: 48.0,
            heading_size: 32.0,
            body_size: 24.0,
            small_size: 16.0,
        }
    }
}

impl Typography {
    pub fn title(&self, font: &Handle<Font>) -> TextFont {
        TextFont {
            font: font.clone(),
            font_size: self.title_size,
            ..default()
        }
    }

    pub fn heading(&self, font: &Handle<Font>) -> TextFont {
        TextFont {
            font: font.clone(),
            font_size: self.heading_size,
            ..default()
        }
    }

    pub fn body(&self, font: &Handle<Font>) -> TextFont {
        TextFont {
            font: font.clone(),
            font_size: self.body_size,
            ..default()
        }
    }

    pub fn small(&self, font: &Handle<Font>) -> TextFont {
        TextFont {
            font: font.clone(),
            font_size: self.small_size,
            ..default()
        }
    }

    pub fn custom(&self, font: &Handle<Font>, size: f32) -> TextFont {
        TextFont {
            font: font.clone(),
            font_size: size,
            ..default()
        }
    }
}
