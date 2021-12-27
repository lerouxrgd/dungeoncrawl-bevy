use crate::prelude::*;

pub fn map_render(
    windows: Res<Windows>,
    map_spec: Res<MapSpec>,
    mut tilemap_query: Query<&mut Tilemap>,
    player_query: Query<&FieldOfView, With<Player>>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    let window = windows.get_primary().unwrap();

    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let p = -window_size / 2.0;
    let camera_transform = camera_query.single().unwrap();
    let world_pos = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

    let x_min = (world_pos.x / 32.).floor() as i32 + CAMERA_OFFSET_X;
    let x_max = x_min + (window_size.x / 32.).ceil() as i32;
    let y_min = (world_pos.y / 32.).floor() as i32 + CAMERA_OFFSET_Y;
    let y_max = y_min + (window_size.y / 32.).ceil() as i32;

    let mut tilemap = tilemap_query.single_mut().unwrap();
    let player_fov = player_query.single().unwrap();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let pt = Point { x, y };
            let tilemap_pos = (x - CAMERA_OFFSET_X, y - CAMERA_OFFSET_Y);

            if map_spec.in_bounds(pt) && player_fov.visible_tiles.contains(&pt) {
                for sprite_order in 0..=2 {
                    if let Some(tile) = tilemap.get_tile_mut(tilemap_pos, sprite_order) {
                        tile.color = Color::WHITE;
                    }
                }
            } else if map_spec.in_bounds(pt) {
                for sprite_order in 0..=2 {
                    if let Some(tile) = tilemap.get_tile_mut(tilemap_pos, sprite_order) {
                        tile.color = Color::BLACK;
                    }
                }
            }
        }
    }
}
