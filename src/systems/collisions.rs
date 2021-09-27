use crate::prelude::*;

pub fn collisions(
    mut commands: Commands,
    mut tilemap_query: Query<&mut Tilemap>,
    player_query: Query<(&Player, &Point)>,
    enemies_query: Query<(Entity, &Point, &Render), With<Enemy>>,
) {
    let (_, &player_pos) = player_query.single().unwrap();

    let mut tilemap = tilemap_query.single_mut().unwrap();

    enemies_query
        .iter()
        .filter(|(_, &pos, _)| pos == player_pos)
        .for_each(|(id, pos, render)| {
            commands.entity(id).despawn();

            tilemap
                .clear_tile(
                    (pos.x - CAMERA_OFFSET_X, pos.y - CAMERA_OFFSET_Y),
                    render.sprite_order,
                )
                .unwrap();
        });
}
