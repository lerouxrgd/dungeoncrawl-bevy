use crate::prelude::*;

pub fn combat(
    mut commands: Commands,
    mut ev_attacks: ResMut<Events<WantsToAttack>>,
    mut tilemap_query: Query<&mut Tilemap>,
    mut victim_query: Query<(&mut Health, &Point, &Render)>,
) {
    let mut tilemap = tilemap_query.single_mut().unwrap();

    for WantsToAttack { victim, .. } in ev_attacks.drain() {
        let (mut health, pos, render) = victim_query.get_mut(victim).unwrap();

        println!("Health before attack: {}", health.current);

        health.current -= 1;
        if health.current < 1 {
            commands.entity(victim).despawn();

            tilemap
                .clear_tile(
                    (pos.x - CAMERA_OFFSET_X, pos.y - CAMERA_OFFSET_Y),
                    render.sprite_order,
                )
                .unwrap();
        }

        println!("Health after attack: {}", health.current);
    }
}