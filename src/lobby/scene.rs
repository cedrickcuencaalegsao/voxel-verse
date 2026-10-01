//! The 3D diorama behind the lobby menu: a 9x9x9 voxel island (ground, rocks,
//! a tree) with a blocky character standing on top, waving.
//!
//! The scene runs its own small day/night cycle (see `lobby_cycle_system`).
//! It is fully separate from `environment::daynight`, so the two never touch
//! each other's lights.

use super::LobbyScene;
use bevy::prelude::*;

const N: usize = 9; // island is N x N x N blocks
const CHARACTER_Y: f32 = 5.0; // top surface of the grass layer

/// How fast the lobby day passes. 0.02 → one full day every ~50 seconds.
const LOBBY_DAY_SPEED: f32 = 0.02;
/// Where in the day the lobby starts (0 = midnight, 0.25 = sunrise, 0.5 = noon).
const LOBBY_START_TIME: f32 = 0.3;

// ── voxel data ───────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Grass,
    Dirt,
    Stone,
    Wood,
    Leaves,
}

const KINDS: [Kind; 5] = [Kind::Grass, Kind::Dirt, Kind::Stone, Kind::Wood, Kind::Leaves];

impl Kind {
    fn index(self) -> usize {
        self as usize
    }

    fn color(self) -> (f32, f32, f32) {
        match self {
            Kind::Grass => (0.36, 0.62, 0.25),
            Kind::Dirt => (0.47, 0.33, 0.22),
            Kind::Stone => (0.50, 0.50, 0.53),
            Kind::Wood => (0.42, 0.29, 0.16),
            Kind::Leaves => (0.22, 0.50, 0.20),
        }
    }
}

type Grid = [[[Option<Kind>; N]; N]; N]; // indexed [x][y][z]

/// Layers 0-1 stone, 2-3 dirt, 4 grass. Rocks and the tree sit on top (y 5-8),
/// so everything fits inside the 9x9x9 cube.
fn build_island() -> Grid {
    let mut g: Grid = [[[None; N]; N]; N];

    for x in 0..N {
        for z in 0..N {
            for y in 0..=4 {
                g[x][y][z] = Some(match y {
                    0 | 1 => Kind::Stone,
                    2 | 3 => Kind::Dirt,
                    _ => Kind::Grass,
                });
            }
        }
    }

    // Tree in the back-left corner: 3-block trunk, 5x5 canopy, 3x3 top.
    for y in 5..=7 {
        g[2][y][2] = Some(Kind::Wood);
    }
    for dx in 0..=4usize {
        for dz in 0..=4usize {
            let corner = (dx == 0 || dx == 4) && (dz == 0 || dz == 4);
            if corner || (dx == 2 && dz == 2) {
                continue;
            }
            g[dx][7][dz] = Some(Kind::Leaves);
        }
    }
    for dx in 1..=3usize {
        for dz in 1..=3usize {
            g[dx][8][dz] = Some(Kind::Leaves);
        }
    }

    // Rocks.
    for (x, y, z) in [(6, 5, 2), (7, 5, 2), (6, 5, 3), (6, 6, 2), (7, 5, 6), (1, 5, 6)] {
        g[x][y][z] = Some(Kind::Stone);
    }

    g
}

/// A block is only worth spawning if at least one face touches air.
fn exposed(g: &Grid, x: usize, y: usize, z: usize) -> bool {
    const DIRS: [(i32, i32, i32); 6] = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    DIRS.iter().any(|&(dx, dy, dz)| {
        let (nx, ny, nz) = (x as i32 + dx, y as i32 + dy, z as i32 + dz);
        let out = |v: i32| v < 0 || v >= N as i32;
        if out(nx) || out(ny) || out(nz) {
            true
        } else {
            g[nx as usize][ny as usize][nz as usize].is_none()
        }
    })
}

// ── animation ────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub(super) enum Anim {
    Sway,    // whole diorama rocks gently left/right
    Bob,     // character bobs up and down
    WaveArm, // the waving arm
    IdleArm, // the other arm
    Head,    // head looks around
}

#[derive(Component)]
pub(super) struct Animated(pub(super) Anim);

pub(super) fn animate_scene(time: Res<Time>, mut parts: Query<(&mut Transform, &Animated)>) {
    let t = time.elapsed_secs();
    for (mut tf, Animated(anim)) in &mut parts {
        match anim {
            Anim::Sway => tf.rotation = Quat::from_rotation_y((t * 0.5).sin() * 0.35),
            Anim::Bob => tf.translation.y = CHARACTER_Y + (t * 4.0).sin() * 0.02,
            Anim::WaveArm => tf.rotation = Quat::from_rotation_z(2.6 + (t * 8.0).sin() * 0.35),
            Anim::IdleArm => tf.rotation = Quat::from_rotation_x((t * 2.0).sin() * 0.06),
            Anim::Head => {
                tf.rotation = Quat::from_rotation_y((t * 1.5).sin() * 0.2)
                    * Quat::from_rotation_z((t * 1.5).cos() * 0.05)
            }
        }
    }
}

