use crate::prelude::*;

mod collisions;
mod player_input;

use collisions::*;
use player_input::*;

pub fn add_systems(app: &mut AppBuilder) -> &mut AppBuilder {
    app.add_system(player_input.system());
    app.add_system(collisions.system());
    app
}
