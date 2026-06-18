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
    let spawn_position = Vec3::new(0.0, world.generator.height_at(0.0, 0.0) as f32 + 3.0, 0.0);
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
