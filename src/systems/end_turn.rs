use crate::prelude::*;

pub fn end_turn(mut turn_state: ResMut<State<TurnState>>) {
    let new_state = match turn_state.current() {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };

    turn_state.set(new_state).unwrap();
}
