use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::game::{Game, Tile, Vertex};
use crate::frontend::visual::tile::{TileShowing, tile_color};

pub fn setup_panels(
    mut context: EguiContexts,
    game: NonSend<Rc<RefCell<Game>>>,
    mut tiles_shown: ResMut<TileShowing>,
) {
    //borrow the game state for display in panels
    let game = &*game.borrow();

    if let Ok(ctx) = context.ctx_mut() {
        //main game window in the center ----------------------------------------------------------
        egui::Window::new("Main Game")
            //Settings for the game window
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_hex("#d4c1b1ff").unwrap())
                    .corner_radius(egui::CornerRadius::same(30)),
            )
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .default_width(1300.0)
            .default_height(600.0)
            //display the main game window
            .show(ctx, |ui| {
                //set text color for the main game window
                ui.visuals_mut().override_text_color =
                    Some(egui::Color32::from_hex("#120f0cff").unwrap());

                ui.separator();

                //add a button to test showing and hiding the tile board, will be removed later
                ui.horizontal(|ui| {
                    let tiles_button = if tiles_shown.enabled {
                        "Hide Tiles"
                    } else {
                        "Show Tiles"
                    };
                    if ui.button(tiles_button).clicked() {
                        tiles_shown.enabled = !tiles_shown.enabled;
                    }
                });

                ui.label(format!("Tiles: {}", game.tiles.len()));

                //test: visual preview of the tiles -----------------------------------------------
                if tiles_shown.enabled {
                    //size of the space needed for the tile setup
                    let size = egui::vec2(1300.0, 700.0);
                    //setup egui-painter, used to draw shapes
                    let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());

                    let scale = 70.0;
                    let origin = response.rect.center();

                    let screen =
                        |(x, y): (f32, f32)| egui::pos2(origin.x + x * scale, origin.y + y * scale);

                    //draw a single hex-tile
                    fn draw_hex(
                        painter: &egui::Painter,
                        tile: &Tile,
                        vertices: &[Vertex],
                        screen: impl Fn((f32, f32)) -> egui::Pos2, //convert game vertices to screen coords
                    ) {
                        let points: Vec<egui::Pos2> = tile
                            .vertices
                            .iter()
                            .map(|&v| screen(vertices[v].pos)) //each vertex point -> screen
                            .collect();

                        //draw the hex-tile as a convex polygon
                        painter.add(egui::Shape::convex_polygon(
                            points.clone(), //give the polygon its corners
                            tile_color(tile.resource),
                            egui::Stroke::new(2.0, egui::Color32::BLACK),
                        ));

                        //also draw the number token on top of the tile
                        if let Some(n) = tile.number_token {
                            //compute center of the hex -> average
                            let center =
                                points.iter().fold(egui::Vec2::ZERO, |x, y| x + y.to_vec2())
                                    / points.len() as f32;

                            //draw the number token
                            painter.text(
                                center.to_pos2(),
                                egui::Align2::CENTER_CENTER,
                                n.to_string(),
                                egui::FontId::proportional(20.0),
                                egui::Color32::BLACK,
                            );
                        }
                    }

                    //draw water
                    let smaller = response.rect.shrink(50.0);
                    painter.rect_filled(smaller, 20.0, egui::Color32::from_rgb(95, 124, 146));

                    //draw all the tiles
                    for tile in &game.tiles {
                        draw_hex(&painter, tile, &game.vertices, &screen);
                    }
                }
            });

        //TO-DO: implement the layout and content of the main game properly
    }
}
