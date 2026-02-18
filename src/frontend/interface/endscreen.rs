use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::game::{Game, Resource};
use crate::frontend::bevy::GameState;
use crate::frontend::interface::style::apply_style;
use crate::frontend::visual::startscreen::{draw_background, LogoTexture, StartscreenTexture};

#[derive(Resource, Default)]
pub struct EndscreenState {
    pub show_credits: bool,
    pub show_stats: bool,
    //winner and stats snapshot captured at game end
    pub winner_id: Option<usize>,
    pub stats: Vec<PlayerStats>,
    pub longest_road_owner: Option<usize>,
    pub largest_army_owner: Option<usize>,
}

impl EndscreenState {
    fn reset(&mut self) {
        //reset endscreen UI state when returning to main menu
        self.show_credits = false;
        self.show_stats = false;
        self.winner_id = None;
        self.stats.clear();
        self.longest_road_owner = None;
        self.largest_army_owner = None;
    }
}

#[derive(Clone)]
pub struct PlayerStats {
    pub id: usize,
    pub name: String,
    pub victory_points: u8,
    pub settlements: usize,
    pub cities: usize,
    pub roads: usize,
    pub resources: std::collections::HashMap<Resource, u8>,
}

pub fn setup_endscreen(
    mut context: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    background: Option<Res<StartscreenTexture>>,
    logo_image: Option<Res<LogoTexture>>,
    mut menu_state: ResMut<EndscreenState>,
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

                //return to main menu button
                if ui
                    .add_sized(
                        button_size,
                        egui::Button::new(
                            egui::RichText::new("Return to Main Menu").size(font_size),
                        ),
                    )
                    .clicked()
                {
                    //clear the endscreen state and return to main menu
                    menu_state.reset();
                    next_state.set(GameState::MainMenu);
                }

                ui.add_space(15.0);

                //stats button
                if ui
                    .add_sized(
                        button_size,
                        egui::Button::new(egui::RichText::new("Stats").size(font_size)),
                    )
                    .clicked()
                {
                    menu_state.show_stats = !menu_state.show_stats;
                }

                ui.add_space(15.0);

                //credits button
                if ui
                    .add_sized(
                        button_size,
                        egui::Button::new(egui::RichText::new("Credits").size(font_size)),
                    )
                    .clicked()
                {
                    menu_state.show_credits = !menu_state.show_credits;
                }

                ui.add_space(15.0);

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

        let winner_id = menu_state.winner_id;
        let longest_road_owner = menu_state.longest_road_owner;
        let largest_army_owner = menu_state.largest_army_owner;

        if menu_state.show_stats {
            let mut show_stats_flag = menu_state.show_stats;
            show_stats(
                context,
                &mut show_stats_flag,
                winner_id,
                &menu_state.stats,
                longest_road_owner,
                largest_army_owner,
            );
            menu_state.show_stats = show_stats_flag;
        }

        if menu_state.show_credits {
            let mut show_credits_flag = menu_state.show_credits;
            show_credits(context, &mut show_credits_flag);
            menu_state.show_credits = show_credits_flag;
        }
    }
}

fn show_stats(
    ctx: &egui::Context,
    show: &mut bool,
    winner_id: Option<usize>,
    stats: &[PlayerStats],
    longest_road_owner: Option<usize>,
    largest_army_owner: Option<usize>,
) {
    //show the game stats with each player's progress
    egui::Window::new("Game Stats")
        .open(show)
        .collapsible(false)
        .default_size(egui::vec2(700.0, 300.0))
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            window_frame().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("Winner").strong().size(20.0));
                    let winner_name = player_name(stats, winner_id).unwrap_or("No winner yet");
                    ui.label(egui::RichText::new(winner_name).size(18.0));
                    ui.add_space(10.0);

                    ui.separator();
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Game Stats").strong().size(20.0));
                    ui.add_space(10.0);

                    egui::Grid::new("stats_grid")
                        .striped(true)
                        .spacing(egui::vec2(20.0, 10.0))
                        .show(ui, |ui| {
                            ui.label("");
                            ui.label("Player");
                            ui.label("VP");
                            ui.label("Settlements");
                            ui.label("Cities");
                            ui.label("Roads");
                            ui.label("Resources");
                            ui.end_row();

                            for player in stats {
                                ui.label("");
                                ui.label(&player.name);
                                ui.label(player.victory_points.to_string());
                                ui.label(player.settlements.to_string());
                                ui.label(player.cities.to_string());
                                ui.label(player.roads.to_string());

                                let resources_text = format_resources(&player.resources);
                                ui.label(resources_text);
                                ui.end_row();
                            }
                        });

                    ui.add_space(10.0);

                    let longest_road = player_name(stats, longest_road_owner).unwrap_or("None");
                    let largest_army = player_name(stats, largest_army_owner).unwrap_or("None");

                    ui.separator();
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Achievements").strong().size(20.0));
                    ui.add_space(10.0);
                    ui.label(format!("Longest Road: {}", longest_road));
                    ui.label(format!("Largest Army: {}", largest_army));
                });
            });
        });
}

