use crate::backend::game::Vertex;
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
    pub settlement: egui::TextureHandle,
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
    let image = image::open("assets/placements/settlement.png")
        .unwrap_or_else(|_| panic!("Failed to load settlement image!"))
        .to_rgba8();

    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();

    let settlement = ctx.load_texture(
        "settlement".to_string(),
        egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
        egui::TextureOptions::LINEAR,
    );

    SettlementTextures { settlement }
}

//draw all settlements at the vertices
pub fn draw_settlements(
    painter: &egui::Painter,
    vertices: &[Vertex],
    textures: &SettlementTextures,
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
) {
    for vertex in vertices {
        let pos = screen(vertex.pos);

        //centered square for settlement
        let rect = egui::Rect::from_center_size(pos, egui::vec2(60.0, 60.0));

        painter.image(
            textures.settlement.id(),
            rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }
}
