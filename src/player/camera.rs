use crate::world::world_manager::Player;
use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

#[derive(Component)]
pub struct PlayerCamera {
    pub distance: f32,
    pub target_height: f32,
    pub pitch: f32,
    pub sensitivity: f32,
    pub mode: CameraMode,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            distance: 10.0,
            target_height: 1.1,
            pitch: -0.3,
            sensitivity: 0.003,
            mode: CameraMode::ThirdPersonBack,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    ThirdPersonBack,
    ThirdPersonFront,
    FirstPerson,
}

/// Marker for the player's visible body mesh
#[derive(Component)]
pub struct PlayerBody;

/// Locks and hides the OS cursor on startup.
pub fn grab_cursor(mut cursor_options: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = cursor_options.single_mut() {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }
}

/// Re-grabs cursor when player clicks the window.
pub fn handle_cursor_auto_grab(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut cursor_options: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        if let Ok(mut cursor) = cursor_options.single_mut() {
            if cursor.grab_mode == CursorGrabMode::None {
                cursor.grab_mode = CursorGrabMode::Locked;
                cursor.visible = false;
            }
        }
    }
}

/// Escape frees the cursor; pressing again re-locks it.
pub fn toggle_cursor_grab(
    keys: Res<ButtonInput<KeyCode>>,
    mut cursor_options: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }
    let Ok(mut cursor) = cursor_options.single_mut() else {
        return;
    };
    let locked = cursor.grab_mode != CursorGrabMode::None;
    if locked {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    } else {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }
}

/// Tab cycles: behind → front → first person → behind.
pub fn cycle_camera_view(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut PlayerCamera>,
) {
    if !keys.just_pressed(KeyCode::Tab) {
        return;
    }
    let Ok(mut camera) = camera_query.single_mut() else {
        return;
    };
    camera.mode = match camera.mode {
        CameraMode::ThirdPersonBack => CameraMode::ThirdPersonFront,
        CameraMode::ThirdPersonFront => CameraMode::FirstPerson,
        CameraMode::FirstPerson => CameraMode::ThirdPersonBack,
    };
}

pub fn player_look(
    mouse_motion: Res<AccumulatedMouseMotion>,
    cursor_options: Query<&CursorOptions, With<PrimaryWindow>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &mut PlayerCamera), Without<Player>>,
    mut body_query: Query<&mut Visibility, With<PlayerBody>>,
) {
    let Ok(mut player_transform) = player_query.single_mut() else {
        return;
    };
    let Ok((mut camera_transform, mut camera)) = camera_query.single_mut() else {
        return;
    };

    let cursor_locked = cursor_options
        .single()
        .map(|c| c.grab_mode != CursorGrabMode::None)
        .unwrap_or(false);

    if cursor_locked {
        let delta = mouse_motion.delta;
        if delta != Vec2::ZERO {
            player_transform.rotate_y(-delta.x * camera.sensitivity);
            camera.pitch = (camera.pitch - delta.y * camera.sensitivity).clamp(-1.54, 1.54);
        }
    }

    let target = player_transform.translation + Vec3::Y * camera.target_height;
    let look_rotation = player_transform.rotation * Quat::from_rotation_x(camera.pitch);

    match camera.mode {
        CameraMode::ThirdPersonBack => {
            camera_transform.translation =
                target + look_rotation * Vec3::new(0.0, 0.0, camera.distance);
            camera_transform.look_at(target, Vec3::Y);
        }
        CameraMode::ThirdPersonFront => {
            camera_transform.translation =
                target + look_rotation * Vec3::new(0.0, 0.0, -camera.distance);
            camera_transform.look_at(target, Vec3::Y);
        }
        CameraMode::FirstPerson => {
            camera_transform.translation = target;
            camera_transform.rotation = look_rotation;
        }
    }

    if let Ok(mut visibility) = body_query.single_mut() {
        *visibility = if camera.mode == CameraMode::FirstPerson {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}
