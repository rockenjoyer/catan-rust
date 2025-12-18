//here, we define our overlay UI with egui, which will be used in the FrontendPlugin bevy.rs

use bevy_egui::{EguiContexts, egui};

pub fn setup_overlay(mut context: EguiContexts) {
    if let Ok(context) = context.ctx_mut() {
        //intro game window on top ------------------------------------------

        egui::Window::new("The Settlers of Catan")
            //layout and style settings
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_hex("#d4c1b1ff").unwrap())
                    .corner_radius(egui::CornerRadius::same(30)),
            )
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_TOP, egui::Vec2::ZERO)
            .default_width(1300.0)
            .default_height(200.0)
            //display content
            .show(context, |ui| {
                //text color
                ui.visuals_mut().override_text_color =
                    Some(egui::Color32::from_hex("#120f0cff").unwrap());

                ui.label("Welcome to Catan - safely implemented in Rust! Good luck and have fun!");
                ui.separator();
                ui.label("Current Player: ");
                ui.label("Resources: ");

                //TO-DO: more info to be added later and also changes for a nicer layout, now only for demo purposes
            });

        //rules window at the bottom ------------------------------------------

        egui::Window::new("Rules")
            //layout and style settings
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_hex("#d4c1b1ff").unwrap())
                    .corner_radius(egui::CornerRadius::same(30)),
            )
            .resizable(false)
            .anchor(egui::Align2::CENTER_BOTTOM, egui::Vec2::ZERO)
            .default_width(1300.0)
            .default_height(200.0)
            //display content
            .show(context, |ui| {
                //text color
                ui.visuals_mut().override_text_color =
                    Some(egui::Color32::from_hex("#120f0cff").unwrap());

                ui.separator();
                ui.label("1. The first player to reach 10 victory points wins the game.");

                //TO-DO: more rules to be added later
            });
    }
}
