use crate::ui::hud::{
    setup_hud, toggle_minimap, update_crosshair, update_fps, update_hotbar, update_minimap,
};
use crate::ui::inventory_ui::InventoryUiPlugin;
use crate::ui::menus::MenuPlugin;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud)
            .add_systems(
                Update,
                (
                    update_hotbar,
                    update_crosshair,
                    update_fps,
                    toggle_minimap,
                    update_minimap,
                ),
            )
            .add_plugins((InventoryUiPlugin, MenuPlugin));
    }
}
