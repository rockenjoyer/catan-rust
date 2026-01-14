use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::game::Game;
use crate::frontend::interface::style::apply_style;

pub fn setup_info(mut context: EguiContexts, game: NonSend<Rc<RefCell<Game>>>,) {
    let game = game.borrow();

    if let Ok(context) = context.ctx_mut() {
        //info window top
        apply_style(context);
        egui::Window::new("The Settlers of Catan")
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_hex("#d4c1b1ff").unwrap())
                    .corner_radius(egui::CornerRadius::same(15)),
            )
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_TOP, (0.0, 0.0))
            .default_size((1300.0, 200.0))
            //display content
            .show(context, |ui| {
                ui.label("Welcome to Catan - safely implemented in Rust! Good luck and have fun!");
            });

        //info window bottom
        egui::Window::new("Current Round Info")
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_hex("#d4c1b1ff").unwrap())
                    .corner_radius(egui::CornerRadius::same(15)),
            )
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_BOTTOM, (0.0, 0.0))
            .default_size((1300.0, 200.0))
            //display content
            .show(context, |ui| {
                let current = &game.players[game.current_player];

                ui.label(format!("Current Player: {} (VP: {})", current.name, current.victory_points));

                ui.label(format!("Resources: "));
                ui.horizontal(|ui| {
                    for (resource, &amount) in &current.resources {
                        if amount > 0 {
                            ui.label(format!("{:?}: {}", resource, amount));
                        }
                    }
                });

                ui.separator();

                ui.label(format!("Settlements: {} | Cities: {} | Roads: {}",
                    current.settlements.len(),
                    current.cities.len(),
                    current.roads.len()
                ));
            });
    }
}
