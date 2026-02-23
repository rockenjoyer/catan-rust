use crate::backend::game::{Game, Vertex};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

#[derive(Component)]
pub struct RoadVisual {
    pub vertices: [usize; 2],
    pub owner_id: usize,
}
//resource to store road textures
#[derive(Resource)]
pub struct RoadTextures {
    pub red_vertical: egui::TextureHandle,
    pub red_diagonal_right: egui::TextureHandle,
    pub red_diagonal_left: egui::TextureHandle,
    pub blue_vertical: egui::TextureHandle,
    pub blue_diagonal_right: egui::TextureHandle,
    pub blue_diagonal_left: egui::TextureHandle,
    pub green_vertical: egui::TextureHandle,
    pub green_diagonal_right: egui::TextureHandle,
    pub green_diagonal_left: egui::TextureHandle,
    pub yellow_vertical: egui::TextureHandle,
    pub yellow_diagonal_right: egui::TextureHandle,
    pub yellow_diagonal_left: egui::TextureHandle,
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
        red_vertical: load("assets/placements/road_red.png"),
        red_diagonal_right: load("assets/placements/road_diagonal_right_red.png"),
        red_diagonal_left: load("assets/placements/road_diagonal_left_red.png"),
        blue_vertical: load("assets/placements/road_blue.png"),
        blue_diagonal_right: load("assets/placements/road_diagonal_right_blue.png"),
        blue_diagonal_left: load("assets/placements/road_diagonal_left_blue.png"),
        green_vertical: load("assets/placements/road_green.png"),
        green_diagonal_right: load("assets/placements/road_diagonal_right_green.png"),
        green_diagonal_left: load("assets/placements/road_diagonal_left_green.png"),
        yellow_vertical: load("assets/placements/road_yellow.png"),
        yellow_diagonal_right: load("assets/placements/road_diagonal_right_yellow.png"),
        yellow_diagonal_left: load("assets/placements/road_diagonal_left_yellow.png"),
    }
}

pub fn draw_roads(
    painter: &egui::Painter,
    game: &Game,
    road_textures: &RoadTextures,
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
    zoom: f32,
) {
    //draw all roads for all players with their colors
    for player in &game.players {
        for &(v1, v2) in &player.roads {
            draw_road(
                painter,
                v1,
                v2,
                player.id,
                &game.vertices,
                screen,
                road_textures,
                zoom,
            );
        }
    }
}

//draw a single road between two vertices
fn draw_road(
    painter: &egui::Painter,
    a: usize,
    b: usize,
    player_id: usize,
    vertices: &[Vertex],
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
    road_textures: &RoadTextures,
    zoom: f32,
) {
    let start = screen(vertices[a].pos);
    let end = screen(vertices[b].pos);
    let dir = end - start;
    let center = start + dir * 0.5;
    let length = dir.length();

    //calculate the angle in radians
    let angle = dir.y.atan2(dir.x);

    let road_texture = select_road_texture(road_textures, player_id, angle);

    let rect = egui::Rect::from_center_size(center, egui::vec2(length / 1.1, 60.0 * zoom));

    //draw the shadow
    painter.image(
        road_texture.id(),
        rect.translate(egui::vec2(0.0, 5.0)),
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
        egui::Color32::from_black_alpha(120),
    );

    //draw the road itself with player color tint
    painter.image(
        road_texture.id(),
        rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
        egui::Color32::WHITE,
    );
}

//helper function to choose the tile texture color
pub fn select_road_texture(
    road_textures: &RoadTextures,
    player_id: usize,
    angle: f32,
) -> &egui::TextureHandle {
    //select texture based on the angle
    match angle.rem_euclid(std::f32::consts::PI) {
        x if x < 60.0_f32.to_radians() => match player_id {
            0 => &road_textures.red_diagonal_right,
            1 => &road_textures.blue_diagonal_right,
            2 => &road_textures.green_diagonal_right,
            3 => &road_textures.yellow_diagonal_right,
            _ => &road_textures.red_diagonal_right,
        },
        x if x < 120.0_f32.to_radians() => match player_id {
            0 => &road_textures.red_vertical,
            1 => &road_textures.blue_vertical,
            2 => &road_textures.green_vertical,
            3 => &road_textures.yellow_vertical,
            _ => &road_textures.red_vertical,
        },
        _ => match player_id {
            0 => &road_textures.red_diagonal_left,
            1 => &road_textures.blue_diagonal_left,
            2 => &road_textures.green_diagonal_left,
            3 => &road_textures.yellow_diagonal_left,
            _ => &road_textures.red_diagonal_left,
        },
    }
}