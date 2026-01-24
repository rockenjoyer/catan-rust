use crate::backend::game::Resource::*;
use crate::backend::game::{Game, Resource, Tile, Vertex};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use std::collections::HashMap;

//resource to store tile textures in a hashmap
#[derive(Resource)]
pub struct TileTextures {
    pub land: HashMap<Resource, egui::TextureHandle>,
    pub water: egui::TextureHandle,
}

#[derive(Clone, Copy)]
pub struct WaterTile {
    pub pos: (f32, f32),
}

//track what was clicked
#[derive(Resource, Default)]
pub struct ClickedVertex {
    pub vertex_id: Option<usize>,
    pub selected_vertex: Option<usize>, //keeps glowing untill clicked off
}

#[derive(Component)]
pub struct TileVisual {
    pub highlighted: bool,
}

//load the tile textures into egui
pub fn setup_tile_textures(
    mut commands: Commands,
    mut contexts: EguiContexts,
    textures: Option<Res<TileTextures>>,
) {
    if textures.is_some() {
        return;
    }
    if let Ok(ctx) = contexts.ctx_mut() {
        commands.insert_resource(load_tile_textures(ctx));
        info!("Tile textures loaded successfully!");
    }
}

pub fn load_tile_textures(ctx: &egui::Context) -> TileTextures {
    let mut land = HashMap::new();

    let load = |path: &str| {
        let img = image::open(path)
            .unwrap_or_else(|_| panic!("Failed to load tile image: {path}"))
            .to_rgba8();

        ctx.load_texture(
            path.to_string(),
            egui::ColorImage::from_rgba_unmultiplied(
                [img.width() as usize, img.height() as usize],
                &img.into_raw(),
            ),
            egui::TextureOptions::LINEAR,
        )
    };

    land.insert(Brick, load("assets/tiles/brick.png"));
    land.insert(Lumber, load("assets/tiles/lumber.png"));
    land.insert(Wool, load("assets/tiles/wool.png"));
    land.insert(Grain, load("assets/tiles/grain.png"));
    land.insert(Ore, load("assets/tiles/ore.png"));
    land.insert(Desert, load("assets/tiles/desert.png"));

    let water = load("assets/tiles/water_background.png");

    TileTextures { land, water }
}

pub fn tile_texture<'a>(textures: &'a TileTextures, resource: Resource) -> &'a egui::TextureHandle {
    textures
        .land
        .get(&resource)
        .expect("The land tile texture is missing!")
}

//draw all the tiles with click detection for vertices
pub fn draw_tiles(
    ui: &mut egui::Ui,
    painter: &egui::Painter,
    board_rect: egui::Rect,
    game: &Game,
    textures: &TileTextures,
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
) {
    //draw water background filling entire screen while maintaining aspect ratio
    let size = board_rect.size();
    let aspect_ratio = 16.0 / 9.0;
    
    let (bg_width, bg_height) = if size.x / size.y > aspect_ratio {
        //if the screen is wider -> match width, height extends beyond screen
        let width = size.x;
        (width, width / aspect_ratio)
    } else {
        // if the screen is taller -> match height, width extends beyond screen
        let height = size.y;
        (height * aspect_ratio, height)
    };
    
    let board_rect = egui::Rect::from_center_size(
        board_rect.center(),
        egui::vec2(bg_width, bg_height)
    );
    
    painter.image(
        textures.water.id(),
        board_rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
        egui::Color32::WHITE,
    );

    let vertex_response = ui.allocate_rect(board_rect, egui::Sense::click());

    //check which vertex is being hovered
    let mut hovered_vertex: Option<usize> = None;
    let mouse_pos = vertex_response.hover_pos();

    if let Some(mouse_pos) = mouse_pos {
        let radius = 10.0;
        for vertex in &game.vertices {
            let pos = screen(vertex.pos);
            if mouse_pos.distance(pos) <= radius {
                hovered_vertex = Some(vertex.id);
                break;
            }
        }
    }

    //find which tile (if any) is being hovered
    let mut hovered_tile: Option<usize> = None;
    
    if hovered_vertex.is_none() {
        if let Some(mouse_pos) = ui.ctx().pointer_hover_pos() {
            for (i, tile) in game.tiles.iter().enumerate() {
                let points: Vec<_> = tile.vertices.iter()
                    .map(|&v| screen(game.vertices[v].pos))
                    .collect();
                let center = points.iter()
                    .fold(egui::Vec2::ZERO, |acc, p| acc + p.to_vec2()) 
                    / points.len() as f32;

                //calculate size based on distance between vertices
                let distance = points[0].distance(points[1]);
                let size = egui::vec2(distance * 1.65, distance * 1.95);

                let base_rect = egui::Rect::from_center_size(center.to_pos2(), size);
                
                if base_rect.contains(mouse_pos) {
                    hovered_tile = Some(i);
                    break; //only hover one tile
                }
            }
        }
    }

    //draw tiles with hover info
    for (i, tile) in game.tiles.iter().enumerate() {
        draw_hex(painter, tile, i, &game.vertices, screen, textures, hovered_tile == Some(i));
    }
}

