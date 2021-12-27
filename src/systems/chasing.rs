use crate::prelude::*;

pub fn chasing(
    map_spec: Res<MapSpec>,
    mut ev_movements: EventWriter<WantsToMove>,
    mut ev_attacks: EventWriter<WantsToAttack>,
    movers_query: Query<(Entity, &Point, &FieldOfView), With<ChasingPlayer>>,
    positions_query: Query<(Entity, &Point, &Health)>,
    player_query: Query<(Entity, &Point), With<Player>>,
) {
    let map_spec = &*map_spec;

    let (player, &player_pos) = player_query.single().unwrap();
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(
        TILEMAP_WIDTH,
        TILEMAP_HEIGHT,
        &search_targets,
        map_spec,
        1024.0,
    );

    movers_query.for_each(|(mover, &source_pos, fov)| {
        if !fov.visible_tiles.contains(&player_pos) {
            return;
        }

        let idx = map_idx(source_pos.x, source_pos.y);
        let destination = match DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map_spec) {
            Some(destination) => destination,
            None => return,
        };

        // 1.2 is smaller than 1.4 which is approx. the diag dist => prevent diag attacks
        let distance = DistanceAlg::Pythagoras.distance2d(source_pos, player_pos);
        let destination = if distance > 1.2 {
            map_spec.index_to_point2d(destination)
        } else {
            player_pos
        };

        let mut attacked = false;
        positions_query
            .iter()
            .filter(|(_, &target_pos, _)| target_pos == destination)
            .for_each(|(victim, _, _)| {
                if victim == player {
                    ev_attacks.send(WantsToAttack {
                        attacker: mover,
                        victim,
                    });
                }
                attacked = true;
            });

        if !attacked {
            ev_movements.send(WantsToMove {
                entity: mover,
                destination,
            });
        }
    });
}
