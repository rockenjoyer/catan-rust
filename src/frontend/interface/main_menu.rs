use bevy::prelude::*;
use bevy::window::{MonitorSelection, PrimaryWindow, WindowMode};
use bevy_egui::{EguiContexts, egui};

use crate::frontend::bevy::GameState;
use crate::frontend::interface::style::apply_style;
use crate::frontend::visual::startscreen::{StartscreenTexture, draw_background};

#[derive(Resource, Default)]
pub struct MainMenuState {
    pub show_credits: bool,
    pub show_rules: bool,
    pub show_window_settings: bool,
}

//draw the main menu with various buttons
pub fn setup_main_menu(
    mut context: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    texture: Option<Res<StartscreenTexture>>,
    mut menu_state: ResMut<MainMenuState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Some(texture) = texture else {
        return;
    };

    if let Ok(context) = context.ctx_mut() {
        apply_style(context);

        egui::CentralPanel::default().show(context, |ui| {
            //draw the background image
            draw_background(ui, &texture, ui.available_size());

            //draw UI
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);

                //game title
                ui.label(
                    egui::RichText::new("The Settlers of Catan")
                        .font(egui::FontId::proportional(72.0))
                        .color(egui::Color32::from_hex("#845549").unwrap())
                        .strong(),
                );
                ui.add_space(-35.0);
                ui.label(
                    egui::RichText::new("Rust Edition")
                        .font(egui::FontId::proportional(35.0))
                        .color(egui::Color32::from_hex("#845549").unwrap()),
                );

                //dynamic spacing based on window height (so it looks formatted on different resolutions)
                let spacing = (ui.available_height() * 0.2).clamp(0.0, 400.0);
                ui.add_space(spacing);

                //change button size and apply button style
                let button_size = egui::vec2(300.0, 60.0);
                button_style(ui);

                //start game button
                if ui
                    .add_sized(
                        button_size,
                        egui::Button::new(egui::RichText::new("Start Game").size(20.0)),
                    )
                    .clicked()
                {
                    next_state.set(GameState::InGame);
                }

                ui.add_space(15.0);

                //rules button
                if ui
                    .add_sized(
                        button_size,
                        egui::Button::new(egui::RichText::new("Rules").size(20.0)),
                    )
                    .clicked()
                {
                    menu_state.show_rules = !menu_state.show_rules;
                }

                ui.add_space(15.0);

                //window settings button
                if ui
                    .add_sized(
                        button_size,
                        egui::Button::new(egui::RichText::new("Window Settings").size(20.0)),
                    )
                    .clicked()
                {
                    menu_state.show_window_settings = !menu_state.show_window_settings;
                }

                ui.add_space(15.0);

                //credits button
                if ui
                    .add_sized(
                        button_size,
                        egui::Button::new(egui::RichText::new("Credits").size(20.0)),
                    )
                    .clicked()
                {
                    menu_state.show_credits = !menu_state.show_credits;
                }

                ui.add_space(15.0);

                //quit button
                if ui
                    .add_sized(
                        button_size,
                        egui::Button::new(egui::RichText::new("Quit Game").size(20.0)),
                    )
                    .clicked()
                {
                    std::process::exit(0);
                }
            });
        });

        //credits panel
        if menu_state.show_credits {
            show_credits(context, &mut menu_state.show_credits);
        }

        //rules panel
        if menu_state.show_rules {
            show_rules(context, &mut menu_state.show_rules);
        }

        //window settings panel
        if menu_state.show_window_settings {
            if let Ok(mut window) = windows.single_mut() {
                show_window_settings(context, &mut menu_state.show_window_settings, &mut window);
            }
        }
    }
}

fn show_credits(ctx: &egui::Context, show: &mut bool) {
    egui::Window::new("Credits")
        .open(show)
        .frame(window_frame())
        .collapsible(false)
        .default_size(egui::vec2(700.0, 300.0))
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("Development").strong().size(25.0));
                ui.label("Antonio | Roman | Vojin | Laura");

                ui.add_space(15.0);

                ui.label(egui::RichText::new("Built with").strong().size(25.0));
                ui.label("Bevy Engine | Rust Programming Language");

                ui.add_space(15.0);

                ui.label(egui::RichText::new("Original Game").strong().size(25.0));
                ui.label("The Settlers of Catan | by Klaus Teuber");
            });
        });
}

