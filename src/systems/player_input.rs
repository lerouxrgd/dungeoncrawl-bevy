use crate::prelude::*;

pub fn player_input(
    mut key_evr: EventReader<KeyboardInput>,
    mut turn_state: ResMut<State<TurnState>>,
    mut ev_movements: EventWriter<WantsToMove>,
    mut player_query: Query<(Entity, &Point, &Render), With<Player>>,
) {
    for ev in key_evr.iter().take(1) {
        let delta = match (ev.state, ev.key_code) {
            (ElementState::Pressed, Some(KeyCode::Up)) => Point::new(0, 1),
            (ElementState::Pressed, Some(KeyCode::Left)) => Point::new(-1, 0),
            (ElementState::Pressed, Some(KeyCode::Down)) => Point::new(0, -1),
            (ElementState::Pressed, Some(KeyCode::Right)) => Point::new(1, 0),
            _ => return,
        };

        let (player, &origin, &render) = player_query.single_mut().unwrap();
        let destination = origin + delta;
        ev_movements.send(WantsToMove {
            entity: player,
            origin,
            destination,
            render,
        });

        turn_state.set(TurnState::PlayerTurn).unwrap();
    }
}
