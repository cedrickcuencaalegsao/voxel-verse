use bevy::asset::RenderAssetUsages;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use std::f32::consts::{PI, TAU};

use crate::lobby::LobbyScene;
use crate::rendering::texture::{
    SKY_TEX_SIZE, STAR_TEX_SIZE, gen_moon, gen_moon_glow, gen_star_dot, gen_star_sparkle, gen_sun,
};

// ── tuning ───────────────────────────────────────────────────────────────────

/// How fast the day passes (1.0 = a full day per second). 0.005 → ~200 s per day.
const DAY_SPEED: f32 = 0.005;

/// How bright the night is. Lower = darker nights.
/// Moonlight strength at full night (the sun is 10_000 at noon).
const MOON_ILLUMINANCE: f32 = 7.0;
/// Ambient fill at night and by day (it blends between the two).
const AMBIENT_NIGHT: f32 = 3.0;
const AMBIENT_DAY: f32 = 80.0;
/// Sky colour at midnight.
const NIGHT_SKY: Color = Color::srgb(0.003, 0.004, 0.015);

/// Shadows from each light. `true` = always on (costs one shadow pass per light).
const SUN_SHADOWS: bool = true;
const MOON_SHADOWS: bool = true;

/// Distance of the sun / moon from the camera.
const SKY_RADIUS: f32 = 400.0;
/// Stars sit a little further out, so the sun and moon always draw in front.
const STAR_RADIUS: f32 = 420.0;

/// Edge length of the square sun / moon / moon glow, in world units.
const SUN_SIZE: f32 = 50.0;
const MOON_SIZE: f32 = 40.0;
const MOON_GLOW_SIZE: f32 = 120.0;

const STAR_COUNT: u64 = 350;
/// Change this to get a different (but still stable) arrangement of stars.
const STAR_SEED: u64 = 0x5EED_57A2_1234_ABCD;

// ── components & resources ───────────────────────────────────────────────────

#[derive(Resource)]
pub struct DayNightCycle {
    pub time: f32, // 0..1 (0 = midnight, 0.5 = noon)
}

impl Default for DayNightCycle {
    fn default() -> Self {
        Self { time: 0.25 } // start at sunrise
    }
}

/// The visible things in the sky. All of them are positioned around the camera
/// every frame, so they stay "infinitely far away" while the player walks.
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum SkyBody {
    Sun,
    Moon,
    MoonGlow,
    /// Parent of every star; it follows the camera and rotates with the day.
    Stars,
}

/// The two real light sources. Tagged so this cycle never touches other
/// directional lights (for example the lobby's).
#[derive(Component, Clone, Copy)]
pub enum SkyLight {
    Sun,
    Moon,
}

/// Material handles whose alpha changes during the night.
#[derive(Resource)]
pub struct SkyAssets {
    /// [dot, sparkle A, sparkle B] — the two sparkle sets twinkle out of phase.
    stars: [Handle<StandardMaterial>; 3],
    moon_glow: Handle<StandardMaterial>,
}

// ── helpers ──────────────────────────────────────────────────────────────────

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

/// Everything derived from the time of day, shared by both systems.
struct SkyState {
    angle: f32,
    /// -1 at midnight, 0 at sunrise/sunset, +1 at noon.
    height: f32,
    /// Direction TOWARD the sun (the moon is on the opposite side).
    sun_dir: Vec3,
    /// 0 at night → 1 by mid-morning.
    sun_strength: f32,
    /// 0 by day → 1 at night. Also drives the stars and the moon glow.
    moon_strength: f32,
}

fn sky_state(time: f32) -> SkyState {
    let angle = time * TAU;
    let height = -angle.cos();
    let horizontal = angle.sin();
    SkyState {
        angle,
        height,
        sun_dir: Vec3::new(horizontal, height, 0.3).normalize(),
        sun_strength: smoothstep(-0.05, 0.3, height),
        moon_strength: smoothstep(0.1, -0.3, height),
    }
}

fn hash64(mut x: u64) -> u64 {
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58476d1ce4e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d049bb133111eb);
    x ^= x >> 31;
    x
}

