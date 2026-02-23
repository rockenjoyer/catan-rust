use crate::backend::game::Game;
use crate::frontend::interface::style::apply_style;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::cell::RefCell;
use std::rc::Rc;

pub fn setup_info(mut context: EguiContexts, game: NonSend<Rc<RefCell<Game>>>) {
    let game = game.borrow();

    if let Ok(context) = context.ctx_mut() {
        apply_style(context);

        let default_size = (550.0, 70.0);

        //info window
        egui::Window::new("Current Round Info")
            .frame(window_frame())
            .order(egui::Order::Foreground)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_TOP, (0.0, 10.0))
            .default_size(default_size)
            .show(context, |ui| {
                let current = &game.players[game.current_player];
                let current_color = player_color(current.id);

                ui.horizontal(|ui| {
                    ui.label("  Current Player:");
                    ui.colored_label(current_color, &current.name);
                    ui.label(format!("(VP: {}), ", current.victory_points));

                    ui.label(format!(
                                    "Settlements: {} | Cities: {} | Roads: {}  ",
                                    current.settlements.len(),
                                    current.cities.len(),
                                    current.roads.len()
                                ));
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label("  VP Overview:");
                    for player in &game.players {
                        let color = player_color(player.id);
                        ui.colored_label(color, &player.name);
                        ui.label(format!(": {} ", player.victory_points));
                        ui.add_space(10.0);
                    }
                });

            });
    }
}

fn player_color(player_id: usize) -> egui::Color32 {
    match player_id {
        0 => egui::Color32::from_rgb(200, 50, 50),   //red
        1 => egui::Color32::from_rgb(50, 100, 200),  //blue
        2 => egui::Color32::from_rgb(50, 200, 50),   //green
        3 => egui::Color32::from_rgb(220, 200, 50),  //yellow
        _ => egui::Color32::WHITE, //default
    }
}

fn window_frame() -> egui::Frame {
    egui::Frame::NONE
        .fill(egui::Color32::from_black_alpha(150))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_white_alpha(100)))
        .inner_margin(10.0)
        .outer_margin(0.0)
        .corner_radius(egui::CornerRadius::same(15))
}
