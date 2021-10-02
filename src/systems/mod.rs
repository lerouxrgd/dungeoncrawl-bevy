use crate::prelude::*;

mod collisions;
mod end_turn;
mod movement;
mod player_input;
mod random_move;

use collisions::*;
use end_turn::*;
use movement::*;
use player_input::*;
use random_move::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
enum Stage {
    PlayerMove,
    PlayerCollision,
    MonsterMove,
    MonsterCollision,
}

pub fn add_systems(app: &mut AppBuilder) -> &mut AppBuilder {
    // Setup stages

    app.add_stage(Stage::PlayerMove, SystemStage::parallel());
    app.add_stage_after(
        Stage::PlayerMove,
        Stage::PlayerCollision,
        SystemStage::parallel(),
    );
    app.add_stage_after(
        Stage::PlayerCollision,
        Stage::MonsterMove,
        SystemStage::parallel(),
    );
    app.add_stage_after(
        Stage::MonsterMove,
        Stage::MonsterCollision,
        SystemStage::parallel(),
    );

    app.add_system_set_to_stage(Stage::PlayerMove, State::<TurnState>::get_driver());
    app.add_system_set_to_stage(Stage::PlayerCollision, State::<TurnState>::get_driver());
    app.add_system_set_to_stage(Stage::MonsterMove, State::<TurnState>::get_driver());
    app.add_system_set_to_stage(Stage::MonsterCollision, State::<TurnState>::get_driver());

    // In state AwaitingInput

    app.add_system_set_to_stage(
        Stage::PlayerMove,
        SystemSet::on_update(TurnState::AwaitingInput).with_system(player_input.system()),
    );

    // In state PlayerTurn

    app.add_system_set_to_stage(
        Stage::PlayerMove,
        SystemSet::on_update(TurnState::PlayerTurn).with_system(movement.system()),
    );

    app.add_system_set_to_stage(
        Stage::PlayerCollision,
        SystemSet::on_update(TurnState::PlayerTurn)
            .with_system(collisions.system().label("collisions"))
            .with_system(end_turn.system().after("collisions")),
    );

    // In state MonsterTurn

    app.add_system_set_to_stage(
        Stage::MonsterMove,
        SystemSet::on_update(TurnState::MonsterTurn)
            .with_system(random_move.system().label("random_move"))
            .with_system(movement.system().after("random_move")),
    );

    app.add_system_set_to_stage(
        Stage::MonsterCollision,
        SystemSet::on_update(TurnState::MonsterTurn)
            .with_system(collisions.system().label("collisions"))
            .with_system(end_turn.system().after("collisions")),
    );

    app
}
