use crate::utils::constants::{GRAVITY, JUMP_VELOCITY, PLAYER_SPEED};
use crate::world::world_manager::WorldManager;
use bevy::prelude::*;

#[derive(Resource)]
pub struct MovementSettings {
    pub speed: f32,
    pub jump_velocity: f32,
    pub gravity: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            speed: PLAYER_SPEED,
            jump_velocity: JUMP_VELOCITY,
            gravity: GRAVITY,
        }
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Grounded(pub bool);

pub fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform, &mut Grounded)>,
    settings: Res<MovementSettings>,
    world: Res<WorldManager>,
) {
    for (mut velocity, mut transform, mut grounded) in query.iter_mut() {
        let dt = time.delta_secs();

        // Horizontal movement
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            direction += *transform.forward();
        }
        if keys.pressed(KeyCode::KeyS) {
            direction += *transform.back();
        }
        if keys.pressed(KeyCode::KeyA) {
            direction += *transform.left();
        }
        if keys.pressed(KeyCode::KeyD) {
            direction += *transform.right();
        }
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction * settings.speed * dt;

        // Gravity & jumping
        if grounded.0 && keys.just_pressed(KeyCode::Space) {
            velocity.0.y = settings.jump_velocity;
            grounded.0 = false;
        }
        velocity.0.y -= settings.gravity * dt;
        transform.translation.y += velocity.0.y * dt;

        let ground_y = world
            .generator
            .height_at(transform.translation.x as f64, transform.translation.z as f64)
            as f32
            + 1.0;

        if transform.translation.y <= ground_y {
            transform.translation.y = ground_y;
            velocity.0.y = 0.0;
            grounded.0 = true;
        }
    }
}
