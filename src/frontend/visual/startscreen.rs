use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

#[derive(Resource)]
pub struct StartscreenTexture {
    pub startscreen: egui::TextureHandle,
}

#[derive(Resource)]
pub struct LogoTexture {
    pub logo: egui::TextureHandle,
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

pub fn setup_logo(
    mut commands: Commands,
    mut contexts: EguiContexts,
    texture: Option<Res<LogoTexture>>,
) {
    if texture.is_some() {
        return;
    }
    if let Ok(ctx) = contexts.ctx_mut() {
        commands.insert_resource(load_logo_texture(ctx));
        info!("Logo texture loaded successfully!");
    }
}


pub fn load_logo_texture(ctx: &egui::Context) -> LogoTexture {
    let load = |path: &str| {
        let img = image::open(path)
            .unwrap_or_else(|_| panic!("Failed to load Logo image: {}", path))
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

    LogoTexture {
        logo: load("assets/game/logo.png"),
    }
}

//draw background image filling the entire available area
pub fn draw_background(
    ui: &mut egui::Ui,
    background: &StartscreenTexture,
    logo_image: &LogoTexture,
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
    let bg_rect = egui::Rect::from_center_size(center, egui::vec2(bg_width, bg_height));

    ui.painter().image(
        background.startscreen.id(),
        bg_rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
        egui::Color32::WHITE,
    );

    //falling particles effect
    let now = ui.input(|i| i.time) as f32;
    let particle_count = 50;
    let mut rng = oorandom::Rand32::new(0);
    for i in 0..particle_count {
        let i = i as f32;
        //each particle has a horizontal offset and speed
        let base_x = rng.rand_float() * bg_width;
        let speed = 20.0 + 25.0 * rng.rand_float(); //pixels per second
        //for a staggered start
        let phase = rng.rand_float(); 
        //vertical position is based on time, speed, and phase
        let time = (now + phase * 10.0) * speed;

        //y wraps around for a coninuous falling effect
        let y = (time % (bg_height + 40.0)) - 20.0;
        let pos = egui::pos2(bg_rect.left() + base_x, bg_rect.top() + y);
        //radius with a pulsing effect
        let radius = 8.0 + 4.0 * (i * 0.5 + now).sin();

        //base color with randomized alpha channel
        let base = egui::Color32::from_hex("#613426fb").unwrap();
        let alpha = (120.0 + 80.0 * (i + now * 0.7).sin()) as u8;
        let color = egui::Color32::from_rgba_unmultiplied(base.r(), base.g(), base.b(), alpha);

        ui.painter().circle_filled(pos, radius, color);
    }

    //draw logo on top, also with dynamic scaling
    let logo_size = logo_image.logo.size_vec2();
    let max_logo_width = available_size.x * 0.7;
    let max_logo_height = available_size.y * 0.3;

    //preserve the aspect ratio
    let scale = (max_logo_width / logo_size.x)
        .min(max_logo_height / logo_size.y)
        .min(1.0);
    let scaled_logo = logo_size * scale;

    let logo_rect = ui.available_rect_before_wrap();
    let top_center = egui::pos2(logo_rect.center().x, logo_rect.min.y);

    //position logo a bit below the top center
    let logo_pos = egui::pos2(
        top_center.x - scaled_logo.x * 0.5,
        top_center.y + 50.0,
    );

    let logo_rect = egui::Rect::from_min_size(logo_pos, scaled_logo);
    
    //draw logo
    ui.painter().image(
        logo_image.logo.id(),
        logo_rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
        egui::Color32::WHITE,
    );
}

