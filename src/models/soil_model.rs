use bevy::prelude::*;
use std::path::PathBuf;

/// `Some(handle)` when assets/textures/soil.glb exists, otherwise `None`
/// (the procedural dirt textures are used then).
#[derive(Resource, Default)]
pub struct SoilModel(pub Option<Handle<Scene>>);

/// Marker on every spawned soil (dirt) instance.
#[derive(Component)]
pub struct SoilInstance;

pub fn load_soil_model(mut commands: Commands, asset_server: Res<AssetServer>) {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let path: PathBuf = [root.as_str(), "assets", "textures", "soil.glb"]
        .iter()
        .collect();

    let handle = path
        .exists()
        .then(|| asset_server.load("textures/soil.glb#Scene0"));

    if handle.is_none() {
        info!("soil.glb not found, using procedural dirt textures");
    }
    commands.insert_resource(SoilModel(handle));
}
