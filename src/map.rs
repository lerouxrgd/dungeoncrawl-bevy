use crate::prelude::*;
use crate::utils::Rect;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct MapSpec {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * TILEMAP_WIDTH) + x) as usize
}

impl MapSpec {
    const NUM_TILES: usize = (TILEMAP_WIDTH * TILEMAP_HEIGHT) as usize;

    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; Self::NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < TILEMAP_WIDTH && point.y >= 0 && point.y < TILEMAP_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}

pub struct MapBuilder {
    pub map_spec: MapSpec,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    const NUM_ROOMS: usize = 20;

    pub fn new(rng: &mut impl Rng) -> Self {
        let mut mb = MapBuilder {
            map_spec: MapSpec::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();

        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map_spec.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut impl Rng) {
        while self.rooms.len() < Self::NUM_ROOMS {
            let room = Rect::with_size(
                rng.gen_range(1..TILEMAP_WIDTH - 10),
                rng.gen_range(1..TILEMAP_HEIGHT - 10),
                rng.gen_range(2..10),
                rng.gen_range(2..10),
            );

            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < TILEMAP_WIDTH && p.y > 0 && p.y < TILEMAP_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map_spec.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room)
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map_spec.try_idx(Point::new(x, y)) {
                self.map_spec.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map_spec.try_idx(Point::new(x, y)) {
                self.map_spec.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut impl Rng) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.gen_range(0..2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}

pub fn move_sprite(tilemap: &mut Tilemap, prev_pos: Point, new_pos: Point, render: &Render) {
    // We need to first remove where we were prior.
    tilemap
        .clear_tile(
            (prev_pos.x - CAMERA_OFFSET_X, prev_pos.y - CAMERA_OFFSET_Y),
            render.sprite_order,
        )
        .unwrap();

    // We then need to update where we are going!
    let tile = Tile {
        point: (new_pos.x - CAMERA_OFFSET_X, new_pos.y - CAMERA_OFFSET_Y),
        sprite_index: render.sprite_index,
        sprite_order: render.sprite_order,
        ..Default::default()
    };

    tilemap.insert_tile(tile).unwrap();
}
