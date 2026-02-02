use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::prelude::*;

#[derive(Resource, Default)]
pub struct AudioState {
    pub is_muted: bool,
    pub handle: Option<Handle<AudioInstance>>,
}

pub fn play_background_music(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut audio_state: ResMut<AudioState>,
) {
    let tracks = [
        "audio/background_music0.ogg",
        "audio/background_music1.ogg",
    ];

    let mut rng = rand::rng();
    let chosen = tracks.choose(&mut rng).unwrap();

    let handle = audio
        .play(asset_server.load(*chosen))
        .looped()
        .with_volume(0.5)
        .handle();

    audio_state.handle = Some(handle);
}

pub fn toggle_music(
    audio_state: &mut AudioState,
    audio_instances: &mut Assets<AudioInstance>,
) {
    audio_state.is_muted = !audio_state.is_muted;

    if let Some(handle) = &audio_state.handle {
        if let Some(instance) = audio_instances.get_mut(handle) {
            if audio_state.is_muted {
                instance.pause(AudioTween::default());
            } else {
                instance.resume(AudioTween::default());
            }
        }
    }
}




