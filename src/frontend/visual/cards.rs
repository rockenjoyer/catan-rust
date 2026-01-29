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

//helper function to draw a card with 3D perspective effect
//returns true if the card was clicked
fn draw_card_with_3d_effect(
    ui: &mut egui::Ui,
    painter: &egui::Painter,
    texture_id: egui::TextureId,
    rect: egui::Rect,
    mouse_pos: Option<egui::Pos2>,
    hover_scale: f32,
    hover_lift: f32,
) -> (egui::Rect, bool) {
    let is_hovered = mouse_pos.map_or(false, |pos| rect.contains(pos));
    
    //check for clicks
    let was_clicked = is_hovered && ui.input(|i| i.pointer.primary_clicked());

    if !is_hovered {
        //normal rendering
        painter.image(texture_id, rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)), egui::Color32::WHITE);
        return (rect, was_clicked);
    }
    
    //calculate 3D perspective effect
    let center = rect.center();
    let mouse = mouse_pos.unwrap();
    
    //calculate rotation based on mouse position relative to card center
    let rel_x = (mouse.x - center.x) / (rect.width() / 2.0); // -1 to 1
    let rel_y = (mouse.y - center.y) / (rect.height() / 2.0); // -1 to 1
    
    //clamp to prevent extreme rotations
    let tilt_x = rel_y.clamp(-1.0, 1.0) * 0.15; // Tilt around X axis (up/down)
    let tilt_y = -rel_x.clamp(-1.0, 1.0) * 0.15; // Tilt around Y axis (left/right)
    
    //calculate new rect with hover effects
    let hover_size = egui::vec2(rect.width(), rect.height()) * hover_scale;
    let hover_center = center - egui::vec2(0.0, hover_lift);
    
    //create perspective-distorted quad
    let half_w = hover_size.x / 2.0;
    let half_h = hover_size.y / 2.0;
    
    //apply 3D rotation perspective to corners
    let perspective_factor = 0.3; //how much perspective distortion
    
    //top-left corner
    let tl_offset_x = -half_w + (tilt_y * half_w * perspective_factor);
    let tl_offset_y = -half_h + (tilt_x * half_h * perspective_factor);
    let top_left = hover_center + egui::vec2(tl_offset_x, tl_offset_y);
    
    //top-right corner
    let tr_offset_x = half_w + (tilt_y * half_w * perspective_factor);
    let tr_offset_y = -half_h - (tilt_x * half_h * perspective_factor);
    let top_right = hover_center + egui::vec2(tr_offset_x, tr_offset_y);
    
    //bottom-left corner
    let bl_offset_x = -half_w - (tilt_y * half_w * perspective_factor);
    let bl_offset_y = half_h - (tilt_x * half_h * perspective_factor);
    let bottom_left = hover_center + egui::vec2(bl_offset_x, bl_offset_y);
    
    //bottom-right corner
    let br_offset_x = half_w - (tilt_y * half_w * perspective_factor);
    let br_offset_y = half_h + (tilt_x * half_h * perspective_factor);
    let bottom_right = hover_center + egui::vec2(br_offset_x, br_offset_y);
    
    //draw shadow first
    let shadow_offset = egui::vec2(5.0, 5.0);
    let shadow_mesh = egui::Mesh {
        indices: vec![0, 1, 2, 0, 2, 3],
        vertices: vec![
            egui::epaint::Vertex {
                pos: top_left + shadow_offset,
                uv: egui::pos2(0.0, 0.0),
                color: egui::Color32::from_black_alpha(80),
            },
            egui::epaint::Vertex {
                pos: top_right + shadow_offset,
                uv: egui::pos2(1.0, 0.0),
                color: egui::Color32::from_black_alpha(80),
            },
            egui::epaint::Vertex {
                pos: bottom_right + shadow_offset,
                uv: egui::pos2(1.0, 1.0),
                color: egui::Color32::from_black_alpha(80),
            },
            egui::epaint::Vertex {
                pos: bottom_left + shadow_offset,
                uv: egui::pos2(0.0, 1.0),
                color: egui::Color32::from_black_alpha(80),
            },
        ],
        texture_id: egui::TextureId::default(),
    };
    painter.add(egui::Shape::mesh(shadow_mesh));
    
    //draw the card with perspective as a textured mesh
    let card_mesh = egui::Mesh {
        indices: vec![0, 1, 2, 0, 2, 3],
        vertices: vec![
            egui::epaint::Vertex {
                pos: top_left,
                uv: egui::pos2(0.0, 0.0),
                color: egui::Color32::WHITE,
            },
            egui::epaint::Vertex {
                pos: top_right,
                uv: egui::pos2(1.0, 0.0),
                color: egui::Color32::WHITE,
            },
            egui::epaint::Vertex {
                pos: bottom_right,
                uv: egui::pos2(1.0, 1.0),
                color: egui::Color32::WHITE,
            },
            egui::epaint::Vertex {
                pos: bottom_left,
                uv: egui::pos2(0.0, 1.0),
                color: egui::Color32::WHITE,
            },
        ],
        texture_id,
    };
    painter.add(egui::Shape::mesh(card_mesh));
    
    //return the rect for badge positioning
    (egui::Rect::from_min_max(top_left.min(bottom_right), top_right.max(bottom_left)), was_clicked)
}

