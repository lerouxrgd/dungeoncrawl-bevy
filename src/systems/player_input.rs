use crate::prelude::*;

pub fn player_input(
    mut key_evr: EventReader<KeyboardInput>,
    mut turn_state: ResMut<State<TurnState>>,
    mut ev_movements: EventWriter<WantsToMove>,
    mut ev_attacks: EventWriter<WantsToAttack>,
    mut ev_item: EventWriter<ActivateItem>,
    mut commands: Commands,
    player_query: Query<(Entity, &Point), With<Player>>,
    mut tilemap_query: Query<&mut Tilemap>,
    items_query: Query<(Entity, &Point, &Render, Option<&Weapon>), With<Item>>,
    carried_items_query: Query<(Entity, &Carried), With<Item>>,
    carried_weapon_query: Query<(Entity, &Carried), With<Weapon>>,
    enemies_query: Query<(Entity, &Point), With<Enemy>>,
    font_handle: Res<Handle<Font>>,
) {
    for ev in key_evr.iter().take(1) {
        let (player, &player_pos) = player_query.single().unwrap();

        let delta = match (ev.state, ev.key_code) {
            // movements
            (ElementState::Pressed, Some(KeyCode::Up)) => Point::new(0, 1),
            (ElementState::Pressed, Some(KeyCode::Left)) => Point::new(-1, 0),
            (ElementState::Pressed, Some(KeyCode::Down)) => Point::new(0, -1),
            (ElementState::Pressed, Some(KeyCode::Right)) => Point::new(1, 0),

            // pick up item
            (ElementState::Pressed, Some(KeyCode::G)) => {
                items_query
                    .iter()
                    .filter(|(_, &item_pos, _, _)| item_pos == player_pos)
                    .for_each(|(item, item_pos, item_render, weapon)| {
                        commands.entity(item).remove::<Point>();

                        let mut tilemap = tilemap_query.single_mut().unwrap();
                        tilemap
                            .clear_tile(
                                (item_pos.x - CAMERA_OFFSET_X, item_pos.y - CAMERA_OFFSET_Y),
                                item_render.sprite_order,
                            )
                            .unwrap();

                        commands
                            .entity(item)
                            .insert(Carried(player))
                            .insert_bundle(Text2dBundle {
                                text: Text::with_section(
                                    String::default(),
                                    TextStyle {
                                        font: font_handle.clone(),
                                        font_size: 10.0,
                                        color: Color::WHITE,
                                    },
                                    TextAlignment {
                                        vertical: VerticalAlign::Bottom,
                                        horizontal: HorizontalAlign::Right,
                                    },
                                ),
                                ..Default::default()
                            });

                        if weapon.is_some() {
                            carried_weapon_query
                                .iter()
                                .filter(|(_, &Carried(entity))| entity == player)
                                .for_each(|(previous_weapon, _)| {
                                    commands.entity(previous_weapon).despawn();
                                });
                        }
                    });

                Point::zero()
            }

            // use item
            (ElementState::Pressed, Some(KeyCode::Key1)) => {
                use_item(0, player, &carried_items_query, &mut ev_item)
            }
            (ElementState::Pressed, Some(KeyCode::Key2)) => {
                use_item(1, player, &carried_items_query, &mut ev_item)
            }
            (ElementState::Pressed, Some(KeyCode::Key3)) => {
                use_item(2, player, &carried_items_query, &mut ev_item)
            }
            (ElementState::Pressed, Some(KeyCode::Key4)) => {
                use_item(3, player, &carried_items_query, &mut ev_item)
            }
            (ElementState::Pressed, Some(KeyCode::Key5)) => {
                use_item(4, player, &carried_items_query, &mut ev_item)
            }
            (ElementState::Pressed, Some(KeyCode::Key6)) => {
                use_item(5, player, &carried_items_query, &mut ev_item)
            }
            (ElementState::Pressed, Some(KeyCode::Key7)) => {
                use_item(6, player, &carried_items_query, &mut ev_item)
            }
            (ElementState::Pressed, Some(KeyCode::Key8)) => {
                use_item(7, player, &carried_items_query, &mut ev_item)
            }
            (ElementState::Pressed, Some(KeyCode::Key9)) => {
                use_item(8, player, &carried_items_query, &mut ev_item)
            }

            // ignore other keys
            (ElementState::Pressed, Some(_)) => Point::zero(),
            // no key were pressed
            _ => return,
        };

        // Move or attack
        if delta != Point::zero() {
            let destination = player_pos + delta;

            let mut hit_something = false;
            enemies_query
                .iter()
                .filter(|(_, &pos)| pos == destination)
                .for_each(|(enemy, _)| {
                    hit_something = true;

                    ev_attacks.send(WantsToAttack {
                        attacker: player,
                        victim: enemy,
                    });
                });

            if !hit_something {
                ev_movements.send(WantsToMove {
                    entity: player,
                    destination,
                });
            }
        }

        turn_state.set(TurnState::PlayerTurn).unwrap();
    }
}

fn use_item(
    n: usize,
    player: Entity,
    carried_items_query: &Query<(Entity, &Carried), With<Item>>,
    ev_item: &mut EventWriter<ActivateItem>,
) -> Point {
    let item = carried_items_query
        .iter()
        .filter(|(_, carried)| carried.0 == player)
        .enumerate()
        .filter(|&(item_idx, (_, _))| item_idx == n)
        .find_map(|(_, (item, _))| Some(item));

    if let Some(item) = item {
        ev_item.send(ActivateItem {
            used_by: player,
            item,
        });
    }

    Point::zero()
}
