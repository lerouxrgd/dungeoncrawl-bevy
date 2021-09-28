use crate::prelude::*;

mod collisions;
mod end_turn;
mod player_input;
mod random_move;

use collisions::*;
use end_turn::*;
use player_input::*;
use random_move::*;

pub fn add_systems(app: &mut AppBuilder) -> &mut AppBuilder {
    app.add_system_set(
        SystemSet::on_update(TurnState::AwaitingInput).with_system(player_input.system()),
    );

    app.add_system_set(
        SystemSet::on_update(TurnState::PlayerTurn)
            .with_system(collisions.system().label("collisions"))
            .with_system(end_turn.system().after("collisions")),
    );

    app.add_system_set(
        SystemSet::on_update(TurnState::MonsterTurn)
            .with_system(random_move.system().label("random_move"))
            .with_system(collisions.system().label("collisions").after("random_move"))
            .with_system(end_turn.system().after("collisions")),
    );

    app
}