/// Stable pseudo-random number in [0, 1) for star `i`, channel `k`.
fn star_rand(i: u64, k: u64) -> f32 {
    let h = hash64(
        STAR_SEED
            ^ i.wrapping_mul(0x9e37_79b9_7f4a_7c15)
            ^ k.wrapping_mul(0xbf58_476d_1ce4_e5b9),
    );
    (h >> 40) as f32 / (1u64 << 24) as f32
}

fn sky_image(rgba: Vec<u8>, size: u32) -> Image {
    let mut image = Image::new(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        rgba,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::nearest(); // keep the pixel-art look
    image
}

/// Unlit, double-sided material for a flat sky quad.
fn sky_material(texture: Handle<Image>, alpha_mode: AlphaMode) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(texture),
        unlit: true,
        alpha_mode,
        double_sided: true,
        cull_mode: None,
        ..default()
    }
}

/// Turns a flat quad so its front faces the camera, keeping it upright.
fn face_camera(tf: &mut Transform, camera: Vec3) {
    let away = tf.translation * 2.0 - camera; // look away so +Z points at the camera
    tf.look_at(away, Vec3::Y);
}

fn set_alpha(materials: &mut Assets<StandardMaterial>, handle: &Handle<StandardMaterial>, a: f32) {
    if let Some(m) = materials.get_mut(handle) {
        m.base_color = Color::srgba(1.0, 1.0, 1.0, a.clamp(0.0, 1.0));
    }
}

// ── setup ────────────────────────────────────────────────────────────────────

/// Run once at startup: the two lights, the square sun, the moon (with glow)
/// and the star field.
pub fn setup_sky(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // Lights. Rotation and strength are driven by `cycle_system`.
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: SUN_SHADOWS,
            ..default()
        },
        Transform::default(),
        SkyLight::Sun,
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 0.0,
            color: Color::srgb(0.55, 0.65, 1.0), // cool blue moonlight
            shadows_enabled: MOON_SHADOWS,
            ..default()
        },
        Transform::default(),
        SkyLight::Moon,
    ));

    // Textures come from `rendering/texture`.
    let sun_tex = images.add(sky_image(gen_sun(), SKY_TEX_SIZE));
    let moon_tex = images.add(sky_image(gen_moon(), SKY_TEX_SIZE));
    let glow_tex = images.add(sky_image(gen_moon_glow(), SKY_TEX_SIZE));
    let dot_tex = images.add(sky_image(gen_star_dot(), STAR_TEX_SIZE));
    let sparkle_tex = images.add(sky_image(gen_star_sparkle(), STAR_TEX_SIZE));

    // Sun — a square.
    commands.spawn((
        SkyBody::Sun,
        Mesh3d(meshes.add(Rectangle::new(SUN_SIZE, SUN_SIZE))),
        MeshMaterial3d(materials.add(sky_material(sun_tex, AlphaMode::Opaque))),
        Transform::default(),
    ));

    // Moon + glow behind it.
    commands.spawn((
        SkyBody::Moon,
        Mesh3d(meshes.add(Rectangle::new(MOON_SIZE, MOON_SIZE))),
        MeshMaterial3d(materials.add(sky_material(moon_tex, AlphaMode::Opaque))),
        Transform::default(),
    ));
    let moon_glow = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
        ..sky_material(glow_tex, AlphaMode::Blend)
    });
    commands.spawn((
        SkyBody::MoonGlow,
        Mesh3d(meshes.add(Rectangle::new(MOON_GLOW_SIZE, MOON_GLOW_SIZE))),
        MeshMaterial3d(moon_glow.clone()),
        Transform::default(),
    ));

    // Stars: three shared materials (alpha is animated on them), randomly
    // sized quads scattered over a sphere around the camera.
    let star_material = |materials: &mut Assets<StandardMaterial>, tex: &Handle<Image>| {
        materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
            ..sky_material(tex.clone(), AlphaMode::Blend)
        })
    };
    let dot = star_material(&mut materials, &dot_tex);
    let sparkle_a = star_material(&mut materials, &sparkle_tex);
    let sparkle_b = star_material(&mut materials, &sparkle_tex);
    let star_mesh = meshes.add(Rectangle::new(1.0, 1.0)); // scaled per star

    commands
        .spawn((SkyBody::Stars, Transform::default(), Visibility::Hidden))
        .with_children(|field| {
            for i in 0..STAR_COUNT {
                // Uniform point on a sphere.
                let z = 1.0 - 2.0 * star_rand(i, 0);
                let phi = TAU * star_rand(i, 1);
                let ring = (1.0 - z * z).max(0.0).sqrt();
                let dir = Vec3::new(ring * phi.cos(), z, ring * phi.sin());
                let pos = dir * STAR_RADIUS;

                // Random size: mostly small, a few big ones. Sparkles are larger.
                let pick = star_rand(i, 3);
                let (material, boost) = if pick < 0.6 {
                    (&dot, 1.0)
                } else if pick < 0.8 {
                    (&sparkle_a, 1.8)
                } else {
                    (&sparkle_b, 1.8)
                };
                let size = (1.5 + star_rand(i, 2).powi(2) * 5.0) * boost;

                field.spawn((
                    Mesh3d(star_mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    // Face the centre of the dome (where the camera is).
                    Transform::from_translation(pos)
                        .looking_at(pos * 2.0, Vec3::Y)
                        .with_scale(Vec3::splat(size)),
                ));
            }
        });

    commands.insert_resource(SkyAssets {
        stars: [dot, sparkle_a, sparkle_b],
        moon_glow,
    });
}

