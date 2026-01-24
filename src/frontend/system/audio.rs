use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub fn play_background_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("audio/catanOST1_compressed.mp3"))
        .looped()
        .with_volume(0.5);
}