// ── day / night cycle ────────────────────────────────────────────────────────

/// Current time of day for the lobby: 0..1 (0 = midnight, 0.5 = noon).
/// Inserted fresh every time the lobby scene is spawned.
#[derive(Resource)]
pub(super) struct LobbyTime(pub(super) f32);

/// Which role a lobby light plays in the cycle.
#[derive(Component, Clone, Copy)]
pub(super) enum LobbyLight {
    Sun,  // key light, casts shadows, warm at sunrise/sunset
    Moon, // dim bluish light that takes over at night
    Fill, // soft light from the opposite side, fades with the sun
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let a = a.to_srgba();
    let b = b.to_srgba();
    let t = t.clamp(0.0, 1.0);
    Color::srgba(
        a.red + (b.red - a.red) * t,
        a.green + (b.green - a.green) * t,
        a.blue + (b.blue - a.blue) * t,
        a.alpha + (b.alpha - a.alpha) * t,
    )
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Advances the lobby clock and updates lights + background colour.
/// Register it next to `animate_scene` in the lobby plugin.
pub(super) fn lobby_cycle_system(
    time: Res<Time>,
    lobby_time: Option<ResMut<LobbyTime>>,
    mut lights: Query<(&mut Transform, &mut DirectionalLight, &LobbyLight)>,
    mut cameras: Query<&mut Camera, With<LobbyScene>>,
) {
    let Some(mut lobby_time) = lobby_time else {
        return;
    };

    lobby_time.0 = (lobby_time.0 + time.delta_secs() * LOBBY_DAY_SPEED).fract();

    let angle = lobby_time.0 * std::f32::consts::TAU;
    // -1 at midnight, 0 at sunrise/sunset, +1 at noon.
    let height = -angle.cos();
    let horizontal = angle.sin();
    // Direction TOWARD the sun. +Z keeps it on the camera's side so the
    // faces you can see are the lit ones; shadows fall back toward the wall.
    let sun_dir = Vec3::new(horizontal * 0.8, height, 0.6).normalize();

    let sun_strength = smoothstep(-0.05, 0.3, height); // 0 at night → 1 by mid-morning
    let moon_strength = smoothstep(0.1, -0.3, height); // 0 by day → 1 at night

    let sunset_orange = Color::srgb(1.0, 0.55, 0.30);
    let day_white = Color::srgb(1.0, 0.96, 0.88);

    for (mut transform, mut light, kind) in &mut lights {
        match kind {
            LobbyLight::Sun => {
                // Light travels away from the sun, toward the island.
                transform.rotation = Quat::from_rotation_arc(Vec3::NEG_Z, -sun_dir);
                light.illuminance = 9_000.0 * sun_strength;
                light.shadows_enabled = sun_strength > 0.01;
                light.color = lerp_color(sunset_orange, day_white, smoothstep(0.0, 0.5, height));
            }
            LobbyLight::Moon => {
                // The moon sits opposite the sun, so its light travels along sun_dir.
                transform.rotation = Quat::from_rotation_arc(Vec3::NEG_Z, sun_dir);
                light.illuminance = 1_500.0 * moon_strength;
            }
            LobbyLight::Fill => {
                light.illuminance = 300.0 + 2_200.0 * sun_strength;
            }
        }
    }

    // Sky colour behind the island.
    let night = Color::srgb(0.01, 0.02, 0.05);
    let day = Color::srgb(0.09, 0.22, 0.13); // the original lobby background
    let day_amount = (height * 0.5 + 0.5).clamp(0.0, 1.0);
    let sky = lerp_color(night, day, day_amount);
    for mut camera in &mut cameras {
        camera.clear_color = ClearColorConfig::Custom(sky);
    }
}

// ── spawning ─────────────────────────────────────────────────────────────────

fn color(r: f32, g: f32, b: f32) -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb(r, g, b),
        perceptual_roughness: 1.0,
        ..default()
    }
}

