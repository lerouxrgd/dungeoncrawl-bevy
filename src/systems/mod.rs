use crate::prelude::*;

mod player_input;

use player_input::*;

pub fn add_systems(app: &mut AppBuilder) -> &mut AppBuilder {
    app.add_system(player_input.system());
    app
}
