use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

//resource to store icon textures
#[derive(Resource)]
pub struct IconTextures {
    pub settings: egui::TextureHandle,
    pub rules: egui::TextureHandle,
    pub volume: egui::TextureHandle,
}

//resources to track window visibility state
#[derive(Resource)]
pub struct SettingsWindowState {
    pub open: bool,
}

impl Default for SettingsWindowState {
    fn default() -> Self {
        SettingsWindowState { open: false }
    }
}

#[derive(Resource)]
pub struct RulesWindowState {
    pub open: bool,
}

impl Default for RulesWindowState {
    fn default() -> Self {
        RulesWindowState { open: false }
    }
}

#[derive(Resource)]
pub struct VolumeWindowState {
    pub open: bool,
}

impl Default for VolumeWindowState {
    fn default() -> Self {
        VolumeWindowState { open: false }
    }
}

//load the icon textures into egui
pub fn setup_icon_textures(
    mut commands: Commands,
    mut contexts: EguiContexts,
    textures: Option<Res<IconTextures>>,
) {
    if textures.is_some() {
        return;
    }

    if let Ok(ctx) = contexts.ctx_mut() {
        let textures = load_icon_textures(ctx);
        commands.insert_resource(textures);
        info!("The icon textures have been loaded successfully!");
    }
}

pub fn load_icon_textures(ctx: &egui::Context) -> IconTextures {
    //load an image file and convert it to an egui texture, specifically rgba8 format
    let load = |path: &str| {
        let img = image::open(path)
            .unwrap_or_else(|_| panic!("Failed to load icon image: {path}"))
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

    IconTextures {
        settings: load("assets/icons/settings.png"),
        rules: load("assets/icons/rules.png"),
        volume: load("assets/icons/volume.png"),
    }
}
