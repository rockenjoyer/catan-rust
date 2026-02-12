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

        let default_size = (500.0, 70.0);

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

                ui.label(format!(
                    "Current Player: {} (VP: {})",
                    current.name, current.victory_points
                ));

                ui.horizontal(|ui| {
                    ui.label(format!("Resources: "));
                    for (resource, &amount) in &current.resources {
                        if amount > 0 {
                            ui.label(format!("{:?}: {}", resource, amount));
                        }
                    }
                });

                ui.separator();

                ui.label(format!(
                    "Settlements: {} | Cities: {} | Roads: {}",
                    current.settlements.len(),
                    current.cities.len(),
                    current.roads.len()
                ));
            });
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
