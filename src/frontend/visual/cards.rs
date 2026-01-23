use crate::backend::game::Resource::*;
use crate::backend::game::{DevCard, Resource};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use std::collections::HashMap;

//resource to store card textures in a hashmap
#[derive(Resource)]
pub struct CardsTextures {
    pub resource: HashMap<Resource, egui::TextureHandle>,
    pub devcard: HashMap<DevCard, egui::TextureHandle>,
}

#[derive(Component)]
pub struct CardsVisual {
    //whether the card is highlighted (e.g. hover or selection)
    pub highlighted: bool,
}

//resource to track whether card visuals are shown or not
#[derive(Resource)]
pub struct CardShowing {
    pub enabled: bool,
}

impl Default for CardShowing {
    fn default() -> Self {
        CardShowing { enabled: true }
    }
}

//load the card textures into egui
pub fn setup_cards_textures(
    mut commands: Commands,
    mut contexts: EguiContexts,
    textures: Option<Res<CardsTextures>>,
) {
    if textures.is_some() {
        return;
    }

    if let Ok(ctx) = contexts.ctx_mut() {
        let textures = load_cards_textures(ctx);
        commands.insert_resource(textures);
        info!("Card textures have been loaded successfully!");
    }
}

pub fn load_cards_textures(ctx: &egui::Context) -> CardsTextures {
    let mut resource = HashMap::new();
    let mut devcard = HashMap::new();

    //load an image file and convert it to an egui texture, specifically rgba8 format
    let load = |path: &str| {
        let image = image::open(path)
            .unwrap_or_else(|_| panic!("Failed to load card image! {path}"))
            .to_rgba8();

        //extract image dimensions etc.
        let size = [image.width() as usize, image.height() as usize];
        let pixels = image.into_raw();

        //convert the previous image data to egui texture
        ctx.load_texture(
            path.to_string(),
            egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
            egui::TextureOptions::LINEAR,
        )
    };

    //load texture for each resource type from the assets and insert into the hashmap
    resource.insert(Brick, load("assets/cards/brick.png"));
    resource.insert(Lumber, load("assets/cards/lumber.png"));
    resource.insert(Wool, load("assets/cards/wool.png"));
    resource.insert(Grain, load("assets/cards/grain.png"));
    resource.insert(Ore, load("assets/cards/ore.png"));

    devcard.insert(DevCard::Knight, load("assets/cards/knight.png"));
    devcard.insert(
        DevCard::VictoryPoint,
        load("assets/cards/victory_point.png"),
    );
    devcard.insert(DevCard::Monopoly, load("assets/cards/monopoly.png"));
    devcard.insert(
        DevCard::RoadBuilding,
        load("assets/cards/road_building.png"),
    );
    devcard.insert(
        DevCard::YearOfPlenty,
        load("assets/cards/year_of_plenty.png"),
    );

    CardsTextures { resource, devcard }
}

//retrieve the texture handle for a given resource type
pub fn resource_texture<'a>(
    textures: &'a CardsTextures,
    resource: Resource,
) -> &'a egui::TextureHandle {
    textures
        .resource
        .get(&resource)
        .expect("The resource card texture is missing!")
}

//retrieve the texture handle for a given development card type
pub fn devcard_texture<'a>(
    textures: &'a CardsTextures,
    devcard: DevCard,
) -> &'a egui::TextureHandle {
    textures
        .devcard
        .get(&devcard)
        .expect("The devcard texture is missing!")
}

//draw all resource and development cards in two lines
pub fn draw_cards(
    painter: &egui::Painter,
    textures: &CardsTextures,
    start_pos: egui::Pos2,
    card_size: egui::Vec2,
    spacing: f32,
) {
    let resource_cards = [Brick, Lumber, Wool, Grain, Ore];
    let dev_cards = [
        DevCard::Knight,
        DevCard::VictoryPoint,
        DevCard::Monopoly,
        DevCard::RoadBuilding,
        DevCard::YearOfPlenty,
    ];

    //first line consisting of all resource cards
    for (i, resource) in resource_cards.iter().enumerate() {
        let pos = start_pos + egui::vec2((card_size.x + spacing) * i as f32, 0.0);
        let rect = egui::Rect::from_min_size(pos, card_size);

        painter.image(
            resource_texture(textures, *resource).id(),
            rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }

    //second line consisting of all development cards
    let line_spacing = card_size.y + spacing;
    for (i, devcard) in dev_cards.iter().enumerate() {
        let pos = start_pos + egui::vec2((card_size.x + spacing) * i as f32, line_spacing);
        let rect = egui::Rect::from_min_size(pos, card_size);

        painter.image(
            devcard_texture(textures, *devcard).id(),
            rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }
}
