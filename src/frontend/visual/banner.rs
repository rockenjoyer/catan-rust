use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

//resource to store banner textures
#[derive(Resource)]
pub struct BannerTextures {
    pub viewer_banner: egui::TextureHandle,
    //more to be added later
}

//load the banner textures into egui
pub fn setup_banner_textures(
    mut commands: Commands,
    mut contexts: EguiContexts,
    textures: Option<Res<BannerTextures>>,
) {
    if textures.is_some() {
        return;
    }

    if let Ok(ctx) = contexts.ctx_mut() {
        let textures = load_banner_textures(ctx);
        commands.insert_resource(textures);
        info!("Banner textures have been loaded successfully!");
    }
}

pub fn load_banner_textures(ctx: &egui::Context) -> BannerTextures {
    let viewer_banner = load_viewer_banner(ctx);

    BannerTextures { viewer_banner }
}

pub fn load_viewer_banner(ctx: &egui::Context) -> egui::TextureHandle {
    let image = image::open("assets/banner/viewer_banner.png")
        .unwrap_or_else(|_| panic!("Failed to load viewer banner image!"))
        .to_rgba8();

    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();

    ctx.load_texture(
        "viewer_banner".to_string(),
        egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
        egui::TextureOptions::LINEAR,
    )
}

//draw the banner, use the function before drawing any UI elements
pub fn draw_viewer_banner_background(ui: &mut egui::Ui, texture: &egui::TextureHandle) {
    let rect = ui.available_rect_before_wrap();
    let painter = ui.painter();
    painter.image(
        texture.id(),
        rect,
        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        egui::Color32::WHITE,
    );
}
