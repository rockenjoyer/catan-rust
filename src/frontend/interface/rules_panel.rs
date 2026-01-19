use crate::frontend::interface::style::apply_style;
use bevy_egui::{EguiContexts, egui};

pub fn setup_rules(mut context: EguiContexts) {
    if let Ok(context) = context.ctx_mut() {
        apply_style(context);

        let default_size = egui::vec2(500.0, 500.0);

        egui::Window::new("Rules")
            .frame(rules_frame())
            .order(egui::Order::Foreground)
            .default_size(default_size)
            .anchor(egui::Align2::RIGHT_TOP, (0.0, 0.0))
            .default_open(false)
            .scroll(true)
            
            .show(context, |ui| {
                ui.separator();
                ui.label(
                    "
1. Setup -------------

- Board: Hex tiles (Brick, Lumber, Wool, Grain, Ore, Desert).
- Number tokens on each hex (2 - 12, except desert).
- Players place 2 settlements and 2 roads at the start.

2. Resources -------------

- Produced by hexes when the dice roll matches the hex’s number.
- Types: Brick, Lumber, Wool, Grain, Ore.
- Cities produce double resources.

3. Turn Structure -------------

- Roll dice -> distribute resources.
- Trade with players or bank/harbors.
- Build:
    Roads (1 Brick + 1 Lumber)
    Settlements (1 Brick + 1 Lumber + 1 Grain + 1 Wool)
    Cities (2 Grain + 3 Ore)
    Buy Development Cards (1 Wool + 1 Grain + 1 Ore)

4. Robber -------------

- Moves when 7 is rolled or Knight card is played.
- Blocks resource production and steals 1 card from a player adjacent to the tile.
- Players with >7 cards discard half when a 7 is rolled.

5. Special Bonuses -------------

- Longest Road: 5+ continuous roads → 2 VP.
- Largest Army: 3+ Knight cards played → 2 VP.

6. Victory Points -------------

- Settlement = 1 VP
- City = 2 VP
- Victory Point card = 1 VP
- Longest Road / Largest Army = 2 VP each
- Goal: First to 10 VP wins.
                ",
                );
            });
    }
}

fn rules_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(egui::Color32::from_hex("#d4c1b1bd").unwrap())
        .corner_radius(egui::CornerRadius::same(15))
}