use crate::prelude::*;

pub fn spawn_tilemap(commands: &mut Commands, tilemap: Tilemap) {
    let tilemap_components = TilemapBundle {
        tilemap,
        visible: Visible {
            is_visible: true,
            is_transparent: true,
        },
        transform: Default::default(),
        global_transform: Default::default(),
    };

    commands.spawn().insert_bundle(tilemap_components);
}

pub fn spawn_camera(commands: &mut Commands, player_start: Point) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.transform.translation.x = (player_start.x as f32 - CAMERA_OFFSET_X as f32) * 32.;
    camera.transform.translation.y = (player_start.y as f32 - CAMERA_OFFSET_Y as f32) * 32.;

    commands.spawn_bundle(camera);
}

pub fn spawn_player(commands: &mut Commands, position: Point, tilemap: &mut Tilemap) {
    commands.spawn().insert_bundle(PlayerBundle {
        player: Player,
        position,
        render: Render {
            sprite_index: to_cp437('@'),
            sprite_order: 1,
        },
    });

    tilemap
        .insert_tile(Tile {
            point: (position.x - CAMERA_OFFSET_X, position.y - CAMERA_OFFSET_Y),
            sprite_index: to_cp437('@'),
            sprite_order: 1,
            tint: Color::WHITE,
        })
        .unwrap();
}

pub fn spawn_monster(
    commands: &mut Commands,
    rng: &mut impl Rng,
    position: Point,
    tilemap: &mut Tilemap,
) {
    let sprite_index = match rng.gen_range(0..4) {
        0 => to_cp437('E'),
        1 => to_cp437('O'),
        2 => to_cp437('o'),
        _ => to_cp437('g'),
    };

    commands.spawn().insert_bundle(EnemyBundle {
        enemy: Enemy,
        position,
        render: Render {
            sprite_index,
            sprite_order: 2,
        },
        moving_randomly: MovingRandomly,
    });

    tilemap
        .insert_tile(Tile {
            point: (position.x - CAMERA_OFFSET_X, position.y - CAMERA_OFFSET_Y),
            sprite_index,
            sprite_order: 2,
            tint: Color::WHITE,
        })
        .unwrap();
}
