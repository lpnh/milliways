use crate::theme::catppuccin::*;
use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    input::common_conditions::input_just_pressed,
    prelude::*,
};

use crate::{
    AppSystems,
    arcade::menu::MenuStyle,
    screens::Screen,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(SPLASH_BACKGROUND_COLOR));

    app.add_systems(OnEnter(Screen::Splash), spawn_bevy_splash);
    app.add_systems(
        Update,
        (
            tick_image_fade.in_set(AppSystems::TickTimers),
            apply_image_fade.in_set(AppSystems::Update),
        )
            .run_if(in_state(Screen::Splash)),
    );
    app.add_systems(OnEnter(Screen::Splash), insert_splash_timer);
    app.add_systems(OnExit(Screen::Splash), remove_splash_timer);
    app.add_systems(
        Update,
        (
            tick_splash_timer.in_set(AppSystems::TickTimers),
            check_bevy_splash_timer.in_set(AppSystems::Update),
        )
            .run_if(in_state(Screen::Splash)),
    );
    app.add_systems(
        Update,
        skip_bevy_splash.run_if(input_just_pressed(KeyCode::Escape).and(in_state(Screen::Splash))),
    );

    app.add_systems(OnEnter(Screen::MilliwaysSplash), spawn_milliways_splash);
    app.add_systems(
        Update,
        (
            tick_text_fade.in_set(AppSystems::TickTimers),
            apply_text_fade.in_set(AppSystems::Update),
        )
            .run_if(in_state(Screen::MilliwaysSplash)),
    );
    app.add_systems(OnEnter(Screen::MilliwaysSplash), insert_splash_timer);
    app.add_systems(OnExit(Screen::MilliwaysSplash), remove_splash_timer);
    app.add_systems(
        Update,
        (
            tick_splash_timer.in_set(AppSystems::TickTimers),
            check_milliways_splash_timer.in_set(AppSystems::Update),
        )
            .run_if(in_state(Screen::MilliwaysSplash)),
    );
    app.add_systems(
        Update,
        skip_milliways_splash
            .run_if(input_just_pressed(KeyCode::Escape).and(in_state(Screen::MilliwaysSplash))),
    );
}

const SPLASH_BACKGROUND_COLOR: Color = MANTLE;
const SPLASH_DURATION_SECS: f32 = 4.0;
const SPLASH_FADE_DURATION_SECS: f32 = 0.8;

fn spawn_bevy_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        widget::ui_root("Bevy Splash Screen"),
        BackgroundColor(SPLASH_BACKGROUND_COLOR),
        DespawnOnExit(Screen::Splash),
        children![(
            Name::new("Bevy logo"),
            Node {
                margin: UiRect::all(Val::Auto),
                width: percent(70),
                ..default()
            },
            ImageNode::new(asset_server.load_with_settings(
                "images/splash.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::linear();
                },
            )),
            ImageNodeFadeInOut {
                total_duration: SPLASH_DURATION_SECS,
                fade_duration: SPLASH_FADE_DURATION_SECS,
                t: 0.0,
            },
        )],
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ImageNodeFadeInOut {
    total_duration: f32,
    fade_duration: f32,
    t: f32,
}

impl ImageNodeFadeInOut {
    fn alpha(&self) -> f32 {
        let t = (self.t / self.total_duration).clamp(0.0, 1.0);
        let fade = self.fade_duration / self.total_duration;
        ((1.0 - (2.0 * t - 1.0).abs()) / fade).min(1.0)
    }
}

fn tick_image_fade(time: Res<Time>, mut query: Query<&mut ImageNodeFadeInOut>) {
    for mut anim in &mut query {
        anim.t += time.delta_secs();
    }
}

fn apply_image_fade(mut query: Query<(&ImageNodeFadeInOut, &mut ImageNode)>) {
    for (anim, mut image) in &mut query {
        image.color.set_alpha(anim.alpha());
    }
}

fn check_bevy_splash_timer(timer: Res<SplashTimer>, mut next_screen: ResMut<NextState<Screen>>) {
    if timer.0.just_finished() {
        next_screen.set(Screen::MilliwaysSplash);
    }
}

fn skip_bevy_splash(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::MilliwaysSplash);
}

fn spawn_milliways_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    let style = MenuStyle::new(&asset_server);

    let char_colors = [
        RED,
        MAROON,
        PEACH,
        YELLOW,
        GREEN,
        TEAL,
        SKY,
        SAPPHIRE,
        BLUE,
    ];

    let root = commands
        .spawn((
            Name::new("Milliways Splash Screen"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(SPLASH_BACKGROUND_COLOR),
            DespawnOnExit(Screen::MilliwaysSplash),
        ))
        .id();

    let container = commands
        .spawn((Name::new("Milliways Title Container"), Node::default()))
        .id();

    commands.entity(root).add_child(container);

    for (ch, &color) in "milliways".chars().zip(char_colors.iter()) {
        let letter = commands
            .spawn((
                Name::new(format!("Letter '{}'", ch)),
                Text::new(ch.to_string()),
                TextFont {
                    font: style.font.clone(),
                    font_size: 64.0,
                    ..default()
                },
                TextColor(color),
                TextFadeInOut {
                    total_duration: SPLASH_DURATION_SECS,
                    fade_duration: SPLASH_FADE_DURATION_SECS,
                    t: 0.0,
                },
            ))
            .id();

        commands.entity(container).add_child(letter);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct TextFadeInOut {
    total_duration: f32,
    fade_duration: f32,
    t: f32,
}

impl TextFadeInOut {
    fn alpha(&self) -> f32 {
        let t = (self.t / self.total_duration).clamp(0.0, 1.0);
        let fade = self.fade_duration / self.total_duration;
        ((1.0 - (2.0 * t - 1.0).abs()) / fade).min(1.0)
    }
}

fn tick_text_fade(time: Res<Time>, mut query: Query<&mut TextFadeInOut>) {
    for mut anim in &mut query {
        anim.t += time.delta_secs();
    }
}

fn apply_text_fade(mut query: Query<(&TextFadeInOut, &mut TextColor)>) {
    for (anim, mut text_color) in &mut query {
        text_color.0.set_alpha(anim.alpha());
    }
}

fn check_milliways_splash_timer(
    timer: Res<SplashTimer>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if timer.0.just_finished() {
        next_screen.set(Screen::Title);
    }
}

fn skip_milliways_splash(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

#[derive(Resource, Debug, Clone, PartialEq, Reflect)]
#[reflect(Resource)]
struct SplashTimer(Timer);

impl Default for SplashTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SPLASH_DURATION_SECS, TimerMode::Once))
    }
}

fn insert_splash_timer(mut commands: Commands) {
    commands.init_resource::<SplashTimer>();
}

fn remove_splash_timer(mut commands: Commands) {
    commands.remove_resource::<SplashTimer>();
}

fn tick_splash_timer(time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    timer.0.tick(time.delta());
}
