use bevy::prelude::*;
use bevy_kira_audio::prelude::Decibels;
use bevy_kira_audio::prelude::*;
use rand::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::game::Game;
use crate::frontend::bevy::GameState;
use crate::frontend::visual::dice::DiceRollState;

#[derive(Resource, Default)]
pub struct MusicChannel;

#[derive(Resource, Default)]
pub struct SoundEffectsChannel;

#[derive(Resource)]
pub struct AudioState {
    pub handle: Option<Handle<AudioInstance>>,
    pub volume: f32,
    pub sfx_volume: f32,
}

impl Default for AudioState {
    fn default() -> Self {
        Self {
            handle: None,
            volume: 1.0,
            sfx_volume: 1.0,
        }
    }
}

pub fn play_background_music(
    asset_server: Res<AssetServer>,
    music_channel: Res<AudioChannel<MusicChannel>>,
    mut audio_state: ResMut<AudioState>,
) {
    let tracks = ["audio/background_music0.ogg", "audio/background_music1.ogg"];

    let mut rng = rand::rng();
    let chosen = tracks.choose(&mut rng).unwrap();

    let volume_db = convert_decibel(audio_state.volume);

    //loop the track and keep a handle so volume can be updated
    let handle = music_channel
        .play(asset_server.load(*chosen))
        .looped()
        .with_volume(volume_db)
        .handle();

    audio_state.handle = Some(handle);
}

//update the volume of the currently playing music track
pub fn update_music_volume(
    audio_state: &mut AudioState,
    audio_instances: &mut Assets<AudioInstance>,
    volume: f32,
) {
    audio_state.volume = volume.clamp(0.0, 1.0);

    if let Some(handle) = &audio_state.handle {
        if let Some(instance) = audio_instances.get_mut(handle) {
            //tween the volume change in decibels
            instance.set_decibels(convert_decibel(volume), AudioTween::default());
        }
    }
}

pub fn update_sfx_volume(audio_state: &mut AudioState, volume: f32) {
    audio_state.sfx_volume = volume.clamp(0.0, 1.0);
}

pub fn play_click_sound(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
    sfx_channel: Res<AudioChannel<SoundEffectsChannel>>,
    audio_state: Res<AudioState>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left)
        || mouse_buttons.just_pressed(MouseButton::Right)
    {
        sfx_channel
            .play(asset_server.load("audio/soundeffects/Click.mp3"))
            .with_volume(convert_decibel(0.15 * audio_state.sfx_volume));
    }
}

pub fn play_sound_on_roll(
    dice_state: Res<DiceRollState>,
    asset_server: Res<AssetServer>,
    sfx_channel: Res<AudioChannel<SoundEffectsChannel>>,
    audio_state: Res<AudioState>,
    mut was_rolling: Local<bool>,
) {
    //trigger once at the start of the roll
    if dice_state.rolling && !*was_rolling {
        sfx_channel
            .play(asset_server.load("audio/soundeffects/Dice.wav"))
            .with_volume(convert_decibel(audio_state.sfx_volume));
    }

    *was_rolling = dice_state.rolling;
}

#[derive(Default)]
pub(crate) struct PlacementCounts {
    roads: usize,
    settlements: usize,
    cities: usize,
    initialized: bool,
}

pub(crate) fn play_sound_on_placement(
    game: NonSend<Rc<RefCell<Game>>>,
    state: Res<State<GameState>>,
    asset_server: Res<AssetServer>,
    sfx_channel: Res<AudioChannel<SoundEffectsChannel>>,
    audio_state: Res<AudioState>,
    mut counts: Local<PlacementCounts>,
) {
    if *state.get() != GameState::InGame {
        counts.initialized = false;
        return;
    }

    let game = game.borrow();
    let mut roads = 0;
    let mut settlements = 0;
    let mut cities = 0;

    for player in &game.players {
        roads += player.roads.len();
        settlements += player.settlements.len();
        cities += player.cities.len();
    }

    //only play the sound if we have initialized counts and detect a new placement
    if counts.initialized
        && (roads > counts.roads || settlements > counts.settlements || cities > counts.cities)
    {
        sfx_channel
            .play(asset_server.load("audio/soundeffects/Placing_down.wav"))
            .with_volume(convert_decibel(audio_state.sfx_volume));
    }

    counts.roads = roads;
    counts.settlements = settlements;
    counts.cities = cities;
    counts.initialized = true;
}

//bevy_kira_audio uses decibels for volume control, but we want a simple 0-1 slider -> convert
fn convert_decibel(volume: f32) -> Decibels {
    if volume <= 0.0 {
        (-60.0).into()
    } else {
        (20.0 * volume.clamp(0.0, 1.0).log10()).into()
    }
}
