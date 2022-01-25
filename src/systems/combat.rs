use crate::prelude::*;

pub fn combat(
    mut commands: Commands,
    mut ev_attacks: ResMut<Events<WantsToAttack>>,
    mut tilemap_query: Query<&mut Tilemap>,
    mut victim_query: Query<(&mut Health, &Point, &Render)>,
    attacker_query: Query<&Damage, Or<(With<Player>, With<Enemy>)>>,
    weapon_query: Query<(&Damage, &Carried), With<Weapon>>,
    player_query: Query<Entity, With<Player>>,
) {
    let mut tilemap = tilemap_query.single_mut().unwrap();
    let player = player_query.single().unwrap();

    for WantsToAttack { victim, attacker } in ev_attacks.drain() {
        let base_damage = attacker_query
            .get(attacker)
            .map(|&Damage(d)| d)
            .unwrap_or(0);

        let weapon_damage = weapon_query
            .iter()
            .filter(|(_, &Carried(entity))| entity == attacker)
            .map(|(&Damage(d), _)| d)
            .sum::<i32>();

        let final_damage = base_damage + weapon_damage;

        let (mut health, pos, render) = victim_query.get_mut(victim).unwrap();

        health.current -= final_damage;
        if health.current < 1 && victim != player {
            commands.entity(victim).despawn();

            tilemap
                .clear_tile(
                    (pos.x - CAMERA_OFFSET_X, pos.y - CAMERA_OFFSET_Y),
                    render.sprite_order,
                )
                .unwrap();
        }
    }
}
