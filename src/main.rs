mod components;
mod map;
mod spawner;
mod systems;
mod turn_state;
mod utils;

mod prelude {
    pub use bevy::app::Events;
    pub use bevy::ecs::component::Component;
    pub use bevy::input::keyboard::KeyboardInput;
    pub use bevy::input::ElementState;
    pub use bevy::prelude::*;
    pub use bevy::render::camera::{Camera, OrthographicProjection};
    pub use bevy::text::Text2dSize;
    pub use bevy::window::WindowResized;
    pub use bevy_prototype_lyon::prelude::*;
    pub use bevy_tilemap::prelude::*;
    pub use bracket_pathfinding::prelude::*;
    pub use lyon_tessellation::path::Path;
    pub use rand::Rng;

    pub const TILEMAP_WIDTH: i32 = 80;
    pub const TILEMAP_HEIGHT: i32 = 50;
    pub const CAMERA_OFFSET_X: i32 = TILEMAP_WIDTH / 2;
    pub const CAMERA_OFFSET_Y: i32 = TILEMAP_HEIGHT / 2;

    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use crate::utils::*;
}

use prelude::*;

fn main() {
    let mut app = App::build();

    app.insert_resource(WindowDescriptor {
        title: "Dungeoncrawl Bevy".to_string(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugins(TilemapDefaultPlugins)
    .add_plugin(ShapePlugin)
    .add_startup_system(setup.system())
    .add_state(TurnState::AwaitingInput)
    .init_resource::<Events<WantsToMove>>()
    .init_resource::<Events<WantsToAttack>>();

    add_systems(&mut app);

    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Setup textures

    let texture_handle = asset_server.load("dungeonfont.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 16, 16);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(texture_atlas_handle.clone());

    // Setup font

    let font_handle: Handle<Font> = asset_server.load("BigBlue_Terminal_437TT.TTF");
    commands.insert_resource(font_handle.clone());

    // Build map

    let (mut tilemap, map_builder) = make_tilemap(texture_atlas_handle.clone());
    let MapBuilder {
        player_start,
        amulet_start,
        rooms,
        map_spec,
    } = map_builder;

    commands.insert_resource(map_spec);

    // Spawn entities

    spawn_player(&mut commands, player_start, &mut tilemap);
    spawn_amulet_of_yala(&mut commands, amulet_start, &mut tilemap);
    rooms
        .iter()
        .skip(1)
        .map(|r| r.center())
        .for_each(|pos| spawn_monster(&mut commands, pos, &mut tilemap));

    spawn_hud(&mut commands, font_handle.clone());
    spawn_tilemap(&mut commands, tilemap);

    spawn_camera(&mut commands, player_start);
    spawn_end_game_screens(&mut commands, font_handle.clone());
}
