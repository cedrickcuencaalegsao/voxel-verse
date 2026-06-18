use bevy::prelude::*;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene);
    }
}

fn setup_scene(mut commands: Commands) {
    // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
