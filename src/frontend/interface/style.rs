use bevy_egui::egui;

pub fn apply_style(ctx: &egui::Context) {
    ctx.style_mut(|style| {
        //window title size
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(26.0, egui::FontFamily::Proportional),
        );

        //window title color
        style.visuals.widgets.noninteractive.fg_stroke.color =
            egui::Color32::from_hex("#2e2114ff").unwrap();

        //label color
        style.visuals.override_text_color = Some(egui::Color32::from_hex("#2e2114ff").unwrap());

        //TO-DO: implement a nice layout
    });
}
