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
    let sprite_index = to_cp437('@');
    let sprite_order = 2;

    commands.spawn().insert_bundle(PlayerBundle {
        player: Player,
        position,
        render: Render {
            sprite_index,
            sprite_order,
        },
        health: Health {
            current: 10,
            max: 10,
        },
    });

    tilemap
        .insert_tile(Tile {
            point: (position.x - CAMERA_OFFSET_X, position.y - CAMERA_OFFSET_Y),
            sprite_index,
            sprite_order,
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
    fn goblin() -> (i32, String, usize) {
        (1, "Goblin".to_string(), to_cp437('g'))
    }

    fn orc() -> (i32, String, usize) {
        (2, "Orc".to_string(), to_cp437('o'))
    }

    let sprite_order = 1;
    let (hp, name, sprite_index) = match rng.gen_range(1..=10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    commands.spawn().insert_bundle(EnemyBundle {
        enemy: Enemy,
        position,
        render: Render {
            sprite_index,
            sprite_order,
        },
        chasing_player: ChasingPlayer,
        health: Health {
            current: hp,
            max: hp,
        },
        name: Name(name),
    });

    tilemap
        .insert_tile(Tile {
            point: (position.x - CAMERA_OFFSET_X, position.y - CAMERA_OFFSET_Y),
            sprite_index,
            sprite_order,
            tint: Color::WHITE,
        })
        .unwrap();
}

pub fn spawn_hud(commands: &mut Commands, asset_server: &AssetServer) {
    let health_bar = GeometryBuilder::build_as(
        &shapes::Rectangle::default(),
        ShapeColors::new(Color::RED),
        DrawMode::Fill(FillOptions::default()),
        Transform::default(),
    );

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Health: {} / {}",
                TextStyle {
                    font: asset_server.load("BigBlue_Terminal_437TT.TTF"),
                    font_size: 10.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Bottom,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(Hud)
        .insert(HealthText)
        .with_children(|parent| {
            parent
                .spawn_bundle(health_bar)
                .insert(Hud)
                .insert(HealthBar);

            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        "Explore the Dungeon. Cursor keys to move.",
                        TextStyle {
                            font: asset_server.load("BigBlue_Terminal_437TT.TTF"),
                            font_size: 10.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Bottom,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(Hud)
                .insert(InfoText);
        });
}
