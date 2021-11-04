use crate::prelude::*;

pub fn random_move(
    mut ev_movements: EventWriter<WantsToMove>,
    mut ev_attacks: EventWriter<WantsToAttack>,
    movers_query: Query<(Entity, &Point), With<MovingRandomly>>,
    player_query: Query<Entity, With<Player>>,
    positions_query: Query<(Entity, &Point, &Health)>,
) {
    movers_query.iter().for_each(|(entity, pos)| {
        let mut rng = rand::thread_rng();

        let destination = match rng.gen_range(0..4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        let player = player_query.single().unwrap();
        let mut attacked = false;
        positions_query
            .iter()
            .filter(|(_, &pos, _)| pos == destination)
            .for_each(|(victim, _, _)| {
                if victim == player {
                    ev_attacks.send(WantsToAttack {
                        attacker: entity,
                        victim: player,
                    });
                }

                attacked = true;
            });

        if !attacked {
            ev_movements.send(WantsToMove {
                entity,
                destination,
            });
        }
    });
}
