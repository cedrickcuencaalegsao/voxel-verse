use crate::lobby::AppState;
use crate::ui::hud::{
    setup_hud, toggle_minimap, update_crosshair, update_fps, update_hotbar, update_minimap,
};
use crate::ui::inventory_ui::InventoryUiPlugin;
use crate::ui::menus::MenuPlugin;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup_hud)
            .add_systems(
                Update,
                (
                    update_hotbar,
                    update_crosshair,
                    update_fps,
                    toggle_minimap,
                    update_minimap,
                )
                    .run_if(in_state(AppState::Playing)),
            )
            .add_plugins((InventoryUiPlugin, MenuPlugin));
    }
}