use crate::prelude::*;

pub struct EmptyArchitect;

impl super::MapArchitect for EmptyArchitect {
    fn new(&mut self, rng: &mut ThreadRng) -> MapBuilder {
        let mut mb = MapBuilder {
            map_spec: MapSpec::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: super::themes::DungeonTheme::new(),
        };

        mb.fill(TileType::Floor);
        mb.player_start = Point::new(TILEMAP_WIDTH / 2, TILEMAP_HEIGHT / 2);
        mb.amulet_start = mb.find_most_distant();

        for _ in 0..MapBuilder::NUM_MONSTERS {
            mb.monster_spawns.push(Point::new(
                rng.gen_range(1..TILEMAP_WIDTH),
                rng.gen_range(1..TILEMAP_HEIGHT),
            ))
        }

        mb
    }
}
