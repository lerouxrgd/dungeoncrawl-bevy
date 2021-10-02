use crate::prelude::*;

pub fn random_move(
    mut ev_movements: EventWriter<WantsToMove>,
    mut movers_query: Query<(Entity, &Point, &Render), With<MovingRandomly>>,
) {
    movers_query.iter_mut().for_each(|(entity, pos, &render)| {
        let mut rng = rand::thread_rng();

        let origin = *pos;
        let destination = match rng.gen_range(0..4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        ev_movements.send(WantsToMove {
            entity,
            origin,
            destination,
            render,
        });
    });
}
