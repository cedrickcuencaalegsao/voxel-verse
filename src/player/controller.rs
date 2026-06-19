use super::{
    camera::{
        PlayerBody, PlayerCamera, cycle_camera_view, grab_cursor, player_look, toggle_cursor_grab,
    },
    movement::{Grounded, MovementSettings, Velocity, player_movement},
};
use crate::world::world_manager::{Player, WorldManager};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementSettings::default())
            .add_systems(Startup, (spawn_player, grab_cursor))
            .add_systems(
                Update,
                (
                    player_movement,
                    toggle_cursor_grab,
                    cycle_camera_view,
                    player_look,
                )
                    .chain(),
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world: Res<WorldManager>,
) {
    // ── spawn position ────────────────────────────────────────────────────────
    //
    // Use `spawn_height_at()` — NOT `height_at()`.
    //
    // `height_at()` returns the biome-blended fBm surface, which can put the
    // player deep inside a mountain or high in the air because the spawn area
    // uses a separate, simpler noise formula to keep it stable.
    //
    // `spawn_height_at()` mirrors the exact same single-octave noise used by
    // `spawn_block_at()`, so the Y value always matches the actual ground block.
    //
    // The +1.0 offset places the player's feet on top of that ground block
    // (block occupies [ground_y, ground_y+1], player stands at ground_y+1).
    let ground_y = world.generator.spawn_height_at(0.0, 0.0) as f32;
    let spawn_position = Vec3::new(0.0, ground_y + 1.0, 0.0);
    let camera_target = spawn_position + Vec3::Y * 1.1;

    commands
        .spawn((
            Player,
            Transform::from_translation(spawn_position),
            Visibility::default(),
            Velocity(Vec3::ZERO),
            Grounded(true),
        ))
        .with_children(|parent| {
            parent.spawn((
                PlayerBody,
                Mesh3d(meshes.add(Cuboid::new(0.8, 1.8, 0.8))),
                MeshMaterial3d(materials.add(Color::srgb(0.25, 0.45, 0.95))),
                Transform::from_xyz(0.0, 0.9, 0.0),
            ));
        });

    commands.spawn((
        PlayerCamera::default(),
        Camera3d::default(),
        Transform::from_translation(camera_target + Vec3::new(0.0, 2.0, 6.0))
            .looking_at(camera_target, Vec3::Y),
    ));
}
