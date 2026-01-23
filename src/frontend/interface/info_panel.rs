use crate::backend::game::Game;
use crate::frontend::interface::style::apply_style;
use crate::frontend::visual::banner::{BannerTextures, draw_viewer_banner_background};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use std::cell::RefCell;
use std::rc::Rc;

pub fn setup_info(mut context: EguiContexts, game: NonSend<Rc<RefCell<Game>>>, textures: Option<Res<BannerTextures>>) {
    let game = game.borrow();

    if let Ok(context) = context.ctx_mut() {
        apply_style(context);

        let default_size = (500.0, 70.0);

        //load viewer banner texture
        let viewer_banner = if let Some(ref textures) = textures {
            Some(&textures.viewer_banner)
        } else {
            None
        };

        //info window top
        egui::Window::new("The Settlers of Catan")
            .frame(egui::Frame::NONE)
            .title_bar(false)
            .order(egui::Order::Foreground)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_TOP, (0.0, 0.0))
            .default_size(default_size)
            .show(context, |ui| {
                if let Some(texture) = viewer_banner {
                    draw_viewer_banner_background(ui, texture);
                }

                //add the content on top of the banner
                ui.vertical_centered(|ui| {
                    ui.add_space(5.0);
                    ui.heading("The Settlers of Catan");
                    ui.label("A Rust implementation of the classic board game");
                });
            });

        //info window bottom
        egui::Window::new("Current Round Info")
            .frame(egui::Frame::NONE)
            .title_bar(false)
            .order(egui::Order::Foreground)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_BOTTOM, (0.0, 0.0))
            .default_size(default_size)
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
