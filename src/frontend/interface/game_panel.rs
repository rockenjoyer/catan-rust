use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::game::{Game, GamePhase, RoadBuildingMode};
use crate::frontend::interface::style::apply_style;
use crate::frontend::visual::tile::{ClickedVertex, TileShowing, draw_tiles};

//resource to track road building state
#[derive(Resource, Default)]
pub struct RoadBuildingState {
    pub first_vertex: Option<usize>,
}

pub fn setup_game(
    mut context: EguiContexts,
    game: NonSend<Rc<RefCell<Game>>>,
    mut tiles_shown: ResMut<TileShowing>,
    mut clicked_vertex: ResMut<ClickedVertex>,
    mut road_state: ResMut<RoadBuildingState>,
) {
    if let Ok(context) = context.ctx_mut() {
        //main game window
        apply_style(context);

        //read game state for UI display
        let (current_phase, current_player_name, current_player_id, setup_placement) = {
            let game = game.borrow();
            (
                game.game_phase,
                game.players[game.current_player].name.clone(),
                game.current_player,
                game.setup_placement,
            )
        };

        
        //track button clicks
        let mut should_build_settlement = false;
        let mut should_build_road = false;
        let mut should_roll_dice = false;
        let mut should_end_turn = false;

        egui::Window::new("Main Game")
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_hex("#d4c1b1ff").unwrap())
                    .corner_radius(egui::CornerRadius::same(15)),
            )
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, (0.0, 0.0))
            .collapsible(false)
            .default_size((1500.0, 600.0))
            .show(context, |ui| {
                //hide and unhide tiles to test basic button behaviour
                if ui
                    .button(if tiles_shown.enabled {
                        "Hide Tiles"
                    } else {
                        "Show Tiles"
                    })
                    .clicked()
                {
                    tiles_shown.enabled = !tiles_shown.enabled;
                }
                ui.separator();
                
                //show current game phase
                ui.label(format!("Phase: {:?}", current_phase));
                ui.label(format!("Current Player: {}", current_player_name));

                ui.separator();

                //action buttons based on phase
                match current_phase {
                    GamePhase::SetupRound1 | GamePhase::SetupRound2 => {
                        //check what step of setup we are on
                        if setup_placement == 0 {
                            ui.label("Setup: Place your settlement");
                        
                        
                            if let Some(vertex_id) = clicked_vertex.vertex_id {
                                ui.label(format!("Selected vertex: {}", vertex_id));
                                if ui.button("Build Settlement").clicked() {
                                    should_build_settlement = true;
                                }
                            } else {
                                ui.label("Click a vertex to place a settlement");
                            }
                        } else if setup_placement == 1 {
                            ui.label("Setup: Place your road");

                            //road requires 2 vertices
                            if let Some(first) = road_state.first_vertex {
                                ui.label(format!("First vertex: {}", first));
                                ui.label("Click second vertex for road");

                                if let Some(second) = clicked_vertex.vertex_id {
                                    if second != first {
                                        ui.label(format!("Second vertex: {}", second));
                                        if ui.button("Build Road").clicked() {
                                            should_build_road = true;
                                        }
                                        if ui.button("Cancel").clicked() {
                                            road_state.first_vertex = None;
                                            clicked_vertex.vertex_id = None;
                                        }
                                    } else {
                                        ui.label("Select a different vertex!");
                                    }
                                }
                            } else {
                                ui.label("Click first vertex for road");
                                if clicked_vertex.vertex_id.is_some() {
                                    ui.label(format!("Vertex {} selected", clicked_vertex.vertex_id.unwrap()));
                                    if ui.button("Use this vertex").clicked() {
                                        road_state.first_vertex = clicked_vertex.vertex_id;
                                        clicked_vertex.vertex_id = None;
                                    }
                                }
                            }
                        }
                    }
                    GamePhase::NormalPlay => {
                        ui.label("Normal Play");

                        if ui.button("Roll Dice").clicked() {
                            should_roll_dice = true;
                        }

                        if let Some(vertex_id) = clicked_vertex.vertex_id {
                            ui.label(format!("Selected vertex: {}", vertex_id));

                            if ui.button(format!("Build settlement at vertex {}", vertex_id)).clicked() {
                                should_build_settlement = true;
                            }
                        }

                        if ui.button("End Turn").clicked() {
                            should_end_turn = true;
                        }
                    }
                }
                
                ui.separator();
                                
                //draw tiles
                if tiles_shown.enabled {
                    let game = game.borrow();
                    draw_tiles(ui, &game, clicked_vertex.as_mut());
                }
            });
        
        //handle actions after thr UI is done
        if should_build_settlement {
            if let Some(vertex_id) = clicked_vertex.vertex_id {
                let mut game = game.borrow_mut();
                match game.build_settlement(current_player_id, vertex_id) {
                    Ok(_) => {
                        info!("Settlement built successfully!");

                        info!("Setup placement is now: {}", game.setup_placement);

                        clicked_vertex.vertex_id = None;
                    }
                    Err(e) => {
                        warn!("Failed to build settlement: {}", e);
                    }
                }
            }
        }

        if should_build_road {
            if let (Some(first), Some(second)) = (road_state.first_vertex, clicked_vertex.vertex_id) {
                let mut game = game.borrow_mut();

                info!("Attempting to build road between {} and {}", first, second);

                match game.build_road(current_player_id, first, second, RoadBuildingMode::Normal) {
                    Ok(_) => {
                        info!("Road built! Setup placement now: {}", game.setup_placement);
                        info!("Current player now: {}", game.current_player);
                        road_state.first_vertex = None;
                        clicked_vertex.vertex_id = None;
                    }
                    Err(e) => {
                        warn!("Failed: {}", e);
                        road_state.first_vertex = None;
                    }
                }
            }
        }

        if should_roll_dice {
            let mut game = game.borrow_mut();
            let roll = game.roll_dice();
            info!("Rolled: {}", roll);
        }

        if should_end_turn {
            let mut game = game.borrow_mut();
            game.next_turn();
            info!("Turn ended");
        }
    }
}
