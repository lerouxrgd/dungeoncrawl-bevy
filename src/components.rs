use crate::prelude::*;

#[derive(Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub position: Point,
    pub render: Render,
}

#[derive(Default)]
pub struct Render {
    pub sprite_index: usize,
    pub sprite_order: usize,
}
