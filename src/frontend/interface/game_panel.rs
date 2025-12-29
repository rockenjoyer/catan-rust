use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::game::Game;
use crate::frontend::interface::style::apply_style;
use crate::frontend::visual::tile::{TileShowing, draw_tiles};

pub fn setup_game(
    mut context: EguiContexts,
    game: NonSend<Rc<RefCell<Game>>>,
    mut tiles_shown: ResMut<TileShowing>,
) {
    let game = &*game.borrow();

    if let Ok(context) = context.ctx_mut() {
        //main game window
        apply_style(context);
        egui::Window::new("Main Game")
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_hex("#d4c1b1ff").unwrap())
                    .corner_radius(egui::CornerRadius::same(15)),
            )
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, (0.0, 0.0))
            .collapsible(false)
            .default_size((1500.0, 600.0))
            .show(context, |ui| {
                //hide and unhide tiles to test basic button behaviour
                if ui
                    .button(if tiles_shown.enabled {
                        "Hide Tiles"
                    } else {
                        "Show Tiles"
                    })
                    .clicked()
                {
                    tiles_shown.enabled = !tiles_shown.enabled;
                }
                ui.separator();
                //draw tiles -> tile.rs
                if tiles_shown.enabled {
                    draw_tiles(ui, game);
                }
            });
        //TO-DO: implement the layout and content of the main game properly, mostly in the /visual files
    }
}
