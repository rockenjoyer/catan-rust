//Here, we define our overlay UI with egui, which will be used in the FrontendPlugin bevy.rs.

use bevy_egui::{egui, EguiContexts};

pub fn setup_overlay(mut context: EguiContexts) {

    if let Ok(context) = context.ctx_mut() {

        //Intro game window on top.  ------------------------------------------

        egui::Window::new("The Settlers of Catan")

        //Layout and style settings.
        .frame(egui::Frame::new().fill(egui::Color32::from_hex("#bd9b80ff").unwrap()))
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_TOP, egui::Vec2::ZERO)
        .default_width(1000.0)
        .default_height(200.0)

        //Display content.
        .show(context, |ui| {

            //Text color.
            ui.visuals_mut().override_text_color = Some(egui::Color32::from_hex("#33261cff").unwrap());

            ui.label("Welcome to Catan - safely implemented in Rust! Good luck and have fun!");
            ui.separator();
            ui.label("Current Player: ");
            ui.label("Resources: ");

            //TO-DO: More info to be added later. Also, change to nicer layout. Now only for demo purposes.
        });

        //Rules window at the bottom. ------------------------------------------

        egui::Window::new("Rules")

        //Layout and style settings.
        .frame(egui::Frame::new().fill(egui::Color32::from_hex("#bd9b80ff").unwrap()))
        .resizable(false)
        .anchor(egui::Align2::CENTER_BOTTOM, egui::Vec2::ZERO)
        .default_width(1000.0)
        .default_height(200.0)

        //Display content.    
        .show(context, |ui| {
            
            //Text color.
            ui.visuals_mut().override_text_color = Some(egui::Color32::from_hex("#33261cff").unwrap());

            ui.separator();
            ui.label("1. The first player to reach 10 victory points wins the game.");

            //TO-DO: More rules to be added later. Also, change to nicer layout.
        });
        
    }
}
