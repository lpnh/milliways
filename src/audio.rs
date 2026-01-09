use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, load_sound_effects);

    app.add_systems(
        Update,
        apply_global_volume.run_if(resource_changed::<GlobalVolume>),
    );

    app.add_systems(OnEnter(Screen::Splash), spawn_arcade_music);
    app.add_systems(OnEnter(Screen::MilliwaysSplash), spawn_arcade_music);

    app.add_systems(OnEnter(Screen::PongGame), despawn_arcade_music);
    app.add_systems(OnEnter(Screen::SpaceInvadersGame), despawn_arcade_music);
}

fn load_sound_effects(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SoundEffects::new(&asset_server));
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

pub fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::LOOP, Music)
}

pub fn sfx(handle: Handle<AudioSource>) -> impl Bundle {
    AudioPlayer(handle)
}

fn apply_global_volume(
    global_volume: Res<GlobalVolume>,
    mut audio_query: Query<(&PlaybackSettings, &mut AudioSink)>,
) {
    for (playback, mut sink) in &mut audio_query {
        sink.set_volume(global_volume.volume * playback.volume);
    }
}

#[derive(Component)]
struct ArcadeMusic;

fn spawn_arcade_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    music_query: Query<(), With<ArcadeMusic>>,
) {
    if music_query.is_empty() {
        commands.spawn((
            Name::new("Arcade Background Music"),
            music(asset_server.load("audio/joshuaempyre_arcade-music-loop.wav")),
            ArcadeMusic,
        ));
    }
}

fn despawn_arcade_music(mut commands: Commands, music_query: Query<Entity, With<ArcadeMusic>>) {
    for entity in &music_query {
        commands.entity(entity).despawn();
    }
}

#[derive(Resource)]
pub struct SoundEffects {
    pub menu_move: Handle<AudioSource>,
    pub menu_select: Handle<AudioSource>,
    pub ball_collision: Handle<AudioSource>,
    pub score: Handle<AudioSource>,
}

impl SoundEffects {
    pub(super) fn new(asset_server: &AssetServer) -> Self {
        Self {
            menu_move: asset_server
                .load("audio/sound_effects/plasterbrain_arcade-ui-move-cursor.flac"),
            menu_select: asset_server.load("audio/sound_effects/plasterbrain_game-start.ogg"),
            ball_collision: asset_server
                .load("audio/sound_effects/plasterbrain_minimalist-sci-fi-ui-cancel.flac"),
            score: asset_server.load("audio/sound_effects/littlerobotsoundfactory_hit_01.wav"),
        }
    }
}
