use bevy_egui::egui;

pub fn apply_style(ctx: &egui::Context) {
    //first, customize text styles and their color
    ctx.style_mut(|style| {
        //heading title size
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(26.0, egui::FontFamily::Proportional),
        );

        //window title color
        style.visuals.widgets.noninteractive.fg_stroke.color =
            egui::Color32::from_hex("#291d11").unwrap();

        //label color
        style.visuals.override_text_color = Some(egui::Color32::from_hex("#291d11").unwrap());
    });

    //load custom fonts
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "catan".to_owned(),
        egui::FontData::from_static(include_bytes!("../../../assets/fonts/Cinzel.ttf")).into(),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "catan".to_owned());

    ctx.set_fonts(fonts);
}
