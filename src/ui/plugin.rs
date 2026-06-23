use crate::ui::hud::{setup_hud, update_crosshair, update_hotbar};
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud)
            .add_systems(Update, (update_hotbar, update_crosshair));
    }
}
