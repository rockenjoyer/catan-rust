use crate::frontend::bevy::GameState;
use crate::frontend::interface::style::apply_style;
use crate::frontend::system::audio::{update_music_volume, update_sfx_volume, AudioState};
use bevy::prelude::*;
use bevy::window::{MonitorSelection, PrimaryWindow, VideoModeSelection, WindowMode};
use bevy_egui::{egui, EguiContexts};
use bevy_kira_audio::AudioInstance;

#[derive(Resource, Default)]
pub struct SettingsPanelState {
    pub open: bool,
}

pub fn setup_settings(
    mut context: EguiContexts,
    mut audio_state: ResMut<AudioState>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut settings_state: ResMut<SettingsPanelState>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(context) = context.ctx_mut() {
        apply_style(context);

        //settings button anchored to the bottom left corner
        egui::Window::new("settings_icon")
            .frame(egui::Frame::NONE)
            .title_bar(false)
            .order(egui::Order::Foreground)
            .movable(false)
            .resizable(false)
            .anchor(egui::Align2::LEFT_BOTTOM, (10.0, -10.0))
            .show(context, |ui| {
                button_style(ui, current_state.get());

                let button_size = egui::vec2(40.0, 40.0);
                if ui
                    .add_sized(
                        button_size,
                        egui::Button::new(egui::RichText::new("🔧").size(20.0)),
                    )
                    .clicked()
                {
                    settings_state.open = true;
                }
            });

        if settings_state.open {
            egui::Window::new("Settings")
                .open(&mut settings_state.open)
                .collapsible(false)
                .resizable(false)
                .default_size(egui::vec2(700.0, 400.0))
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(context, |ui| {
                    window_frame(current_state.get()).show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            button_style(ui, current_state.get());

                            ui.separator();

                            ui.add_space(15.0);
                            //slider controls the music channel volume
                            let volume = audio_state.volume.clamp(0.0, 1.0);
                            let mut slider_value = volume;
                            let slider = egui::Slider::new(&mut slider_value, 0.0..=1.0)
                                .text("Music Volume")
                                .show_value(true);

                            if ui.add(slider).changed() {
                                update_music_volume(
                                    &mut audio_state,
                                    &mut audio_instances,
                                    slider_value,
                                );
                            }

                            ui.add_space(10.0);
                            //slider controls the sound effects channel volume
                            let sfx_volume = audio_state.sfx_volume.clamp(0.0, 1.0);
                            let mut sfx_slider_value = sfx_volume;
                            let sfx_slider = egui::Slider::new(&mut sfx_slider_value, 0.0..=1.0)
                                .text("SFX Volume")
                                .show_value(true);

                            if ui.add(sfx_slider).changed() {
                                update_sfx_volume(&mut audio_state, sfx_slider_value);
                            }

                            if current_state.get() == &GameState::InGame {
                                ui.add_space(15.0);
                                if ui.button("🏠 Return to Main Menu").clicked() {
                                    next_state.set(GameState::MainMenu);
                                }
                            }
                            ui.add_space(15.0);
                            ui.separator();
                            ui.add_space(15.0);

                            if let Ok(mut window) = window_query.single_mut() {
                                ui.label(egui::RichText::new("Window Mode").strong().size(24.0));
                                ui.add_space(10.0);

                                ui.horizontal(|ui| {
                                    ui.add_space(35.0);
                                    let current_mode = window.mode;

                                    if ui
                                        .selectable_label(
                                            matches!(current_mode, WindowMode::Windowed),
                                            egui::RichText::new("Windowed").size(20.0),
                                        )
                                        .clicked()
                                    {
                                        window.mode = WindowMode::Windowed;
                                    }

                                    ui.add_space(20.0);

                                    if ui
                                        .selectable_label(
                                            matches!(
                                                current_mode,
                                                WindowMode::BorderlessFullscreen(_)
                                            ),
                                            egui::RichText::new("Borderless Fullscreen").size(20.0),
                                        )
                                        .clicked()
                                    {
                                        window.mode = WindowMode::BorderlessFullscreen(
                                            MonitorSelection::Current,
                                        );
                                    }

                                    ui.add_space(20.0);

                                    if ui
                                        .selectable_label(
                                            matches!(current_mode, WindowMode::Fullscreen(_, _)),
                                            egui::RichText::new("Fullscreen").size(20.0),
                                        )
                                        .clicked()
                                    {
                                        window.mode = WindowMode::Fullscreen(
                                            MonitorSelection::Current,
                                            VideoModeSelection::Current,
                                        );
                                    }
                                });

                                ui.add_space(15.0);
                                ui.separator();
                                ui.add_space(15.0);

                                ui.label(
                                    egui::RichText::new("Window Resolution").strong().size(24.0),
                                );
                                ui.add_space(10.0);

                                let resolutions = [
                                    ("1280 x 720", 1280.0, 720.0),
                                    ("1920 x 1080", 1920.0, 1080.0),
                                    ("2560 x 1440", 2560.0, 1440.0),
                                ];

                                for (label, width, height) in resolutions {
                                    ui.horizontal(|ui| {
                                        ui.add_space(275.0);
                                        if ui
                                            .button(egui::RichText::new(label).size(18.0))
                                            .clicked()
                                        {
                                            window.resolution.set(width, height);
                                        }
                                    });
                                    ui.add_space(8.0);
                                }

                                ui.add_space(12.0);

                                ui.separator();
                                ui.add_space(15.0);

                                ui.label(
                                    egui::RichText::new(format!(
                                        "Current: {} x {}",
                                        window.resolution.width(),
                                        window.resolution.height()
                                    ))
                                    .size(16.0),
                                );
                            }
                        });
                    });
                });
        }
    }
}

fn window_frame(state: &GameState) -> egui::Frame {
    let (fill_color, stroke_color) = match state {
        GameState::MainMenu | GameState::EndScreen => (
            egui::Color32::from_hex("#724235ec").unwrap(),
            egui::Color32::from_hex("#331812").unwrap(),
        ),
        _ => (
            egui::Color32::from_hex("#4b798ae5").unwrap(),
            egui::Color32::from_hex("#0f252de5").unwrap(),
        ),
    };

    egui::Frame::NONE
        .fill(fill_color)
        .stroke(egui::Stroke::new(2.0, stroke_color))
        .inner_margin(20.0)
        .corner_radius(egui::CornerRadius::same(10))
}

fn button_style(ui: &mut egui::Ui, state: &GameState) {
    let (button_color, outline_color, selected_color) = match state {
        GameState::MainMenu | GameState::EndScreen => (
            egui::Color32::from_hex("#724235ec").unwrap(),
            egui::Color32::from_hex("#3e211a").unwrap(),
            egui::Color32::from_hex("#724235ec").unwrap(),
        ),
        _ => (
            egui::Color32::from_hex("#4b798ae5").unwrap(),
            egui::Color32::from_hex("#0f252d60").unwrap(),
            egui::Color32::from_hex("#4b798ae5").unwrap(),
        ),
    };

    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = button_color;
    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = button_color;
    ui.style_mut().visuals.widgets.active.weak_bg_fill = button_color;

    ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::new(3.0, outline_color);
    ui.style_mut().visuals.widgets.hovered.bg_stroke = egui::Stroke::new(3.0, outline_color);
    ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke::new(3.0, outline_color);

    ui.style_mut().visuals.selection.bg_fill = selected_color;
    ui.style_mut().visuals.selection.stroke = egui::Stroke::new(3.0, outline_color);
}
