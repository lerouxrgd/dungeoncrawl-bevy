use crate::prelude::*;

mod chasing;
mod combat;
mod end_game;
mod end_turn;
mod fov;
mod hud;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod tooltips;
mod use_items;

use chasing::*;
use combat::*;
use end_game::*;
use end_turn::*;
use fov::*;
use hud::*;
use map_render::*;
use movement::*;
use player_input::*;
use random_move::*;
use tooltips::*;
use use_items::*;

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

    app.stage(CoreStage::Startup, |schedule: &mut Schedule| {
        schedule.add_system_to_stage(StartupStage::PostStartup, fov.system())
    });

    app.add_system_set(
        SystemSet::on_update(TurnState::AwaitingInput)
            .with_system(player_input.system())
            .with_system(map_render.system())
            .with_system(main_hud.system())
            .with_system(inventory_hud.system())
            .with_system(level_hud.system()),
    );

    app.add_system_set_to_stage(
        GameStage::PlayerTurn,
        SystemSet::on_update(TurnState::PlayerTurn)
            .with_system(use_items.system())
            .with_system(combat.system().label("combat"))
            .with_system(movement.system().label("movement").after("combat"))
            .with_system(end_turn.system().after("movement")),
    );

    app.add_system_set_to_stage(
        GameStage::MonsterTurn,
        SystemSet::on_update(TurnState::MonsterTurn)
            .with_system(random_move.system().label("random_move"))
            .with_system(chasing.system().label("chasing"))
            .with_system(combat.system().label("combat").after("chasing"))
            .with_system(movement.system().label("movement").after("combat"))
            .with_system(end_turn.system().after("movement")),
    );

    app.add_system_set_to_stage(
        GameStage::MonsterTurn,
        SystemSet::on_enter(TurnState::AwaitingInput).with_system(fov.system()),
    );

    // Next level systems

    app.add_system_set_to_stage(
        GameStage::MonsterTurn,
        SystemSet::on_enter(TurnState::NextLevel).with_system(despawn_level.system()),
    );
    app.add_system_set_to_stage(
        GameStage::MonsterTurn,
        SystemSet::on_exit(TurnState::NextLevel).with_system(respawn_level.system()),
    );

    // Victory systems

    app.add_system_set_to_stage(
        GameStage::MonsterTurn,
        SystemSet::on_enter(TurnState::Victory).with_system(despawn_game_state.system()),
    );
    app.add_system_set(
        SystemSet::on_update(TurnState::Victory).with_system(text_screen::<VictoryText>.system()),
    );
    app.add_system_set(
        SystemSet::on_exit(TurnState::Victory).with_system(respawn_game_state.system()),
    );

    // Gameover systems

    app.add_system_set_to_stage(
        GameStage::MonsterTurn,
        SystemSet::on_enter(TurnState::GameOver).with_system(despawn_game_state.system()),
    );
    app.add_system_set(
        SystemSet::on_update(TurnState::GameOver).with_system(text_screen::<GameoverText>.system()),
    );
    app.add_system_set(
        SystemSet::on_exit(TurnState::GameOver).with_system(respawn_game_state.system()),
    );

    // Tooltips systems

    app.add_system_to_stage(TooltipStage::Create, tooltips.system());
    app.add_system_to_stage(TooltipStage::Display, tooltips_display.system());

    app
}
