use crate::backend::game::{Game};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

//Component attached to entities representing settlements.
#[derive(Component)]
pub struct SettlementVisual {
    pub vertex: usize,
    pub owner_id: usize,
}

//resource to store settlement textures
#[derive(Resource)]
pub struct SettlementTextures {
    pub red: egui::TextureHandle,
    pub blue: egui::TextureHandle,
    pub green: egui::TextureHandle,
    pub yellow: egui::TextureHandle,
}

//load the settlement textures into egui
pub fn setup_settlement_textures(
    mut commands: Commands,
    mut contexts: EguiContexts,
    textures: Option<Res<SettlementTextures>>,
) {
    if textures.is_some() {
        return;
    }

    if let Ok(ctx) = contexts.ctx_mut() {
        let textures = load_settlement_textures(ctx);
        commands.insert_resource(textures);
        info!("Settlement textures have been loaded successfully!");
    }
}

pub fn load_settlement_textures(ctx: &egui::Context) -> SettlementTextures {
    let load = |path: &str| {
        let image = image::open(path)
            .unwrap_or_else(|_| panic!("Failed to load settlement image: {}", path))
            .to_rgba8();

        let size = [image.width() as usize, image.height() as usize];
        let pixels = image.into_raw();

        ctx.load_texture(
            path.to_string(),
            egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
            egui::TextureOptions::LINEAR,
        )
    };

    SettlementTextures {
        red: load("assets/placements/settlement_red.png"),
        blue: load("assets/placements/settlement_blue.png"),
        green: load("assets/placements/settlement_green.png"),
        yellow: load("assets/placements/settlement_yellow.png"),
    }
}
//draw all settlements at the vertices (based on actual game state)
pub fn draw_settlements(
    painter: &egui::Painter,
    game: &Game,
    textures: &SettlementTextures,
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
) {
    for player in &game.players {
        let texture = match player.id {
            0 => &textures.red,
            1 => &textures.blue,
            2 => &textures.green,
            3 => &textures.yellow,
            _ => &textures.red,
        };

        for &vertex_id in &player.settlements {
            let pos = screen(game.vertices[vertex_id].pos);
            let rect = egui::Rect::from_center_size(pos, egui::vec2(35.7, 50.0));

            //draw shadow
            painter.image(
                texture.id(),
                rect.translate(egui::vec2(0.0, 3.0)),
                egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
                egui::Color32::from_black_alpha(100),
            );

            //draw settlement
            painter.image(
                texture.id(),
                rect,
                egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }
    }
}