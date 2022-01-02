use crate::prelude::*;

pub fn player_input(
    mut key_evr: EventReader<KeyboardInput>,
    mut turn_state: ResMut<State<TurnState>>,
    mut ev_movements: EventWriter<WantsToMove>,
    mut ev_attacks: EventWriter<WantsToAttack>,
    mut commands: Commands,
    mut player_query: Query<(Entity, &Point, &mut Health), With<Player>>,
    mut tilemap_query: Query<&mut Tilemap>,
    items_query: Query<(Entity, &Point, &Render), With<Item>>,
    enemies_query: Query<(Entity, &Point), With<Enemy>>,
    font_handle: Res<Handle<Font>>,
) {
    for ev in key_evr.iter().take(1) {
        let delta = match (ev.state, ev.key_code) {
            // movements
            (ElementState::Pressed, Some(KeyCode::Up)) => Point::new(0, 1),
            (ElementState::Pressed, Some(KeyCode::Left)) => Point::new(-1, 0),
            (ElementState::Pressed, Some(KeyCode::Down)) => Point::new(0, -1),
            (ElementState::Pressed, Some(KeyCode::Right)) => Point::new(1, 0),

            // pick up item
            (ElementState::Pressed, Some(KeyCode::G)) => {
                let (player, &player_pos, _) = player_query.single_mut().unwrap();

                items_query
                    .iter()
                    .filter(|(_, &item_pos, _)| item_pos == player_pos)
                    .for_each(|(item, item_pos, item_render)| {
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
                    });

                Point::zero()
            }

            // ignore other keys
            (ElementState::Pressed, Some(_)) => Point::zero(),
            // no key were pressed
            _ => return,
        };

        let (player, &origin, mut health) = player_query.single_mut().unwrap();

        // No movement => heal
        if delta == Point::zero() {
            health.current = i32::min(health.max, health.current + 1);
        } else
        // Move or attack
        {
            let destination = origin + delta;

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
