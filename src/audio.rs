use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        apply_global_volume.run_if(resource_changed::<GlobalVolume>),
    );

    app.add_systems(OnEnter(Screen::Splash), spawn_arcade_music);
    app.add_systems(OnEnter(Screen::MilliwaysSplash), spawn_arcade_music);

    app.add_systems(OnEnter(Screen::PongGame), despawn_arcade_music);
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

pub fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::LOOP, Music)
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
