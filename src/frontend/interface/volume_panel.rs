use crate::frontend::interface::style::apply_style;
use bevy_egui::{EguiContexts, egui};

pub fn setup_volume(mut context: EguiContexts) {
    if let Ok(context) = context.ctx_mut() {
        apply_style(context);

        let default_size = egui::vec2(300.0, 600.0);

        egui::Window::new("Volume")
            .frame(window_frame())
            .order(egui::Order::Foreground)
            .anchor(egui::Align2::LEFT_TOP, (0.0, 0.0))
            .default_size(default_size)
            .default_open(false)

            .show(context, |ui| {
                ui.separator();
                ui.label("This will display settings for volume soon.");
                //TO-DO: add a volume icon, add music to the game etc.
            });
        
    }
}

fn window_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(egui::Color32::from_hex("#d4c1b1bd").unwrap())
        .corner_radius(egui::CornerRadius::same(15))
}
