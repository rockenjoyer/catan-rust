use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use crate::frontend::interface::style::apply_style;

pub fn setup_info(mut context: EguiContexts) {
    if let Ok(context) = context.ctx_mut() {
        apply_style(context);

        let default_size = (1300.0, 200.0);

        //info window top
        egui::Window::new("The Settlers of Catan")
            .frame(info_frame())
            .order(egui::Order::Foreground)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_TOP, (0.0, 0.0))
            .default_size(default_size)

            .show(context, |ui| {
                ui.label("Welcome to Catan - safely implemented in Rust! Good luck and have fun!");
            });

        //info window bottom
        egui::Window::new("Current Round Info")
            .frame(info_frame())
            .order(egui::Order::Foreground)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_BOTTOM, (0.0, 0.0))
            .default_size(default_size)

            .show(context, |ui| {
                ui.label(format!("Current Player: "));
                ui.label(format!("Resources: "));
                //TO-DO: more info to be added later
            });
    }
}

fn info_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(egui::Color32::from_hex("#d4c1b1bd").unwrap())
        .corner_radius(egui::CornerRadius::same(15))
}