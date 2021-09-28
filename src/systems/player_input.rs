use crate::prelude::*;

pub fn player_input(
    map_spec: Res<MapSpec>,
    mut key_evr: EventReader<KeyboardInput>,
    mut turn_state: ResMut<State<TurnState>>,
    mut tilemap_query: Query<&mut Tilemap>,
    mut player_query: Query<(&Player, &mut Point, &Render)>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
) {
    for ev in key_evr.iter().take(1) {
        let delta = match (ev.state, ev.key_code) {
            (ElementState::Pressed, Some(KeyCode::Up)) => Point::new(0, 1),
            (ElementState::Pressed, Some(KeyCode::Left)) => Point::new(-1, 0),
            (ElementState::Pressed, Some(KeyCode::Down)) => Point::new(0, -1),
            (ElementState::Pressed, Some(KeyCode::Right)) => Point::new(1, 0),
            _ => return,
        };

        let (_, mut pos, render) = player_query.single_mut().unwrap();

        let prev_pos = *pos;
        let dest_pos = *pos + delta;
        if map_spec.can_enter_tile(dest_pos) {
            *pos = dest_pos;

            let (_, mut camera_transform) = camera_query.single_mut().unwrap();
            let camera_translation = &mut camera_transform.translation;
            camera_translation.x += delta.x as f32 * 32.;
            camera_translation.y += delta.y as f32 * 32.;

            let mut tilemap = tilemap_query.single_mut().unwrap();
            move_sprite(&mut tilemap, prev_pos, *pos, render);

            turn_state.set(TurnState::PlayerTurn).unwrap();
        }
    }
}
