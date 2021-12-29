mod cellular;
mod drunkard;
mod empty;
mod rooms;

use crate::prelude::*;
use crate::utils::Rect;

trait MapArchitect {
    fn new(&mut self, rng: &mut impl Rng) -> MapBuilder;
}

pub struct MapBuilder {
    pub map_spec: MapSpec,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
}

impl MapBuilder {
    const NUM_ROOMS: usize = 20;
    const NUM_MONSTERS: usize = 50;

    fn new(rng: &mut impl Rng) -> Self {
        let mut architect = drunkard::DrunkardsWalkArchitect;
        architect.new(rng)
    }

    fn fill(&mut self, tile: TileType) {
        self.map_spec.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            TILEMAP_WIDTH,
            TILEMAP_HEIGHT,
            &vec![self.map_spec.point2d_to_index(self.player_start)],
            &self.map_spec,
            1024.0,
        );

        const UNREACHABLE: f32 = f32::MAX;
        self.map_spec.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, &dist)| dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
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

    fn spawn_monsters(&self, start: &Point, rng: &mut impl Rng) -> Vec<Point> {
        let mut spawnable_tiles: Vec<Point> = self
            .map_spec
            .tiles
            .iter()
            .enumerate()
            .filter(|&(idx, &t)| {
                t == TileType::Floor
                    && DistanceAlg::Pythagoras
                        .distance2d(*start, self.map_spec.index_to_point2d(idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map_spec.index_to_point2d(idx))
            .collect();

        let mut spawns = Vec::new();
        for _ in 0..MapBuilder::NUM_MONSTERS {
            let target_index = rng.gen_range(0..spawnable_tiles.len());
            spawns.push(spawnable_tiles.remove(target_index));
        }

        spawns
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct MapSpec {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * TILEMAP_WIDTH) + x) as usize
}

impl MapSpec {
    const NUM_TILES: usize = (TILEMAP_WIDTH * TILEMAP_HEIGHT) as usize;

    fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; Self::NUM_TILES],
            revealed_tiles: vec![false; Self::NUM_TILES],
        }
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;

        if self.can_enter_tile(destination) {
            let idx = self.point2d_to_index(destination);
            Some(idx)
        } else {
            None
        }
    }
}

impl BaseMap for MapSpec {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != TileType::Floor
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for MapSpec {
    fn dimensions(&self) -> Point {
        Point::new(TILEMAP_WIDTH, TILEMAP_HEIGHT)
    }

    fn point2d_to_index(&self, pt: Point) -> usize {
        map_idx(pt.x, pt.y)
    }

    fn index_to_point2d(&self, idx: usize) -> Point {
        let bounds = self.dimensions();
        Point::new(idx % bounds.x as usize, idx / bounds.x as usize)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        let bounds = self.dimensions();
        pos.x >= 0 && pos.x < bounds.x && pos.y >= 0 && pos.y < bounds.y
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

pub fn make_tilemap(texture_atlas: Handle<TextureAtlas>) -> (Tilemap, MapBuilder) {
    let mut rng = rand::thread_rng();
    let map_builder = MapBuilder::new(&mut rng);

    let mut tilemap = Tilemap::builder()
        .dimensions(TILEMAP_WIDTH as u32, TILEMAP_HEIGHT as u32)
        .chunk_dimensions(8, 8, 1)
        .texture_dimensions(32, 32)
        .texture_atlas(texture_atlas)
        .auto_chunk()
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Sparse,
                ..Default::default()
            },
            1,
        )
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Sparse,
                ..Default::default()
            },
            2,
        )
        .finish()
        .unwrap();

    let tiles = map_builder
        .map_spec
        .tiles
        .iter()
        .enumerate()
        .map(|(i, tile)| {
            let sprite_index = match tile {
                TileType::Floor => to_cp437('.'),
                TileType::Wall => to_cp437('#'),
            };

            Tile {
                point: (
                    (i % TILEMAP_WIDTH as usize) as i32 - CAMERA_OFFSET_X,
                    (i / TILEMAP_WIDTH as usize) as i32 - CAMERA_OFFSET_Y,
                ),
                sprite_index,
                sprite_order: 0,
                tint: Color::WHITE,
            }
        })
        .collect::<Vec<_>>();
    tilemap.insert_tiles(tiles).unwrap();

    (tilemap, map_builder)
}
