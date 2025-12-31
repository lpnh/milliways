use std::borrow::Cow;

use bevy::prelude::*;

use crate::ui::{PressStart2P, Typography, palette::*};

pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(20),
            ..default()
        },
        Pickable::IGNORE,
    )
}

pub fn label(text: impl Into<String>, font: &PressStart2P, typography: &Typography) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into()),
        typography.body(&font.0),
        TextColor(TEXT),
    )
}
