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
    pub health: Health,
}

#[derive(Default)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub position: Point,
    pub render: Render,
    pub moving_randomly: MovingRandomly,
    pub health: Health,
    pub name: Name,
}

#[derive(Default)]
pub struct MovingRandomly;

pub struct WantsToMove {
    pub entity: Entity,
    pub origin: Point,
    pub destination: Point,
    pub render: Render,
}

#[derive(Default)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Default)]
pub struct Name(pub String);

#[derive(Default)]
pub struct Hud;

#[derive(Default)]
pub struct HealthText;

#[derive(Default)]
pub struct HealthBar;

#[derive(Default)]
pub struct InfoText;

#[derive(Default)]
pub struct TooltipText;
