use crate::prelude::*;

mod collisions;
mod end_turn;
mod hud;
mod movement;
mod player_input;
mod random_move;
mod tooltips;

use collisions::*;
use end_turn::*;
use hud::*;
use movement::*;
use player_input::*;
use random_move::*;
use tooltips::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
enum GameStage {
    PlayerMove,
    PlayerCollision,
    MonsterMove,
    MonsterCollision,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
enum TooltipStage {
    Create,
    Display,
}

pub fn add_systems(app: &mut AppBuilder) -> &mut AppBuilder {
    // Setup game stages

    app.add_stage(GameStage::PlayerMove, SystemStage::parallel());
    app.add_stage_after(
        GameStage::PlayerMove,
        GameStage::PlayerCollision,
        SystemStage::parallel(),
    );
    app.add_stage_after(
        GameStage::PlayerCollision,
        GameStage::MonsterMove,
        SystemStage::parallel(),
    );
    app.add_stage_after(
        GameStage::MonsterMove,
        GameStage::MonsterCollision,
        SystemStage::parallel(),
    );

    app.add_system_set_to_stage(GameStage::PlayerMove, State::<TurnState>::get_driver());
    app.add_system_set_to_stage(GameStage::PlayerCollision, State::<TurnState>::get_driver());
    app.add_system_set_to_stage(GameStage::MonsterMove, State::<TurnState>::get_driver());
    app.add_system_set_to_stage(
        GameStage::MonsterCollision,
        State::<TurnState>::get_driver(),
    );

    // Setup tooltip stages

    app.add_stage_before(
        CoreStage::PostUpdate,
        TooltipStage::Display,
        SystemStage::parallel(),
    );
    app.add_stage_before(
        TooltipStage::Display,
        TooltipStage::Create,
        SystemStage::parallel(),
    );

    // TurnState::AwaitingInput systems

    app.add_system_set_to_stage(
        GameStage::PlayerMove,
        SystemSet::on_update(TurnState::AwaitingInput).with_system(player_input.system()),
    );

    // TurnState::PlayerTurn systems

    app.add_system_set_to_stage(
        GameStage::PlayerMove,
        SystemSet::on_update(TurnState::PlayerTurn).with_system(movement.system()),
    );
    app.add_system_set_to_stage(
        GameStage::PlayerCollision,
        SystemSet::on_update(TurnState::PlayerTurn)
            .with_system(collisions.system().label("collisions"))
            .with_system(end_turn.system().after("collisions")),
    );

    // TurnState::MonsterTurn systems

    app.add_system_set_to_stage(
        GameStage::MonsterMove,
        SystemSet::on_update(TurnState::MonsterTurn)
            .with_system(random_move.system().label("random_move"))
            .with_system(movement.system().after("random_move")),
    );
    app.add_system_set_to_stage(
        GameStage::MonsterCollision,
        SystemSet::on_update(TurnState::MonsterTurn)
            .with_system(collisions.system().label("collisions"))
            .with_system(end_turn.system().after("collisions")),
    );

    // Tooltips systems

    app.add_system_to_stage(TooltipStage::Create, tooltips.system());
    app.add_system_to_stage(TooltipStage::Display, tooltips_display.system());

    // Independent systems

    app.add_system(hud.system());

    app
}
