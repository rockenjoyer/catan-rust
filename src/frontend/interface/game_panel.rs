use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::game::Game;
use crate::frontend::interface::style::apply_style;
use crate::frontend::visual::tile::{TileTextures, draw_tiles};

pub fn setup_game(
    mut context: EguiContexts,
    game: NonSend<Rc<RefCell<Game>>>,
    textures: Option<Res<TileTextures>>,
) {
    let game = &*game.borrow();
    let Some(textures) = textures else {
        return;
    };

    if let Ok(context) = context.ctx_mut() {
        //main game window
        apply_style(context);

        egui::Window::new("")
            .frame(egui::Frame::new())
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, (0.0, 0.0))
            .collapsible(false)
            .default_size((1500.0, 1000.0))
            .show(context, |ui| {
                draw_tiles(ui, game, &textures);
            });
    }
}
