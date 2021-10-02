use crate::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct Render {
    pub sprite_index: usize,
    pub sprite_order: usize,
}

#[derive(Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub position: Point,
    pub render: Render,
}

#[derive(Default)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub position: Point,
    pub render: Render,
    pub moving_randomly: MovingRandomly,
}

#[derive(Default)]
pub struct MovingRandomly;

pub struct WantsToMove {
    pub entity: Entity,
    pub origin: Point,
    pub destination: Point,
    pub render: Render,
}
