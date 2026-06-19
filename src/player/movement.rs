use crate::utils::constants::{GRAVITY, JUMP_VELOCITY, PLAYER_SPEED};
use crate::world::world_manager::WorldManager;
use bevy::prelude::*;

#[derive(Resource)]
pub struct MovementSettings {
    pub speed: f32,
    pub sprint_speed: f32,
    pub jump_velocity: f32,
    pub gravity: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            speed: PLAYER_SPEED,
            sprint_speed: PLAYER_SPEED * 1.7, // Default sprint speed is 1.5x regular speed
            jump_velocity: JUMP_VELOCITY,
            gravity: GRAVITY,
        }
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Grounded(pub bool);

fn block_ground_y(world: &WorldManager, x: f32, z: f32) -> f32 {
    let bx = x.floor() as f64;
    let bz = z.floor() as f64;
    let surface = if bx * bx + bz * bz <= 40.0 * 40.0 {
        world.generator.spawn_height_at(bx, bz)
    } else {
        world.generator.height_at(bx, bz)
    };
    surface.floor() as f32 + 1.0
}

pub fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform, &mut Grounded)>,
    settings: Res<MovementSettings>,
    world: Res<WorldManager>,
) {
    for (mut velocity, mut transform, mut grounded) in query.iter_mut() {
        let dt = time.delta_secs();

        // ── horizontal movement ───────────────────────────────────────────────
        let mut direction = Vec3::ZERO;
        let mut is_moving_forward = false;

        if keys.pressed(KeyCode::KeyW) {
            direction += *transform.forward();
            is_moving_forward = true;
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

        // Check if shifting while moving forward to determine sprint speed
        let has_shift = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
        let current_speed = if is_moving_forward && has_shift {
            settings.sprint_speed
        } else {
            settings.speed
        };

        let horizontal_move = direction * current_speed * dt;
        let step_tolerance = 0.15;

        // Try moving along the X axis and verify collision
        let mut next_pos_x = transform.translation;
        next_pos_x.x += horizontal_move.x;
        let ground_at_next_x = block_ground_y(&world, next_pos_x.x, next_pos_x.z);
        if ground_at_next_x <= transform.translation.y + step_tolerance {
            transform.translation.x = next_pos_x.x;
        }

        // Try moving along the Z axis and verify collision
        let mut next_pos_z = transform.translation;
        next_pos_z.z += horizontal_move.z;
        let ground_at_next_z = block_ground_y(&world, next_pos_z.x, next_pos_z.z);
        if ground_at_next_z <= transform.translation.y + step_tolerance {
            transform.translation.z = next_pos_z.z;
        }

        // ── jump & gravity ────────────────────────────────────────────────────
        if grounded.0 && keys.just_pressed(KeyCode::Space) {
            velocity.0.y = settings.jump_velocity;
            grounded.0 = false;
        }
        velocity.0.y -= settings.gravity * dt;
        transform.translation.y += velocity.0.y * dt;

        // ── ground snap & collision ───────────────────────────────────────────
        let ground_after = block_ground_y(&world, transform.translation.x, transform.translation.z);

        if transform.translation.y <= ground_after && velocity.0.y <= 0.0 {
            transform.translation.y = ground_after;
            velocity.0.y = 0.0;
            grounded.0 = true;
        } else {
            grounded.0 = false;
        }
    }
}
