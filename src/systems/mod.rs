use crate::prelude::*;

mod combat;
mod end_turn;
mod hud;
mod movement;
mod player_input;
mod random_move;
mod tooltips;

use combat::*;
use end_turn::*;
use hud::*;
use movement::*;
use player_input::*;
use random_move::*;
use tooltips::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
enum GameStage {
    PlayerTurn,
    MonsterTurn,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
enum TooltipStage {
    Create,
    Display,
}

pub fn add_systems(app: &mut AppBuilder) -> &mut AppBuilder {
    // Setup game stages

    app.add_stage(GameStage::PlayerTurn, SystemStage::parallel());
    app.add_stage_after(
        GameStage::PlayerTurn,
        GameStage::MonsterTurn,
        SystemStage::parallel(),
    );

    app.add_system_set_to_stage(GameStage::PlayerTurn, State::<TurnState>::get_driver());
    app.add_system_set_to_stage(GameStage::MonsterTurn, State::<TurnState>::get_driver());

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

    // Game systems

    app.add_system_set(
        SystemSet::on_update(TurnState::AwaitingInput).with_system(player_input.system()),
    );

    app.add_system_set_to_stage(
        GameStage::PlayerTurn,
        SystemSet::on_update(TurnState::PlayerTurn)
            .with_system(combat.system().label("combat"))
            .with_system(movement.system().label("movement").after("combat"))
            .with_system(end_turn.system().after("movement")),
    );

    app.add_system_set_to_stage(
        GameStage::MonsterTurn,
        SystemSet::on_update(TurnState::MonsterTurn)
            .with_system(random_move.system().label("random_move"))
            .with_system(combat.system().label("combat").after("random_move"))
            .with_system(movement.system().label("movement").after("combat"))
            .with_system(end_turn.system().after("movement")),
    );

    // Tooltips systems

    app.add_system_to_stage(TooltipStage::Create, tooltips.system());
    app.add_system_to_stage(TooltipStage::Display, tooltips_display.system());

    // Independent systems

    app.add_system(hud.system());

    app
}