pub(super) fn spawn_lobby_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Start every lobby visit in the morning.
    commands.insert_resource(LobbyTime(LOBBY_START_TIME));

    // Camera looks slightly past the island's left so the island sits on the
    // right half of the screen, next to the menu.
    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.09, 0.22, 0.13)),
            ..default()
        },
        Transform::from_xyz(-5.0, 9.0, 17.0).looking_at(Vec3::new(-5.0, 4.0, 0.0), Vec3::Y),
        LobbyScene,
    ));

    // Sun (key light, with shadows). Its rotation and strength are driven by
    // `lobby_cycle_system` every frame; the initial transform is just a start.
    commands.spawn((
        DirectionalLight {
            illuminance: 9_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-6.0, 12.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        LobbyLight::Sun,
        LobbyScene,
    ));

    // Moon: cool, dim light that fades in at night.
    commands.spawn((
        DirectionalLight {
            illuminance: 0.0,
            color: Color::srgb(0.55, 0.65, 1.0),
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(6.0, 12.0, -8.0).looking_at(Vec3::ZERO, Vec3::Y),
        LobbyLight::Moon,
        LobbyScene,
    ));

    // Soft fill from the other side.
    commands.spawn((
        DirectionalLight {
            illuminance: 2_500.0,
            ..default()
        },
        Transform::from_xyz(8.0, 4.0, -6.0).looking_at(Vec3::new(0.0, 3.0, 0.0), Vec3::Y),
        LobbyLight::Fill,
        LobbyScene,
    ));

    // Three shades per block type so the island doesn't look flat.
    let palette: [[Handle<StandardMaterial>; 3]; 5] = KINDS.map(|k| {
        let (r, g, b) = k.color();
        [0.90f32, 1.00, 1.10].map(|s| {
            materials.add(color((r * s).min(1.0), (g * s).min(1.0), (b * s).min(1.0)))
        })
    });
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let grid = build_island();

    commands
        .spawn((
            Transform::default(),
            Visibility::default(),
            LobbyScene,
            Animated(Anim::Sway),
        ))
        .with_children(|root| {
            for x in 0..N {
                for y in 0..N {
                    for z in 0..N {
                        let Some(kind) = grid[x][y][z] else { continue };
                        if !exposed(&grid, x, y, z) {
                            continue;
                        }
                        let variant = (x * 31 + y * 17 + z * 13) % 3;
                        root.spawn((
                            Mesh3d(cube.clone()),
                            MeshMaterial3d(palette[kind.index()][variant].clone()),
                            Transform::from_xyz(x as f32 - 4.0, y as f32 + 0.5, z as f32 - 4.0),
                        ));
                    }
                }
            }
            spawn_character(root, &mut meshes, &mut materials);
        });
}

fn spawn_character(
    root: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    mats: &mut Assets<StandardMaterial>,
) {
    let skin = mats.add(color(0.96, 0.76, 0.62));
    let shirt = mats.add(color(0.20, 0.45, 0.85));
    let pants = mats.add(color(0.22, 0.24, 0.42));
    let hair = mats.add(color(0.30, 0.20, 0.12));
    let eye = mats.add(color(0.08, 0.08, 0.10));

    let leg = meshes.add(Cuboid::new(0.24, 0.70, 0.26));
    let body = meshes.add(Cuboid::new(0.52, 0.70, 0.28));
    let arm = meshes.add(Cuboid::new(0.20, 0.70, 0.24));
    let head = meshes.add(Cuboid::new(0.48, 0.48, 0.48));
    let hair_top = meshes.add(Cuboid::new(0.52, 0.14, 0.52));
    let eye_mesh = meshes.add(Cuboid::new(0.08, 0.10, 0.02));

    root.spawn((
        Transform::from_xyz(0.0, CHARACTER_Y, 1.5),
        Visibility::default(),
        Animated(Anim::Bob),
    ))
    .with_children(|c| {
        // Legs
        for x in [-0.13, 0.13] {
            c.spawn((
                Mesh3d(leg.clone()),
                MeshMaterial3d(pants.clone()),
                Transform::from_xyz(x, 0.35, 0.0),
            ));
        }

        // Body
        c.spawn((
            Mesh3d(body),
            MeshMaterial3d(shirt),
            Transform::from_xyz(0.0, 1.05, 0.0),
        ));

        // Arms hang from a pivot at the shoulder so they rotate around it.
        for (x, anim) in [(-0.36, Anim::IdleArm), (0.36, Anim::WaveArm)] {
            c.spawn((
                Transform::from_xyz(x, 1.4, 0.0),
                Visibility::default(),
                Animated(anim),
            ))
            .with_children(|pivot| {
                pivot.spawn((
                    Mesh3d(arm.clone()),
                    MeshMaterial3d(skin.clone()),
                    Transform::from_xyz(0.0, -0.35, 0.0),
                ));
            });
        }

        // Head with hair and eyes
        c.spawn((
            Mesh3d(head),
            MeshMaterial3d(skin),
            Transform::from_xyz(0.0, 1.64, 0.0),
            Animated(Anim::Head),
        ))
        .with_children(|h| {
            h.spawn((
                Mesh3d(hair_top),
                MeshMaterial3d(hair),
                Transform::from_xyz(0.0, 0.19, 0.0),
            ));
            for x in [-0.11, 0.11] {
                h.spawn((
                    Mesh3d(eye_mesh.clone()),
                    MeshMaterial3d(eye.clone()),
                    Transform::from_xyz(x, 0.04, 0.245),
                ));
            }
        });
    });
}