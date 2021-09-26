use crate::prelude::*;

pub fn player_input(
    map_spec: Res<MapSpec>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut map_query: Query<(&mut Tilemap, &mut Timer)>,
    mut player_query: Query<(&Player, &mut Point, &Render)>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
) {
    let (mut tile_map, mut timer) = map_query.single_mut().unwrap();

    timer.tick(time.delta());
    if !timer.finished() {
        return;
    }

    for key in keyboard_input.get_pressed() {
        let delta = match key {
            KeyCode::Up => Point::new(0, 1),
            KeyCode::Left => Point::new(-1, 0),
            KeyCode::Down => Point::new(0, -1),
            KeyCode::Right => Point::new(1, 0),
            _ => continue,
        };

        let (_, mut pos, render) = player_query.single_mut().unwrap();
        let (_, mut camera_transform) = camera_query.single_mut().unwrap();

        let prev_pos = *pos;
        let dest_pos = *pos + delta;
        if map_spec.can_enter_tile(dest_pos) {
            *pos = dest_pos;

            let camera_translation = &mut camera_transform.translation;
            camera_translation.x += delta.x as f32 * 32.;
            camera_translation.y += delta.y as f32 * 32.;

            move_sprite(&mut tile_map, prev_pos, *pos, render);
        }
    }
}

fn move_sprite(tile_map: &mut Tilemap, prev_pos: Point, new_pos: Point, render: &Render) {
    // We need to first remove where we were prior.
    tile_map
        .clear_tile(
            (prev_pos.x - CAMERA_OFFSET_X, prev_pos.y - CAMERA_OFFSET_Y),
            render.sprite_order,
        )
        .unwrap();

    // We then need to update where we are going!
    let tile = Tile {
        point: (new_pos.x - CAMERA_OFFSET_X, new_pos.y - CAMERA_OFFSET_Y),
        sprite_index: render.sprite_index,
        sprite_order: render.sprite_order,
        ..Default::default()
    };

    tile_map.insert_tile(tile).unwrap();
}
