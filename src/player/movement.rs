use crate::utils::constants::{GRAVITY, JUMP_VELOCITY, PLAYER_SPEED};
use crate::world::world_manager::Player;
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

    // Step frequency scales with speed. Real humans take ~1.7-2.2 steps/sec
    // walking and ~2.8-3.2 steps/sec sprinting. The full gait cycle is two
    // steps, so the phase advances by 2*PI per cycle (one step is PI).
    let base_frequency = if is_crouching { 1.6 } else { 2.0 };
    let frequency = base_frequency + 1.0 * sprint_blend;
    let phase = t * frequency * std::f32::consts::TAU;
    // s = swing signal in [-1, 1] for a leg; positive = forward swing,
    // negative = stance. c = complementary cosine (used for toe-off / heel-strike).
    let ls = phase.sin();
    let lc = phase.cos();
    let rs = (phase + std::f32::consts::PI).sin();
    let rc = (phase + std::f32::consts::PI).cos();

    if is_grounded_or_transitioning {
        let torso_base_y = if is_crouching { 0.76 } else { 0.92 };
        let crouch_lean = if is_crouching { 0.45 } else { 0.0 };

        let forward_lean = 0.08 * anim_weight + 0.18 * sprint_blend + crouch_lean;

        // Vertical bob: humans reach their lowest point once per step at heel-strike.
        // Use a single sin (not |cos|) so it has a smooth up/down, and shift the phase
        // so the dip happens at heel-strike (when each foot lands).
        let bob_amp = 0.04 * anim_weight + 0.05 * sprint_blend;
        let torso_bob = -bob_amp * (phase - std::f32::consts::FRAC_PI_2).sin().max(0.0);

        // Counter-rotation of the torso against the hips — a real human twists the
        // upper body slightly opposite to the leading leg.
        let twist_amp = 0.04 * anim_weight + 0.05 * sprint_blend;
        let torso_yaw = -twist_amp * ls;

        let torso_pitch = -forward_lean;

        if let Some(mut tf) = torso.iter_mut().next() {
            tf.translation.y = torso_base_y + torso_bob;
            // Roll is held at zero to suppress the left/right wobble.
            tf.rotation = Quat::from_euler(EulerRot::YXZ, torso_yaw, torso_pitch, 0.0);
        }

        // Head counter-balances: stays mostly level with tiny compensatory movements.
        if let Some(mut tf) = head.iter_mut().next() {
            let head_pitch = forward_lean * 0.4;
            let head_yaw = -torso_yaw * 0.6;
            tf.rotation = Quat::from_euler(EulerRot::YXZ, head_yaw, head_pitch, 0.0);
        }

        let swing_mult = if is_crouching { 0.6 } else { 1.0 };

        // Hip swing: the femur swings ~30-40° at the hip when walking, more
        // when sprinting. The thigh reaches its FORWARD limit at heel-strike
        // (s=0, c=1) and its BACKWARD limit at toe-off (s=0, c=-1). At mid-
        // swing and mid-stance the thigh is roughly vertical. We drive this
        // with the cosine of the phase so the extremes line up with foot
        // contact, which gives the sprint the long-reach "reach out in front,
        // push off behind" silhouette.
        let hip_fwd = (0.55 * anim_weight + 0.75 * sprint_blend) * swing_mult;
        let hip_back = (0.40 * anim_weight + 0.55 * sprint_blend) * swing_mult;

        // Slight outward splay during swing, mostly visible at sprint.
        let splay_amp = 0.01 * anim_weight + 0.02 * sprint_blend;

        // Knee bend peaks in mid-swing (knee drives up) and mid-stance
        // (shin hangs down so it forms a horizontal "_" with the up-angled
        // "/" thigh). The knee extends toward both edges (heel-strike and
        // toe-off) so the leg forms a "\" out front and pushes off cleanly
        // behind.
        let knee_swing_peak = (0.75 * anim_weight + 0.90 * sprint_blend) * swing_mult;
        let knee_stance_peak = (0.90 * anim_weight + 1.30 * sprint_blend) * swing_mult;
        let knee_stance_flex = 0.06 * anim_weight + 0.10 * sprint_blend;

        let knee_stance_min = if is_crouching { 0.55 } else { knee_stance_flex };

        // Ankle: a real foot goes through heel-strike (dorsiflexed) → foot-flat
        // → toe-off (plantarflexed) during stance, and neutral/slightly dorsiflexed
        // during swing for ground clearance.
        let ankle_heel_strike = 0.20 * anim_weight + 0.15 * sprint_blend;
        let ankle_toe_off = 0.45 * anim_weight + 0.55 * sprint_blend;
        let ankle_swing_dorsi = 0.20 * anim_weight + 0.25 * sprint_blend;

        // Thigh pitch: driven by cos(phase) so the extremes land at heel-strike
        // (forward, c=+1) and toe-off (back, c=-1). Mid-swing and mid-stance
        // pass through vertical (c=0). Combined with the knee/ankle below, this
        // gives the leg a "\" silhouette at heel-strike and a "_/" silhouette
        // through mid/late stance.
        let thigh_pitch = |_s: f32, c: f32| -> f32 {
            let amp = if c >= 0.0 { hip_fwd } else { hip_back };
            let base_pitch = -c * amp;
            if is_crouching {
                base_pitch + 0.42
            } else {
                base_pitch
            }
        };

        // Knee bend: a "double bell" peaking at s=+1 (mid-swing) and s=-1
        // (mid-stance). |s| is already a natural bell peaking at ±1 with zeros
        // at s=0, so we just shape it with a power < 1 (fast rise, flat top).
        //   s = 0  (heel-strike / toe-off): bend ≈ 0 → leg straight ("\" / push-off)
        //   s = +1 (mid-swing):              bend = peak → knee drives up
        //   s = -1 (mid-stance):             bend = peak → shin hangs horizontal ("_/")
        let knee_bend = |s: f32, _c: f32| -> f32 {
            let raw = s.abs().powf(0.7);
            if s >= 0.0 {
                raw * knee_swing_peak
            } else {
                // Small constant flex on top of the bell so the stance leg
                // isn't perfectly straight at the transitions.
                (raw * knee_stance_peak + knee_stance_flex * 0.4).max(knee_stance_min)
            }
        };

        // Ankle pitch: heel-strike (early stance) → foot-flat (mid stance) →
        // toe-off (late stance) → neutral (swing).
        let ankle_pitch = |s: f32, c: f32| -> f32 {
            if s >= 0.0 {
                // Swing: keep foot slightly dorsiflexed (toes up) so the toes
                // clear the ground. Tilt up further at the very end of swing
                // (c≈+1, heel-strike) for a natural landing prep.
                ankle_swing_dorsi * s + 0.15 * sprint_blend * c.max(0.0)
            } else {
                // Stance: heel-strike has the foot angled up (dorsiflexed),
                // then rolls flat, then pushes off on the toes (plantarflexed).
                let stance_progress = -s; // 0..1 across stance
                // Bell-curve toe-off peaking near the end of stance.
                let toe_off = (stance_progress * std::f32::consts::PI).sin() * ankle_toe_off;
                // Initial heel-strike dorsiflexion fades out.
                let heel_strike = (1.0 - stance_progress).powi(2) * ankle_heel_strike;
                toe_off + heel_strike
            }
        };

        if let Some(mut tf) = l_thigh.iter_mut().next() {
            tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, thigh_pitch(ls, lc), splay_amp);
        }
        if let Some(mut tf) = l_shin.iter_mut().next() {
            tf.rotation = Quat::from_rotation_x(-knee_bend(ls, lc));
        }
        if let Some(mut tf) = l_foot.iter_mut().next() {
            tf.rotation = Quat::from_rotation_x(ankle_pitch(ls, lc));
        }

        if let Some(mut tf) = r_thigh.iter_mut().next() {
            tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, thigh_pitch(rs, rc), -splay_amp);
        }
        if let Some(mut tf) = r_shin.iter_mut().next() {
            tf.rotation = Quat::from_rotation_x(-knee_bend(rs, rc));
        }
        if let Some(mut tf) = r_foot.iter_mut().next() {
            tf.rotation = Quat::from_rotation_x(ankle_pitch(rs, rc));
        }

        // Arms swing in antiphase to the same-side leg (left arm forward when
        // right leg is forward). Humans naturally bend the elbow more when
        // the arm swings forward (carrying motion) and straighten it on the
        // back-swing.
        let upper_hang = if is_crouching { 0.12 } else { 0.08 };
        let forearm_hang = if is_crouching { 0.28 } else { 0.18 };

        let swing_range = (0.30 * anim_weight + 0.55 * sprint_blend) * swing_mult;
        let forearm_swing_amp = (0.20 * anim_weight + 0.45 * sprint_blend) * swing_mult;
        let forearm_base = forearm_hang + 0.05 * anim_weight + 0.10 * sprint_blend;

        // Left arm antiphase to left leg.
        let la_s = -ls;
        if let Some(mut tf) = l_upper_arm.iter_mut().next() {
            // Smooth, slightly skewed swing using a power curve to mimic the
            // pendulum's natural ease-in/ease-out.
            let swing_norm = if la_s >= 0.0 {
                la_s.powf(0.85)
            } else {
                -(-la_s).powf(0.85)
            };
            let pitch = upper_hang + swing_norm * swing_range;
            // Slight outward swing of the elbow (z-rotation) when arm goes back.
            let roll = -0.04 * anim_weight * la_s.max(0.0);
            tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, pitch, roll);
        }
        if let Some(mut tf) = l_forearm.iter_mut().next() {
            // Elbow bends more on forward swing (carrying motion).
            let elbow_bend = if la_s >= 0.0 {
                la_s * forearm_swing_amp
            } else {
                0.0
            };
            tf.rotation = Quat::from_rotation_x(forearm_base + elbow_bend);
        }

        // Right arm antiphase to right leg.
        let ra_s = -rs;
        if let Some(mut tf) = r_upper_arm.iter_mut().next() {
            let swing_norm = if ra_s >= 0.0 {
                ra_s.powf(0.85)
            } else {
                -(-ra_s).powf(0.85)
            };
            let pitch = upper_hang + swing_norm * swing_range;
            let roll = 0.04 * anim_weight * ra_s.max(0.0);
            tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, pitch, roll);
        }
        if let Some(mut tf) = r_forearm.iter_mut().next() {
            let elbow_bend = if ra_s >= 0.0 {
                ra_s * forearm_swing_amp
            } else {
                0.0
            };
            tf.rotation = Quat::from_rotation_x(forearm_base + elbow_bend);
        }
    } else {
        let vertical_speed = velocity.0.y;
        let leap_factor = (vertical_speed / settings.jump_velocity).clamp(-1.0, 1.0);

        if leap_factor > 0.0 {
            let blend = leap_factor;

            if let Some(mut tf) = torso.iter_mut().next() {
                tf.translation.y = 0.92;
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
                tf.translation.y = 0.92;
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
