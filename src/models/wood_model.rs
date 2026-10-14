use bevy::prelude::*;
use std::path::PathBuf;

/// `Some(handle)` when assets/textures/wood.glb exists, otherwise `None`
/// (the procedural wood textures are used then).
#[derive(Resource, Default)]
pub struct WoodModel(pub Option<Handle<Scene>>);

/// Marker on every spawned wood instance.
#[derive(Component)]
pub struct WoodInstance;

pub fn load_wood_model(mut commands: Commands, asset_server: Res<AssetServer>) {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let path: PathBuf = [root.as_str(), "assets", "textures", "wood.glb"]
        .iter()
        .collect();

    let handle = path
        .exists()
        .then(|| asset_server.load("textures/wood.glb#Scene0"));

    if handle.is_none() {
        info!("wood.glb not found, using procedural wood textures");
    }
    commands.insert_resource(WoodModel(handle));
}
