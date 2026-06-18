mod plugins;
mod startup;

use bevy::prelude::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(plugins::VoxelVersePlugins)
        .run();
}
