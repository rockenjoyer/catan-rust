use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub fn play_background_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("audio/background_music.wav"))
        .looped()
        .with_volume(0.5);
}
