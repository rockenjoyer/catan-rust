use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::game::{Game, GamePhase, RoadBuildingMode, Resource as GameResource};
use crate::frontend::interface::style::apply_style;
use crate::frontend::interface::log_panel::GameLog;
use crate::frontend::visual::{
    cards::{CardsTextures, draw_cards},
    road::{RoadTextures, draw_roads},
    settlement::{SettlementTextures, draw_settlements},
    city::{CityTextures, draw_cities},
    tile::{TileTextures, ClickedVertex, draw_tiles, draw_vertices},
    dice::{DiceRollState, draw_dice_roll},
};

//resource to track road building state
#[derive(Resource, Default)]
pub struct RoadBuildingState {
    pub last_two_vertices: Vec<usize>,
}

//resource to track building mode in normal play
#[derive(Resource, Default, PartialEq, Clone)]
pub enum BuildingMode {
    #[default]
    None,
    BuildingRoad,
    BuildingSettlement,
    UpgradingCity,
}

pub fn setup_game(
    mut context: EguiContexts,
    game: NonSend<Rc<RefCell<Game>>>,
    mut clicked_vertex: ResMut<ClickedVertex>,
    mut road_state: ResMut<RoadBuildingState>,
    mut dice_state: ResMut<DiceRollState>,
    mut building_mode: ResMut<BuildingMode>,
    mut game_log: ResMut<GameLog>,
    tile_textures: Option<Res<TileTextures>>,
    road_textures: Option<Res<RoadTextures>>,
    card_textures: Option<Res<CardsTextures>>,
    settlement_textures: Option<Res<SettlementTextures>>,
    city_textures: Option<Res<CityTextures>>,
    time: Res<Time>,
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
        apply_style(context);

        //update dice animation
        dice_state.update(time.delta_secs());

        //read game state for UI display
        let (current_phase, current_player_name, current_player_id, setup_placement, player_resources, player_settlements) = {
            let game = game.borrow();
            let player = &game.players[game.current_player];
            (
                game.game_phase,
                player.name.clone(),
                game.current_player,
                game.setup_placement,
                player.resources.clone(),
                player.settlements.clone(),
            )
        };

        //check if player has enough resources
        let has_road_resources = player_resources.get(&GameResource::Brick).unwrap_or(&0) >= &1
            && player_resources.get(&GameResource::Lumber).unwrap_or(&0) >= &1;
        
        let has_settlement_resources = player_resources.get(&GameResource::Brick).unwrap_or(&0) >= &1
            && player_resources.get(&GameResource::Lumber).unwrap_or(&0) >= &1
            && player_resources.get(&GameResource::Wool).unwrap_or(&0) >= &1
            && player_resources.get(&GameResource::Grain).unwrap_or(&0) >= &1;
        
        let has_city_resources = player_resources.get(&GameResource::Grain).unwrap_or(&0) >= &2
            && player_resources.get(&GameResource::Ore).unwrap_or(&0) >= &3;

        //track button clicks
        let mut should_build_settlement = false;
        let mut should_build_road = false;
        let mut should_build_city = false;
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

                //draw everything (in correct layer order)
                let game_borrow = game.borrow();
                
                //layer 1 (board background and tiles)
                draw_tiles(
                    ui,
                    &painter,
                    response.rect,
                    &game_borrow,
                    &*tile_textures,
                    &screen,
                );

                //layer 2 (roads)
                draw_roads(&painter, &game_borrow, &*road_textures, &screen);

                //layer 3 (vertices)
                draw_vertices(ui, &painter, response.rect, &game_borrow, &screen, clicked_vertex.as_mut());

                //layer 4 (settlements and cities)
                draw_settlements(&painter, &game_borrow, &*settlement_textures, &screen);
                draw_cities(&painter, &game_borrow, &*city_textures, &screen);
                
                //layer 5 (cards)
                 let cards_pos = egui::pos2(
                    response.rect.right() - 500.0,
                    response.rect.center().y + 200.0
                );
                draw_cards(
                    &painter,
                    &*card_textures,
                    cards_pos,
                    egui::vec2(85.0, 100.0),
                    6.0,
                );

                //layer 6 (dice)
                if dice_state.final_result.is_some() {
                    let dice_pos = egui::pos2(response.rect.right() - 150.0, response.rect.top() + 100.0);
                    draw_dice_roll(&painter, dice_pos, &*dice_state);
                }

                drop(game_borrow);
            });

        //control panel overlay
        egui::Window::new("Controls")
            .frame(egui::Frame::NONE)
            .resizable(false)
            .anchor(egui::Align2::LEFT_TOP, (10.0, 10.0))
            .collapsible(false)
            .default_size((300.0, 400.0))
            .order(egui::Order::Foreground)
            .show(context, |ui| {
                //show current game phase
                ui.label(format!("Phase: {:?}", current_phase));
                
                //show current player name with their color
                let player_color = match current_player_id {
                    0 => egui::Color32::from_rgb(200, 50, 50),   //red
                    1 => egui::Color32::from_rgb(50, 100, 200),  //blue
                    2 => egui::Color32::from_rgb(50, 200, 50),   //green
                    3 => egui::Color32::from_rgb(220, 200, 50),  //yellow
                    _ => egui::Color32::WHITE,
                };
                ui.horizontal(|ui| {
                    ui.label("Current Player: ");
                    ui.colored_label(player_color, &current_player_name);
                });

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

                            // Track clicked vertices
                            if let Some(vertex_id) = clicked_vertex.vertex_id {
                                // Add to last two vertices
                                road_state.last_two_vertices.push(vertex_id);
                                // Keep only last 2
                                if road_state.last_two_vertices.len() > 2 {
                                    road_state.last_two_vertices.remove(0);
                                }
                                clicked_vertex.vertex_id = None;
                            }

                            // Show current selection
                            if road_state.last_two_vertices.len() == 1 {
                                ui.label(format!("Selected: {}", road_state.last_two_vertices[0]));
                                ui.label("Click another vertex");
                            } else if road_state.last_two_vertices.len() == 2 {
                                ui.label(format!("Road: {} -> {}", 
                                    road_state.last_two_vertices[0], 
                                    road_state.last_two_vertices[1]));
                                
                                if ui.button("Build Road").clicked() {
                                    should_build_road = true;
                                }
                                
                                if ui.button("Cancel").clicked() {
                                    road_state.last_two_vertices.clear();
                                }
                            } else {
                                ui.label("Click first vertex");
                            }
                        }
                    }
                    GamePhase::NormalPlay => {
                        ui.label("Normal Play");
                        ui.separator();

                        if !dice_state.rolling && !dice_state.processed {
                            if ui.button("Roll Dice").clicked() {
                                should_roll_dice = true;
                            }
                        } else if dice_state.rolling {
                            ui.label("Rolling...");
                        } else {
                            ui.label("Dice rolled!");
                        }

                        ui.separator();
                        ui.label("Build Actions:");

                        //building buttons based on current mode
                        match building_mode.as_ref() {
                            BuildingMode::None => {
                                //show all available building options
                                if has_road_resources {
                                    if ui.button("Build Road (Brick + Lumber)").clicked() {
                                        *building_mode = BuildingMode::BuildingRoad;
                                        road_state.last_two_vertices.clear();
                                        game_log.add_info(format!("Road building mode activated"), time.elapsed_secs());
                                    }
                                } else {
                                    ui.add_enabled(false, egui::Button::new("Build Road (Need: Brick + Lumber)"));
                                }

                                if has_settlement_resources {
                                    if ui.button("Build Settlement (Brick + Lumber + Wool + Grain)").clicked() {
                                        *building_mode = BuildingMode::BuildingSettlement;
                                        game_log.add_info(format!("Settlement building mode activated"), time.elapsed_secs());
                                    }
                                } else {
                                    ui.add_enabled(false, egui::Button::new("Build Settlement (Need: Brick + Lumber + Wool + Grain)"));
                                }

                                if has_city_resources {
                                    if ui.button("Upgrade to City (2 Grain + 3 Ore)").clicked() {
                                        *building_mode = BuildingMode::UpgradingCity;
                                        game_log.add_info(format!("City upgrade mode activated"), time.elapsed_secs());
                                    }
                                } else {
                                    ui.add_enabled(false, egui::Button::new("Upgrade to City (Need: 2 Grain + 3 Ore)"));
                                }
                            }
                            BuildingMode::BuildingRoad => {
                                ui.colored_label(egui::Color32::YELLOW, "Building Road Mode");
                                
                                // Track clicked vertices
                                if let Some(vertex_id) = clicked_vertex.vertex_id {
                                    road_state.last_two_vertices.push(vertex_id);
                                    if road_state.last_two_vertices.len() > 2 {
                                        road_state.last_two_vertices.remove(0);
                                    }
                                    clicked_vertex.vertex_id = None;
                                }

                                if road_state.last_two_vertices.len() == 1 {
                                    ui.label(format!("First vertex: {}", road_state.last_two_vertices[0]));
                                    ui.label("Click second vertex");
                                } else if road_state.last_two_vertices.len() == 2 {
                                    ui.label(format!("Road: {} → {}", 
                                        road_state.last_two_vertices[0], 
                                        road_state.last_two_vertices[1]));
                                    
                                    if ui.button("Confirm Build Road").clicked() {
                                        should_build_road = true;
                                    }
                                } else {
                                    ui.label("Click first vertex");
                                }

                                if ui.button("Cancel").clicked() {
                                    *building_mode = BuildingMode::None;
                                    road_state.last_two_vertices.clear();
                                }
                            }
                            BuildingMode::BuildingSettlement => {
                                ui.colored_label(egui::Color32::YELLOW, "Building Settlement Mode");
                                
                                if let Some(vertex_id) = clicked_vertex.vertex_id {
                                    ui.label(format!("Selected vertex: {}", vertex_id));
                                    
                                    if ui.button("Confirm Build Settlement").clicked() {
                                        should_build_settlement = true;
                                    }
                                } else {
                                    ui.label("Click a vertex to build settlement");
                                }

                                if ui.button("Cancel").clicked() {
                                    *building_mode = BuildingMode::None;
                                    clicked_vertex.vertex_id = None;
                                }
                            }
                            BuildingMode::UpgradingCity => {
                                ui.colored_label(egui::Color32::YELLOW, "Upgrade to City Mode");
                                
                                if let Some(vertex_id) = clicked_vertex.vertex_id {
                                    //check if this vertex has a settlement owned by the current player
                                    if player_settlements.contains(&vertex_id) {
                                        ui.label(format!("Selected settlement at vertex: {}", vertex_id));
                                        
                                        if ui.button("Confirm Upgrade to City").clicked() {
                                            should_build_city = true;
                                        }
                                    } else {
                                        ui.colored_label(egui::Color32::RED, "No settlement here or not yours!");
                                    }
                                } else {
                                    ui.label("Click on one of your settlements");
                                }

                                if ui.button("Cancel").clicked() {
                                    *building_mode = BuildingMode::None;
                                    clicked_vertex.vertex_id = None;
                                }
                            }
                        }

                        ui.separator();

                        if dice_state.processed {
                            if ui.button("End Turn").clicked() {
                                should_end_turn = true;
                            }
                        } else {
                            ui.add_enabled(false, egui::Button::new("End Turn (Roll dice first!)"));
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
                        game_log.add_info(format!("Settlement built successfully!"), time.elapsed_secs());
                        clicked_vertex.vertex_id = None;
                        *building_mode = BuildingMode::None;
                    }
                    Err(e) => {
                        game_log.add_warn(format!("Failed to build settlement: {}", e), time.elapsed_secs());
                    }
                }
            }
        }

        if should_build_road {
             if road_state.last_two_vertices.len() == 2 {
                
                let first = road_state.last_two_vertices[0];
                let second = road_state.last_two_vertices[1];

                let mut game = game.borrow_mut();

                match game.build_road(current_player_id, first, second, RoadBuildingMode::Normal) {
                    Ok(_) => {
                        game_log.add_info(format!("Road built between {} and {}", first, second), time.elapsed_secs());
                        road_state.last_two_vertices.clear();
                        clicked_vertex.selected_vertex = None;
                        *building_mode = BuildingMode::None;
                    }
                    Err(e) => {
                        game_log.add_warn(format!("Failed to build road: {}", e), time.elapsed_secs());
                        road_state.last_two_vertices.clear();
                    }
                }
            }
        }

        if should_build_city {
            if let Some(vertex_id) = clicked_vertex.vertex_id {
                let mut game = game.borrow_mut();
                match game.build_city(current_player_id, vertex_id) {
                    Ok(_) => {
                        game_log.add_info(format!("Settlement upgraded to city!"), time.elapsed_secs());
                        clicked_vertex.vertex_id = None;
                        *building_mode = BuildingMode::None;
                    }
                    Err(e) => {
                        game_log.add_warn(format!("Failed to upgrade to city: {}", e), time.elapsed_secs());
                    }
                }
            }
        }

        if should_roll_dice && !dice_state.rolling {
            //generate dice roll values
            use rand::Rng;
            let mut rng = rand::rng();
            let die1 = rng.random_range(1..=6);
            let die2 = rng.random_range(1..=6);
            dice_state.start_roll((die1, die2));
        }

        //process the roll only when the animation finishes and hasnt been processed yet
        if !dice_state.rolling && dice_state.final_result.is_some() && !dice_state.processed {
            if let Some((d1, d2)) = dice_state.final_result {
                let total = d1 + d2;
                let mut game = game.borrow_mut();
                
                //distribute resources/handle robber based on the roll
                if total == 7 {
                    // Placeholder - implement robbery and robber movement in UI later
                    game_log.add_info(format!("Rolled 7 - robber activates!"), time.elapsed_secs());
                } else {
                    //collect resource updates first
                    let mut updates: Vec<(usize, GameResource, u8)> = Vec::new();
                    
                    for (tile_idx, tile) in game.tiles.iter().enumerate() {
                        if tile.number_token == Some(total) && tile_idx != game.robber_tile {
                            for &vertex_idx in &tile.vertices {
                                for (player_idx, player) in game.players.iter().enumerate() {
                                    let mut amount = 0;
                                    if player.settlements.contains(&vertex_idx) { amount += 1; }
                                    if player.cities.contains(&vertex_idx) { amount += 2; }
                                    if amount > 0 {
                                        updates.push((player_idx, tile.resource, amount));
                                    }
                                }
                            }
                        }
                    }
                    
                    //apply updates
                    for (player_idx, resource, amount) in updates {
                        *game.players[player_idx].resources.entry(resource).or_insert(0) += amount;
                    }
                }
                
                game_log.add_info(format!("Roll completed: {}", total), time.elapsed_secs());
                dice_state.processed = true; //mark as processed but keep dice visible
            }
        }

        if should_end_turn {
            let mut game = game.borrow_mut();
            game.next_turn();
            game_log.add_info(format!("Turn ended"), time.elapsed_secs());
            *building_mode = BuildingMode::None; //reset building mode on turn end

            //reset dice for next turn
            dice_state.final_result = None;
            dice_state.processed = false;
        }
    }
}