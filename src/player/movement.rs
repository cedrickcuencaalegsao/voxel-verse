use crate::utils::constants::{GRAVITY, JUMP_VELOCITY, PLAYER_SPEED};
use crate::world::world_manager::WorldManager;
use bevy::prelude::*;

// ── Limb segment joint marker components ──────────────────────────────────────
#[derive(Component)]
pub struct Torso;

#[derive(Component)]
pub struct Head;

#[derive(Component)]
pub struct LeftUpperArm;
#[derive(Component)]
pub struct LeftForeArm;
#[derive(Component)]
pub struct LeftHand;

#[derive(Component)]
pub struct RightUpperArm;
#[derive(Component)]
pub struct RightForeArm;
#[derive(Component)]
pub struct RightHand;

#[derive(Component)]
pub struct LeftThigh;
#[derive(Component)]
pub struct LeftShin;
#[derive(Component)]
pub struct LeftFoot;

#[derive(Component)]
pub struct RightThigh;
#[derive(Component)]
pub struct RightShin;
#[derive(Component)]
pub struct RightFoot;

// ── Movement settings ─────────────────────────────────────────────────────────
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
            sprint_speed: PLAYER_SPEED * 1.7,
            jump_velocity: JUMP_VELOCITY,
            gravity: GRAVITY,
        }
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Grounded(pub bool);

// ── Helpers ───────────────────────────────────────────────────────────────────
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

