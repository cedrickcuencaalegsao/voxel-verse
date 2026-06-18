use bevy::prelude::*;

#[derive(Resource)]
pub struct DayNightCycle {
    pub time: f32, // 0..1 (0 = midnight, 0.5 = noon)
}

impl Default for DayNightCycle {
    fn default() -> Self {
        Self { time: 0.25 } // start at sunrise
    }
}

/// Marker for the visible sun disc — a separate object from the light itself.
#[derive(Component)]
pub struct Sun;

/// Run once at startup: spawns the actual light source plus a bright,
/// unlit sphere that stands in for "the sun" you can see in the sky.
pub fn setup_sky(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default(),
    ));

    commands.spawn((
        Sun,
        Mesh3d(meshes.add(Sphere::new(15.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.95, 0.7),
            unlit: true, // ignore lighting entirely so it always reads as bright
            ..default()
        })),
        Transform::default(),
    ));
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

pub fn cycle_system(
    time: Res<Time>,
    mut cycle: ResMut<DayNightCycle>,
    mut clear_color: ResMut<ClearColor>,
    mut light_query: Query<&mut Transform, (With<DirectionalLight>, Without<Sun>)>,
    mut sun_query: Query<&mut Transform, (With<Sun>, Without<DirectionalLight>)>,
) {
    cycle.time += time.delta_secs() * 0.005;
    if cycle.time > 1.0 {
        cycle.time -= 1.0;
    }

    let angle = cycle.time * 2.0 * std::f32::consts::PI;
    // -1 at midnight, 0 at sunrise/sunset, +1 at noon — matches the doc comments above.
    let height = -angle.cos();
    let horizontal = angle.sin();
    let sun_dir = Vec3::new(horizontal, height, 0.3).normalize(); // direction TOWARD the sun
    let light_dir = -sun_dir; // direction the light travels, toward the ground

    for mut transform in light_query.iter_mut() {
        // Rotate -Z (the light's forward axis) to point along light_dir,
        // not Y — Y was rotating an axis the light doesn't actually use.
        transform.rotation = Quat::from_rotation_arc(Vec3::NEG_Z, light_dir);
    }

    for mut transform in sun_query.iter_mut() {
        transform.translation = sun_dir * 400.0; // far enough to look like sky, not a nearby object
    }

    let night = Color::srgb(0.01, 0.01, 0.05);
    let day = Color::srgb(0.4, 0.65, 0.95);
    let day_amount = (height * 0.5 + 0.5).clamp(0.0, 1.0);
    clear_color.0 = lerp_color(night, day, day_amount);
}
