use super::{
    camera::{
        PlayerBody, PlayerCamera, cycle_camera_view, grab_cursor, handle_cursor_auto_grab,
        player_look, toggle_cursor_grab,
    },
    movement::{
        AirborneTimer, Crouching, Grounded, Head, LeftFoot, LeftForeArm, LeftHand, LeftShin,
        LeftThigh, LeftUpperArm, MovementSettings, RightFoot, RightForeArm, RightHand, RightShin,
        RightThigh, RightUpperArm, Torso, Velocity, animate_limbs, player_movement,
    },
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
                    animate_limbs,
                    toggle_cursor_grab,
                    handle_cursor_auto_grab,
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
    let ground_y = world.generator.spawn_height_at(0.0, 0.0) as f32;
    let spawn_position = Vec3::new(0.0, ground_y + 1.0, 0.0);
    let camera_target = spawn_position + Vec3::Y * 1.1;

    let skin = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.76, 0.55),
        perceptual_roughness: 0.8,
        ..default()
    });
    let shirt = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.45, 0.95),
        perceptual_roughness: 0.8,
        ..default()
    });
    let pants = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.25, 0.55),
        perceptual_roughness: 0.8,
        ..default()
    });
    let hair = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.15, 0.05),
        perceptual_roughness: 0.9,
        ..default()
    });
    let shoe = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.08, 0.04),
        perceptual_roughness: 0.8,
        ..default()
    });

    let head_mesh = meshes.add(Cuboid::new(0.38, 0.38, 0.38));
    let hair_mesh = meshes.add(Cuboid::new(0.40, 0.12, 0.40));
    let torso_mesh = meshes.add(Cuboid::new(0.5, 0.75, 0.25));

    let upper_arm_mesh = meshes.add(Cuboid::new(0.22, 0.38, 0.22));
    let forearm_mesh = meshes.add(Cuboid::new(0.20, 0.35, 0.20));
    let hand_mesh = meshes.add(Cuboid::new(0.20, 0.18, 0.20));

    let thigh_mesh = meshes.add(Cuboid::new(0.24, 0.38, 0.24));
    let shin_mesh = meshes.add(Cuboid::new(0.22, 0.36, 0.22));
    let foot_mesh = meshes.add(Cuboid::new(0.24, 0.12, 0.34));

    commands
        .spawn((
            Player,
            Transform::from_translation(spawn_position),
            Visibility::default(),
            Velocity(Vec3::ZERO),
            Grounded(true),
            AirborneTimer(0.0),
            Crouching(false), // Added the Crouching state component
        ))
        .with_children(|p| {
            p.spawn((PlayerBody, Transform::IDENTITY, Visibility::default()))
                .with_children(|body| {
                    // ── TORSO & LOWER BODY SKELETON ───────────────────────────
                    body.spawn((
                        Torso,
                        Transform::from_xyz(0.0, 0.75, 0.0),
                        Visibility::default(),
                    ))
                    .with_children(|torso| {
                        torso.spawn((
                            Mesh3d(torso_mesh.clone()),
                            MeshMaterial3d(shirt.clone()),
                            Transform::from_xyz(0.0, 0.375, 0.0),
                        ));

                        // ── HEAD JOINT ──
                        torso
                            .spawn((
                                Head,
                                Transform::from_xyz(0.0, 0.75, 0.0),
                                Visibility::default(),
                            ))
                            .with_children(|head_joint| {
                                head_joint.spawn((
                                    Mesh3d(head_mesh.clone()),
                                    MeshMaterial3d(skin.clone()),
                                    Transform::from_xyz(0.0, 0.19, 0.0),
                                ));
                                head_joint.spawn((
                                    Mesh3d(hair_mesh.clone()),
                                    MeshMaterial3d(hair.clone()),
                                    Transform::from_xyz(0.0, 0.44, 0.0),
                                ));
                                head_joint.spawn((
                                    Mesh3d(meshes.add(Cuboid::new(0.42, 0.40, 0.06))),
                                    MeshMaterial3d(hair.clone()),
                                    Transform::from_xyz(0.0, 0.19, 0.22),
                                ));
                                head_joint.spawn((
                                    Mesh3d(meshes.add(Cuboid::new(0.06, 0.40, 0.25))),
                                    MeshMaterial3d(hair.clone()),
                                    Transform::from_xyz(0.22, 0.19, 0.075),
                                ));
                                head_joint.spawn((
                                    Mesh3d(meshes.add(Cuboid::new(0.06, 0.40, 0.25))),
                                    MeshMaterial3d(hair.clone()),
                                    Transform::from_xyz(-0.22, 0.19, 0.075),
                                ));
                            });

                        // ── LEFT ARM ──
                        torso
                            .spawn((
                                LeftUpperArm,
                                Transform::from_xyz(0.36, 0.75, 0.0),
                                Visibility::default(),
                            ))
                            .with_children(|u_arm| {
                                u_arm.spawn((
                                    Mesh3d(upper_arm_mesh.clone()),
                                    MeshMaterial3d(shirt.clone()),
                                    Transform::from_xyz(0.0, -0.19, 0.0),
                                ));

                                u_arm
                                    .spawn((
                                        LeftForeArm,
                                        Transform::from_xyz(0.0, -0.38, 0.0),
                                        Visibility::default(),
                                    ))
                                    .with_children(|f_arm| {
                                        f_arm.spawn((
                                            Mesh3d(forearm_mesh.clone()),
                                            MeshMaterial3d(skin.clone()),
                                            Transform::from_xyz(0.0, -0.175, 0.0),
                                        ));

                                        f_arm
                                            .spawn((
                                                LeftHand,
                                                Transform::from_xyz(0.0, -0.35, 0.0),
                                                Visibility::default(),
                                            ))
                                            .with_children(|hand| {
                                                hand.spawn((
                                                    Mesh3d(hand_mesh.clone()),
                                                    MeshMaterial3d(skin.clone()),
                                                    Transform::from_xyz(0.0, -0.09, 0.0),
                                                ));
                                            });
                                    });
                            });

                        // ── RIGHT ARM ──
                        torso
                            .spawn((
                                RightUpperArm,
                                Transform::from_xyz(-0.36, 0.75, 0.0),
                                Visibility::default(),
                            ))
                            .with_children(|u_arm| {
                                u_arm.spawn((
                                    Mesh3d(upper_arm_mesh.clone()),
                                    MeshMaterial3d(shirt.clone()),
                                    Transform::from_xyz(0.0, -0.19, 0.0),
                                ));

                                u_arm
                                    .spawn((
                                        RightForeArm,
                                        Transform::from_xyz(0.0, -0.38, 0.0),
                                        Visibility::default(),
                                    ))
                                    .with_children(|f_arm| {
                                        f_arm.spawn((
                                            Mesh3d(forearm_mesh.clone()),
                                            MeshMaterial3d(skin.clone()),
                                            Transform::from_xyz(0.0, -0.175, 0.0),
                                        ));

                                        f_arm
                                            .spawn((
                                                RightHand,
                                                Transform::from_xyz(0.0, -0.35, 0.0),
                                                Visibility::default(),
                                            ))
                                            .with_children(|hand| {
                                                hand.spawn((
                                                    Mesh3d(hand_mesh.clone()),
                                                    MeshMaterial3d(skin.clone()),
                                                    Transform::from_xyz(0.0, -0.09, 0.0),
                                                ));
                                            });
                                    });
                            });

                        // ── LEFT LEG ──
                        torso
                            .spawn((
                                LeftThigh,
                                Transform::from_xyz(0.13, -0.06, 0.0),
                                Visibility::default(),
                            ))
                            .with_children(|thigh| {
                                thigh.spawn((
                                    Mesh3d(thigh_mesh.clone()),
                                    MeshMaterial3d(pants.clone()),
                                    Transform::from_xyz(0.0, -0.19, 0.0),
                                ));

                                thigh
                                    .spawn((
                                        LeftShin,
                                        Transform::from_xyz(0.0, -0.38, 0.0),
                                        Visibility::default(),
                                    ))
                                    .with_children(|shin| {
                                        shin.spawn((
                                            Mesh3d(shin_mesh.clone()),
                                            MeshMaterial3d(pants.clone()),
                                            Transform::from_xyz(0.0, -0.18, 0.0),
                                        ));

                                        shin.spawn((
                                            LeftFoot,
                                            Transform::from_xyz(0.0, -0.36, 0.0),
                                            Visibility::default(),
                                        ))
                                        .with_children(
                                            |foot| {
                                                foot.spawn((
                                                    Mesh3d(foot_mesh.clone()),
                                                    MeshMaterial3d(shoe.clone()),
                                                    Transform::from_xyz(0.0, -0.06, 0.05),
                                                ));
                                            },
                                        );
                                    });
                            });

                        // ── RIGHT LEG ──
                        torso
                            .spawn((
                                RightThigh,
                                Transform::from_xyz(-0.13, -0.06, 0.0),
                                Visibility::default(),
                            ))
                            .with_children(|thigh| {
                                thigh.spawn((
                                    Mesh3d(thigh_mesh.clone()),
                                    MeshMaterial3d(pants.clone()),
                                    Transform::from_xyz(0.0, -0.19, 0.0),
                                ));

                                thigh
                                    .spawn((
                                        RightShin,
                                        Transform::from_xyz(0.0, -0.38, 0.0),
                                        Visibility::default(),
                                    ))
                                    .with_children(|shin| {
                                        shin.spawn((
                                            Mesh3d(shin_mesh.clone()),
                                            MeshMaterial3d(pants.clone()),
                                            Transform::from_xyz(0.0, -0.18, 0.0),
                                        ));

                                        shin.spawn((
                                            RightFoot,
                                            Transform::from_xyz(0.0, -0.36, 0.0),
                                            Visibility::default(),
                                        ))
                                        .with_children(
                                            |foot| {
                                                foot.spawn((
                                                    Mesh3d(foot_mesh.clone()),
                                                    MeshMaterial3d(shoe.clone()),
                                                    Transform::from_xyz(0.0, -0.06, 0.05),
                                                ));
                                            },
                                        );
                                    });
                            });
                    });
                });
        });

    commands.spawn((
        PlayerCamera::default(),
        Camera3d::default(),
        Transform::from_translation(camera_target + Vec3::new(0.0, 2.0, 6.0))
            .looking_at(camera_target, Vec3::Y),
    ));
}
