use bevy::prelude::*;
use bevy_egui::egui;

#[derive(Resource)]
pub struct DiceRollState {
    pub rolling: bool,
    pub animation_timer: f32,
    pub current_display: (u8, u8),
    pub final_result: Option<(u8, u8)>,
    pub roll_duration: f32,
}

impl Default for DiceRollState {
    fn default() -> Self {
        Self {
            rolling: false,
            animation_timer: 0.0,
            current_display: (1, 1),
            final_result: None,
            roll_duration: 1.0, //1 second animation
        }
    }
}

impl DiceRollState {
    pub fn start_roll(&mut self, result: (u8, u8)) {
        self.rolling = true;
        self.animation_timer = 0.0;
        self.final_result = Some(result);
        self.current_display = (1, 1);
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.rolling {
            return;
        }

        self.animation_timer += delta_time;

        if self.animation_timer >= self.roll_duration {
            //animation finished
            self.rolling = false;
            if let Some((d1, d2)) = self.final_result {
                self.current_display = (d1, d2);
            }
        } else {
            //still animating -> show random numbers
            use rand::Rng;
            let mut rng = rand::rng();
            self.current_display = (
                rng.random_range(1..=6),
                rng.random_range(1..=6),
            );
        }
    }
}

//draw a dice
pub fn draw_die(
    painter: &egui::Painter,
    pos: egui::Pos2,
    size: f32,
    value: u8,
    rolling: bool,
) {
    let rect = egui::Rect::from_center_size(pos, egui::vec2(size, size));
    
    //wobble effect while rolling
    let wobble = if rolling {
        let wobble_amount = 5.0;
        egui::vec2(
            (pos.x * 0.1).sin() * wobble_amount,
            (pos.y * 0.1).cos() * wobble_amount,
        )
    } else {
        egui::vec2(0.0, 0.0)
    };
    
    let wobbled_rect = rect.translate(wobble);

    //draw die body
    painter.rect(
        wobbled_rect,
        2.0,
        egui::Color32::WHITE,
        egui::Stroke::new(3.0, egui::Color32::BLACK),
        egui::StrokeKind::Outside,
    );

    //draw dots based on value
    let dot_size = size / 8.0;
    let spacing = size / 4.0;
    let center = wobbled_rect.center();

    let draw_dot = |x_offset: f32, y_offset: f32| {
        let dot_pos = egui::pos2(
            center.x + x_offset * spacing,
            center.y + y_offset * spacing,
        );
        painter.circle_filled(dot_pos, dot_size, egui::Color32::BLACK);
    };

    match value {
        1 => {
            draw_dot(0.0, 0.0); //center
        }
        2 => {
            draw_dot(-1.0, -1.0); //top-left
            draw_dot(1.0, 1.0);   //bottom-right
        }
        3 => {
            draw_dot(-1.0, -1.0); //top-left
            draw_dot(0.0, 0.0);   //center
            draw_dot(1.0, 1.0);   //bottom-right
        }
        4 => {
            draw_dot(-1.0, -1.0); //top-left
            draw_dot(1.0, -1.0);  //top-right
            draw_dot(-1.0, 1.0);  //bottom-left
            draw_dot(1.0, 1.0);   //bottom-right
        }
        5 => {
            draw_dot(-1.0, -1.0); //top-left
            draw_dot(1.0, -1.0);  //top-right
            draw_dot(0.0, 0.0);   //center
            draw_dot(-1.0, 1.0);  //bottom-left
            draw_dot(1.0, 1.0);   //bottom-right
        }
        6 => {
            draw_dot(-1.0, -1.0); //top-left
            draw_dot(1.0, -1.0);  //top-right
            draw_dot(-1.0, 0.0);  //middle-left
            draw_dot(1.0, 0.0);   //middle-right
            draw_dot(-1.0, 1.0);  //bottom-left
            draw_dot(1.0, 1.0);   //bottom-right
        }
        _ => {} //invalid value
    }
}

//draw both dice with rolling animation
pub fn draw_dice_roll(
    painter: &egui::Painter,
    center_pos: egui::Pos2,
    dice_state: &DiceRollState,
) {
    let die_size = 60.0;
    let spacing = 80.0;

    //draw left die
    draw_die(
        painter,
        egui::pos2(center_pos.x - spacing / 2.0, center_pos.y),
        die_size,
        dice_state.current_display.0,
        dice_state.rolling,
    );

    //draw right die
    draw_die(
        painter,
        egui::pos2(center_pos.x + spacing / 2.0, center_pos.y),
        die_size,
        dice_state.current_display.1,
        dice_state.rolling,
    );

    //show total if not rolling
    if !dice_state.rolling {
        let total = dice_state.current_display.0 + dice_state.current_display.1;
        painter.text(
            egui::pos2(center_pos.x, center_pos.y + 60.0),
            egui::Align2::CENTER_CENTER,
            format!("Total: {}", total),
            egui::FontId::proportional(24.0),
            egui::Color32::BLACK,
        );
    }
}