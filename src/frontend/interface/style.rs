use bevy_egui::egui;
use std::sync::Arc;

pub fn apply_style(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    fonts.font_data.insert(
        "cinzel".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!("../../../assets/fonts/Cinzel.ttf"))),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "cinzel".to_owned());

    ctx.set_fonts(fonts);

    ctx.style_mut(|style| {
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

        //all text white
        style.visuals.override_text_color = Some(egui::Color32::WHITE);
    });
}

pub fn text_with_background(
    ui: &mut egui::Ui,
    text: String,
    font_size: f32,
) -> egui::Response {
    let text_str = text.to_string().replace('\n', " ");

    let galley = ui.fonts_mut(|f| {
        f.layout_no_wrap(
            text_str.clone(),
            egui::FontId::proportional(font_size),
            egui::Color32::WHITE,
        )
    });
    let text_size = egui::vec2(galley.rect.width(), font_size);
    let (_, rect) = ui.allocate_space(text_size + egui::vec2(20.0, 10.0));

    ui.painter().rect_filled(
        rect,
        egui::CornerRadius::same(5),
        egui::Color32::from_rgba_unmultiplied(120, 80, 60, 200),
    );

    ui.put(rect, egui::Label::new(text_str))
}