use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use crate::backend::game::{Game, GamePhase, RoadBuildingMode, Resource as GameResource, DevCard, DevCardInput};
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

//resource to track dev card playing state
#[derive(Resource, Default)]
pub struct DevCardPlayState {
    pub selected_card: Option<(DevCard, usize)>, // (card_type, card_id)
    pub awaiting_input: Option<DevCard>,
    // For Knight
    pub selected_tile: Option<usize>,
    pub selected_victim: Option<usize>,
    // For Monopoly
    pub selected_resource: Option<GameResource>,
    // For Road Building
    pub road_building_roads: Vec<(usize, usize)>,
    // For Year of Plenty
    pub year_resources: Vec<GameResource>,
}

//resource to track robber movement when rolling 7
#[derive(Resource, Default)]
pub struct RobberMoveState {
    pub needs_movement: bool,
    pub selected_tile: Option<usize>,
    pub selected_victim: Option<usize>,
}

pub fn setup_game(
    mut context: EguiContexts,
    game: NonSend<Rc<RefCell<Game>>>,
    mut clicked_vertex: ResMut<ClickedVertex>,
    mut road_state: ResMut<RoadBuildingState>,
    mut dice_state: ResMut<DiceRollState>,
    mut building_mode: ResMut<BuildingMode>,
    mut dev_card_state: ResMut<DevCardPlayState>,
    mut robber_state: ResMut<RobberMoveState>,
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
        let (current_phase, current_player_name, current_player_id, setup_placement, player_resources, player_settlements, player_dev_cards_vec) = {
            let game = game.borrow();
            let player = &game.players[game.current_player];
            (
                game.game_phase,
                player.name.clone(),
                game.current_player,
                game.setup_placement,
                player.resources.clone(),
                player.settlements.clone(),
                player.dev_cards.clone(),
            )
        };

        //convert dev cards vec to hashmap for card display
        let mut player_dev_cards_map: HashMap<DevCard, usize> = HashMap::new();
        for dev_card_inst in &player_dev_cards_vec {
            *player_dev_cards_map.entry(dev_card_inst.card).or_insert(0) += 1;
        }

        //get unplayed dev card instances
        let player_dev_cards_instances: Vec<(DevCard, usize)> = player_dev_cards_vec.iter()
            .enumerate()
            .map(|(idx, dc)| (dc.card, idx))
            .collect();

        //check if player has enough resources
        let has_road_resources = player_resources.get(&GameResource::Brick).unwrap_or(&0) >= &1
            && player_resources.get(&GameResource::Lumber).unwrap_or(&0) >= &1;
        
        let has_settlement_resources = player_resources.get(&GameResource::Brick).unwrap_or(&0) >= &1
            && player_resources.get(&GameResource::Lumber).unwrap_or(&0) >= &1
            && player_resources.get(&GameResource::Wool).unwrap_or(&0) >= &1
            && player_resources.get(&GameResource::Grain).unwrap_or(&0) >= &1;
        
        let has_city_resources = player_resources.get(&GameResource::Grain).unwrap_or(&0) >= &2
            && player_resources.get(&GameResource::Ore).unwrap_or(&0) >= &3;

        let has_devcard_resources = player_resources.get(&GameResource::Wool).unwrap_or(&0) >= &1
            && player_resources.get(&GameResource::Grain).unwrap_or(&0) >= &1
            && player_resources.get(&GameResource::Ore).unwrap_or(&0) >= &1;

        //track button clicks
        let mut should_build_settlement = false;
        let mut should_build_road = false;
        let mut should_build_city = false;
        let mut should_buy_devcard = false;
        let mut should_roll_dice = false;
        let mut should_end_turn = false;
        let mut should_play_devcard = false;
        let mut clicked_dev_card: Option<(DevCard, usize)> = None;

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

                //scale based on screen size to match different resolutions
                let scale = size.x.min(size.y) / 17.0;

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
                    clicked_vertex.as_mut(),
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
                    response.rect.right() - 150.0,
                    response.rect.center().y + 150.0
                );
                clicked_dev_card = draw_cards(
                    ui,
                    &painter,
                    &*card_textures,
                    cards_pos,
                    egui::vec2(90.0, 125.0),
                    20.0,
                    &player_resources,
                    &player_dev_cards_map,
                    &player_dev_cards_instances,
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
            .frame(window_frame())
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

                            //track clicked vertices
                            if let Some(vertex_id) = clicked_vertex.vertex_id {
                                //add to last two vertices
                                road_state.last_two_vertices.push(vertex_id);
                                //keep only last 2
                                if road_state.last_two_vertices.len() > 2 {
                                    road_state.last_two_vertices.remove(0);
                                }
                                clicked_vertex.vertex_id = None;
                            }

                            //show current selection
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

                        //robber movement when rolling a 7
                        if robber_state.needs_movement {
                            ui.colored_label(egui::Color32::RED, "Rolled 7! Move the Robber");
                            ui.label("Select robber tile (click on board)");
                            
                            //get robbable players if tile is selected
                            let mut robbable_players: Vec<usize> = Vec::new();
                            if let Some(tile_id) = clicked_vertex.clicked_tile {
                                robber_state.selected_tile = Some(tile_id);
                                ui.label(format!("Selected tile: {}", tile_id));
                                
                                //calculate robbable players from the tile
                                let game_borrow = game.borrow();
                                if let Some(tile) = game_borrow.tiles.get(tile_id) {
                                    for &vertex_idx in &tile.vertices {
                                        for player in &game_borrow.players {
                                            if player.id != current_player_id && 
                                                (player.settlements.contains(&vertex_idx) || player.cities.contains(&vertex_idx)) {
                                                if !robbable_players.contains(&player.id) {
                                                    robbable_players.push(player.id);
                                                }
                                            }
                                        }
                                    }
                                }
                                drop(game_borrow);
                                
                                if robbable_players.is_empty() {
                                    ui.label("No players to rob on this tile");
                                    if ui.button("Confirm (No steal)").clicked() {
                                        robber_state.selected_victim = None;
                                        should_play_devcard = true; //reuse flag to trigger robber movement
                                    }
                                } else {
                                    ui.label("Select victim:");
                                    for player_id in &robbable_players {
                                        if ui.button(format!("Player {}", player_id)).clicked() {
                                            robber_state.selected_victim = Some(*player_id);
                                        }
                                    }
                                    
                                    if let Some(victim) = robber_state.selected_victim {
                                        ui.label(format!("Will steal from Player {}", victim));
                                        if ui.button("Confirm Robber Move").clicked() {
                                            should_play_devcard = true; //reuse flag to trigger robber movement
                                        }
                                    }
                                }
                            }
                        } else if !dice_state.rolling && !dice_state.processed {
                            if ui.button("Roll Dice").clicked() {
                                should_roll_dice = true;
                            }
                        } else if dice_state.rolling {
                            ui.label("Rolling...");
                        } else {
                            ui.label("Dice rolled!");
                        }

                        //dev card ui
                        if let Some((card_type, _card_id)) = dev_card_state.selected_card {
                            ui.separator();
                            ui.colored_label(egui::Color32::YELLOW, format!("Playing: {:?}", card_type));

                            match card_type {
                                DevCard::Knight => {
                                    ui.label("Select robber tile (click on board)");
                                    
                                    //get robbable players if tile is selected
                                    let mut robbable_players: Vec<usize> = Vec::new();
                                    if let Some(tile_id) = clicked_vertex.clicked_tile {
                                        dev_card_state.selected_tile = Some(tile_id);
                                        ui.label(format!("Selected tile: {}", tile_id));
                                        
                                        //calculate robbable players from the tile
                                        let game_borrow = game.borrow();
                                        if let Some(tile) = game_borrow.tiles.get(tile_id) {
                                            for &vertex_idx in &tile.vertices {
                                                for player in &game_borrow.players {
                                                    if player.id != current_player_id && 
                                                       (player.settlements.contains(&vertex_idx) || player.cities.contains(&vertex_idx)) {
                                                        if !robbable_players.contains(&player.id) {
                                                            robbable_players.push(player.id);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        drop(game_borrow);

                                        if robbable_players.is_empty() {
                                            ui.label("No players to rob on this tile");
                                            if ui.button("Confirm Knight (No steal)").clicked() {
                                                dev_card_state.selected_victim = None;
                                                should_play_devcard = true;
                                            }
                                        } else {
                                            ui.label("Select victim:");
                                            for player_id in &robbable_players {
                                                if ui.button(format!("Player {}", player_id)).clicked() {
                                                    dev_card_state.selected_victim = Some(*player_id);
                                                }
                                            }
                                            
                                            if let Some(victim) = dev_card_state.selected_victim {
                                                ui.label(format!("Will steal from Player {}", victim));
                                                if ui.button("Confirm Knight").clicked() {
                                                    should_play_devcard = true;
                                                }
                                            }
                                        }
                                    }
                                    
                                    if ui.button("Cancel").clicked() {
                                        dev_card_state.selected_card = None;
                                        dev_card_state.awaiting_input = None;
                                        dev_card_state.selected_tile = None;
                                        dev_card_state.selected_victim = None;
                                    }
                                }
                                DevCard::Monopoly => {
                                    ui.label("Select resource to monopolize:");
                                    if ui.button("Brick").clicked() {
                                        dev_card_state.selected_resource = Some(GameResource::Brick);
                                    }
                                    if ui.button("Lumber").clicked() {
                                        dev_card_state.selected_resource = Some(GameResource::Lumber);
                                    }
                                    if ui.button("Wool").clicked() {
                                        dev_card_state.selected_resource = Some(GameResource::Wool);
                                    }
                                    if ui.button("Grain").clicked() {
                                        dev_card_state.selected_resource = Some(GameResource::Grain);
                                    }
                                    if ui.button("Ore").clicked() {
                                        dev_card_state.selected_resource = Some(GameResource::Ore);
                                    }
                                    
                                    if dev_card_state.selected_resource.is_some() {
                                        ui.label(format!("Selected: {:?}", dev_card_state.selected_resource.unwrap()));
                                        if ui.button("Confirm Monopoly").clicked() {
                                            should_play_devcard = true;
                                        }
                                    }
                                    
                                    if ui.button("Cancel").clicked() {
                                        dev_card_state.selected_card = None;
                                        dev_card_state.awaiting_input = None;
                                    }
                                }
                                DevCard::RoadBuilding => {
                                    ui.label(format!("Roads built: {}/2", dev_card_state.road_building_roads.len()));
                                    
                                    //track clicked vertices for road building
                                    if let Some(vertex_id) = clicked_vertex.vertex_id {
                                        road_state.last_two_vertices.push(vertex_id);
                                        if road_state.last_two_vertices.len() > 2 {
                                            road_state.last_two_vertices.remove(0);
                                        }
                                        clicked_vertex.vertex_id = None;
                                    }

                                    if road_state.last_two_vertices.len() == 2 {
                                        ui.label(format!("Road: {} → {}", 
                                            road_state.last_two_vertices[0], 
                                            road_state.last_two_vertices[1]));
                                        
                                        if ui.button("Add Road").clicked() {
                                            let r = (road_state.last_two_vertices[0], road_state.last_two_vertices[1]);
                                            dev_card_state.road_building_roads.push(r);
                                            road_state.last_two_vertices.clear();
                                        }
                                    }
                                    
                                    if dev_card_state.road_building_roads.len() == 2 {
                                        if ui.button("Confirm Road Building").clicked() {
                                            should_play_devcard = true;
                                        }
                                    }
                                    
                                    if ui.button("Cancel").clicked() {
                                        dev_card_state.selected_card = None;
                                        dev_card_state.awaiting_input = None;
                                        dev_card_state.road_building_roads.clear();
                                    }
                                }
                                DevCard::YearOfPlenty => {
                                    ui.label(format!("Resources selected: {}/2", dev_card_state.year_resources.len()));
                                    
                                    if dev_card_state.year_resources.len() < 2 {
                                        if ui.button("Brick").clicked() {
                                            dev_card_state.year_resources.push(GameResource::Brick);
                                        }
                                        if ui.button("Lumber").clicked() {
                                            dev_card_state.year_resources.push(GameResource::Lumber);
                                        }
                                        if ui.button("Wool").clicked() {
                                            dev_card_state.year_resources.push(GameResource::Wool);
                                        }
                                        if ui.button("Grain").clicked() {
                                            dev_card_state.year_resources.push(GameResource::Grain);
                                        }
                                        if ui.button("Ore").clicked() {
                                            dev_card_state.year_resources.push(GameResource::Ore);
                                        }
                                    }
                                    
                                    if dev_card_state.year_resources.len() == 2 {
                                        ui.label(format!("Selected: {:?}, {:?}", 
                                            dev_card_state.year_resources[0],
                                            dev_card_state.year_resources[1]));
                                        if ui.button("Confirm Year of Plenty").clicked() {
                                            should_play_devcard = true;
                                        }
                                    }
                                    
                                    if ui.button("Cancel").clicked() {
                                        dev_card_state.selected_card = None;
                                        dev_card_state.awaiting_input = None;
                                        dev_card_state.year_resources.clear();
                                    }
                                }
                                DevCard::VictoryPoint => {
                                    ui.label("Victory Point Card");
                                    ui.label("Gain +1 Victory Point!");
                                    
                                    if ui.button("Reveal Victory Point").clicked() {
                                        should_play_devcard = true;
                                    }
                                    
                                    if ui.button("Cancel").clicked() {
                                        dev_card_state.selected_card = None;
                                        dev_card_state.awaiting_input = None;
                                    }
                                }
                            }
                        } else {
                            //normal build menu when not playing a dev card
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

                                    if has_devcard_resources {
                                        if ui.button("Buy Development Card (Wool + Grain + Ore)").clicked() {
                                            should_buy_devcard = true;
                                        }
                                    } else {
                                        ui.add_enabled(false, egui::Button::new("Buy Development Card (Need: Wool + Grain + Ore)"));
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
                                        ui.label(format!("Road: {} -> {}", 
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
                }
            });


        //handle actions after the UI is done

        // Handle dev card click
        if let Some((card_type, card_id)) = clicked_dev_card {
            dev_card_state.selected_card = Some((card_type, card_id));
            dev_card_state.awaiting_input = Some(card_type);
            game_log.add_info(format!("Selected {:?} card", card_type), time.elapsed_secs());
        }

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

        if should_buy_devcard {
            let mut game = game.borrow_mut();
            match game.buy_dev_card(current_player_id) {
                Ok(_) => {
                    game_log.add_info(format!("Development card purchased!"), time.elapsed_secs());
                }
                Err(e) => {
                    game_log.add_warn(format!("Failed to buy dev card: {}", e), time.elapsed_secs());
                }
            }
        }

        if should_play_devcard {
            //handle rolling 7 robber movement
            if robber_state.needs_movement {
                if let Some(tile) = robber_state.selected_tile {
                    let mut game = game.borrow_mut();
                    match game.move_robber(tile, robber_state.selected_victim) {
                        Ok(_) => {
                            game_log.add_info(format!("Robber moved!"), time.elapsed_secs());
                            //reset robber state
                            robber_state.needs_movement = false;
                            robber_state.selected_tile = None;
                            robber_state.selected_victim = None;
                        }
                        Err(e) => {
                            game_log.add_warn(format!("Failed to move robber: {}", e), time.elapsed_secs());
                        }
                    }
                }
            }
            // Handle dev card playing
            else if let Some((card_type, card_id)) = dev_card_state.selected_card {
                let input = match card_type {
                    DevCard::Knight => {
                        if let Some(tile) = dev_card_state.selected_tile {
                            DevCardInput::Knight { 
                                tile, 
                                victim: dev_card_state.selected_victim 
                            }
                        } else {
                            DevCardInput::None
                        }
                    }
                    DevCard::Monopoly => {
                        if let Some(resource) = dev_card_state.selected_resource {
                            DevCardInput::Monopoly { resource }
                        } else {
                            DevCardInput::None
                        }
                    }
                    DevCard::RoadBuilding => {
                        if dev_card_state.road_building_roads.len() == 2 {
                            DevCardInput::RoadBuilding { 
                                r1: dev_card_state.road_building_roads[0],
                                r2: dev_card_state.road_building_roads[1],
                            }
                        } else {
                            DevCardInput::None
                        }
                    }
                    DevCard::YearOfPlenty => {
                        if dev_card_state.year_resources.len() == 2 {
                            DevCardInput::YearOfPlenty { 
                                r1: dev_card_state.year_resources[0],
                                r2: dev_card_state.year_resources[1],
                            }
                        } else {
                            DevCardInput::None
                        }
                    }
                    DevCard::VictoryPoint => {
                        DevCardInput::None
                    }
                };

                let mut game = game.borrow_mut();
                match game.play_dev_card(current_player_id, card_id, input) {
                    Ok(_) => {
                        game_log.add_info(format!("{:?} card played successfully!", card_type), time.elapsed_secs());
                        //reset dev card state
                        dev_card_state.selected_card = None;
                        dev_card_state.awaiting_input = None;
                        dev_card_state.selected_tile = None;
                        dev_card_state.selected_victim = None;
                        dev_card_state.selected_resource = None;
                        dev_card_state.road_building_roads.clear();
                        dev_card_state.year_resources.clear();
                    }
                    Err(e) => {
                        game_log.add_warn(format!("Failed to play dev card: {}", e), time.elapsed_secs());
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
                    //trigger robber movement - call robbery first
                    game.robbery();
                    game_log.add_info(format!("Rolled 7 - Move the robber!"), time.elapsed_secs());
                    
                    //set flag to show robber UI
                    robber_state.needs_movement = true;
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

fn window_frame() -> egui::Frame {
    egui::Frame::NONE
        .fill(egui::Color32::from_black_alpha(150))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_white_alpha(100)))
        .inner_margin(10.0)
        .outer_margin(0.0)
        .corner_radius(egui::CornerRadius::same(15))
}