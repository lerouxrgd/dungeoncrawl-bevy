mod components;
mod map;
mod spawner;
mod systems;
mod utils;

mod prelude {
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
    .add_startup_system(setup.system());

    add_systems(&mut app);

    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // setup textures

    let texture_handle = asset_server.load("dungeonfont.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 16, 16);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // setup tilemap

    let mut rng = rand::thread_rng();
    let map_builder = MapBuilder::new(&mut rng);

    let mut tilemap = Tilemap::builder()
        .dimensions(TILEMAP_WIDTH as u32, TILEMAP_HEIGHT as u32)
        .chunk_dimensions(8, 8, 1)
        .texture_dimensions(32, 32)
        .texture_atlas(texture_atlas_handle)
        .auto_chunk()
        .finish()
        .unwrap();

    let mut tiles = map_builder
        .map_spec
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
                tint: Color::WHITE, // tint and alpha of the tile. White means no change.
            }
        })
        .collect::<Vec<_>>();

    tiles.push(Tile {
        point: (
            map_builder.player_start.x - CAMERA_OFFSET_X,
            map_builder.player_start.y - CAMERA_OFFSET_Y,
        ),
        sprite_index: to_cp437('@'),
        sprite_order: 1,
        tint: Color::WHITE,
    });

    tilemap.insert_tiles(tiles).unwrap();

    let tilemap_components = TilemapBundle {
        tilemap,
        visible: Visible {
            is_visible: true,
            is_transparent: true,
        },
        transform: Default::default(),
        global_transform: Default::default(),
    };

    commands.insert_resource(map_builder.map_spec);
    commands
        .spawn()
        .insert_bundle(tilemap_components)
        .insert(Timer::from_seconds(0.075, true));

    // setup camera

    let mut camera = OrthographicCameraBundle::new_2d();

    camera.transform.translation.x =
        (map_builder.player_start.x as f32 - CAMERA_OFFSET_X as f32) * 32.;
    camera.transform.translation.y =
        (map_builder.player_start.y as f32 - CAMERA_OFFSET_Y as f32) * 32.;

    commands.spawn_bundle(camera);

    // setup player

    spawn_player(&mut commands, map_builder.player_start);
}
