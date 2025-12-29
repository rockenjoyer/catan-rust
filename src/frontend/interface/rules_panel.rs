use crate::frontend::interface::style::apply_style;
use bevy_egui::{EguiContexts, egui};

pub fn setup_rules(mut context: EguiContexts) {
    if let Ok(context) = context.ctx_mut() {
        //rules window
        apply_style(context);
        egui::Window::new("Rules")
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_hex("#d4c1b1ff").unwrap())
                    .corner_radius(egui::CornerRadius::same(15)),
            )
            .default_size((300.0, 200.0))
            .anchor(egui::Align2::RIGHT_BOTTOM, (0.0, 0.0))
            .default_open(false)
            //display content
            .show(context, |ui| {
                ui.separator();
                ui.label("1. The first player to reach 10 victory points wins the game.");

                //TO-DO: more rules to be added later
            });
    }
}