//draw vertices as clickable circles (called separately to control layer order)
pub fn draw_vertices(
    ui: &mut egui::Ui,
    painter: &egui::Painter,
    board_rect: egui::Rect,
    game: &Game,
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
    clicked_vertex: &mut ClickedVertex,
) {
    let vertex_response = ui.allocate_rect(board_rect, egui::Sense::click());

    //check which vertex is being hovered
    let mut hovered_vertex: Option<usize> = None;
    let mouse_pos = vertex_response.hover_pos();

    if let Some(mouse_pos) = mouse_pos {
        let radius = 10.0;
        for vertex in &game.vertices {
            let pos = screen(vertex.pos);
            if mouse_pos.distance(pos) <= radius {
                hovered_vertex = Some(vertex.id);
                break;
            }
        }
    }

    //draw vertices as clickable circles (only empty vertices)

    for vertex in &game.vertices {
        let pos = screen(vertex.pos);
        let radius = 10.0;

        //check if any player has a settlement or city here
        let mut is_occupied = false;

        for player in &game.players {
            if player.cities.contains(&vertex.id) || player.settlements.contains(&vertex.id) {
                is_occupied = true;
                break;
            }
        }

        //only draw circle if vertex is empty
        if !is_occupied {
            //check if mouse is over this vertex or if it is the selected vertex
            let is_hovered = hovered_vertex == Some(vertex.id);
            let is_selected = clicked_vertex.selected_vertex == Some(vertex.id);

            let color = if is_hovered || is_selected {
                egui::Color32::YELLOW
            } else {
                egui::Color32::WHITE
            };

            painter.circle_filled(pos, radius, color);
            painter.circle_stroke(pos, radius, egui::Stroke::new(2.0, egui::Color32::BLACK));
        
            //draw vertex ID on top
            painter.text(
                pos,
                egui::Align2::CENTER_CENTER,
                vertex.id.to_string(),
                egui::FontId::proportional(14.0),
                egui::Color32::BLACK,
            );
        }

        //handle click
        if vertex_response.clicked() {
            if let Some(mouse_pos) = vertex_response.interact_pointer_pos() {
                // Check if clicked on a vertex
                if mouse_pos.distance(pos) <= radius {
                    clicked_vertex.vertex_id = Some(vertex.id);
                    clicked_vertex.selected_vertex = Some(vertex.id);
                } else if hovered_vertex.is_none() {
                    // Clicked elsewhere (not on any vertex) - deselect
                    clicked_vertex.selected_vertex = None;
                }
            }
        }
    }
}

//draw a single hex-tile with texture
fn draw_hex(
    painter: &egui::Painter,
    tile: &Tile,
    _index: usize,
    vertices: &[Vertex],
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
    textures: &TileTextures,
    is_hovered: bool,
) {
    //convert all of the hex vertices from game coords to pixel coords
    let points: Vec<_> = tile
        .vertices
        .iter()
        .map(|&v| screen(vertices[v].pos))
        .collect();

    let center = points
        .iter()
        .fold(egui::Vec2::ZERO, |acc, p| acc + p.to_vec2())
        / points.len() as f32;

    //calculate size based on distance between vertices (responsive to scale)
    let distance = points[0].distance(points[1]);
    let size = egui::vec2(distance * 1.65, distance * 1.95);

    let base_rect = egui::Rect::from_center_size(center.to_pos2(), size);

    let lift = if is_hovered {-10.0} else {0.0};
    let shadow_offset = if is_hovered {10.0} else {5.0};

    let tile_rect = base_rect.translate(egui::vec2(0.0, lift));

    //draw shadow
    painter.image(
        tile_texture(textures, tile.resource).id(),
        tile_rect.translate(egui::vec2(5.0, shadow_offset)),
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
        egui::Color32::from_black_alpha(100),
    );

    //draw tile texture
    painter.image(
        tile_texture(textures, tile.resource).id(),
        tile_rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
        egui::Color32::WHITE,
    );

    //draw number token
    if let Some(n) = tile.number_token {
        painter.text(
            center.to_pos2() + egui::vec2(0.0, lift),
            egui::Align2::CENTER_CENTER,
            n.to_string(),
            egui::FontId::proportional(25.0),
            egui::Color32::BLACK,
        );
    }
}