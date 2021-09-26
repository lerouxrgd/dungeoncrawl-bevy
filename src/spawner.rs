use crate::prelude::*;

pub fn spawn_player(commands: &mut Commands, position: Point) {
    commands.spawn().insert_bundle(PlayerBundle {
        player: Player,
        position,
        render: Render {
            sprite_index: to_cp437('@'),
            sprite_order: 1,
        },
    });
}
