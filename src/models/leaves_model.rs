use bevy::prelude::*;
use std::path::PathBuf;

/// `Some(handle)` when assets/textures/leaves.glb exists, otherwise `None`
/// (the procedural leaves textures are used then).
#[derive(Resource, Default)]
pub struct LeavesModel(pub Option<Handle<Scene>>);

/// Marker on every spawned leaves instance.
#[derive(Component)]
pub struct LeavesInstance;

pub fn load_leaves_model(mut commands: Commands, asset_server: Res<AssetServer>) {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let path: PathBuf = [root.as_str(), "assets", "textures", "leaves.glb"]
        .iter()
        .collect();

    let handle = path
        .exists()
        .then(|| asset_server.load("textures/leaves.glb#Scene0"));

    if handle.is_none() {
        info!("leaves.glb not found, using procedural leaves textures");
    }
    commands.insert_resource(LeavesModel(handle));
}
