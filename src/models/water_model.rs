use bevy::prelude::*;
use std::path::PathBuf;

/// `Some(handle)` when assets/textures/water.glb exists, otherwise `None`
/// (the translucent procedural water mesh is used then).
#[derive(Resource, Default)]
pub struct WaterModel(pub Option<Handle<Scene>>);

/// Marker on every spawned water instance.
#[derive(Component)]
pub struct WaterInstance;

pub fn load_water_model(mut commands: Commands, asset_server: Res<AssetServer>) {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let path: PathBuf = [root.as_str(), "assets", "textures", "water.glb"]
        .iter()
        .collect();

    let handle = path
        .exists()
        .then(|| asset_server.load("textures/water.glb#Scene0"));

    if handle.is_none() {
        info!("water.glb not found, using procedural water");
    }
    commands.insert_resource(WaterModel(handle));
}
