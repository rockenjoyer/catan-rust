use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::frontend::bevy::GameState;
use crate::frontend::visual::startscreen::{StartscreenTexture, draw_background};
use crate::frontend::interface::style::apply_style;

//draw the main menu with a button to start the game
pub fn setup_main_menu(
    mut context: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    texture: Option<Res<StartscreenTexture>>,
) {
    let Some(texture) = texture else {
        return;
    };

    if let Ok(context) = context.ctx_mut() {
        apply_style(context);
        
        egui::CentralPanel::default().show(context, |ui| {
            //draw the background image
            draw_background(ui, &texture);

            //draw UI
            ui.vertical_centered(|ui| {
                ui.add_space(200.0);
                
                //title with background
                ui.horizontal(|ui| {
                    ui.add_space((ui.available_width() - 650.0) / 2.0);
                    title_frame().show(ui, |ui| {
                        ui.label(egui::RichText::new("The Settlers of Catan")
                            .size(55.0)
                            .color(egui::Color32::WHITE));
                    });
                });
                
                ui.add_space(100.0);

                //styled buttons with transparency and reddish color
                let button_size = egui::vec2(200.0, 70.0);
                button_style(ui);
                
                if ui.add_sized(button_size, 
                    egui::Button::new(egui::RichText::new("Start Game").size(20.0))
                ).clicked() {
                    next_state.set(GameState::InGame);
                }
                
                ui.add_space(20.0);

                if ui.add_sized(button_size,
                    egui::Button::new(egui::RichText::new("Exit").size(20.0))
                ).clicked() {
                    std::process::exit(0);
                }
            });
        });
    }
}

fn button_style(ui: &mut egui::Ui) {
    let button_color = egui::Color32::from_rgba_unmultiplied(180, 50, 50, 200);
    let button_hover = egui::Color32::from_rgba_unmultiplied(200, 70, 70, 230);
    
    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = button_color;
    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = button_hover;
    ui.style_mut().visuals.widgets.active.weak_bg_fill = button_hover;
}


fn title_frame() -> egui::Frame {
    egui::Frame::NONE
        .fill(egui::Color32::from_rgba_unmultiplied(50, 50, 50, 200))
        .inner_margin(egui::Margin::symmetric(30, 30))
        .corner_radius(10)
}