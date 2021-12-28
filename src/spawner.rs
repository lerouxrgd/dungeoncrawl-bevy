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
        fov: FieldOfView::new(6),
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

pub fn spawn_monster(commands: &mut Commands, position: Point, tilemap: &mut Tilemap) {
    fn goblin() -> (i32, String, usize) {
        (1, "Goblin".to_string(), to_cp437('g'))
    }

    fn orc() -> (i32, String, usize) {
        (2, "Orc".to_string(), to_cp437('o'))
    }

    let mut rng = rand::thread_rng();
    let (hp, name, sprite_index) = match rng.gen_range(1..=10) {
        1..=8 => goblin(),
        _ => orc(),
    };
    let sprite_order = 1;

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
        fov: FieldOfView::new(6),
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

pub fn spawn_amulet_of_yala(commands: &mut Commands, position: Point, tilemap: &mut Tilemap) {
    let sprite_index = to_cp437('|');
    let sprite_order = 2;

    commands.spawn().insert_bundle(AmuletBundle {
        amulet: AmuletOfYala,
        position,
        render: Render {
            sprite_index,
            sprite_order,
        },
        name: Name("Amulet of Yala".to_string()),
        item: Item,
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

pub fn spawn_hud(commands: &mut Commands, font_handle: Handle<Font>) {
    let alignment = TextAlignment {
        vertical: VerticalAlign::Bottom,
        horizontal: HorizontalAlign::Center,
    };

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
                    font: font_handle.clone(),
                    font_size: 10.0,
                    color: Color::WHITE,
                },
                alignment,
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
                            font: font_handle.clone(),
                            font_size: 10.0,
                            color: Color::WHITE,
                        },
                        alignment,
                    ),
                    ..Default::default()
                })
                .insert(Hud)
                .insert(InfoText);
        });
}

pub fn spawn_end_game_screens(commands: &mut Commands, font_handle: Handle<Font>) {
    let alignment = TextAlignment {
        vertical: VerticalAlign::Bottom,
        horizontal: HorizontalAlign::Center,
    };
    let visible = Visible {
        is_visible: false,
        is_transparent: false,
    };

    // Victory

    let mut pos = 0;
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "You have won!",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 15.0,
                    color: Color::GREEN,
                },
                alignment,
            ),
            visible: visible.clone(),
            ..Default::default()
        })
        .insert(VictoryText::with_offset(pos, 15.))
        .with_children(|parent| {
            pos += 1;
            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        "You put on the Amulet of Yala and feel its power course through your veins.",
                        TextStyle {
                            font: font_handle.clone(),
                            font_size: 10.0,
                            color: Color::WHITE,
                        },
                        alignment,
                    ),
                    visible: visible.clone(),
                    ..Default::default()
                })
                .insert(VictoryText::new(pos));

            pos += 1;
            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        "Your town is saved, and you can return to your normal life.",
                        TextStyle {
                            font: font_handle.clone(),
                            font_size: 10.0,
                            color: Color::WHITE,
                        },
                        alignment,
                    ),
                    visible: visible.clone(),
                    ..Default::default()
                })
                .insert(VictoryText::with_offset(pos, 15.));

            pos += 1;
            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        "Press 1 to play again.",
                        TextStyle {
                            font: font_handle.clone(),
                            font_size: 10.0,
                            color: Color::GREEN,
                        },
                        alignment,
                    ),
                    visible: visible.clone(),
                    ..Default::default()
                })
                .insert(VictoryText::new(pos));
        });

    // Gameover

    let mut pos = 0;
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Your quest has ended.",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 15.0,
                    color: Color::RED,
                },
                alignment,
            ),
            visible: visible.clone(),
            ..Default::default()
        })
        .insert(GameoverText::with_offset(pos, 15.))
        .with_children(|parent| {
            pos += 1;
            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        "Slain by a monster, your hero's journey has come to a premature end.",
                        TextStyle {
                            font: font_handle.clone(),
                            font_size: 10.0,
                            color: Color::WHITE,
                        },
                        alignment,
                    ),
                    visible: visible.clone(),
                    ..Default::default()
                })
                .insert(GameoverText::new(pos));

            pos += 1;
            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        "The Amulet of Yala remains unclaimed, and your home town is not saved.",
                        TextStyle {
                            font: font_handle.clone(),
                            font_size: 10.0,
                            color: Color::WHITE,
                        },
                        alignment,
                    ),
                    visible: visible.clone(),
                    ..Default::default()
                })
                .insert(GameoverText::with_offset(pos, 15.));

            pos += 1;
            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        "Don't worry, you can always try again with a new hero.",
                        TextStyle {
                            font: font_handle.clone(),
                            font_size: 10.0,
                            color: Color::YELLOW,
                        },
                        alignment,
                    ),
                    visible: visible.clone(),
                    ..Default::default()
                })
                .insert(GameoverText::new(pos));

            pos += 1;
            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        "Press 1 to play again.",
                        TextStyle {
                            font: font_handle.clone(),
                            font_size: 10.0,
                            color: Color::GREEN,
                        },
                        alignment,
                    ),
                    visible: visible.clone(),
                    ..Default::default()
                })
                .insert(GameoverText::new(pos));
        });
}
