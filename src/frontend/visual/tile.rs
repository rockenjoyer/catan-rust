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
//resource to track whether tile visuals are shown or not
#[derive(Resource)]
pub struct TileShowing {
    pub enabled: bool,
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

    //load an image file and convert it to an egui texture, specifically rgba8 format
    let load = |path: &str| {
        let img = image::open(path)
            .unwrap_or_else(|_| panic!("Failed to load tile image: {path}"))
            .to_rgba8();

        //convert the previous image data to egui texture
        ctx.load_texture(
            path.to_string(),
            egui::ColorImage::from_rgba_unmultiplied(
                [img.width() as usize, img.height() as usize],
                &img.into_raw(),
            ),
            egui::TextureOptions::LINEAR,
        )
    };

    //load texture for each resource type from the assets and insert into the hashmap
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
pub fn draw_tiles(
    ui: &mut egui::Ui,
    painter: &egui::Painter,
    board_rect: egui::Rect,
    game: &Game,
    textures: &TileTextures,
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
) {
    //draw water FIRST! as the background
    painter.image(
        textures.water.id(),
        board_rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
        egui::Color32::WHITE,
    );

    //draw the land tile on top of the water background
    for (i, tile) in game.tiles.iter().enumerate() {
        draw_hex(ui, painter, tile, i, &game.vertices, screen, textures);
    }
}

//draw a single land hex-tile with its texture
pub fn draw_hex(
    ui: &mut egui::Ui,
    painter: &egui::Painter,
    tile: &Tile,
    index: usize,
    vertices: &[Vertex],
    screen: impl Fn((f32, f32)) -> egui::Pos2, //convert game vertices to screen coords
    textures: &TileTextures,
) {
    //convert all of the hex vertices from game coords to pixel coords
    let points: Vec<_> = tile
        .vertices
        .iter()
        .map(|&v| screen(vertices[v].pos))
        .collect();

    //calculate the center point of the hexagon
    let center = points
        .iter()
        .fold(egui::Vec2::ZERO, |acc, p| acc + p.to_vec2())
        / points.len() as f32;

    //the size of the texture rectangle that is to be drawn
    let size = egui::vec2(110.0, 130.0);
    let base_rect = egui::Rect::from_center_size(center.to_pos2(), size);

    //hover detection
    let id = egui::Id::new(("tile", index));
    let hovered = ui.interact(base_rect, id, egui::Sense::hover()).hovered();

    //elevation and shadow of the hovered tile
    let lift = if hovered { -10.0 } else { 0.0 };
    let shadow_offset = if hovered { 10.0 } else { 5.0 };
    let tile_rect = base_rect.translate(egui::vec2(0.0, lift));

    //draw a shadow underneath the tile
    painter.image(
        tile_texture(textures, tile.resource).id(),
        tile_rect.translate(egui::vec2(5.0, shadow_offset)),
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
        egui::Color32::from_black_alpha(100),
    );

    //draw the resource tile texture at the center position
    painter.image(
        tile_texture(textures, tile.resource).id(),
        tile_rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
        egui::Color32::WHITE,
    );

    //draw the number token on top of the tile, if it exits (not desert)
    //also with a shadow and lifted if hovered
    if let Some(n) = tile.number_token {
        painter.text(
            center.to_pos2() + egui::vec2(0.0, lift),
            egui::Align2::CENTER_CENTER,
            n.to_string(),
            egui::FontId::proportional(20.0),
            egui::Color32::from_hex("#22170c").unwrap(),
        );
    }
}
