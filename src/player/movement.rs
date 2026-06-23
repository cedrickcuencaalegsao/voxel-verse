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

    // Dynamic stride speed
    let frequency = 6.5 + 5.5 * sprint_blend;
    let phase = t * frequency;

    if is_grounded {
        // ── 1. TORSO (Forward Lean + Reduced Sway & Bobbing) ─────────────────
        let forward_lean = 0.10 * anim_weight + 0.22 * sprint_blend;
        // Negative pitch = forward lean in Bevy (+X rotation tilts the top backward).
        let torso_pitch = -forward_lean - 0.02 * anim_weight * phase.sin() * 0.5;
        let torso_yaw = (0.03 * anim_weight + 0.05 * sprint_blend) * phase.sin();
        let torso_roll = -(0.02 * anim_weight + 0.03 * sprint_blend)
            * (phase + std::f32::consts::FRAC_PI_2).sin();
        let torso_bob = (-0.025 * anim_weight - 0.05 * sprint_blend) * (2.0 * phase).cos().abs();

        if let Some(mut tf) = torso.iter_mut().next() {
            tf.translation.y = 0.75 + torso_bob;
            tf.rotation = Quat::from_euler(EulerRot::YXZ, torso_yaw, torso_pitch, torso_roll);
        }

        // ── 2. HEAD (Secondary Motion lag) ───────────────────────────────────
        if let Some(mut tf) = head.iter_mut().next() {
            // Positive pitch here counteracts the torso's forward lean, keeping the head upright.
            let head_pitch = forward_lean * 0.7 + (0.02 * anim_weight) * (2.0 * phase).sin();
            let head_yaw = -torso_yaw * 0.5;
            let head_roll = -torso_roll * 0.4;
            tf.rotation = Quat::from_euler(EulerRot::YXZ, head_yaw, head_pitch, head_roll);
        }

        // ── 3. LEGS (Biomechanically Accurate Running Gait) ───────────────────
        //
        // Human running gait — per-leg phase drives three joints together:
        //
        //   Phase angle relative to leg (left = `phase`, right = `phase + π`):
        //
        //   sin → +1  : leg at peak FORWARD swing
        //               · Thigh pitched max forward
        //               · Knee bent high (foot clearing ground)
        //               · Foot dorsiflexed (toes up, ready to strike)
        //
        //   sin →  0, cos → -1 : FOOT STRIKE / early stance
        //               · Thigh near neutral, leg extending downward
        //               · Knee softly bent to absorb impact
        //               · Foot neutral / slight plantarflexion at contact
        //
        //   sin → -1  : leg at full EXTENSION behind body (push-off)
        //               · Thigh pitched max backward (hip extension)
        //               · Knee nearly straight — leg is a rigid lever
        //               · Ankle plantarflexes hard (toes push off ground)
        //
        //   sin →  0, cos → +1 : TOE-OFF / start of swing
        //               · Knee re-bends rapidly to clear foot from ground
        //               · Foot flicks up (rebound dorsiflex)
        //               · Thigh begins swinging forward again

        let hip_fwd = 0.48 * anim_weight + 0.52 * sprint_blend; // forward hip pitch
        let hip_back = 0.32 * anim_weight + 0.42 * sprint_blend; // backward hip extension
        let splay = 0.03 * anim_weight + 0.05 * sprint_blend;

        // Knee ROM: high lift during swing, near-zero at stance, re-bend at toe-off
        let knee_swing_peak = 0.75 * anim_weight + 0.65 * sprint_blend;
        let knee_toe_off = 0.50 * anim_weight + 0.45 * sprint_blend;
        let knee_stance_min = 0.05_f32; // never fully locked straight

        // Ankle ROM
        let ankle_dorsiflex = 0.20 * anim_weight + 0.15 * sprint_blend; // foot up on swing
        let ankle_plantarflex = -0.35 * anim_weight - 0.30 * sprint_blend; // foot down on push-off

        // ── Per-leg helper closures ───────────────────────────────────────────

        // Thigh pitch: asymmetric — more forward ROM than backward (natural gait)
        let thigh_pitch = |s: f32| -> f32 {
            if s >= 0.0 {
                -s * hip_fwd // forward swing
            } else {
                -s * hip_back // hip extension (pushes backward less than it swings forward)
            }
        };

        // Knee bend: peaks during swing, near-zero at stance, spikes at toe-off.
        // `s` = sin of leg phase, `c` = cos of leg phase
        //   · Swing phase (s > 0, c going toward -1): c.max(0) drives the lift.
        //   · Toe-off (s transitioning from -1 back toward 0, c near 0 going positive):
        //     we detect this as s ∈ (-1, 0) and c < 0.3, adding an extra burst.
        let knee_bend = |s: f32, c: f32| -> f32 {
            // Primary lift: knee bends when leg swings FORWARD (s > 0).
            // Use -c.max(0) because cos is -1 at peak forward swing (s=+1),
            // and near +1 when the leg is behind — so we flip it.
            let swing = (-c).max(0.0) * knee_swing_peak;

            // Toe-off burst: leg just left the ground (s crossing 0 from negative,
            // c crossing from +1 toward 0). Knee snaps up to clear the foot.
            let in_toe_off_zone = ((-s).clamp(0.0, 1.0)) * ((1.0 - c.abs()).max(0.0));
            let toe_off = in_toe_off_zone * knee_toe_off;

            // At stance (s near 0, c near -1): both terms are ~0 → near-straight leg.
            (swing + toe_off).max(knee_stance_min)
        };

        // Ankle pitch — human gait:
        //
        //   s =  1  (front reach): PLANTARFLEX — foot pointed/stretched down
        //           toward ground, reaching toe forward before heel-strike.
        //   s ~  0  (mid-stance):  NEUTRAL — foot flat, ankle near 0.
        //   s = -1  (push-off):    PLANTARFLEX — ankle drives hard into ground.
        //   s ~  0, c = +1 (toe-off): DORSIFLEX — toes pull up to clear ground.
        //
        // s² drives a constant plantarflex at both extremes (front AND back).
        // A narrow dorsiflex burst fires only at the toe-off transition.
        let ankle_pitch = |s: f32, c: f32| -> f32 {
            // Both front reach (s=+1) and push-off (s=-1) point the foot down.
            let stretch = -(s * s) * ankle_plantarflex.abs();

            // Dorsiflex burst at toe-off: s just crossed 0 from negative, c near +1.
            let in_toe_off = (-s).clamp(0.0, 1.0) * c.max(0.0);
            let pull_up = in_toe_off * ankle_dorsiflex;

            stretch + pull_up
        };

        // ── Left leg ─────────────────────────────────────────────────────────
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

        // ── Right leg (half-cycle offset) ────────────────────────────────────
        let rs = (phase + std::f32::consts::PI).sin(); // == -ls
        let rc = (phase + std::f32::consts::PI).cos(); // == -lc

        if let Some(mut tf) = r_thigh.iter_mut().next() {
            tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, thigh_pitch(rs), -splay);
        }
        if let Some(mut tf) = r_shin.iter_mut().next() {
            tf.rotation = Quat::from_rotation_x(-knee_bend(rs, rc));
        }
        if let Some(mut tf) = r_foot.iter_mut().next() {
            tf.rotation = Quat::from_rotation_x(ankle_pitch(rs, rc));
        }

        // ── 4. ARMS ───────────────────────────────────────────────────────────
        //
        // Idle:   arm hangs at side, small forward cant, forearm with slight droop.
        // Walk:   elbow begins to lift; gentle pendulum arc.
        // Sprint: full running pump — elbow bent ~90°, forearm sweeps from face-
        //         height on the forward swing to extended-back on the back-swing.
        //
        //   Forward:   /     ← upper arm pitches forward, forearm tucked up (~90° bend)
        //             /
        //
        //   Back:   \__      ← upper arm pitches back, forearm extends horizontal
        //
        // Counter-swing: left arm forward when right leg forward.
        //   la_s = -phase.sin()   ra_s = phase.sin()

        let upper_hang = 0.08_f32; // rest-pose forward cant of upper arm
        let forearm_hang = 0.18_f32; // rest-pose: small natural elbow flex (positive = up toward face)

        // Elbow bend base — positive = bent up toward face in this rig.
        // Grows with speed so the arm holds a deeper bend at sprint.
        let forearm_base = forearm_hang
            + 0.15 * anim_weight     // walk: elbow lifts slightly
            + 0.25 * sprint_blend; // sprint: elbow fully bent up (~90°)

        // Upper arm swing arc: small at walk, large at sprint.
        let swing_range = 0.18 * anim_weight + 0.42 * sprint_blend;

        // Forearm sweep: ADD on forward swing (tucks up toward face),
        // SUBTRACT on back-swing (extends back / opens the elbow).
        let forearm_sweep = 0.35 * anim_weight + 0.70 * sprint_blend;

        let la_s = -phase.sin();
        if let Some(mut tf) = l_upper_arm.iter_mut().next() {
            let pitch = upper_hang + la_s * swing_range;
            tf.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, pitch, 0.06 * anim_weight);
        }
        if let Some(mut tf) = l_forearm.iter_mut().next() {
            // la_s > 0 (forward swing) → + sweep → more positive → forearm tucked up toward face
            // la_s < 0 (back swing)    → - sweep → less/negative  → forearm extends back
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
        // ── 5. JUMP & FALL (Velocity-Driven Pose) ────────────────────────────
        let vertical_speed = velocity.0.y;
        let leap_factor = (vertical_speed / settings.jump_velocity).clamp(-1.0, 1.0);

        if leap_factor > 0.0 {
            let blend = leap_factor;

            if let Some(mut tf) = torso.iter_mut().next() {
                tf.translation.y = 0.75;
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
                tf.rotation = Quat::from_rotation_x(0.15 * blend); // slight dorsiflex on jump
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
                tf.translation.y = 0.75;
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
                tf.rotation = Quat::from_rotation_x(-0.20 * blend); // plantarflex on fall
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