// ── per-frame systems ────────────────────────────────────────────────────────

/// Advances the clock and drives the lights, ambient light and sky colour.
pub fn cycle_system(
    time: Res<Time>,
    mut cycle: ResMut<DayNightCycle>,
    mut clear_color: ResMut<ClearColor>,
    // Bevy 0.17 and earlier: `AmbientLight` resource (renamed `GlobalAmbientLight` in 0.18).
    mut ambient: ResMut<AmbientLight>,
    mut lights: Query<(&mut Transform, &mut DirectionalLight, &SkyLight)>,
) {
    cycle.time = (cycle.time + time.delta_secs() * DAY_SPEED).fract();
    let sky = sky_state(cycle.time);

    let sunset_orange = Color::srgb(1.0, 0.55, 0.30);
    let day_white = Color::srgb(1.0, 0.96, 0.88);

    for (mut transform, mut light, kind) in &mut lights {
        match kind {
            SkyLight::Sun => {
                // Light travels away from the sun, toward the ground.
                transform.rotation = Quat::from_rotation_arc(Vec3::NEG_Z, -sky.sun_dir);
                light.illuminance = 10_000.0 * sky.sun_strength;
                light.shadows_enabled = SUN_SHADOWS;
                light.color = lerp_color(sunset_orange, day_white, smoothstep(0.0, 0.5, sky.height));
            }
            SkyLight::Moon => {
                // The moon is opposite the sun, so its light travels along sun_dir.
                transform.rotation = Quat::from_rotation_arc(Vec3::NEG_Z, sky.sun_dir);
                light.illuminance = MOON_ILLUMINANCE * sky.moon_strength;
                light.shadows_enabled = MOON_SHADOWS;
            }
        }
    }

    // Dim, blue-tinted ambient fill at night so faces aren't visible "for free".
    ambient.brightness = AMBIENT_NIGHT + (AMBIENT_DAY - AMBIENT_NIGHT) * sky.sun_strength;
    ambient.color = lerp_color(
        Color::srgb(0.55, 0.65, 1.0),
        Color::WHITE,
        sky.sun_strength,
    );

    let night = NIGHT_SKY;
    let day = Color::srgb(0.4, 0.65, 0.95);
    let day_amount = (sky.height * 0.5 + 0.5).clamp(0.0, 1.0);
    clear_color.0 = lerp_color(night, day, day_amount);
}

