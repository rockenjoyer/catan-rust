//"TileVisual" stores the data needed to render or interact with a tile

use crate::backend::game::Resource::*;
use crate::backend::game::{Game, Resource, Tile, Vertex};
use bevy::prelude::*;
use bevy_egui::egui;

//"Component": attached to tile entities containing game-relevant metadata
#[derive(Component)]
pub struct TileVisual {
    //whether the tile is highlighted (e.g. hover or selection)
    pub highlighted: bool,
}

//resource to track whether tile visuals are shown or not
#[derive(Resource)]
pub struct TileShowing {
    pub enabled: bool,
}

impl Default for TileShowing {
    fn default() -> Self {
        TileShowing { enabled: true }
    }
}

//later, we can replace the colors with textures, this is for testing purposes
pub fn tile_color(resource: Resource) -> egui::Color32 {
    match resource {
        Brick => egui::Color32::from_rgb(182, 105, 43),
        Lumber => egui::Color32::from_rgb(152, 168, 105),
        Wool => egui::Color32::from_rgb(211, 211, 211),
        Grain => egui::Color32::from_rgb(255, 238, 140),
        Ore => egui::Color32::from_rgb(137, 137, 137),
        Desert => egui::Color32::from_rgb(218, 202, 78),
    }
}

pub fn draw_tiles(ui: &mut egui::Ui, game: &Game) {
    //size of the space needed for the tile setup
    let size = egui::vec2(1300.0, 700.0);
    //setup egui-painter, used to draw shapes
    let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());

    let scale = 70.0;
    let origin = response.rect.center();

    let screen = |(x, y): (f32, f32)| egui::pos2(origin.x + x * scale, origin.y + y * scale);

    //draw water
    let water = response.rect.shrink(50.0);
    painter.rect_filled(water, 20.0, egui::Color32::from_rgb(95, 124, 146));

    //draw all the tiles
    for tile in &game.tiles {
        draw_hex(&painter, tile, &game.vertices, &screen);
    }
}

//draw a single hex-tile
pub fn draw_hex(
    painter: &egui::Painter,
    tile: &Tile,
    vertices: &[Vertex],
    screen: impl Fn((f32, f32)) -> egui::Pos2, //convert game vertices to screen coords
) {
    let points: Vec<_> = tile
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
            points.iter().fold(egui::Vec2::ZERO, |a, b| a + b.to_vec2()) / points.len() as f32;

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
