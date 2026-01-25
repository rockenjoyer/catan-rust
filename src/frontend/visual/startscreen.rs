use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

#[derive(Resource)]
pub struct StartscreenTexture {
    pub startscreen: egui::TextureHandle,
}

pub fn setup_startscreen_texture(
    mut commands: Commands,
    mut contexts: EguiContexts,
    texture: Option<Res<StartscreenTexture>>,
) {
    if texture.is_some() {
        return;
    }
    if let Ok(ctx) = contexts.ctx_mut() {
        commands.insert_resource(load_startscreen_texture(ctx));
        info!("Startscreen texture loaded successfully!");
    }
}

pub fn load_startscreen_texture(ctx: &egui::Context) -> StartscreenTexture {
    let load = |path: &str| {
        let img = image::open(path)
            .unwrap_or_else(|_| panic!("Failed to load Startscreen image: {}", path))
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

    StartscreenTexture {
        startscreen: load("assets/game/startscreen.jpg"),
    }
}

//draw background image filling the entire available area
pub fn draw_background(ui: &mut egui::Ui, texture: &StartscreenTexture) {
    let rect = ui.max_rect();
    ui.painter().image(
        texture.startscreen.id(),
        rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
        egui::Color32::WHITE,
    );
}
