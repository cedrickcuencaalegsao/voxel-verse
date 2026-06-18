use super::startup;
use crate::environment::plugin::EnvironmentPlugin;
use crate::inventory::plugin::InventoryPlugin;
use crate::networking::placeholder::NetworkingPlugin;
use crate::player::controller::PlayerPlugin;
use crate::rendering::plugin::RenderingPlugin;
use crate::ui::plugin::UiPlugin;
use crate::world::plugin::WorldPlugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub struct VoxelVersePlugins;

impl PluginGroup for VoxelVersePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(WorldPlugin)
            .add(RenderingPlugin)
            .add(PlayerPlugin)
            .add(InventoryPlugin)
            .add(UiPlugin)
            .add(EnvironmentPlugin)
            .add(NetworkingPlugin)
            .add(startup::StartupPlugin)
    }
}
