use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::frontend::bevy::GameState;
use crate::frontend::interface::style::apply_style;
use crate::frontend::visual::startscreen::{draw_background, LogoTexture, StartscreenTexture};

#[derive(Resource, Default)]
pub struct MainMenuState {
    pub show_rules: bool,
    pub show_multiplayer_settings: bool,
}

//draw the main menu with various buttons
pub fn setup_main_menu(
    mut context: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    background: Option<Res<StartscreenTexture>>,
    logo_image: Option<Res<LogoTexture>>,
    mut menu_state: ResMut<MainMenuState>,
) {
    let Some(background) = background else {
        return;
    };
    let Some(logo_image) = logo_image else {
        return;
    };

    if let Ok(context) = context.ctx_mut() {
        apply_style(context);

        egui::CentralPanel::default().show(context, |ui| {
            //draw the background image
            draw_background(ui, &background, &logo_image, ui.available_size());

            //draw UI
            ui.vertical_centered(|ui| {
                button_style(ui);
                let (button_size, font_size) = scaling(ui);

                //start game button
                if ui
                    .add_sized(
                        button_size,
                        egui::Button::new(egui::RichText::new("Start Game").size(font_size)),
                    )
                    .clicked()
                {
                    next_state.set(GameState::InGame);
                }

                ui.add_space(15.0);

                //multiplayer settings button
                if ui
                    .add_sized(
                        button_size,
                        egui::Button::new(egui::RichText::new("Multiplayer").size(font_size)),
                    )
                    .clicked()
                {
                    menu_state.show_multiplayer_settings = !menu_state.show_multiplayer_settings;
                }

                ui.add_space(15.0);

                //rules button
                if ui
                    .add_sized(
                        button_size,
                        egui::Button::new(egui::RichText::new("Rules").size(font_size)),
                    )
                    .clicked()
                {
                    menu_state.show_rules = !menu_state.show_rules;
                }

                ui.add_space(15.0);

                /* temporary entry point to the endscreen UI
                if ui
                    .add_sized(
                        button_size,
                        egui::Button::new(egui::RichText::new("Endscreen Test").size(font_size)),
                    )
                    .clicked()
                {
                    next_state.set(GameState::EndScreen);
                }
                ui.add_space(15.0);
                */

                //quit button
                if ui
                    .add_sized(
                        button_size,
                        egui::Button::new(egui::RichText::new("Quit Game").size(font_size)),
                    )
                    .clicked()
                {
                    std::process::exit(0);
                }
            });
        });

        //multiplayer settings panel
        if menu_state.show_multiplayer_settings {
            show_multiplayer_settings(context, &mut menu_state.show_multiplayer_settings);
        }

        //rules panel
        if menu_state.show_rules {
            show_rules(context, &mut menu_state.show_rules);
        }
    }
}

fn show_multiplayer_settings(ctx: &egui::Context, show: &mut bool) {
    egui::Window::new("Multiplayer Settings")
        .open(show)
        .collapsible(false)
        .default_size(egui::vec2(700.0, 300.0))
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            window_frame().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("Local...").strong().size(25.0));
                    ui.label("To be implemented");
                });
            });
        });
}

fn show_rules(ctx: &egui::Context, show: &mut bool) {
    egui::Window::new("Rules")
        .open(show)
        .default_size(egui::vec2(700.0, 300.0))
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            window_frame()
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {

                ui.label(egui::RichText::new("1. Setup").strong().size(20.0));
                ui.add_space(10.0);
                ui.label(egui::RichText::new("There are six types of tiles: Brick, Lumber, Wool, Grain, Ore and Desert.")
                    .size(14.0));
                ui.label(egui::RichText::new("Each player starts with 2 roads and 2 settlements.")
                    .size(14.0));

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                ui.label(egui::RichText::new("2. Resources").strong().size(20.0));
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Produced by tiles when the dice roll matches the tiles' number. Cities produce double resources.")
                    .size(14.0));

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                ui.label(egui::RichText::new("3. Turn Structure").strong().size(20.0));
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Roll dice -> distribute resources. Trade with players or the bank at harbors.")
                    .size(14.0));
                ui.label(egui::RichText::new("Roads (1 Brick + 1 Lumber) | Settlements (1 Brick + 1 Lumber + 1 Grain + 1 Wool) | Cities (2 Grain + 3 Ore) | Buy Development Cards (1 Wool + 1 Grain + 1 Ore)")
                    .size(14.0));

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                ui.label(egui::RichText::new("4. Robber").strong().size(20.0));
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Moves when 7 is rolled or Knight card is played.")
                    .size(14.0));
                ui.label(egui::RichText::new("Blocks resource production and steals 1 card from a player adjacent to the tile. Players with > 7 cards discard half.")
                    .size(14.0));
                
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                ui.label(egui::RichText::new("5. Victory Points").strong().size(20.0));
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Settlement = 1 VP | City = 2 VP | VP card = 1 VP | Longest Road & Largest Army = 2 VP")
                    .size(14.0));
                ui.label(egui::RichText::new("First to 10 VP wins.")
                    .size(14.0));

            });
        });
        });
}

fn window_frame() -> egui::Frame {
    egui::Frame::NONE
        .fill(egui::Color32::from_hex("#724235ec").unwrap())
        .stroke(egui::Stroke::new(
            2.0,
            egui::Color32::from_hex("#331812").unwrap(),
        ))
        .inner_margin(20.0)
        .corner_radius(egui::CornerRadius::same(10))
}

fn button_style(ui: &mut egui::Ui) {
    let button_color = egui::Color32::from_hex("#724235ec").unwrap();
    let outline_color = egui::Color32::from_hex("#3e211a").unwrap();
    let selected_color = egui::Color32::from_hex("#724235ec").unwrap();

    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = button_color;
    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = button_color;
    ui.style_mut().visuals.widgets.active.weak_bg_fill = button_color;

    ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::new(3.0, outline_color);
    ui.style_mut().visuals.widgets.hovered.bg_stroke = egui::Stroke::new(3.0, outline_color);
    ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke::new(3.0, outline_color);

    ui.style_mut().visuals.selection.bg_fill = selected_color;
    ui.style_mut().visuals.selection.stroke = egui::Stroke::new(3.0, outline_color);
}

fn scaling(ui: &mut egui::Ui) -> (egui::Vec2, f32) {
    //scale the buttons like the logo
    let available_size = ui.available_size();
    let base_size = egui::vec2(2048.0, 1152.0);

    let scale = (available_size.x / base_size.x)
        .min(available_size.y / base_size.y)
        .clamp(0.2, 1.0);

    //calculate top space for the header logo
    let top_space = (500.0 * scale).min(420.0);

    ui.add_space(top_space);

    //change button width and height based on screen size
    let button_width = (300.0 * scale).clamp(100.0, 340.0);
    let button_height = (80.0 * scale).clamp(25.0, 70.0);

    let button_size = egui::vec2(button_width, button_height);
    let font_size = (20.0 * scale).clamp(12.0, 22.0);
    (button_size, font_size)
}
