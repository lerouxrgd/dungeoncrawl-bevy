use crate::prelude::*;

pub fn fov(map_spec: Res<MapSpec>, mut fov_query: Query<(&Point, &mut FieldOfView)>) {
    fov_query
        .iter_mut()
        .filter(|(_, fov)| fov.is_dirty)
        .for_each(|(&pos, mut fov)| {
            fov.visible_tiles = field_of_view_set(pos, fov.radius, &*map_spec);
            fov.is_dirty = false;
        });
}