fn show_credits(ctx: &egui::Context, show: &mut bool) {
    //show credits
    egui::Window::new("Credits")
        .open(show)
        .collapsible(false)
        .default_size(egui::vec2(700.0, 300.0))
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            window_frame().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("Development").strong().size(25.0));
                    ui.label("Antonio | Roman | Vojin | Laura");

                    ui.add_space(15.0);

                    ui.label(egui::RichText::new("Built with").strong().size(25.0));
                    ui.label("Bevy Engine | Rust Programming Language");

                    ui.add_space(15.0);

                    ui.label(egui::RichText::new("Original Game").strong().size(25.0));
                    ui.label("The Settlers of Catan | by Klaus Teuber");
                });
            });
        });
}

pub fn check_for_endgame(
    game: NonSend<Rc<RefCell<Game>>>,
    mut endscreen_state: ResMut<EndscreenState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (winner_id, stats, longest_road_owner, largest_army_owner) = {
        let game = game.borrow();
        let winner_id = game
            .players
            .iter()
            .find(|player| player.victory_points >= 10)
            .map(|player| player.id);

        //retrieve player stats to show on the endscreen
        let stats = game
            .players
            .iter()
            .map(|player| PlayerStats {
                id: player.id,
                name: player.name.clone(),
                victory_points: player.victory_points,
                settlements: player.settlements.len(),
                cities: player.cities.len(),
                roads: player.roads.len(),
                resources: player.resources.clone(),
            })
            .collect();

        (
            winner_id,
            stats,
            game.longest_road_owner,
            game.largest_army_owner,
        )
    };

    //keep the endscreen stats in sync with the current game state
    endscreen_state.stats = stats;
    endscreen_state.longest_road_owner = longest_road_owner;
    endscreen_state.largest_army_owner = largest_army_owner;

    if endscreen_state.winner_id.is_none() {
        if let Some(winner_id) = winner_id {
            endscreen_state.winner_id = Some(winner_id);
            next_state.set(GameState::EndScreen);
        }
    }
}

fn format_resources(resources: &std::collections::HashMap<Resource, u8>) -> String {
    //resource ordering for display rows
    let order = [
        Resource::Brick,
        Resource::Lumber,
        Resource::Wool,
        Resource::Grain,
        Resource::Ore,
    ];

    order
        .iter()
        .map(|res| {
            let amount = resources.get(res).copied().unwrap_or(0);
            format!("{:?}: {}", res, amount)
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn player_name(stats: &[PlayerStats], player_id: Option<usize>) -> Option<&str> {
    let player_id = player_id?;
    stats
        .iter()
        .find(|player| player.id == player_id)
        .map(|player| player.name.as_str())
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
    let top_space = (550.0 * scale).min(420.0);

    ui.add_space(top_space);

    //change button width and height based on screen size
    let button_width = (300.0 * scale).clamp(100.0, 340.0);
    let button_height = (80.0 * scale).clamp(25.0, 70.0);

    let button_size = egui::vec2(button_width, button_height);
    let font_size = (20.0 * scale).clamp(12.0, 22.0);
    (button_size, font_size)
}