// ── Player movement ───────────────────────────────────────────────────────────
pub fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform, &mut Grounded)>,
    settings: Res<MovementSettings>,
    world: Res<WorldManager>,
) {
    for (mut velocity, mut transform, mut grounded) in query.iter_mut() {
        let dt = time.delta_secs();

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

        let has_shift = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
        let current_speed = if is_moving_forward && has_shift {
            settings.sprint_speed
        } else {
            settings.speed
        };

        let target_velocity = direction * current_speed;
        velocity.0.x = target_velocity.x;
        velocity.0.z = target_velocity.z;

        let horizontal_move = Vec3::new(velocity.0.x, 0.0, velocity.0.z) * dt;
        let step_tolerance = 0.15;

        let mut next_pos_x = transform.translation;
        next_pos_x.x += horizontal_move.x;
        if block_ground_y(&world, next_pos_x.x, next_pos_x.z)
            <= transform.translation.y + step_tolerance
        {
            transform.translation.x = next_pos_x.x;
        } else {
            velocity.0.x = 0.0;
        }

        let mut next_pos_z = transform.translation;
        next_pos_z.z += horizontal_move.z;
        if block_ground_y(&world, next_pos_z.x, next_pos_z.z)
            <= transform.translation.y + step_tolerance
        {
            transform.translation.z = next_pos_z.z;
        } else {
            velocity.0.z = 0.0;
        }

        if grounded.0 && keys.just_pressed(KeyCode::Space) {
            velocity.0.y = settings.jump_velocity;
            grounded.0 = false;
        }
        velocity.0.y -= settings.gravity * dt;
        transform.translation.y += velocity.0.y * dt;

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

// ── Limb animation ────────────────────────────────────────────────────────────
use crate::world::world_manager::Player;

pub fn animate_limbs(
    time: Res<Time>,
    settings: Res<MovementSettings>,
    player_query: Query<(&Velocity, &Grounded), With<Player>>,
    mut torso: Query<&mut Transform, With<Torso>>,
    mut head: Query<&mut Transform, (With<Head>, Without<Torso>)>,
    mut l_upper_arm: Query<&mut Transform, (With<LeftUpperArm>, Without<Torso>, Without<Head>)>,
    mut r_upper_arm: Query<
        &mut Transform,
        (
            With<RightUpperArm>,
            Without<Torso>,
            Without<Head>,
            Without<LeftUpperArm>,
        ),
    >,
    mut l_forearm: Query<
        &mut Transform,
        (
            With<LeftForeArm>,
            Without<Torso>,
            Without<Head>,
            Without<LeftUpperArm>,
            Without<RightUpperArm>,
        ),
    >,
    mut r_forearm: Query<
        &mut Transform,
        (
            With<RightForeArm>,
            Without<Torso>,
            Without<Head>,
            Without<LeftUpperArm>,
            Without<RightUpperArm>,
            Without<LeftForeArm>,
        ),
    >,
    mut l_thigh: Query<
        &mut Transform,
        (
            With<LeftThigh>,
            Without<Torso>,
            Without<Head>,
            Without<LeftUpperArm>,
            Without<RightUpperArm>,
            Without<LeftForeArm>,
            Without<RightForeArm>,
        ),
    >,
    mut r_thigh: Query<
        &mut Transform,
        (
            With<RightThigh>,
            Without<Torso>,
            Without<Head>,
            Without<LeftUpperArm>,
            Without<RightUpperArm>,
            Without<LeftForeArm>,
            Without<RightForeArm>,
            Without<LeftThigh>,
        ),
    >,
    mut l_shin: Query<
        &mut Transform,
        (
            With<LeftShin>,
            Without<Torso>,
            Without<Head>,
            Without<LeftUpperArm>,
            Without<RightUpperArm>,
            Without<LeftForeArm>,
            Without<RightForeArm>,
            Without<LeftThigh>,
            Without<RightThigh>,
        ),
    >,
    mut r_shin: Query<
        &mut Transform,
        (
            With<RightShin>,
            Without<Torso>,
            Without<Head>,
            Without<LeftUpperArm>,
            Without<RightUpperArm>,
            Without<LeftForeArm>,
            Without<RightForeArm>,
            Without<LeftThigh>,
            Without<RightThigh>,
            Without<LeftShin>,
        ),
    >,
    mut l_foot: Query<
        &mut Transform,
        (
            With<LeftFoot>,
            Without<Torso>,
            Without<Head>,
            Without<LeftUpperArm>,
            Without<RightUpperArm>,
            Without<LeftForeArm>,
            Without<RightForeArm>,
            Without<LeftThigh>,
            Without<RightThigh>,
            Without<LeftShin>,
            Without<RightShin>,
        ),
    >,
    mut r_foot: Query<
        &mut Transform,
        (
            With<RightFoot>,
            Without<Torso>,
            Without<Head>,
            Without<LeftUpperArm>,
            Without<RightUpperArm>,
            Without<LeftForeArm>,
            Without<RightForeArm>,
            Without<LeftThigh>,
            Without<RightThigh>,
            Without<LeftShin>,
            Without<RightShin>,
            Without<LeftFoot>,
        ),
    >,
) {
    let Some((velocity, grounded)) = player_query.iter().next() else {
        return;
    };

    let t = time.elapsed_secs();
    let is_grounded = grounded.0;

    let horizontal_speed = Vec3::new(velocity.0.x, 0.0, velocity.0.z).length();
    let speed_ratio = (horizontal_speed / settings.speed).clamp(0.0, 2.0);
    let anim_weight = speed_ratio.min(1.0);
    let sprint_blend = (speed_ratio - 1.0).max(0.0).min(1.0);

    // Speed up cycle frequency when sprinting
    let frequency = 6.5 + 5.5 * sprint_blend;
    let phase = t * frequency;

    if is_grounded {
        // ── 1. TORSO (Heavy Weight Sway & Elastic Bobbing) ───────────────────
        let torso_pitch = 0.05 * anim_weight + 0.20 * sprint_blend;
        let torso_yaw = (0.07 * anim_weight + 0.12 * sprint_blend) * phase.sin();
        let torso_roll = -(0.04 * anim_weight + 0.08 * sprint_blend)
            * (phase + std::f32::consts::FRAC_PI_2).sin();
        let torso_bob = (-0.05 * anim_weight - 0.10 * sprint_blend) * (2.0 * phase).cos().abs();

        if let Some(mut tf) = torso.iter_mut().next() {
            tf.translation.y = 0.75 + torso_bob;
            tf.rotation = Quat::from_euler(EulerRot::YXZ, torso_yaw, torso_pitch, torso_roll);
        }

        // ── 2. HEAD (Secondary Motion, lag & stability tilt) ─────────────────
        if let Some(mut tf) = head.iter_mut().next() {
            let head_pitch = -torso_pitch * 0.4 + 0.04 * (2.0 * phase).sin() * anim_weight;
            let head_yaw = -torso_yaw * 0.6;
            let head_roll = -torso_roll * 0.5;
            tf.rotation = Quat::from_euler(EulerRot::YXZ, head_yaw, head_pitch, head_roll);
        }

        // ── 3. LEGS (Trailing Knee Physics & Dynamic Splay) ──────────────────
        let leg_swing = 0.40 * anim_weight + 0.35 * sprint_blend;
        let knee_bend_max = 0.50 * anim_weight + 0.45 * sprint_blend;

        let left_thigh_pitch = -phase.sin() * leg_swing;
        let right_thigh_pitch = phase.sin() * leg_swing;

        // Slight outward leg splay for dynamic balance
        let splay = 0.03 * anim_weight + 0.05 * sprint_blend;

        if let Some(mut tf) = l_thigh.iter_mut().next() {
            tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, left_thigh_pitch, splay);
        }
        if let Some(mut tf) = r_thigh.iter_mut().next() {
            tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, right_thigh_pitch, -splay);
        }

        // Knees bend when sweeping forward (lifting the foot to prevent dragging)
        let left_swing_speed = -phase.cos();
        let right_swing_speed = phase.cos();

        if let Some(mut tf) = l_shin.iter_mut().next() {
            let bend = if left_swing_speed > 0.0 {
                left_swing_speed * knee_bend_max
            } else {
                0.02
            };
            tf.rotation = Quat::from_rotation_x(bend);
        }
        if let Some(mut tf) = r_shin.iter_mut().next() {
            let bend = if right_swing_speed > 0.0 {
                right_swing_speed * knee_bend_max
            } else {
                0.02
            };
            tf.rotation = Quat::from_rotation_x(bend);
        }

        // ── 4. ARMS (Movie-Style Cross-Body Athletic Swing) ───────────────────
        let arm_swing = 0.35 * anim_weight + 0.55 * sprint_blend;
        let elbow_base = 0.20 + 0.65 * sprint_blend;
        let elbow_swing = 0.15 * anim_weight + 0.35 * sprint_blend;

        // Cross-body swing yaw (arms move slightly inward across the chest)
        let cross_body = 0.22 * sprint_blend * phase.cos();

        if let Some(mut tf) = l_upper_arm.iter_mut().next() {
            let pitch = phase.sin() * arm_swing;
            let roll = 0.08 * anim_weight + 0.15 * sprint_blend;
            tf.rotation = Quat::from_euler(EulerRot::YXZ, cross_body, pitch, roll);
        }
        if let Some(mut tf) = r_upper_arm.iter_mut().next() {
            let pitch = -phase.sin() * arm_swing;
            let roll = -(0.08 * anim_weight + 0.15 * sprint_blend);
            tf.rotation = Quat::from_euler(EulerRot::YXZ, -cross_body, pitch, roll);
        }

        if let Some(mut tf) = l_forearm.iter_mut().next() {
            let bend = elbow_base + elbow_swing * (phase + std::f32::consts::FRAC_PI_2).sin();
            tf.rotation = Quat::from_rotation_x(-bend);
        }
        if let Some(mut tf) = r_forearm.iter_mut().next() {
            let bend = elbow_base + elbow_swing * (-phase + std::f32::consts::FRAC_PI_2).sin();
            tf.rotation = Quat::from_rotation_x(-bend);
        }

        // ── 5. FEET (Ankle Flexion) ──────────────────────────────────────────
        if let Some(mut tf) = l_foot.iter_mut().next() {
            tf.rotation = Quat::from_rotation_x(-0.15 * phase.cos() * anim_weight);
        }
        if let Some(mut tf) = r_foot.iter_mut().next() {
            tf.rotation = Quat::from_rotation_x(0.15 * phase.cos() * anim_weight);
        }
    } else {
        // ── 6. JUMP & FALL (Velocity-Driven Dynamic Pose Blending) ───────────
        let vertical_speed = velocity.0.y;

        // Map vertical speed to a normalized leap state (-1.0 = falling fast, 1.0 = leaping up)
        let leap_factor = (vertical_speed / settings.jump_velocity).clamp(-1.0, 1.0);

        if leap_factor > 0.0 {
            // Ascending (Heroic Tuck Jump)
            let blend = leap_factor;

            if let Some(mut tf) = torso.iter_mut().next() {
                tf.translation.y = 0.75;
                tf.rotation = Quat::from_rotation_x(-0.08 * blend); // Lean back slightly
            }
            if let Some(mut tf) = head.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(0.12 * blend); // Look slightly up
            }
            // Tuck legs up
            if let Some(mut tf) = l_thigh.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(-0.55 * blend);
            }
            if let Some(mut tf) = r_thigh.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(-0.25 * blend);
            }
            if let Some(mut tf) = l_shin.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(0.85 * blend);
            }
            if let Some(mut tf) = r_shin.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(0.55 * blend);
            }
            // Elevate arms outward & upward (balancing)
            if let Some(mut tf) = l_upper_arm.iter_mut().next() {
                tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, -0.60 * blend, 0.45 * blend);
            }
            if let Some(mut tf) = r_upper_arm.iter_mut().next() {
                tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, -0.60 * blend, -0.45 * blend);
            }
            if let Some(mut tf) = l_forearm.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(-0.30 * blend);
            }
            if let Some(mut tf) = r_forearm.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(-0.30 * blend);
            }
        } else {
            // Descending (Pre-landing Fall Pose)
            let blend = -leap_factor;

            if let Some(mut tf) = torso.iter_mut().next() {
                tf.translation.y = 0.75;
                tf.rotation = Quat::from_rotation_x(0.18 * blend); // Aggressive forward lean
            }
            if let Some(mut tf) = head.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(-0.10 * blend); // Look downward
            }
            // Reach legs straight down to prepare for landing cushioning
            if let Some(mut tf) = l_thigh.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(-0.10 * blend);
            }
            if let Some(mut tf) = r_thigh.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(-0.10 * blend);
            }
            if let Some(mut tf) = l_shin.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(0.20 * blend);
            }
            if let Some(mut tf) = r_shin.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(0.20 * blend);
            }
            // Arms reach slightly back & down
            if let Some(mut tf) = l_upper_arm.iter_mut().next() {
                tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.40 * blend, 0.15 * blend);
            }
            if let Some(mut tf) = r_upper_arm.iter_mut().next() {
                tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.40 * blend, -0.15 * blend);
            }
            if let Some(mut tf) = l_forearm.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(-0.20 * blend);
            }
            if let Some(mut tf) = r_forearm.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(-0.20 * blend);
            }
        }
    }
}
