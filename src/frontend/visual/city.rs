use crate::backend::game::{Game};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

#[derive(Component)]
pub struct CityVisual {
    pub vertex: usize,
    pub owner_id: usize,
}

//resource to store city textures
#[derive(Resource)]
pub struct CityTextures {
    pub red: egui::TextureHandle,
    pub blue: egui::TextureHandle,
    pub green: egui::TextureHandle,
    pub yellow: egui::TextureHandle,
}

//load the city textures into egui
pub fn setup_city_textures(
    mut commands: Commands,
    mut contexts: EguiContexts,
    textures: Option<Res<CityTextures>>,
) {
    if textures.is_some() {
        return;
    }

    if let Ok(ctx) = contexts.ctx_mut() {
        let textures = load_city_textures(ctx);
        commands.insert_resource(textures);
        info!("City textures have been loaded successfully!");
    }
}

pub fn load_city_textures(ctx: &egui::Context) -> CityTextures {
    let load = |path: &str| {
        let image = image::open(path)
            .unwrap_or_else(|_| panic!("Failed to load city image: {}", path))
            .to_rgba8();

        let size = [image.width() as usize, image.height() as usize];
        let pixels = image.into_raw();

        ctx.load_texture(
            path.to_string(),
            egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
            egui::TextureOptions::LINEAR,
        )
    };

    CityTextures {
        red: load("assets/placements/city_red.png"),
        blue: load("assets/placements/city_blue.png"),
        green: load("assets/placements/city_green.png"),
        yellow: load("assets/placements/city_yellow.png"),
    }
}

//draw all cities at the vertices (based on actual game state)
pub fn draw_cities(
    painter: &egui::Painter,
    game: &Game,
    textures: &CityTextures,
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
    zoom: f32,
) {
     for player in &game.players {
        let texture = match player.id {
            0 => &textures.red,
            1 => &textures.blue,
            2 => &textures.green,
            3 => &textures.yellow,
            _ => &textures.red,
        };

        for &vertex_id in &player.cities {
            let pos = screen(game.vertices[vertex_id].pos);
            let rect = egui::Rect::from_center_size(pos, egui::vec2(68.6 * zoom, 60.0 * zoom));

            //draw shadow
            painter.image(
                texture.id(),
                rect.translate(egui::vec2(0.0, 3.0 * zoom)),
                egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
                egui::Color32::from_black_alpha(100),
            );

            //draw city with player color
            painter.image(
                texture.id(),
                rect,
                egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }
    }
}