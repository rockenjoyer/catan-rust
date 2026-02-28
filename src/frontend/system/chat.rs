use bevy::prelude::*;
use bevy_ecs::system::ResMut;
use bevy_egui::{egui, EguiContexts};
use bevy_quinnet::client::QuinnetClient;
use crate::backend::networking::protocol::ClientMessage;

#[derive(Resource, Default)]
pub struct ChatState {
    pub messages: Vec<String>,
    pub input: String,
}

/// Generates the chat box.
/// Currently only used in the lobby and game instance started from the lobby.
pub fn render_chat_ui(
    mut contexts: EguiContexts,
    mut chat_state: ResMut<ChatState>,
    mut client: ResMut<QuinnetClient>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Chat")
        .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(60.0, -10.0))
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .frame(egui::Frame::NONE.fill(egui::Color32::from_black_alpha(100)).inner_margin(5.0))
        .show(ctx.expect(""), |ui| {
            ui.set_width(300.0);

            egui::ScrollArea::vertical()
                .max_height(150.0)
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for msg in &chat_state.messages {
                        ui.label(egui::RichText::new(msg).color(egui::Color32::WHITE));
                    }
                });

            ui.separator();

            let text_edit_response = ui.add(
                egui::TextEdit::singleline(&mut chat_state.input)
                    .hint_text("Click to chat...")
                    .desired_width(f32::INFINITY)
            );

            if text_edit_response.changed() {
                println!("TextEdit changed");
                text_edit_response.request_focus();
            }

            if ui.input(|i| i.key_pressed(egui::Key::Enter)) && !chat_state.input.trim().is_empty() {
                let msg_text = chat_state.input.trim().to_string();
                handle_chat_input(msg_text, &mut client);
                chat_state.input.clear();
            }
        });
}

/// Handles chat inputs and looks out for specific commands
pub fn handle_chat_input(
    input: String,
    client: &mut ResMut<QuinnetClient>,
) {
    if input.starts_with('/') {
        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts[0] {
            "/quit" => {
                let payload = bincode::serialize(&ClientMessage::Disconnect).unwrap();
                let _ = client.connection_mut().try_send_payload(payload);
            }
            "/help" => {
                println!("No help >:( ... Just kidding, try /quit");
            }
            _ => { println!("ERROR: Not a valid command") }
        }
    } else {
        let msg = ClientMessage::ChatMessage { message: input.clone() };
        if let Ok(payload) = bincode::serialize(&msg) {
            let _ = client.connection_mut().try_send_payload(payload);
        } else {
            println!("ERROR: Failed to serialize chat message!")
        }
    }
}