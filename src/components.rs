use std::collections::HashSet;

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
    pub fov: FieldOfView,
}

#[derive(Default)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub position: Point,
    pub render: Render,
    pub chasing_player: ChasingPlayer,
    pub health: Health,
    pub name: Name,
    pub fov: FieldOfView,
}

#[derive(Default)]
pub struct MovingRandomly;

pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

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

pub trait ScreenText: Component {
    fn pos(&self) -> usize;
    fn offset(&self) -> f32;
}

pub struct VictoryText {
    pub pos: usize,
    pub offset: f32,
}

impl VictoryText {
    pub fn new(pos: usize) -> Self {
        Self { pos, offset: 0. }
    }

    pub fn with_offset(pos: usize, offset: f32) -> Self {
        Self { pos, offset }
    }
}

impl ScreenText for VictoryText {
    fn pos(&self) -> usize {
        self.pos
    }

    fn offset(&self) -> f32 {
        self.offset
    }
}

pub struct GameoverText {
    pub pos: usize,
    pub offset: f32,
}

impl GameoverText {
    pub fn new(pos: usize) -> Self {
        Self { pos, offset: 0. }
    }

    pub fn with_offset(pos: usize, offset: f32) -> Self {
        Self { pos, offset }
    }
}

impl ScreenText for GameoverText {
    fn pos(&self) -> usize {
        self.pos
    }

    fn offset(&self) -> f32 {
        self.offset
    }
}

#[derive(Default)]
pub struct Item;

#[derive(Default)]
pub struct AmuletOfYala;

#[derive(Bundle)]
pub struct AmuletBundle {
    pub amulet: AmuletOfYala,
    pub position: Point,
    pub render: Render,
    pub name: Name,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}
