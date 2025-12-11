//rendering the hex tiles etc.

pub trait RenderSystem {
    fn board_render(&self);
    fn tile_generate(&self);
    fn update(&self);
    //...
}