use crate::prelude::*;
use crate::utils::Rect;

const FORTRESS: (&str, i32, i32) = (
    "
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
",
    12,
    11,
);

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut impl Rng) {
    let mut placement = None;

    let dijkstra_map = DijkstraMap::new(
        TILEMAP_WIDTH,
        TILEMAP_HEIGHT,
        &vec![mb.map_spec.point2d_to_index(mb.player_start)],
        &mb.map_spec,
        1024.0,
    );

    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        let dimensions = Rect::with_size(
            rng.gen_range(0..TILEMAP_WIDTH - FORTRESS.1),
            rng.gen_range(0..TILEMAP_HEIGHT - FORTRESS.2),
            FORTRESS.1,
            FORTRESS.2,
        );

        let mut can_place = false;
        dimensions.for_each(|pt| {
            let idx = mb.map_spec.point2d_to_index(pt);
            let distance = dijkstra_map.map[idx];
            if distance < 2000.0 && distance > 20.0 && mb.amulet_start != pt {
                can_place = true;
            }
        });

        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.monster_spawns.retain(|pt| !points.contains(pt));
        }

        attempts += 1;
    }

    if let Some(placement) = placement {
        let string_vec: Vec<char> = FORTRESS
            .0
            .chars()
            .filter(|&a| a != '\r' && a != '\n')
            .collect();

        let mut i = 0;
        for ty in placement.y..placement.y + FORTRESS.2 {
            for tx in placement.x..placement.x + FORTRESS.1 {
                let idx = map_idx(tx, ty);
                let c = string_vec[i];

                match c {
                    'M' => {
                        mb.map_spec.tiles[idx] = TileType::Floor;
                        mb.monster_spawns.push(Point::new(tx, ty));
                    }
                    '-' => mb.map_spec.tiles[idx] = TileType::Floor,
                    '#' => mb.map_spec.tiles[idx] = TileType::Wall,
                    _ => println!("No idea what to do with [{}]", c),
                }

                i += 1;
            }
        }
    }
}
