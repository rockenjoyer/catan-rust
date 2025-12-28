use crate::frontend::interface::style::apply_style;
use bevy_egui::{EguiContexts, egui};

pub fn setup_volume(mut context: EguiContexts) {
    if let Ok(context) = context.ctx_mut() {
        //rules window ------------------------------------------

        apply_style(context);
        egui::Window::new("Volume")
            //layout and style settings
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_hex("#d4c1b1ff").unwrap())
                    .corner_radius(egui::CornerRadius::same(15)),
            )
            .anchor(egui::Align2::LEFT_BOTTOM, (0.0, 0.0))
            .default_size((300.0, 600.0))
            .default_open(false)
            //display content
            .show(context, |ui| {
                ui.separator();
                ui.label("This will display settings for volume soon.");
            });
            
        //TO-DO: add a volume icon, add music to the game etc.
    }
}
