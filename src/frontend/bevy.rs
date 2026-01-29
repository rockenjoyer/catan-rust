use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::frontend::interface::{
    game_panel, info_panel, rules_panel, settings_panel, volume_panel, log_panel,
};
use crate::frontend::system::{camera, input};
use crate::frontend::visual::{tile, road, cards, settlement, city, dice};

pub struct FrontendPlugin;
impl Plugin for FrontendPlugin {
    fn build(&self, app: &mut App) {
        app
            //background-color
            .insert_resource(ClearColor(Color::srgb(0.66, 0.58, 0.57)))
            .add_plugins(EguiPlugin::default())
            //startup runs once
            .add_systems(
                Startup,
                (
                    change_title,
                    camera::setup_camera,
                    input::setup_cursor,
                ),
            )
            //resources for game state
            .insert_resource(tile::TileShowing::default())
            .insert_resource(tile::ClickedVertex::default())
            .insert_resource(game_panel::RoadBuildingState::default())
            .insert_resource(game_panel::BuildingMode::default())
            .insert_resource(game_panel::DevCardPlayState::default())
            .insert_resource(game_panel::RobberMoveState::default())
            .insert_resource(dice::DiceRollState::default())
            .insert_resource(log_panel::GameLog::default())
            
            //egui pass: texture loading and UI systems
            .add_systems(
                bevy_egui::EguiPrimaryContextPass,
                (
                    //texture loaders (run first)
                    tile::setup_tile_textures,
                    road::setup_road_textures,
                    cards::setup_cards_textures,
                    settlement::setup_settlement_textures,
                    city::setup_city_textures,
                    
                    //UI panels (run after textures loaded)
                    info_panel::setup_info,
                    game_panel::setup_game,
                    rules_panel::setup_rules,
                    settings_panel::setup_settings,
                    volume_panel::setup_volume,
                    log_panel::setup_log_panel,
                ),
            );
    }
}

fn change_title(mut window: Single<&mut Window>) {
    window.title = format!("The Settlers of Catan - Rust Edition");
}