use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::game::Game;
use crate::frontend::interface::style::apply_style;
use crate::frontend::visual::{
    cards::{CardsTextures, draw_cards},
    city::{CityTextures, draw_cities},
    road::{RoadTextures, draw_roads},
    settlement::{SettlementTextures, draw_settlements},
    tile::{TileTextures, draw_tiles}
};


pub fn setup_game(
    mut context: EguiContexts,
    game: NonSend<Rc<RefCell<Game>>>,
    tile_textures: Option<Res<TileTextures>>,
    road_textures: Option<Res<RoadTextures>>,
    card_textures: Option<Res<CardsTextures>>,
    settlement_textures: Option<Res<SettlementTextures>>,
    city_textures: Option<Res<CityTextures>>,
) {
    let game = &*game.borrow();
    let Some(tile_textures) = tile_textures else { return; };
    let Some(road_textures) = road_textures else { return; };
    let Some(card_textures) = card_textures else { return; };
    let Some(settlement_textures) = settlement_textures else { return; };
    let Some(city_textures) = city_textures else { return; };

    if let Ok(context) = context.ctx_mut() {
        apply_style(context);

        egui::Window::new("")

            .frame(egui::Frame::NONE)
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .collapsible(false)
            .default_size(context.available_rect().size())

            .show(context, |ui| {

                //size of the space needed for the tile setup
                let size = ui.available_size();

                //setup egui-painter, used to draw shapes and now textures
                let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());

                //scale factor for converting game coordinates to pixels
                let scale = 65.0;

                //center point of the display area for positioning the board
                let origin = response.rect.center();

                //conversion function from coordinates to pixel coordinates
                let screen =
                    |(x, y): (f32, f32)| egui::pos2(origin.x + x * scale, origin.y + y * scale);

                draw_tiles(ui, &painter, response.rect, game, &tile_textures, &screen);
                draw_roads(&painter, game, &road_textures, &screen);
                draw_cards(
                    &painter,
                    &card_textures,
                    egui::pos2(100.0, 100.0),
                    egui::vec2(100.0, 130.0),
                    10.0,
                );
                draw_settlements(&painter, &game.vertices, &settlement_textures, &screen);
                draw_cities(&painter, &game.vertices, &city_textures, &screen);
            });
    }
}
