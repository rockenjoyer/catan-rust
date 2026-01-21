use bevy_egui::egui;
use std::sync::Arc;

pub fn apply_style(ctx: &egui::Context) {
    // Load the Cinzel font
    let mut fonts = egui::FontDefinitions::default();
    
    fonts.font_data.insert(
        "cinzel".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!("../../../assets/fonts/Cinzel.ttf"))),
    );

    // Set Cinzel as the first priority font for Proportional family
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "cinzel".to_owned());

    ctx.set_fonts(fonts);

    // Style the text
    ctx.style_mut(|style| {
        // Make text larger
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(16.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(16.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(28.0, egui::FontFamily::Proportional),
        );

        // All text black
        style.visuals.override_text_color = Some(egui::Color32::BLACK);
    });
}