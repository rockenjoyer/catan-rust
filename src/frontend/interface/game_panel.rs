use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashMap, HashSet};

use crate::backend::game::{Game, GamePhase, RoadBuildingMode, Resource as GameResource, DevCard, DevCardInput};
use crate::frontend::interface::style::apply_style;
use crate::frontend::interface::log_panel::GameLog;
use crate::frontend::visual::{
    cards::{CardsTextures, draw_cards},
    road::{RoadTextures, draw_roads, select_road_texture},
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

//resource to track building effect state
#[derive(Resource, Default)]
pub struct BuildEffectsState {
    pub bursts: Vec<BuildBurst>,
}

//resource to track building effect animation
#[derive(Clone, Copy)]
pub struct BuildBurst {
    //burst origin
    pub pos: (f32, f32),
    //timestamp for the burst
    pub spawned_at: f32,
    //player color tint for the burst particles
    pub color: (u8, u8, u8),
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
    mut build_effects: ResMut<BuildEffectsState>,
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
        //shared time base for pulses, fades, and dice animation
        let now = time.elapsed_secs();

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
        //freeze the current mode for this UI frame
        let active_building_mode = building_mode.clone();
        let dev_card_road_building_active = matches!(
            dev_card_state.selected_card,
            Some((DevCard::RoadBuilding, _))
        );

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

                //build mode overlays (highlights and ghost previews for placements)
                let is_setup = matches!(current_phase, GamePhase::SetupRound1 | GamePhase::SetupRound2);
                let (buildable_vertices, buildable_edges) = calculate_builds(
                    &game_borrow,
                    current_player_id,
                    is_setup,
                    setup_placement,
                    &active_building_mode,
                    dev_card_road_building_active,
                );

                //find the closest build target under the cursor
                let (hovered_vertex, hovered_edge) = hovered_build_targets(
                    &game_borrow,
                    &screen,
                    ui.ctx().pointer_hover_pos(),
                    &buildable_edges,
                );

                //visuals to show where the player can build
                draw_building_highlights(
                    &painter,
                    &game_borrow,
                    &screen,
                    &*road_textures,
                    now,
                    current_player_id,
                    &buildable_vertices,
                    &buildable_edges,
                );

                //ghosted preview under the cursor for the current build mode
                draw_ghosts_preview(
                    &painter, 
                    &game_borrow, 
                    &screen, 
                    &*settlement_textures, 
                    &*city_textures,
                    &*road_textures,
                    current_player_id, 
                    &active_building_mode, 
                    is_setup,
                    setup_placement, 
                    dev_card_road_building_active, 
                    hovered_vertex, 
                    clicked_vertex.selected_vertex, 
                    hovered_edge,
                    &buildable_vertices,
                    &buildable_edges,
                    &road_state.last_two_vertices
                );

                //layer 4 (settlements and cities)
                draw_settlements(&painter, &game_borrow, &*settlement_textures, &screen);
                draw_cities(&painter, &game_borrow, &*city_textures, &screen);

                //particle burst for placement
                draw_build_particles(&painter, &screen, now, build_effects.as_mut());
                
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
                        let burst_pos = game.vertices[vertex_id].pos;
                        let burst_color = player_color_rgb(current_player_id);
                        game_log.add_info(format!("Settlement built successfully!"), time.elapsed_secs());
                        //spawn the particle burst
                        apply_build_burst(build_effects.as_mut(), burst_pos, burst_color, now);
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
                        let a = game.vertices[first].pos;
                        let b = game.vertices[second].pos;
                        let burst_pos = ((a.0 + b.0) * 0.5, (a.1 + b.1) * 0.5);
                        let burst_color = player_color_rgb(current_player_id);
                        game_log.add_info(format!("Road built between {} and {}", first, second), time.elapsed_secs());
                        //spawn the particle burst
                        apply_build_burst(build_effects.as_mut(), burst_pos, burst_color, now);
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
                        let burst_pos = game.vertices[vertex_id].pos;
                        let burst_color = player_color_rgb(current_player_id);
                        game_log.add_info(format!("Settlement upgraded to city!"), time.elapsed_secs());
                        //spawn the particle burst
                        apply_build_burst(build_effects.as_mut(), burst_pos, burst_color, now);
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

//helper functions for player color and textures
fn is_standard_board(game: &Game) -> bool {
    game.tiles.len() <= 19
}

fn player_color_rgb(player_id: usize) -> (u8, u8, u8) {
    match player_id {
        0 => (200, 50, 50),  //red
        1 => (50, 100, 200), //blue
        2 => (50, 200, 50),  //green
        3 => (220, 200, 50), //yellow
        _ => (255, 255, 255),
    }
}

fn settlement_texture_for_player(
    textures: &SettlementTextures,
    player_id: usize,
) -> &egui::TextureHandle {
    match player_id {
        0 => &textures.red,
        1 => &textures.blue,
        2 => &textures.green,
        3 => &textures.yellow,
        _ => &textures.red,
    }
}

fn city_texture_for_player(textures: &CityTextures, player_id: usize) -> &egui::TextureHandle {
    match player_id {
        0 => &textures.red,
        1 => &textures.blue,
        2 => &textures.green,
        3 => &textures.yellow,
        _ => &textures.red,
    }
}

fn buildable_settlement_vertices(game: &Game, player_id: usize, is_setup: bool) -> HashSet<usize> {
    let player = &game.players[player_id];
    if is_standard_board(game) && player.settlements.len() >= 5 {
        return HashSet::new();
    }

    let mut buildable = HashSet::new();
    for vertex in &game.vertices {
        let vertex_id = vertex.id;
        let occupied = game.players.iter().any(|p| {
            p.settlements.contains(&vertex_id) || p.cities.contains(&vertex_id)
        });
        if occupied {
            continue;
        }

        let neighbor_occupied = vertex.neighbors.iter().any(|neighbor| {
            game.players
                .iter()
                .any(|p| p.settlements.contains(neighbor) || p.cities.contains(neighbor))
        });
        if neighbor_occupied {
            continue;
        }

        let connected = is_setup
            || player
                .roads
                .iter()
                .any(|&(x, y)| x == vertex_id || y == vertex_id);
        if !connected {
            continue;
        }
        buildable.insert(vertex_id);
    }
    buildable
}

fn buildable_city_vertices(game: &Game, player_id: usize) -> HashSet<usize> {
    let player = &game.players[player_id];
    if is_standard_board(game) && player.cities.len() >= 4 {
        return HashSet::new();
    }
    player
        .settlements
        .iter()
        .filter(|v| !player.cities.contains(v))
        .copied()
        .collect()
}

fn buildable_road_edges(
    game: &Game,
    player_id: usize,
    is_setup: bool,
    required_vertex: Option<usize>,
) -> HashSet<(usize, usize)> {
    let player = &game.players[player_id];
    if is_standard_board(game) && player.roads.len() >= 15 {
        return HashSet::new();
    }

    if is_setup && required_vertex.is_none() {
        return HashSet::new();
    }

    let mut edges = HashSet::new();
    for vertex in &game.vertices {
        let a = vertex.id;
        for &b in &vertex.neighbors {
            if a >= b {
                continue;
            }

            let edge = (a.min(b), a.max(b));

            let matches_required = required_vertex
                .map(|required| edge.0 == required || edge.1 == required)
                .unwrap_or(true);
            if !matches_required {
                continue;
            }

            let road_exists = game.players.iter().any(|p| p.roads.contains(&edge));
            if road_exists {
                continue;
            }

            let connected = is_setup
                || player.settlements.contains(&a)
                || player.cities.contains(&a)
                || player.roads.iter().any(|&(x, y)| x == a || y == a)
                || player.settlements.contains(&b)
                || player.cities.contains(&b)
                || player.roads.iter().any(|&(x, y)| x == b || y == b);
            if !connected {
                continue;
            }
            edges.insert(edge);
        }
    }
    edges
}

fn calculate_builds(
    game: &Game,
    player_id: usize,
    is_setup: bool,
    setup_placement: u8,
    active_building_mode: &BuildingMode,
    dev_card_road_building_active: bool,
) -> (HashSet<usize>, HashSet<(usize, usize)>) {
    let mut buildable_vertices = HashSet::new();
    let mut buildable_edges = HashSet::new();

    if is_setup {
        //settlement first, then a road from that settlement
        if setup_placement == 0 {
            buildable_vertices = buildable_settlement_vertices(game, player_id, true);
        } else if setup_placement == 1 {
            let required_vertex = game.players[player_id].last_setup_settlement;
            buildable_edges = buildable_road_edges(game, player_id, true, required_vertex);
        }
    } else {
        //else follow the active build mode
        match active_building_mode {
            BuildingMode::BuildingSettlement => {
                buildable_vertices = buildable_settlement_vertices(game, player_id, false);
            }
            BuildingMode::UpgradingCity => {
                buildable_vertices = buildable_city_vertices(game, player_id);
            }
            BuildingMode::BuildingRoad => {
                buildable_edges = buildable_road_edges(game, player_id, false, None);
            }
            BuildingMode::None => {}
        }

        //road-building dev card always enables road previews
        if dev_card_road_building_active {
            buildable_edges = buildable_road_edges(game, player_id, false, None);
        }
    }

    (buildable_vertices, buildable_edges)
}

fn hovered_build_targets(
    game: &Game,
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
    mouse_pos: Option<egui::Pos2>,
    buildable_edges: &HashSet<(usize, usize)>,
) -> (Option<usize>, Option<(usize, usize)>) {
    //pick the closest vertex and edge under the cursor for previews
    let Some(mouse_pos) = mouse_pos else {
        return (None, None);
    };

    let hovered_vertex = find_hovered_vertex(game, screen, mouse_pos, 10.0);
    let hovered_edge = find_hovered_edge(game, screen, mouse_pos, buildable_edges, 10.0);

    (hovered_vertex, hovered_edge)
}

fn find_hovered_vertex(
    game: &Game,
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
    mouse_pos: egui::Pos2,
    radius: f32,
) -> Option<usize> {
    //pick the first vertex within the hover radius
    game.vertices.iter().find_map(|vertex| {
        let pos = screen(vertex.pos);
        (mouse_pos.distance(pos) <= radius).then_some(vertex.id)
    })
}

fn point_to_segment(p: egui::Pos2, a: egui::Pos2, b: egui::Pos2) -> f32 {
    //project the mouse onto the road segment and return the distance
    let ap = p - a;
    let ab = b - a;
    let ab_len_sq = ab.x * ab.x + ab.y * ab.y;
    if ab_len_sq == 0.0 {
        return p.distance(a);
    }

    let t = (ap.x * ab.x + ap.y * ab.y) / ab_len_sq;
    let t = t.clamp(0.0, 1.0);
    let closest = a + ab * t;
    p.distance(closest)
}

fn find_hovered_edge(
    game: &Game,
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
    mouse_pos: egui::Pos2,
    candidates: &HashSet<(usize, usize)>,
    max_distance: f32,
) -> Option<(usize, usize)> {
    //scan edges and return the nearest
    let mut best_edge: Option<(usize, usize)> = None;
    let mut best_dist = max_distance;

    for &(a, b) in candidates {
        let start = screen(game.vertices[a].pos);
        let end = screen(game.vertices[b].pos);
        let dist = point_to_segment(mouse_pos, start, end);
        if dist <= best_dist {
            best_dist = dist;
            best_edge = Some((a, b));
        }
    }
    best_edge
}

fn select_ghost_edge(
    road_selection: &[usize],
    hovered_vertex: Option<usize>,
    hovered_edge: Option<(usize, usize)>,
    buildable_edges: &HashSet<(usize, usize)>,
) -> Option<(usize, usize)> {
    if road_selection.len() >= 2 {
        let edge = (
            road_selection[0].min(road_selection[1]),
            road_selection[0].max(road_selection[1]),
        );
        if buildable_edges.contains(&edge) {
            return Some(edge);
        }
    }

    if road_selection.len() == 1 {
        if let Some(b) = hovered_vertex {
            let a = road_selection[0];
            let edge = (a.min(b), a.max(b));
            if buildable_edges.contains(&edge) {
                return Some(edge);
            }
        }
    }

    if let Some(edge) = hovered_edge {
        if buildable_edges.contains(&edge) {
            return Some(edge);
        }
    }

    None
}

fn draw_building_highlights(
    painter: &egui::Painter,
    game: &Game,
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
    road_textures: &RoadTextures,
    now: f32,
    current_player_id: usize,
    buildable_vertices: &HashSet<usize>,
    buildable_edges: &HashSet<(usize, usize)>,
) {
    //pulse to make buildable spots easy to see
    let pulse = (now * 5.0).sin() * 0.5 + 0.5;
    let vertex_alpha = (80.0 + 100.0 * pulse) as u8;
    let edge_alpha = (70.0 + 100.0 * pulse) as u8;

    let (r, g, b) = player_color_rgb(current_player_id);
    let vertex_color = egui::Color32::from_rgba_unmultiplied(r, g, b, vertex_alpha);
    let edge_color = egui::Color32::from_rgba_unmultiplied(r, g, b, edge_alpha);

    //draw vertex rings in the player's color
    for &vertex_id in buildable_vertices {
        let pos = screen(game.vertices[vertex_id].pos);
        painter.circle_stroke(pos, 16.0, egui::Stroke::new(3.0, vertex_color));
    }

    for &(a, b) in buildable_edges {
        let start = screen(game.vertices[a].pos);
        let end = screen(game.vertices[b].pos);
        let dir = end - start;
        let center = start + dir * 0.5;
        let length = dir.length();
        let angle = dir.y.atan2(dir.x);
        let texture = select_road_texture(road_textures, current_player_id, angle);
        let rect = egui::Rect::from_center_size(center, egui::vec2(length / 1.1, 60.0));

        painter.image(
            texture.id(),
            rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
            edge_color,
        );
    }
}

fn draw_ghosts_preview(
    painter: &egui::Painter,
    game: &Game,
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
    settlement_textures: &SettlementTextures,
    city_textures: &CityTextures,
    road_textures: &RoadTextures,
    current_player_id: usize,
    active_building_mode: &BuildingMode,
    is_setup: bool,
    setup_placement: u8,
    dev_card_road_building_active: bool,
    hovered_vertex: Option<usize>,
    locked_vertex: Option<usize>,
    hovered_edge: Option<(usize, usize)>,
    buildable_vertices: &HashSet<usize>,
    buildable_edges: &HashSet<(usize, usize)>,
    road_selection: &[usize],
) {
    //decide which ghost to show based on mode
    let show_settlement = is_setup && setup_placement == 0
        || matches!(active_building_mode, BuildingMode::BuildingSettlement);
    let show_city = matches!(active_building_mode, BuildingMode::UpgradingCity);
    let show_road = (is_setup && setup_placement == 1)
        || matches!(active_building_mode, BuildingMode::BuildingRoad)
        || dev_card_road_building_active;

    if show_settlement {
        let target_vertex = hovered_vertex.or(locked_vertex);
        if let Some(vertex_id) = target_vertex {
            if buildable_vertices.contains(&vertex_id) {
                let texture = settlement_texture_for_player(settlement_textures, current_player_id);
                let pos = screen(game.vertices[vertex_id].pos);
                let rect = egui::Rect::from_center_size(pos, egui::vec2(35.7, 50.0));
                painter.image(
                    texture.id(),
                    rect,
                    egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
                    egui::Color32::from_white_alpha(160),
                );
            }
        }
    }

    if show_city {
        let target_vertex = hovered_vertex.or(locked_vertex);
        if let Some(vertex_id) = target_vertex {
            if buildable_vertices.contains(&vertex_id) {
                let texture = city_texture_for_player(city_textures, current_player_id);
                let pos = screen(game.vertices[vertex_id].pos);
                let rect = egui::Rect::from_center_size(pos, egui::vec2(68.6, 60.0));
                painter.image(
                    texture.id(),
                    rect,
                    egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
                    egui::Color32::from_white_alpha(160),
                );
            }
        }
    }

    if show_road {
        if let Some((a, b)) = select_ghost_edge(
            road_selection,
            hovered_vertex,
            hovered_edge,
            buildable_edges,
        ) {
            let start = screen(game.vertices[a].pos);
            let end = screen(game.vertices[b].pos);
            let dir = end - start;
            let center = start + dir * 0.5;
            let length = dir.length();
            let angle = dir.y.atan2(dir.x);
            let texture = select_road_texture(road_textures, current_player_id, angle);
            let rect = egui::Rect::from_center_size(center, egui::vec2(length / 1.1, 60.0));

            painter.image(
                texture.id(),
                rect,
                egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
                egui::Color32::from_white_alpha(160),
            );
        }
    }
}

fn draw_build_particles(
    painter: &egui::Painter,
    screen: &dyn Fn((f32, f32)) -> egui::Pos2,
    now: f32,
    build_effects: &mut BuildEffectsState,
) {
    //radial burst animation after placements
    let duration = 0.8;
    build_effects
        .bursts
        .retain(|burst| now - burst.spawned_at <= duration);

    let particle_count = 8;
    //each burst expands and fades over its lifetime
    for burst in &build_effects.bursts {
        let t = ((now - burst.spawned_at) / duration).clamp(0.0, 1.0);
        let radius = 6.0 + t * 26.0;

        //fade out and shrink over time
        let alpha = ((1.0 - t) * 200.0) as u8;
        let size = 3.0 + (1.0 - t) * 3.0;
        let center = screen(burst.pos);

        let (r, g, b) = burst.color;
        let color = egui::Color32::from_rgba_unmultiplied(r, g, b, alpha);

        //compute angle and offset for every particle, then draw the circle
        for i in 0..particle_count {
            let angle = (i as f32 / particle_count as f32) * std::f32::consts::TAU;
            let offset = egui::vec2(angle.cos(), angle.sin()) * radius;
            painter.circle_filled(center + offset, size, color);
        }
    }
}

fn apply_build_burst(
    build_effects: &mut BuildEffectsState,
    pos: (f32, f32),
    color: (u8, u8, u8),
    now: f32,
) {
    //enqueue a new burst to render in the next frames
    build_effects.bursts.push(BuildBurst {
        pos,
        spawned_at: now,
        color,
    });
}

fn window_frame() -> egui::Frame {
    egui::Frame::NONE
        .fill(egui::Color32::from_black_alpha(150))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_white_alpha(100)))
        .inner_margin(10.0)
        .outer_margin(0.0)
        .corner_radius(egui::CornerRadius::same(15))
}
