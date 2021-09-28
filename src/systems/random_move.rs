use crate::prelude::*;

pub fn random_move(
    map_spec: Res<MapSpec>,
    mut movers_query: Query<(&mut Point, &Render), With<MovingRandomly>>,
    mut tilemap_query: Query<&mut Tilemap>,
) {
    let mut tilemap = tilemap_query.single_mut().unwrap();

    movers_query.iter_mut().for_each(|(mut pos, render)| {
        let mut rng = rand::thread_rng();

        let prev_pos = *pos;
        let destination = match rng.gen_range(0..4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        if map_spec.can_enter_tile(destination) {
            *pos = destination;
            move_sprite(&mut tilemap, prev_pos, destination, render);
        }
    });
}
