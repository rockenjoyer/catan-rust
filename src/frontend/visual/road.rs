use crate::backend::game::{Game, Vertex};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use std::collections::HashSet;

#[derive(Component)]
pub struct RoadVisual {
    pub vertices: [usize; 2],
    pub owner_id: usize,
}

//resource to store road textures
#[derive(Resource)]
pub struct RoadTextures {
    pub road_vertical: egui::TextureHandle,
    pub road_diagonal_right: egui::TextureHandle,
    pub road_diagonal_left: egui::TextureHandle,
}

//load the road textures into egui
pub fn setup_road_textures(
    mut commands: Commands,
    mut contexts: EguiContexts,
    textures: Option<Res<RoadTextures>>,
) {
    if textures.is_some() {
        return;
    }

    if let Ok(ctx) = contexts.ctx_mut() {
        let textures = load_road_textures(ctx);
        commands.insert_resource(textures);
        info!("Road textures have been loaded successfully!");
    }
}

pub fn load_road_textures(ctx: &egui::Context) -> RoadTextures {
    //load an image file and convert it to an egui texture, specifically rgba8 format
    let load = |path: &str| {
        let img = image::open(path)
            .unwrap_or_else(|_| panic!("Failed to load road image: {path}"))
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

    //load the road textures for different angles
    RoadTextures {
        road_vertical: load("assets/placements/road.png"),
        road_diagonal_right: load("assets/placements/road_diagonal_right.png"),
        road_diagonal_left: load("assets/placements/road_diagonal_left.png"),
    }
}

pub fn draw_roads(
    painter: &egui::Painter,
    game: &Game,
    road_textures: &RoadTextures,
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
) {
    let mut drawn = HashSet::new();

    //draw all the possible roads
    for v in &game.vertices {
        for &neighbor in &v.neighbors {
            let road = if v.id < neighbor {
                (v.id, neighbor)
            } else {
                (neighbor, v.id)
            };
            if drawn.insert(road) {
                draw_road(
                    painter,
                    road.0,
                    road.1,
                    &game.vertices,
                    screen,
                    road_textures,
                );
            }
        }
    }
}

//draw a single road between two vertices
fn draw_road(
    painter: &egui::Painter,
    a: usize,
    b: usize,
    vertices: &[Vertex],
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
    road_textures: &RoadTextures,
) {
    let start = screen(vertices[a].pos);
    let end = screen(vertices[b].pos);
    let dir = end - start;
    let center = start + dir * 0.5;
    let length = dir.length();

    //calculate the angle in radians
    let angle = dir.y.atan2(dir.x);

    //select texture based on the angle
    let road_texture = match angle.rem_euclid(std::f32::consts::PI) {
        x if x < 60.0_f32.to_radians() => &road_textures.road_diagonal_right,
        x if x < 120.0_f32.to_radians() => &road_textures.road_vertical,
        _ => &road_textures.road_diagonal_left,
    };

    //define road rectangle
    let rect = egui::Rect::from_center_size(center, egui::vec2(length / 1.5, 60.0));

    //draw the shadow
    painter.image(
        road_texture.id(),
        rect.translate(egui::vec2(0.0, 5.0)),
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
        egui::Color32::from_black_alpha(120),
    );

    //draw the road itself
    painter.image(
        road_texture.id(),
        rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
        egui::Color32::WHITE,
    );
}
