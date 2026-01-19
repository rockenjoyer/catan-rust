use crate::backend::game::Vertex;
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
    pub city: egui::TextureHandle,
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
    let image = image::open("assets/placements/city.png")
        .unwrap_or_else(|_| panic!("Failed to load city image!"))
        .to_rgba8();

    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();

    let city = ctx.load_texture(
        "city".to_string(),
        egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
        egui::TextureOptions::LINEAR,
    );

    CityTextures { city }
}

//draw all cities at the vertices
pub fn draw_cities(
    painter: &egui::Painter,
    vertices: &[Vertex],
    textures: &CityTextures,
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
) {
    for vertex in vertices {
        let pos = screen(vertex.pos);

        //centered square for city
        let rect = egui::Rect::from_center_size(pos, egui::vec2(60.0, 60.0));

        painter.image(
            textures.city.id(),
            rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }
}
