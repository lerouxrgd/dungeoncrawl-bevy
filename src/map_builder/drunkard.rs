use super::MapArchitect;
use crate::prelude::*;

const STAGGER_DISTANCE: usize = 400;
const NUM_TILES: usize = (TILEMAP_WIDTH * TILEMAP_HEIGHT) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;

pub struct DrunkardsWalkArchitect;

impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self, rng: &mut ThreadRng) -> MapBuilder {
        let mut mb = MapBuilder {
            map_spec: MapSpec::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        mb.fill(TileType::Wall);

        let center = Point::new(TILEMAP_WIDTH / 2, TILEMAP_HEIGHT / 2);
        self.drunkard(&center, rng, &mut mb.map_spec);

        while mb
            .map_spec
            .tiles
            .iter()
            .filter(|&&t| t == TileType::Floor)
            .count()
            < DESIRED_FLOOR
        {
            let random_pos = Point::new(
                rng.gen_range(0..TILEMAP_WIDTH),
                rng.gen_range(0..TILEMAP_HEIGHT),
            );
            self.drunkard(&random_pos, rng, &mut mb.map_spec);

            let dijkstra_map = DijkstraMap::new(
                TILEMAP_WIDTH,
                TILEMAP_HEIGHT,
                &vec![mb.map_spec.point2d_to_index(center)],
                &mb.map_spec,
                1024.0,
            );

            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, &distance)| distance > 2000.0)
                .for_each(|(idx, _)| mb.map_spec.tiles[idx] = TileType::Wall);
        }

        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();

        mb
    }
}

impl DrunkardsWalkArchitect {
    fn drunkard(&mut self, start: &Point, rng: &mut impl Rng, map: &mut MapSpec) {
        let mut drunkard_pos = start.clone();
        let mut distance_staggered = 0;

        loop {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            match rng.gen_range(0..4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            }
            if !map.in_bounds(drunkard_pos) {
                break;
            }

            distance_staggered += 1;
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}
