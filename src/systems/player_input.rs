use crate::prelude::*;

pub fn player_input(
    mut key_evr: EventReader<KeyboardInput>,
    mut turn_state: ResMut<State<TurnState>>,
    mut ev_movements: EventWriter<WantsToMove>,
    mut ev_attacks: EventWriter<WantsToAttack>,
    mut player_query: Query<(Entity, &Point, &mut Health), With<Player>>,
    enemies_query: Query<(Entity, &Point), With<Enemy>>,
) {
    for ev in key_evr.iter().take(1) {
        let delta = match (ev.state, ev.key_code) {
            (ElementState::Pressed, Some(KeyCode::Up)) => Point::new(0, 1),
            (ElementState::Pressed, Some(KeyCode::Left)) => Point::new(-1, 0),
            (ElementState::Pressed, Some(KeyCode::Down)) => Point::new(0, -1),
            (ElementState::Pressed, Some(KeyCode::Right)) => Point::new(1, 0),
            (ElementState::Pressed, Some(_)) => Point::new(0, 0),
            _ => return,
        };

        let (player, &origin, mut health) = player_query.single_mut().unwrap();

        // No movement => heal
        if delta == Point::new(0, 0) {
            health.current = i32::min(health.max, health.current + 1);
        } else
        // Move or attack
        {
            let destination = origin + delta;

            let mut hit_something = false;
            enemies_query
                .iter()
                .filter(|(_, &pos)| pos == destination)
                .for_each(|(enemy, _)| {
                    hit_something = true;

                    ev_attacks.send(WantsToAttack {
                        attacker: player,
                        victim: enemy,
                    });
                });

            if !hit_something {
                ev_movements.send(WantsToMove {
                    entity: player,
                    destination,
                });
            }
        }

        turn_state.set(TurnState::PlayerTurn).unwrap();
    }
}