/// Positions the sun, moon, glow and stars around the camera, and fades the
/// stars / glow in and out with the night.
pub fn sky_bodies_system(
    time: Res<Time>,
    cycle: Res<DayNightCycle>,
    assets: Option<Res<SkyAssets>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera: Query<&GlobalTransform, With<Camera3d>>,
    mut bodies: Query<(&mut Transform, &mut Visibility, &SkyBody)>,
) {
    let Some(assets) = assets else {
        return;
    };
    let cam = camera
        .iter()
        .next()
        .map(|g| g.translation())
        .unwrap_or(Vec3::ZERO);
    let sky = sky_state(cycle.time);
    let moon_dir = -sky.sun_dir;

    let sun_up = sky.height > -0.05;
    let moon_up = sky.height < 0.05;
    let show = |visible: bool| {
        if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        }
    };

    for (mut tf, mut visibility, body) in &mut bodies {
        match body {
            SkyBody::Sun => {
                tf.translation = cam + sky.sun_dir * SKY_RADIUS;
                face_camera(&mut tf, cam);
                *visibility = show(sun_up);
            }
            SkyBody::Moon => {
                tf.translation = cam + moon_dir * SKY_RADIUS;
                face_camera(&mut tf, cam);
                *visibility = show(moon_up);
            }
            SkyBody::MoonGlow => {
                // A little further out so it sits behind the moon.
                tf.translation = cam + moon_dir * (SKY_RADIUS + 4.0);
                face_camera(&mut tf, cam);
                *visibility = show(moon_up);
            }
            SkyBody::Stars => {
                tf.translation = cam;
                // Same axis the sun orbits around, so stars turn with the day.
                tf.rotation = Quat::from_rotation_z(sky.angle);
                *visibility = show(sky.moon_strength > 0.01);
            }
        }
    }

    // Stars fade in at dusk; the two sparkle groups twinkle out of phase.
    let t = time.elapsed_secs();
    let night = sky.moon_strength;
    set_alpha(&mut materials, &assets.stars[0], night);
    set_alpha(
        &mut materials,
        &assets.stars[1],
        night * (0.65 + 0.35 * (t * 2.5).sin()),
    );
    set_alpha(
        &mut materials,
        &assets.stars[2],
        night * (0.65 + 0.35 * (t * 2.5 + PI).sin()),
    );
    set_alpha(&mut materials, &assets.moon_glow, night * 0.9);
}

// ── temporary debugging ──────────────────────────────────────────────────────

/// TEMPORARY: logs every light in the world every 2 seconds, so you can see
/// which one is keeping the blocks bright at night. Remove it (and its line in
/// `plugin.rs`) once the night looks right.
pub fn debug_lights_system(
    time: Res<Time>,
    mut elapsed: Local<f32>,
    cycle: Res<DayNightCycle>,
    ambient: Res<AmbientLight>,
    directional: Query<(Entity, &DirectionalLight, Has<SkyLight>)>,
    points: Query<(Entity, &PointLight)>,
    spots: Query<(Entity, &SpotLight)>,
) {
    *elapsed += time.delta_secs();
    if *elapsed < 2.0 {
        return;
    }
    *elapsed = 0.0;

    let sky = sky_state(cycle.time);
    info!(
        "[lights] time={:.3} height={:.2} sun_strength={:.2} moon_strength={:.2} ambient={:.1}",
        cycle.time, sky.height, sky.sun_strength, sky.moon_strength, ambient.brightness
    );
    for (entity, light, is_sky) in &directional {
        info!(
            "[lights]   DirectionalLight {entity:?} illuminance={:.0} shadows={} ours={}",
            light.illuminance, light.shadows_enabled, is_sky
        );
    }
    for (entity, light) in &points {
        info!("[lights]   PointLight {entity:?} intensity={:.0}", light.intensity);
    }
    for (entity, light) in &spots {
        info!("[lights]   SpotLight {entity:?} intensity={:.0}", light.intensity);
    }
}

// ── stray lights ─────────────────────────────────────────────────────────────

/// Despawns any `DirectionalLight` that is neither ours (`SkyLight`) nor the
/// lobby's (`LobbyScene`). Such a light never follows the day/night cycle, so
/// it keeps everything lit like daytime. Once you have found and deleted the
/// code that spawns it, you can remove this system and its line in `plugin.rs`.
pub fn remove_stray_lights(
    mut commands: Commands,
    stray: Query<Entity, (With<DirectionalLight>, Without<SkyLight>, Without<LobbyScene>)>,
) {
    for entity in &stray {
        warn!(
            "Removing stray DirectionalLight {entity:?}: it ignores the day/night cycle. \
             Find where it is spawned and delete that code."
        );
        commands.entity(entity).despawn();
    }
}