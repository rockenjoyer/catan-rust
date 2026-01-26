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
pub fn draw_background(
    ui: &mut egui::Ui, 
    texture: &StartscreenTexture,
    available_size: egui::Vec2,
) {

    let aspect_ratio = 16.0 / 9.0;

    let (bg_width, bg_height) = if available_size.x / available_size.y > aspect_ratio {
        //if the screen is wider -> match width, height extends beyond screen
        let width = available_size.x;
        (width, width / aspect_ratio)
    } else {
        // if the screen is taller -> match height, width extends beyond screen
        let height = available_size.y;
        (height * aspect_ratio, height)
    };
    
    let center = ui.available_rect_before_wrap().center();
    let bg_rect = egui::Rect::from_center_size(
        center,
        egui::vec2(bg_width, bg_height)
    );

    ui.painter().image(
        texture.startscreen.id(),
        bg_rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
        egui::Color32::WHITE,
    );
}
