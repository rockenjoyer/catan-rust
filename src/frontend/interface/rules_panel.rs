use crate::frontend::interface::style::apply_style;
use bevy_egui::{EguiContexts, egui};

pub fn setup_rules(mut context: EguiContexts) {
    if let Ok(context) = context.ctx_mut() {
        apply_style(context);

        let default_size = egui::vec2(300.0, 400.0);

        egui::Window::new("Rules")
            .frame(window_frame())
            .order(egui::Order::Foreground)
            .default_size(default_size)
            .anchor(egui::Align2::RIGHT_BOTTOM, (0.0, 0.0))
            .default_open(false)
            .scroll(true)
            .show(context, |ui| {
                ui.separator();
                ui.label(
                    "
1. Setup
- Six types of tiles on the board: Brick, Lumber, Wool, Grain, Ore and Desert.
- Number tokens on each tile: 2 - 12, except for desert.
- Players place 2 settlements and 2 roads at the start.

2. Resources

- Produced by tiles when the dice roll matches the tiles' number.
- Types: Brick, Lumber, Wool, Grain, Ore.
- Cities produce double resources.

3. Turn Structure

- Roll dice -> distribute resources.
- Trade with players or the bank at harbors.
- Build:
    Roads (1 Brick + 1 Lumber).
    Settlements (1 Brick + 1 Lumber + 1 Grain + 1 Wool).
    Cities (2 Grain + 3 Ore).
    Buy Development Cards (1 Wool + 1 Grain + 1 Ore).

4. Robber

- Moves when 7 is rolled or Knight card is played.
- Blocks resource production and steals 1 card from a player adjacent to the tile.
- Players with > 7 cards discard half when a 7 is rolled.

5. Victory Points

- Settlement = 1 VP.
- City = 2 VP.
- Victory Point card = 1 VP.
- Longest Road & Largest Army = 2 VP each.
- Goal: First to 10 VP wins.
                ",
                );
            });
    }
}

fn window_frame() -> egui::Frame {
    egui::Frame::NONE
        .fill(egui::Color32::from_black_alpha(150))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_white_alpha(100)))
        .inner_margin(10.0)
        .outer_margin(0.0)
        .corner_radius(egui::CornerRadius::same(15))
}
