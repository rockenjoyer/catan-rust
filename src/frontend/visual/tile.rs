//"TileVisual" stores the data needed to render or interact with a tile

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

#[derive(Component)]
pub struct TileVisual {
    //whether the tile is highlighted (e.g. hover or selection)
    pub highlighted: bool,
}

#[derive(Clone, Copy)]
pub struct WaterTile {
    pub pos: (f32, f32),
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

//load the tile textures into egui
pub fn setup_tile_textures(
    mut commands: Commands,
    mut contexts: EguiContexts,
    textures: Option<Res<TileTextures>>,
) {
    if textures.is_some() {
        return;
    }

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let textures = load_tile_textures(ctx);
    commands.insert_resource(textures);

    info!("Tile textures have been loaded in successfully!");
}

pub fn load_tile_textures(ctx: &egui::Context) -> TileTextures {
    let mut land = HashMap::new();

    //load an image file and convert it to an egui texture, specifically rgba8 format
    let load = |path: &str| {
        let image = image::open(path)
            .unwrap_or_else(|_| panic!("Failed to load tile image! {path}"))
            .to_rgba8();

        //extract image dimensions etc.
        let size = [image.width() as usize, image.height() as usize];
        let pixels = image.into_raw();

        //convert the previous image data to egui texture
        ctx.load_texture(
            path.to_string(),
            egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
            egui::TextureOptions::LINEAR,
        )
    };

    //load texture for each resource type from the assetsand insert into the hashmap
    land.insert(Brick, load("assets/tiles/brick.png"));
    land.insert(Lumber, load("assets/tiles/lumber.png"));
    land.insert(Wool, load("assets/tiles/wool.png"));
    land.insert(Grain, load("assets/tiles/grain.png"));
    land.insert(Ore, load("assets/tiles/ore.png"));
    land.insert(Desert, load("assets/tiles/desert.png"));

    //load the water texture separately
    let water = load("assets/tiles/water_background.png");

    TileTextures { land, water }
}

//retrieve the texture handle for a given resource type
pub fn tile_texture<'a>(textures: &'a TileTextures, resource: Resource) -> &'a egui::TextureHandle {
    textures
        .land
        .get(&resource)
        .expect("The land tile texture is missing!")
}

//draw all the tiles in the game
pub fn draw_tiles(ui: &mut egui::Ui, game: &Game, textures: &TileTextures) {
    //size of the space needed for the tile setup
    let size = egui::vec2(1300.0, 800.0);
    //setup egui-painter, used to draw shapes and now textures
    let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());

    //scale factor for converting game coordinates to pixels
    let scale = 65.0;
    //center point of the display area for positioning the board
    let origin = response.rect.center();

    //conversion function from coordinates to pixel coordinates
    let screen = |(x, y): (f32, f32)| egui::pos2(origin.x + x * scale, origin.y + y * scale);

    //draw water FIRST! as the background
    draw_water_background(&painter, &textures.water, response.rect);

    //draw the land tile on top of the water background
    for tile in &game.tiles {
        draw_hex(&painter, tile, &game.vertices, &screen, textures);
    }
}

//draw a single land hex-tile with its texture
pub fn draw_hex(
    painter: &egui::Painter,
    tile: &Tile,
    vertices: &[Vertex],
    screen: impl Fn((f32, f32)) -> egui::Pos2, //convert game vertices to screen coords
    textures: &TileTextures,
) {
    //convert all of the hex vertices from game coords to pixel coords
    let points: Vec<_> = tile
        .vertices
        .iter()
        .map(|&v| screen(vertices[v].pos)) //each vertex point -> screen
        .collect();

    //calculate the center point of the hexagon by averaging all vertex positions
    let center = points.iter().fold(egui::Vec2::ZERO, |a, b| a + b.to_vec2()) / points.len() as f32;

    //the size of the texture rectangle that is to be drawn
    let size = egui::vec2(120.0, 120.0);

    //draw the resource tile texture at the center position
    painter.image(
        tile_texture(textures, tile.resource).id(),
        egui::Rect::from_center_size(center.to_pos2(), size),
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
        egui::Color32::WHITE,
    );

    /*
    //draw the hex-tile as a convex polygon
    painter.add(egui::Shape::convex_polygon(
        points.clone(),             //give the polygon its corners
        egui::Color32::TRANSPARENT, //fill color
        egui::Stroke::new(2.0, egui::Color32::BLACK),
    ));
    */

    //draw the number token on top of the tile, if it exits (not desert)
    if let Some(n) = tile.number_token {
        painter.text(
            center.to_pos2(),
            egui::Align2::CENTER_CENTER,
            n.to_string(),
            egui::FontId::proportional(20.0),
            egui::Color32::BLACK,
        );
    }
}

//draw the water background texture across the entire display area
fn draw_water_background(painter: &egui::Painter, texture: &egui::TextureHandle, rect: egui::Rect) {
    //the entire rect will be filled with the water texture
    painter.image(
        texture.id(),
        rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
        egui::Color32::WHITE,
    );
}
