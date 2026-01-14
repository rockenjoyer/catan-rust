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

//track what was clicked
#[derive(Resource, Default)]
pub struct ClickedVertex {
    pub vertex_id: Option<usize>,
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

pub fn draw_tiles(ui: &mut egui::Ui, game: &Game, clicked_vertex: &mut ClickedVertex) {
    //size of the space needed for the tile setup
    let size = egui::vec2(1300.0, 700.0);
    //setup egui-painter, used to draw shapes
    let (response, painter) = ui.allocate_painter(size, egui::Sense::click());

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

    //draw roads (colored)
    for player in &game.players {
        let road_color = match player.id {
            0 => egui::Color32::RED,
            1 => egui::Color32::BLUE,
            2 => egui::Color32::GREEN,
            3 => egui::Color32::ORANGE,
            _ => egui::Color32::WHITE,
        };

        for &(v1, v2) in &player.roads {
            let pos1 = screen(game.vertices[v1].pos);
            let pos2 = screen(game.vertices[v2].pos);

            painter.line_segment(
                [pos1, pos2],
                egui::Stroke::new(4.0, road_color),
            );
        }
    }

    //draw vertices as clickable circles
    for vertex in &game.vertices {
        let pos = screen(vertex.pos);
        let radius = 10.0;

        //check if any player has a settlement or city here
        let mut owner_id: Option<usize> = None;
        let mut is_city = false;

        for player in &game.players {
            
            if player.cities.contains(&vertex.id) {
                owner_id = Some(player.id);
                is_city = true;
                break;
            }

            if player.settlements.contains(&vertex.id) {
                owner_id = Some(player.id);
                break;
            }
        }

        //check if mouse is over this vertex
        let is_hovered = response.hover_pos().map_or(false, |mouse_pos| {
            mouse_pos.distance(pos) <= radius
        });

        //draw vertex circle (with player colors if owned)
        let color = if let Some(player_id) = owner_id {
            //different colors for each player
            match player_id {
                0 => egui::Color32::RED,
                1 => egui::Color32::BLUE,
                2 => egui::Color32::GREEN,
                3 => egui::Color32::ORANGE,
                _ => egui::Color32::WHITE,
            }
        } else if is_hovered {
            egui::Color32::YELLOW
        } else {
            egui::Color32::WHITE
        };

        //larger radius for cities
        let actual_radius = if is_city {radius * 1.3} else {radius};

        painter.circle_filled(pos, actual_radius, color);
        painter.circle_stroke(pos, actual_radius, egui::Stroke::new(2.0, egui::Color32::BLACK));

        //draw player number on settlement or city
        if let Some(player_id) = owner_id {
            painter.text(
                pos,
                egui::Align2::CENTER_CENTER,
                (player_id + 1).to_string(),
                egui::FontId::proportional(12.0),
                egui::Color32::WHITE,
            );
        }

        //handle click
        if is_hovered && response.clicked() {
            clicked_vertex.vertex_id = Some(vertex.id);
        }
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
