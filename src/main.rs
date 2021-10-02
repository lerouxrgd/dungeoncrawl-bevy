mod components;
mod map;
mod spawner;
mod systems;
mod turn_state;
mod utils;

mod prelude {
    pub use bevy::app::Events;
    pub use bevy::input::keyboard::KeyboardInput;
    pub use bevy::input::ElementState;
    pub use bevy::prelude::*;
    pub use bevy::render::camera::Camera;
    pub use bevy_tilemap::prelude::*;
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
    .add_startup_system(setup.system())
    .add_state(TurnState::AwaitingInput)
    .init_resource::<Events<WantsToMove>>();

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

    // Setup map

    let mut rng = rand::thread_rng();
    let MapBuilder {
        map_spec,
        rooms,
        player_start,
    } = MapBuilder::new(&mut rng);

    let mut tilemap = Tilemap::builder()
        .dimensions(TILEMAP_WIDTH as u32, TILEMAP_HEIGHT as u32)
        .chunk_dimensions(8, 8, 1)
        .texture_dimensions(32, 32)
        .texture_atlas(texture_atlas_handle)
        .auto_chunk()
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Sparse,
                ..Default::default()
            },
            1,
        )
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Sparse,
                ..Default::default()
            },
            2,
        )
        .finish()
        .unwrap();

    let tiles = map_spec
        .tiles
        .iter()
        .enumerate()
        .map(|(i, tile)| {
            let sprite_index = match tile {
                TileType::Floor => to_cp437('.'),
                TileType::Wall => to_cp437('#'),
            };

            Tile {
                point: (
                    (i % TILEMAP_WIDTH as usize) as i32 - CAMERA_OFFSET_X,
                    (i / TILEMAP_WIDTH as usize) as i32 - CAMERA_OFFSET_Y,
                ),
                sprite_index,
                sprite_order: 0,
                tint: Color::WHITE,
            }
        })
        .collect::<Vec<_>>();
    tilemap.insert_tiles(tiles).unwrap();

    // Insert resources

    commands.insert_resource(map_spec);

    // Spawn entities

    spawn_player(&mut commands, player_start, &mut tilemap);

    rooms
        .iter()
        .skip(1)
        .map(|r| r.center())
        .for_each(|pos| spawn_monster(&mut commands, &mut rng, pos, &mut tilemap));

    spawn_tilemap(&mut commands, tilemap);

    spawn_camera(&mut commands, player_start);
}
