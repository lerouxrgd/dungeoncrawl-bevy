use std::collections::VecDeque;

use crate::prelude::*;

pub fn despawn_level(
    mut turn_state: ResMut<State<TurnState>>,
    mut commands: Commands,
    entities_query: Query<
        Entity,
        Or<(
            (With<Item>, Without<Carried>),
            With<Enemy>,
            With<Hud>,
            With<Tilemap>,
        )>,
    >,
) {
    for e in entities_query.iter() {
        commands.entity(e).despawn_recursive();
    }
    turn_state.set(TurnState::AwaitingInput).unwrap();
}

pub fn respawn_level(
    windows: Res<Windows>,
    font_handle: Res<Handle<Font>>,
    texture_atlas_handle: Res<Handle<TextureAtlas>>,
    mut commands: Commands,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut player_query: Query<(&mut Player, &mut Point, &mut FieldOfView)>,
    mut ev_window: EventWriter<WindowResized>,
) {
    // Build new map

    let (mut tilemap, map_builder) = make_tilemap(texture_atlas_handle.clone());
    let MapBuilder {
        player_start,
        amulet_start,
        monster_spawns,
        mut map_spec,
        theme,
        ..
    } = map_builder;

    // Reset player

    let (mut player, mut player_pos, mut player_fov) = player_query.single_mut().unwrap();

    player_fov.is_dirty = true;
    *player_pos = player_start;
    player.map_level += 1;

    tilemap
        .insert_tile(Tile {
            point: (
                player_pos.x - CAMERA_OFFSET_X,
                player_pos.y - CAMERA_OFFSET_Y,
            ),
            sprite_index: to_cp437('@'),
            sprite_order: 3,
            tint: Color::WHITE,
        })
        .unwrap();

    // Respawn entities

    if player.map_level == 2 {
        spawn_amulet_of_yala(
            &mut commands,
            amulet_start,
            &mut tilemap,
            &mut map_spec,
            theme.as_ref(),
        );
    }
    commands.insert_resource(map_spec);

    monster_spawns
        .into_iter()
        .for_each(|pos| spawn_entity(&mut commands, pos, &mut tilemap));

    spawn_hud(&mut commands, font_handle.clone());
    spawn_tilemap(&mut commands, tilemap);

    // Reset camera

    let mut camera_transform = camera_query.single_mut().unwrap();
    camera_transform.translation.x = (player_start.x as f32 - CAMERA_OFFSET_X as f32) * 32.;
    camera_transform.translation.y = (player_start.y as f32 - CAMERA_OFFSET_Y as f32) * 32.;

    // Hacky fix for https://github.com/joshuajbouw/bevy_tilemap/issues/152
    let window = windows.get_primary().unwrap();
    ev_window.send(WindowResized {
        id: window.id(),
        width: window.width(),
        height: window.height(),
    });
}

pub fn despawn_game_state(
    mut commands: Commands,
    entities_query: Query<
        Entity,
        Or<(
            With<Player>,
            With<Item>,
            With<Enemy>,
            With<Hud>,
            With<Tilemap>,
        )>,
    >,
) {
    for e in entities_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn respawn_game_state(
    windows: Res<Windows>,
    font_handle: Res<Handle<Font>>,
    texture_atlas_handle: Res<Handle<TextureAtlas>>,
    mut commands: Commands,
    mut texts_query: Query<&mut Visible, Or<(With<VictoryText>, With<GameoverText>)>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut ev_window: EventWriter<WindowResized>,
) {
    // Remove victory/gameover screen

    for mut text in texts_query.iter_mut() {
        text.is_visible = false;
    }

    // Build new map

    let (mut tilemap, map_builder) = make_tilemap(texture_atlas_handle.clone());
    let MapBuilder {
        player_start,
        monster_spawns,
        map_spec,
        ..
    } = map_builder;

    commands.insert_resource(map_spec);

    // Respawn entities

    spawn_player(&mut commands, player_start, &mut tilemap);

    monster_spawns
        .into_iter()
        .for_each(|pos| spawn_entity(&mut commands, pos, &mut tilemap));

    spawn_hud(&mut commands, font_handle.clone());
    spawn_tilemap(&mut commands, tilemap);

    // Reset camera

    let mut camera_transform = camera_query.single_mut().unwrap();
    camera_transform.translation.x = (player_start.x as f32 - CAMERA_OFFSET_X as f32) * 32.;
    camera_transform.translation.y = (player_start.y as f32 - CAMERA_OFFSET_Y as f32) * 32.;

    // Hacky fix for https://github.com/joshuajbouw/bevy_tilemap/issues/152
    let window = windows.get_primary().unwrap();
    ev_window.send(WindowResized {
        id: window.id(),
        width: window.width(),
        height: window.height(),
    });
}

pub fn text_screen<T>(
    mut turn_state: ResMut<State<TurnState>>,
    mut key_evr: EventReader<KeyboardInput>,
    mut q: QuerySet<(
        Query<(&Transform, &OrthographicProjection), With<Camera>>,
        Query<(&T, &mut Transform, &Text2dSize, &mut Visible)>,
    )>,
) where
    T: ScreenText,
{
    // Camera query

    let (camera_transform, proj) = q.q0().single().unwrap();
    let proj_bottom = proj.bottom;
    let camera_translation = camera_transform.translation;

    // Text query

    let mut texts = q.q1_mut().iter_mut().collect::<VecDeque<_>>();
    texts.make_contiguous().sort_by_key(|(text, ..)| text.pos());

    let mut offset = 0.;
    if let Some((text, mut transform, size, mut visible)) = texts.pop_front() {
        visible.is_visible = true;
        transform.translation.x = camera_translation.x;
        transform.translation.y = camera_translation.y - proj_bottom;
        transform.translation.z = 999.;
        offset += size.size.height;
        offset += text.offset();
    }
    for (text, mut transform, size, mut visible) in texts.into_iter() {
        visible.is_visible = true;
        transform.translation.y = -offset;
        offset += size.size.height;
        offset += text.offset();
    }

    // Handle user input

    for ev in key_evr.iter().take(1) {
        match (ev.state, ev.key_code) {
            (ElementState::Pressed, Some(KeyCode::Key1)) => {
                turn_state.set(TurnState::AwaitingInput).unwrap()
            }
            _ => (),
        }
    }
}
