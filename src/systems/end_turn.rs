use crate::prelude::*;

pub fn end_turn(
    map_spec: Res<MapSpec>,
    mut turn_state: ResMut<State<TurnState>>,
    player_query: Query<(&Point, &Health), With<Player>>,
    amulet_query: Query<&Point, With<AmuletOfYala>>,
) {
    let current_state = turn_state.current().clone();
    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    let (player_pos, player_hp) = player_query.single().unwrap();
    if player_hp.current < 1 {
        new_state = TurnState::GameOver;
    }

    let amulet_default = Point::new(-1, -1);
    let amulet_pos = amulet_query.single().unwrap_or(&amulet_default);
    if player_pos == amulet_pos {
        new_state = TurnState::Victory;
    }

    let idx = map_idx(player_pos.x, player_pos.y);
    if map_spec.tiles[idx] == TileType::Exit {
        new_state = TurnState::NextLevel;
    }

    if &new_state != turn_state.current() {
        turn_state.set(new_state).unwrap();
    }
}