fn show_rules(ctx: &egui::Context, show: &mut bool) {
    egui::Window::new("Rules")
        .open(show)
        .frame(window_frame())
        .default_size(egui::vec2(700.0, 300.0))
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {

                ui.label(egui::RichText::new("1. Setup").strong().size(20.0));
                ui.add_space(10.0);
                ui.label(egui::RichText::new("There are six types of tiles: Brick, Lumber, Wool, Grain, Ore and Desert.")
                    .size(14.0));
                ui.label(egui::RichText::new("Each player starts with 2 roads and 2 settlements.")
                    .size(14.0));

                ui.add_space(10.0);

                ui.label(egui::RichText::new("2. Resources").strong().size(20.0));
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Produced by tiles when the dice roll matches the tiles' number. Cities produce double resources.")
                    .size(14.0));

                ui.add_space(10.0);

                ui.label(egui::RichText::new("3. Turn Structure").strong().size(20.0));
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Roll dice -> distribute resources. Trade with players or the bank at harbors.")
                    .size(14.0));
                ui.label(egui::RichText::new("Roads (1 Brick + 1 Lumber) | Settlements (1 Brick + 1 Lumber + 1 Grain + 1 Wool) | Cities (2 Grain + 3 Ore) | Buy Development Cards (1 Wool + 1 Grain + 1 Ore)")
                    .size(14.0));

                ui.add_space(10.0);

                ui.label(egui::RichText::new("4. Robber").strong().size(20.0));
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Moves when 7 is rolled or Knight card is played.")
                    .size(14.0));
                ui.label(egui::RichText::new("Blocks resource production and steals 1 card from a player adjacent to the tile. Players with > 7 cards discard half.")
                    .size(14.0));
                ui.add_space(10.0);

                ui.label(egui::RichText::new("5. Victory Points").strong().size(20.0));
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Settlement = 1 VP | City = 2 VP | VP card = 1 VP | Longest Road & Largest Army = 2 VP")
                    .size(14.0));
                ui.label(egui::RichText::new("First to 10 VP wins.")
                    .size(14.0));

            });
        });
}

fn show_window_settings(ctx: &egui::Context, show: &mut bool, window: &mut Window) {
    egui::Window::new("Window Settings")
        .open(show)
        .frame(window_frame())
        .default_size(egui::vec2(700.0, 400.0))
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                button_style(ui);
                ui.add_space(10.0);

                //space for window mode settings
                ui.label(egui::RichText::new("Window Mode").strong().size(24.0));
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.add_space(50.0);
                    let current_mode = window.mode;

                    //add window mode selection buttons: windowed and borderless fullscreen
                    if ui
                        .selectable_label(
                            matches!(current_mode, WindowMode::Windowed),
                            egui::RichText::new("Windowed").size(18.0),
                        )
                        .clicked()
                    {
                        window.mode = WindowMode::Windowed;
                    }

                    ui.add_space(20.0);

                    if ui
                        .selectable_label(
                            matches!(current_mode, WindowMode::BorderlessFullscreen(_)),
                            egui::RichText::new("Borderless Fullscreen").size(18.0),
                        )
                        .clicked()
                    {
                        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
                    }
                });

                ui.add_space(20.0);

                //space for window resolution settings
                ui.label(egui::RichText::new("Window Resolution").strong().size(24.0));
                ui.add_space(10.0);

                let resolutions = vec![
                    ("1280 x 720", 1280.0, 720.0),
                    ("1920 x 1080", 1920.0, 1080.0),
                    ("2560 x 1440", 2560.0, 1440.0),
                ];

                //add resolution buttons
                for (label, width, height) in resolutions {
                    ui.horizontal(|ui| {
                        ui.add_space(150.0);
                        if ui.button(egui::RichText::new(label).size(18.0)).clicked() {
                            window.resolution.set(width, height);
                        }
                    });
                    ui.add_space(8.0);
                }

                ui.add_space(20.0);

                //display current resolution
                ui.label(
                    egui::RichText::new(format!(
                        "Current: {} x {}",
                        window.resolution.width(),
                        window.resolution.height()
                    ))
                    .size(16.0),
                );
            });
        });
}

fn window_frame() -> egui::Frame {
    egui::Frame::NONE
        .fill(egui::Color32::from_hex("#4c2a21").unwrap())
        .stroke(egui::Stroke::new(
            2.0,
            egui::Color32::from_hex("#331812").unwrap(),
        ))
        .inner_margin(20.0)
        .outer_margin(0.0)
        .corner_radius(egui::CornerRadius::same(20))
}

fn button_style(ui: &mut egui::Ui) {
    let button_color = egui::Color32::from_hex("#845549ea").unwrap();
    let outline_color = egui::Color32::from_hex("#3e211ae6").unwrap();

    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = button_color;
    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = button_color;
    ui.style_mut().visuals.widgets.active.weak_bg_fill = button_color;

    ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::new(2.0, outline_color);
    ui.style_mut().visuals.widgets.hovered.bg_stroke = egui::Stroke::new(2.0, outline_color);
    ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke::new(2.0, outline_color);
}