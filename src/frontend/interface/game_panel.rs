use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::game::{Game, GamePhase, RoadBuildingMode};
use crate::frontend::interface::style::apply_style;
use crate::frontend::visual::{
    cards::{CardsTextures, draw_cards},
    road::{RoadTextures, draw_roads},
    settlement::{SettlementTextures, draw_settlements},
    city::{CityTextures, draw_cities},
    tile::{TileTextures, TileShowing, ClickedVertex, draw_tiles},
};

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
    tile_textures: Option<Res<TileTextures>>,
    road_textures: Option<Res<RoadTextures>>,
    card_textures: Option<Res<CardsTextures>>,
    settlement_textures: Option<Res<SettlementTextures>>,
    city_textures: Option<Res<CityTextures>>,
) {
    //wait for textures to load
    let Some(tile_textures) = tile_textures else { 
        info!("Waiting for tile textures...");
        return; 
    };
    let Some(road_textures) = road_textures else { 
        info!("Waiting for road textures...");
        return; 
    };
    let Some(card_textures) = card_textures else { 
        info!("Waiting for card textures...");
        return; 
    };
    let Some(settlement_textures) = settlement_textures else { 
        info!("Waiting for settlement textures...");
        return; 
    };
    let Some(city_textures) = city_textures else { 
        info!("Waiting for city textures...");
        return; 
    };

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

        egui::Window::new("Game Board")
            .frame(egui::Frame::NONE)
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .title_bar(false)
            .fixed_pos(egui::Pos2::ZERO)
            .fixed_size(context.available_rect().size())
            .order(egui::Order::Background)
            .show(context, |ui| {
                //size of the board area
                let size = ui.available_size();
                let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());

                let scale = 65.0;
                let origin = response.rect.center();
                let screen = |(x, y): (f32, f32)| egui::pos2(origin.x + x * scale, origin.y + y * scale);

                //draw everything
                let game_borrow = game.borrow();
                draw_tiles(
                    ui,
                    &painter,
                    response.rect,
                    &game_borrow,
                    &tile_textures,
                    &screen,
                    clicked_vertex.as_mut(),
                );
                draw_roads(&painter, &game_borrow, &road_textures, &screen);
                draw_settlements(&painter, &game_borrow, &settlement_textures, &screen);
                draw_cities(&painter, &game_borrow, &city_textures, &screen);
                draw_cards(
                    &painter,
                    &card_textures,
                    egui::pos2(100.0, 100.0),
                    egui::vec2(100.0, 130.0),
                    10.0,
                );
                drop(game_borrow);
            });

        //control panel overlay
        egui::Window::new("Controls")
            .frame(egui::Frame::NONE)
            .resizable(false)
            .anchor(egui::Align2::LEFT_TOP, (10.0, 10.0))
            .collapsible(false)
            .default_size((300.0, 400.0))
            .show(context, |ui| {
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
            });

        //handle actions after the UI is done
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
