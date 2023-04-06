mod grid;

use bevy::prelude::*;

use crate::universe::types::Universe;
use grid::GridPlugin;

pub fn run(universe: Universe) {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GridPlugin { universe })
        .run();
}
