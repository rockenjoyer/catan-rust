use bevy::prelude::*;
use rand::prelude::*;

pub fn play_background_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let tracks = [
        "audio/background_music0.ogg",
        "audio/background_music1.ogg",
    ];

    let mut rng = rand::rng();
    let chosen = tracks.choose(&mut rng).unwrap();

    let handle = asset_server.load(*chosen);

    commands.spawn((
        AudioPlayer::new(handle),
        PlaybackSettings::LOOP,
    ));
}