//draw all resource and development cards in two lines with hover effect
//returns Option<(DevCard, usize)> (card type and instance id) if dev card was clicked
pub fn draw_cards(
    ui: &mut egui::Ui,
    painter: &egui::Painter,
    textures: &CardsTextures,
    start_pos: egui::Pos2,
    card_size: egui::Vec2,
    spacing: f32,
    player_resources: &HashMap<Resource, u8>,
    player_dev_cards: &HashMap<DevCard, usize>,
    player_dev_cards_instances: &[(DevCard, usize)],
) -> Option<(DevCard, usize)> {
    let hover_scale = 2.0; //2x when hovered
    let hover_lift = 20.0; //move up by 20 pixels
    let mouse_pos = ui.ctx().pointer_hover_pos();
    let mut clicked_dev_card: Option<(DevCard, usize)> = None;

    //RESOURCE CARDS (right aligned and only owned)
    let mut owned_resources: Vec<(Resource, u8)> = player_resources
        .iter()
        .filter(|(res, amt)| **amt > 0 && **res != Resource::Desert)
        .map(|(res, amt)| (*res, *amt))
        .collect();

    owned_resources.sort_by_key(|(res, _)| format!("{:?}", res));
    
    let total_resource_width = if owned_resources.is_empty() {
        0.0
    } else {
        (card_size.x * owned_resources.len() as f32) + (spacing * (owned_resources.len() - 1) as f32)
    };
    let resource_start_pos = egui::pos2(start_pos.x - total_resource_width + card_size.x, start_pos.y);
    
    let mut hovered_res_index: Option<usize> = None;
    for (i, _) in owned_resources.iter().enumerate() {
        let pos = resource_start_pos + egui::vec2((card_size.x + spacing) * i as f32, 0.0);
        let rect = egui::Rect::from_min_size(pos, card_size);

        if let Some(mouse) = mouse_pos {
            if rect.contains(mouse) {
                hovered_res_index = Some(i);
                break;
            }
        }
    }
    
    //draw not hovered cards first
    for (i, (resource, amount)) in owned_resources.iter().enumerate() {
        if Some(i) == hovered_res_index { continue; }

        let pos = resource_start_pos + egui::vec2((card_size.x + spacing) * i as f32, 0.0);
        let rect = egui::Rect::from_min_size(pos, card_size);
        
        let (final_rect, _) = draw_card_with_3d_effect(ui, painter, resource_texture(textures, *resource).id(),
         rect, None, hover_scale, hover_lift);
        
        if *amount > 1 {
            let badge_pos = egui::pos2(final_rect.right() - 15.0, final_rect.top() + 5.0);

            painter.circle_filled(badge_pos, 10.0, egui::Color32::from_rgb(255, 200, 0));
            painter.circle_stroke(badge_pos, 10.0, egui::Stroke::new(2.0, egui::Color32::BLACK));
            painter.text(badge_pos, egui::Align2::CENTER_CENTER, format!("{}x", amount),
                egui::FontId::proportional(12.0), egui::Color32::BLACK);
        }
    }
    
    //draw hovered card on top with 3D effect
    if let Some(i) = hovered_res_index {
        if let Some((resource, amount)) = owned_resources.get(i) {
            let pos = resource_start_pos + egui::vec2((card_size.x + spacing) * i as f32, 0.0);
            let rect = egui::Rect::from_min_size(pos, card_size);
            let (final_rect, _) = draw_card_with_3d_effect(ui, painter, resource_texture(textures, *resource).id(),
             rect, mouse_pos, hover_scale, hover_lift);
            
            if *amount > 1 {
                let badge_pos = egui::pos2(final_rect.right() - 15.0, final_rect.top() + 5.0);
                painter.circle_filled(badge_pos, 10.0, egui::Color32::from_rgb(255, 200, 0));
                painter.circle_stroke(badge_pos, 10.0, egui::Stroke::new(2.0, egui::Color32::BLACK));
                painter.text(badge_pos, egui::Align2::CENTER_CENTER, format!("{}x", amount),
                    egui::FontId::proportional(12.0), egui::Color32::BLACK);
            }
        }
    }

    //DEV CARDS (right aligned and only owned)
    let line_spacing = card_size.y + spacing;

    let mut owned_dev_cards: Vec<(DevCard, usize)> = player_dev_cards
        .iter()
        .filter(|(_, count)| **count > 0)
        .map(|(card, &count)| (*card, count))
        .collect();

    owned_dev_cards.sort_by_key(|(card, _)| format!("{:?}", card));
    
    let total_dev_width = if owned_dev_cards.is_empty() {
        0.0
    } else {
        (card_size.x * owned_dev_cards.len() as f32) + (spacing * (owned_dev_cards.len() - 1) as f32)
    };

    let dev_start_pos = egui::pos2(start_pos.x - total_dev_width + card_size.x, start_pos.y + line_spacing);
    
    let mut hovered_dev_index: Option<usize> = None;

    for (i, _) in owned_dev_cards.iter().enumerate() {
        let pos = dev_start_pos + egui::vec2((card_size.x + spacing) * i as f32, 0.0);
        let rect = egui::Rect::from_min_size(pos, card_size);

        if let Some(mouse) = mouse_pos {
            if rect.contains(mouse) {
                hovered_dev_index = Some(i);
                break;
            }
        }
    }
    
    //draw not hovered dev cards
    for (i, (devcard, count)) in owned_dev_cards.iter().enumerate() {
        if Some(i) == hovered_dev_index { continue; }

        let pos = dev_start_pos + egui::vec2((card_size.x + spacing) * i as f32, 0.0);
        let rect = egui::Rect::from_min_size(pos, card_size);

        let (final_rect, was_clicked) = draw_card_with_3d_effect(ui, painter, devcard_texture(textures, *devcard).id(),
         rect, None, hover_scale, hover_lift);
        
        //find the card ID if clicked
        if was_clicked && clicked_dev_card.is_none() {
            //get the first unplayed instance of this card type
            for (card_type, card_id) in player_dev_cards_instances {
                if card_type == devcard {
                    clicked_dev_card = Some((*devcard, *card_id));
                    break;
                }
            }
        }

        if *count > 1 {
            let badge_pos = egui::pos2(final_rect.right() - 15.0, final_rect.top() + 5.0);
            painter.circle_filled(badge_pos, 10.0, egui::Color32::from_rgb(255, 200, 0));
            painter.circle_stroke(badge_pos, 10.0, egui::Stroke::new(2.0, egui::Color32::BLACK));
            painter.text(badge_pos, egui::Align2::CENTER_CENTER, format!("{}x", count),
                egui::FontId::proportional(12.0), egui::Color32::BLACK);
        }
    }
    
    //draw hovered dev card on top with 3D effect
    if let Some(i) = hovered_dev_index {
        if let Some((devcard, count)) = owned_dev_cards.get(i) {
            let pos = dev_start_pos + egui::vec2((card_size.x + spacing) * i as f32, 0.0);
            let rect = egui::Rect::from_min_size(pos, card_size);
            let (final_rect, was_clicked) = draw_card_with_3d_effect(ui, painter, devcard_texture(textures, *devcard).id(),
             rect, mouse_pos, hover_scale, hover_lift);
            
            //find the card ID if clicked
            if was_clicked && clicked_dev_card.is_none() {
                //get the first unplayed instance of this card type
                for (card_type, card_id) in player_dev_cards_instances {
                    if card_type == devcard {
                        clicked_dev_card = Some((*devcard, *card_id));
                        break;
                    }
                }
            }

            if *count > 1 {
                let badge_pos = egui::pos2(final_rect.right() - 15.0, final_rect.top() + 5.0);
                painter.circle_filled(badge_pos, 10.0, egui::Color32::from_rgb(255, 200, 0));
                painter.circle_stroke(badge_pos, 10.0, egui::Stroke::new(2.0, egui::Color32::BLACK));
                painter.text(badge_pos, egui::Align2::CENTER_CENTER, format!("{}x", count),
                    egui::FontId::proportional(12.0), egui::Color32::BLACK);
            }
        }
    }
    clicked_dev_card
}