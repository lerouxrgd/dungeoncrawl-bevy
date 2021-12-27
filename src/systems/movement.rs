use crate::prelude::*;

pub fn movement(
    map_spec: Res<MapSpec>,
    mut ev_movements: ResMut<Events<WantsToMove>>,
    mut commands: Commands,
    mut tilemap_query: Query<&mut Tilemap>,
    player_query: Query<Entity, With<Player>>,
    mut mover_query: Query<(&Point, &Render, &mut FieldOfView)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let mut tilemap = tilemap_query.single_mut().unwrap();

    for WantsToMove {
        entity,
        destination,
    } in ev_movements.drain()
    {
        if map_spec.can_enter_tile(destination) {
            let (&origin, render, mut fov) = mover_query.get_mut(entity).unwrap();

            commands.entity(entity).insert(destination);
            move_sprite(&mut tilemap, origin, destination, &render);

            *fov = fov.clone_dirty();

            let player = player_query.single().unwrap();
            if entity == player {
                let mut camera_transform = camera_query.single_mut().unwrap();
                let camera_translation = &mut camera_transform.translation;
                camera_translation.x = (destination.x - CAMERA_OFFSET_X) as f32 * 32.;
                camera_translation.y = (destination.y - CAMERA_OFFSET_Y) as f32 * 32.;
            }
        }
    }
}
