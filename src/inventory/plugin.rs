use super::hotbar::hotbar_input;
use super::inventory::Inventory;
use bevy::prelude::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Inventory::default())
            .add_systems(Update, hotbar_input);
    }
}
