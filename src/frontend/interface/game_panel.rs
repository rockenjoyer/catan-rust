use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use std::rc::Rc;
use std::cell::RefCell;

use crate::backend::game::Game;

pub fn setup_panels(mut context: EguiContexts, game: NonSend<Rc<RefCell<Game>>>) {

    // Borrow the game state for display in panels.
    let game = &*game.borrow();

    if let Ok(ctx) = context.ctx_mut() {

        //Main game window in the center.  ------------------------------------------
        //This is going to be the main panel with the actual game.
        egui::Window::new("Main Game")

        //Settings for the game window.
        .frame(egui::Frame::new().fill(egui::Color32::from_hex("#bd9b80ff").unwrap()))
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .default_width(1000.0)
        .default_height(1000.0)

        //Display the main game window.
        .show(ctx, |ui| {

            //Set text color for the main game window.
            ui.visuals_mut().override_text_color = Some(egui::Color32::from_hex("#33261cff").unwrap());
            
            ui.label(format!("Tiles: {}", game.tiles.len()));
            ui.separator();

            //Test: List all tiles with their resource and number token.
            for (num, tile) in game.tiles.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("Number {num}: {:?}", tile.resource));
                    if let Some(dice) = tile.number_token { 
                        ui.label(format!("({dice})")); 
                    }
                });
            }
        });

        //TO-DO: Implement the layout and content of the main game properly.

    }
} 