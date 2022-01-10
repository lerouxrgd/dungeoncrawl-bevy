use crate::prelude::*;

pub fn use_items(
    mut ev_items: ResMut<Events<ActivateItem>>,
    mut commands: Commands,
    mut map_spec: ResMut<MapSpec>,
    items_query: Query<
        (
            Entity,
            Option<&ProvidesHealing>,
            Option<&ProvidesDungeonMap>,
        ),
        With<Item>,
    >,
    mut healed_query: Query<(Entity, &mut Health)>,
) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();
    ev_items
        .drain()
        .filter_map(|ActivateItem { used_by, item }| {
            items_query
                .get(item)
                .ok()
                .map(|item_components| (used_by, item_components))
        })
        .for_each(|(used_by, (item, healing, dungeon_map))| {
            if let Some(healing) = healing {
                healing_to_apply.push((used_by, healing.amount));
            }

            if let Some(_dungeon_map) = dungeon_map {
                map_spec.revealed_tiles.iter_mut().for_each(|t| *t = true);
            }

            commands.entity(item).despawn();
        });

    for (entity, heal) in healing_to_apply.into_iter() {
        if let Ok((_, mut health)) = healed_query.get_mut(entity) {
            health.current = i32::min(health.max, health.current + heal);
        }
    }
}
