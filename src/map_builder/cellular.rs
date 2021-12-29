use super::MapArchitect;
use crate::prelude::*;

pub struct CellularAutomataArchitect;

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut ThreadRng) -> MapBuilder {
        let mut mb = MapBuilder {
            map_spec: MapSpec::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        self.random_noise_map(rng, &mut mb.map_spec);
        for _ in 0..10 {
            self.iteration(&mut mb.map_spec);
        }
        let start = self.find_start(&mb.map_spec);
        mb.monster_spawns = mb.spawn_monsters(&start, rng);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();

        mb
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, rng: &mut impl Rng, map_spec: &mut MapSpec) {
        map_spec.tiles.iter_mut().for_each(|t| {
            let roll = rng.gen_range(0..100);
            if roll > 55 {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        });
    }

    fn count_neighbors(&self, x: i32, y: i32, map_spec: &MapSpec) -> usize {
        let mut neighbors = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix == 0 && iy == 0)
                    && map_spec.tiles[map_idx(x + ix, y + iy)] == TileType::Wall
                {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }

    fn iteration(&mut self, map_spec: &mut MapSpec) {
        let mut new_tiles = map_spec.tiles.clone();
        for y in 1..TILEMAP_HEIGHT - 1 {
            for x in 1..TILEMAP_WIDTH - 1 {
                let neighbors = self.count_neighbors(x, y, map_spec);
                let idx = map_idx(x, y);
                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        map_spec.tiles = new_tiles;
    }

    fn find_start(&self, map_spec: &MapSpec) -> Point {
        let center = Point::new(TILEMAP_WIDTH / 2, TILEMAP_HEIGHT / 2);
        let closest_point = map_spec
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, &t)| t == TileType::Floor)
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(center, map_spec.index_to_point2d(idx)),
                )
            })
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(&d2).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();
        map_spec.index_to_point2d(closest_point)
    }
}
