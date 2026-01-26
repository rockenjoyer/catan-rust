use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::frontend::bevy::GameState;
use crate::frontend::visual::startscreen::{StartscreenTexture, draw_background};
use crate::frontend::interface::style::apply_style;

#[derive(Resource, Default)]
pub struct MainMenuState {
    pub show_credits: bool,
    pub show_rules: bool,
    pub show_window_settings: bool,
}

//draw the main menu with various buttons
pub fn setup_main_menu(
    mut context: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    texture: Option<Res<StartscreenTexture>>,
    mut menu_state: ResMut<MainMenuState>,
) {
    let Some(texture) = texture else {
        return;
    };

    if let Ok(context) = context.ctx_mut() {
        apply_style(context);
        
        egui::CentralPanel::default().show(context, |ui| {
            //draw the background image
            draw_background(ui, &texture, ui.available_size());

            //draw UI
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                
                //game title
                ui.label(egui::RichText::new("The Settlers of Catan")
                    .font(egui::FontId::proportional(72.0))
                    .color(egui::Color32::from_hex("#845549").unwrap())
                    .strong());
                ui.add_space(-35.0);
                ui.label(egui::RichText::new("Rust Edition")
                    .font(egui::FontId::proportional(40.0))
                    .color(egui::Color32::from_hex("#845549").unwrap()));
                
                ui.add_space(350.0);

                //change button size and apply button style
                let button_size = egui::vec2(300.0, 60.0);
                button_style(ui);
                
                //start game button
                if ui.add_sized(button_size, 
                    egui::Button::new(egui::RichText::new("Start Game").size(20.0))
                ).clicked() {
                    next_state.set(GameState::InGame);
                }
                
                ui.add_space(15.0);

                //rules button
                if ui.add_sized(button_size,
                    egui::Button::new(egui::RichText::new("Rules").size(20.0))
                ).clicked() {
                    menu_state.show_rules = !menu_state.show_rules;
                }
                                
                ui.add_space(15.0);

                //window settings button
                if ui.add_sized(button_size,
                    egui::Button::new(egui::RichText::new("Window Settings").size(20.0))
                ).clicked() {
                    menu_state.show_window_settings = !menu_state.show_window_settings;
                }
                
                ui.add_space(15.0);

                //credits button
                if ui.add_sized(button_size,
                    egui::Button::new(egui::RichText::new("Credits").size(20.0))
                ).clicked() {
                    menu_state.show_credits = !menu_state.show_credits;
                }

                ui.add_space(15.0);
                
                //quit button
                if ui.add_sized(button_size,
                    egui::Button::new(egui::RichText::new("Quit Game").size(20.0))
                ).clicked() {
                    std::process::exit(0);
                }
            });
        });

        //credits panel
        if menu_state.show_credits {
            show_credits(context, &mut menu_state.show_credits);
        }

        //rules panel
        if menu_state.show_rules {
            show_rules(context, &mut menu_state.show_rules);
        }

        //window settings panel
        if menu_state.show_window_settings {
            show_window_settings(context, &mut menu_state.show_window_settings);
        }
    }
}

fn show_credits(ctx: &egui::Context, show: &mut bool) {
    egui::Window::new("Credits")
        .open(show)
        .frame(window_frame())
        .collapsible(false)
        .default_size(egui::vec2(700.0, 300.0))
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                
                ui.label(egui::RichText::new("Development").strong().size(25.0));
                ui.label("Antonio | Roman | Vojin | Laura");
                
                ui.separator();
                
                ui.label(egui::RichText::new("Built with").strong().size(25.0));
                ui.label("Bevy Engine | Rust Programming Language");

                ui.separator();
                
                ui.label(egui::RichText::new("Original Game").strong().size(25.0));
                ui.label("The Settlers of Catan | by Klaus Teuber");
            });
        });
}

fn show_rules(ctx: &egui::Context, show: &mut bool) {
    egui::Window::new("Rules")
        .open(show)
        .frame(window_frame())
        .default_size(egui::vec2(700.0, 300.0))
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("
                        1. Setup
                        - Six types of tiles on the board: Brick, Lumber, Wool, Grain, Ore and Desert.
                        - Number tokens on each tile: 2 - 12, except for desert.
                        - Players place 2 settlements and 2 roads at the start.

                        2. Resources

                        - Produced by tiles when the dice roll matches the tiles' number.
                        - Types: Brick, Lumber, Wool, Grain, Ore.
                        - Cities produce double resources.

                        3. Turn Structure

                        - Roll dice -> distribute resources.
                        - Trade with players or the bank at harbors.
                        - Build:
                            Roads (1 Brick + 1 Lumber).
                            Settlements (1 Brick + 1 Lumber + 1 Grain + 1 Wool).
                            Cities (2 Grain + 3 Ore).
                            Buy Development Cards (1 Wool + 1 Grain + 1 Ore).

                        4. Robber

                        - Moves when 7 is rolled or Knight card is played.
                        - Blocks resource production and steals 1 card from a player adjacent to the tile.
                        - Players with > 7 cards discard half when a 7 is rolled.

                        5. Victory Points

                        - Settlement = 1 VP.
                        - City = 2 VP.
                        - Victory Point card = 1 VP.
                        - Longest Road & Largest Army = 2 VP each.
                        - Goal: First to 10 VP wins.
                ");

            });
        });
}

fn show_window_settings(ctx: &egui::Context, show: &mut bool) {
    egui::Window::new("Window Settings")
        .open(show)
        .frame(window_frame())
        .default_size(egui::vec2(700.0, 300.0))
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Window Settings will be implemented soon.");
            });
        });
}

fn window_frame() -> egui::Frame {
    egui::Frame::NONE
        .fill(egui::Color32::from_hex("#4c2a21").unwrap())
        .stroke(egui::Stroke::new(2.0, egui::Color32::from_hex("#331812").unwrap()))
        .inner_margin(20.0)
        .outer_margin(0.0)        
        .corner_radius(egui::CornerRadius::same(20))
}

fn button_style(ui: &mut egui::Ui) {
    let button_color = egui::Color32::from_hex("#845549ea").unwrap();
    let outline_color = egui::Color32::from_hex("#3e211ae6").unwrap();
    
    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = button_color;
    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = button_color;
    ui.style_mut().visuals.widgets.active.weak_bg_fill = button_color;
    
    ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::new(2.0, outline_color);
    ui.style_mut().visuals.widgets.hovered.bg_stroke = egui::Stroke::new(2.0, outline_color);
    ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke::new(2.0, outline_color);
}