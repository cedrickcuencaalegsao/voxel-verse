use crate::utils::constants::{GRAVITY, JUMP_VELOCITY, PLAYER_SPEED};
use crate::world::world_manager::WorldManager;
use bevy::prelude::*;

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

#[derive(Resource)]
pub struct MovementSettings {
    pub speed: f32,
    pub sprint_speed: f32,
    pub crouch_speed: f32,
    pub jump_velocity: f32,
    pub gravity: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            speed: PLAYER_SPEED,
            sprint_speed: PLAYER_SPEED * 1.7,
            crouch_speed: PLAYER_SPEED * 0.55,
            jump_velocity: JUMP_VELOCITY,
            gravity: GRAVITY,
        }
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Grounded(pub bool);

#[derive(Component)]
pub struct AirborneTimer(pub f32);

#[derive(Component)]
pub struct Crouching(pub bool);

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
    mut query: Query<(
        &mut Velocity,
        &mut Transform,
        &mut Grounded,
        &mut AirborneTimer,
        &mut Crouching,
    )>,
    settings: Res<MovementSettings>,
    world: Res<WorldManager>,
) {
    for (mut velocity, mut transform, mut grounded, mut airborne_timer, mut crouching) in
        query.iter_mut()
    {
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

        let has_ctrl = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
        crouching.0 = has_ctrl && grounded.0;

        let has_shift = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

        let current_speed = if crouching.0 {
            settings.crouch_speed
        } else if is_moving_forward && has_shift {
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
            airborne_timer.0 = 2.5;
            crouching.0 = false;
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

        if grounded.0 {
            airborne_timer.0 = 0.0;
        } else {
            airborne_timer.0 += dt;
        }
    }
}

use crate::world::world_manager::Player;

pub fn animate_limbs(
    time: Res<Time>,
    settings: Res<MovementSettings>,
    player_query: Query<(&Velocity, &Grounded, &AirborneTimer, &Crouching), With<Player>>,
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
    let Some((velocity, grounded, airborne_timer, crouching)) = player_query.iter().next() else {
        return;
    };

    let t = time.elapsed_secs();

    let is_grounded_or_transitioning = grounded.0 || airborne_timer.0 < 2.5;
    let is_crouching = crouching.0 && is_grounded_or_transitioning;

    let horizontal_speed = Vec3::new(velocity.0.x, 0.0, velocity.0.z).length();
    let speed_ratio = (horizontal_speed / settings.speed).clamp(0.0, 2.0);
    let anim_weight = speed_ratio.min(1.0);
    let sprint_blend = (speed_ratio - 1.0).max(0.0).min(1.0);

    let base_frequency = if is_crouching { 6.5 } else { 8.0 };
    let frequency = base_frequency + 5.5 * sprint_blend;
    let phase = t * frequency;

    if is_grounded_or_transitioning {
        // Corrected heights to perfectly align standing leg length
        let torso_base_y = if is_crouching { 0.76 } else { 0.92 };
        let crouch_lean = if is_crouching { 0.45 } else { 0.0 };

        let forward_lean = 0.10 * anim_weight + 0.22 * sprint_blend + crouch_lean;
        let torso_pitch = -forward_lean - 0.02 * anim_weight * phase.sin() * 0.5;
        let torso_yaw = 0.03 * anim_weight * phase.sin();
        let torso_roll = -0.02 * anim_weight * (phase + std::f32::consts::FRAC_PI_2).sin();
        let torso_bob = -0.025 * anim_weight * (2.0 * phase).cos().abs();

        if let Some(mut tf) = torso.iter_mut().next() {
            tf.translation.y = torso_base_y + torso_bob;
            tf.rotation = Quat::from_euler(EulerRot::YXZ, torso_yaw, torso_pitch, torso_roll);
        }

        if let Some(mut tf) = head.iter_mut().next() {
            let head_pitch = forward_lean + (0.02 * anim_weight) * (2.0 * phase).sin();
            let head_yaw = -torso_yaw * 0.5;
            let head_roll = -torso_roll * 0.4;
            tf.rotation = Quat::from_euler(EulerRot::YXZ, head_yaw, head_pitch, head_roll);
        }

        let swing_mult = if is_crouching { 0.75 } else { 1.0 };

        let hip_fwd = (0.48 * anim_weight + 0.52 * sprint_blend) * swing_mult;
        let hip_back = (0.32 * anim_weight + 0.42 * sprint_blend) * swing_mult;
        let splay = 0.03 * anim_weight + 0.05 * sprint_blend;

        let knee_swing_peak = (0.75 * anim_weight + 0.65 * sprint_blend) * swing_mult;
        let knee_toe_off = (0.50 * anim_weight + 0.45 * sprint_blend) * swing_mult;

        let knee_stance_min = if is_crouching { 0.10 } else { 0.05 };

        let ankle_dorsiflex = (0.20 * anim_weight + 0.15 * sprint_blend) * swing_mult;
        let ankle_plantarflex = (-0.35 * anim_weight - 0.30 * sprint_blend) * swing_mult;

        let thigh_pitch = |s: f32| -> f32 {
            let base_pitch = if s >= 0.0 {
                -s * hip_fwd
            } else {
                -s * hip_back
            };
            if is_crouching {
                base_pitch + 0.42
            } else {
                base_pitch
            }
        };

        let knee_bend = |s: f32, c: f32| -> f32 {
            let swing = (-c).max(0.0) * knee_swing_peak;

            let in_toe_off_zone = ((-s).clamp(0.0, 1.0)) * ((1.0 - c.abs()).max(0.0));
            let toe_off = in_toe_off_zone * knee_toe_off;

            (swing + toe_off).max(knee_stance_min)
        };

        let ankle_pitch = |s: f32, c: f32| -> f32 {
            let stretch = -(s * s) * ankle_plantarflex.abs();

            let in_toe_off = (-s).clamp(0.0, 1.0) * c.max(0.0);
            let pull_up = in_toe_off * ankle_dorsiflex;

            let crouch_ankle_offset = 0.0;
            stretch + pull_up + crouch_ankle_offset
        };

        let ls = phase.sin();
        let lc = phase.cos();

        if let Some(mut tf) = l_thigh.iter_mut().next() {
            tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, thigh_pitch(ls), splay);
        }
        if let Some(mut tf) = l_shin.iter_mut().next() {
            tf.rotation = Quat::from_rotation_x(-knee_bend(ls, lc));
        }
        if let Some(mut tf) = l_foot.iter_mut().next() {
            tf.rotation = Quat::from_rotation_x(ankle_pitch(ls, lc));
        }

        let rs = (phase + std::f32::consts::PI).sin();
        let rc = (phase + std::f32::consts::PI).cos();

        if let Some(mut tf) = r_thigh.iter_mut().next() {
            tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, thigh_pitch(rs), -splay);
        }
        if let Some(mut tf) = r_shin.iter_mut().next() {
            tf.rotation = Quat::from_rotation_x(-knee_bend(rs, rc));
        }
        if let Some(mut tf) = r_foot.iter_mut().next() {
            tf.rotation = Quat::from_rotation_x(ankle_pitch(rs, rc));
        }

        let upper_hang = if is_crouching { 0.12 } else { 0.08 };
        let forearm_hang = if is_crouching { 0.28 } else { 0.18 };

        let forearm_base = forearm_hang + 0.15 * anim_weight + 0.25 * sprint_blend;
        let swing_range = (0.18 * anim_weight + 0.42 * sprint_blend) * swing_mult;
        let forearm_sweep = (0.35 * anim_weight + 0.70 * sprint_blend) * swing_mult;

        let la_s = -phase.sin();
        if let Some(mut tf) = l_upper_arm.iter_mut().next() {
            let pitch = upper_hang + la_s * swing_range;
            tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, pitch, 0.06 * anim_weight);
        }
        if let Some(mut tf) = l_forearm.iter_mut().next() {
            tf.rotation = Quat::from_rotation_x(forearm_base + la_s * forearm_sweep);
        }

        let ra_s = phase.sin();
        if let Some(mut tf) = r_upper_arm.iter_mut().next() {
            let pitch = upper_hang + ra_s * swing_range;
            tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, pitch, -(0.06 * anim_weight));
        }
        if let Some(mut tf) = r_forearm.iter_mut().next() {
            tf.rotation = Quat::from_rotation_x(forearm_base + ra_s * forearm_sweep);
        }
    } else {
        // Fall/Jump animations (Updated to remain consistent with the standing height 0.92)
        let vertical_speed = velocity.0.y;
        let leap_factor = (vertical_speed / settings.jump_velocity).clamp(-1.0, 1.0);

        if leap_factor > 0.0 {
            let blend = leap_factor;

            if let Some(mut tf) = torso.iter_mut().next() {
                tf.translation.y = 0.92; // Match standing height
                tf.rotation = Quat::from_rotation_x(-0.08 * blend);
            }
            if let Some(mut tf) = head.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(-0.12 * blend);
            }
            if let Some(mut tf) = l_thigh.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(0.55 * blend);
            }
            if let Some(mut tf) = r_thigh.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(0.25 * blend);
            }
            if let Some(mut tf) = l_shin.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(0.85 * blend);
            }
            if let Some(mut tf) = r_shin.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(0.55 * blend);
            }
            if let Some(mut tf) = l_foot.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(0.15 * blend);
            }
            if let Some(mut tf) = r_foot.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(0.15 * blend);
            }
            if let Some(mut tf) = l_upper_arm.iter_mut().next() {
                tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.60 * blend, 0.45 * blend);
            }
            if let Some(mut tf) = r_upper_arm.iter_mut().next() {
                tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.60 * blend, -0.45 * blend);
            }
            if let Some(mut tf) = l_forearm.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(0.30 * blend);
            }
            if let Some(mut tf) = r_forearm.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(0.30 * blend);
            }
        } else {
            let blend = -leap_factor;

            if let Some(mut tf) = torso.iter_mut().next() {
                tf.translation.y = 0.92; // Match standing height
                tf.rotation = Quat::from_rotation_x(0.18 * blend);
            }
            if let Some(mut tf) = head.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(0.10 * blend);
            }
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
            if let Some(mut tf) = l_foot.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(-0.20 * blend);
            }
            if let Some(mut tf) = r_foot.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(-0.20 * blend);
            }
            if let Some(mut tf) = l_upper_arm.iter_mut().next() {
                tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, -0.40 * blend, 0.15 * blend);
            }
            if let Some(mut tf) = r_upper_arm.iter_mut().next() {
                tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, -0.40 * blend, -0.15 * blend);
            }
            if let Some(mut tf) = l_forearm.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(0.20 * blend);
            }
            if let Some(mut tf) = r_forearm.iter_mut().next() {
                tf.rotation = Quat::from_rotation_x(0.20 * blend);
            }
        }
    }
}
