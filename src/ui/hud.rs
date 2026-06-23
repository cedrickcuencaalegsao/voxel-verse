use crate::inventory::inventory::Inventory;
use crate::player::camera::PlayerCamera;
use crate::world::world_manager::WorldManager;
use bevy::prelude::*;

#[derive(Component)]
pub struct HotbarSlot(pub usize);

#[derive(Component)]
pub struct CrosshairLine;

pub fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Percent(50.0),
                width: Val::Auto,
                height: Val::Auto,
                ..default()
            },
            GlobalZIndex(100),
        ))
        .with_children(|parent| {
            for i in 0..9 {
                parent.spawn((
                    Node {
                        width: Val::Px(48.0),
                        height: Val::Px(48.0),
                        border: UiRect::all(Val::Px(2.0)),
                        margin: UiRect::horizontal(Val::Px(2.0)),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                    BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                    HotbarSlot(i),
                ));
            }
        });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                width: Val::Px(0.0),
                height: Val::Px(0.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            GlobalZIndex(200),
        ))
        .with_children(|parent| {
            // Horizontal Line
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(14.0),
                    height: Val::Px(2.0),
                    left: Val::Px(-7.0),
                    top: Val::Px(-1.0),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
                CrosshairLine,
            ));
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(2.0),
                    height: Val::Px(14.0),
                    left: Val::Px(-1.0),
                    top: Val::Px(-7.0),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
                CrosshairLine,
            ));
        });
}

pub fn update_hotbar(
    inventory: Res<Inventory>,
    mut query: Query<(&HotbarSlot, &mut BackgroundColor)>,
) {
    for (slot, mut bg) in query.iter_mut() {
        if let Some(item) = &inventory.hotbar()[slot.0] {
            let color = match item.item {
                crate::world::block::BlockKind::Stone => Color::srgb(0.5, 0.5, 0.5),
                crate::world::block::BlockKind::Dirt => Color::srgb(0.6, 0.4, 0.2),
                crate::world::block::BlockKind::Grass => Color::srgb(0.2, 0.8, 0.2),
                _ => Color::srgb(1.0, 1.0, 1.0),
            };
            *bg = BackgroundColor(color.with_alpha(item.count as f32 / 64.0));
        } else {
            *bg = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
        }
    }
}

fn get_block_ground_y(world: &WorldManager, x: f32, z: f32) -> f32 {
    let bx = x.floor() as f64;
    let bz = z.floor() as f64;
    let surface = if bx * bx + bz * bz <= 40.0 * 40.0 {
        world.generator.spawn_height_at(bx, bz)
    } else {
        world.generator.height_at(bx, bz)
    };
    surface.floor() as f32 + 1.0
}

fn raycast_voxels(
    origin: Vec3,
    direction: Vec3,
    max_distance: f32,
    world: &WorldManager,
) -> Option<crate::world::block::BlockKind> {
    if direction.length_squared() < 0.001 {
        return None;
    }

    let mut x = origin.x.floor() as i32;
    let mut y = origin.y.floor() as i32;
    let mut z = origin.z.floor() as i32;

    let step_x = if direction.x > 0.0 { 1 } else { -1 };
    let step_y = if direction.y > 0.0 { 1 } else { -1 };
    let step_z = if direction.z > 0.0 { 1 } else { -1 };

    let t_delta_x = (1.0 / direction.x).abs();
    let t_delta_y = (1.0 / direction.y).abs();
    let t_delta_z = (1.0 / direction.z).abs();

    let mut t_max_x = if direction.x > 0.0 {
        (x as f32 + 1.0 - origin.x) * t_delta_x
    } else {
        (origin.x - x as f32) * t_delta_x
    };
    let mut t_max_y = if direction.y > 0.0 {
        (y as f32 + 1.0 - origin.y) * t_delta_y
    } else {
        (origin.y - y as f32) * t_delta_y
    };
    let mut t_max_z = if direction.z > 0.0 {
        (z as f32 + 1.0 - origin.z) * t_delta_z
    } else {
        (origin.z - z as f32) * t_delta_z
    };

    let mut t = 0.0;
    while t < max_distance {
        let ground_y = get_block_ground_y(world, x as f32, z as f32) - 1.0;
        let y_f = y as f32;

        if y_f <= ground_y {
            if y_f == ground_y {
                return Some(crate::world::block::BlockKind::Grass);
            } else if y_f >= ground_y - 3.0 {
                return Some(crate::world::block::BlockKind::Dirt);
            } else {
                return Some(crate::world::block::BlockKind::Stone);
            }
        }

        if t_max_x < t_max_y {
            if t_max_x < t_max_z {
                x += step_x;
                t = t_max_x;
                t_max_x += t_delta_x;
            } else {
                z += step_z;
                t = t_max_z;
                t_max_z += t_delta_z;
            }
        } else {
            if t_max_y < t_max_z {
                y += step_y;
                t = t_max_y;
                t_max_y += t_delta_y;
            } else {
                z += step_z;
                t = t_max_z;
                t_max_z += t_delta_z;
            }
        }
    }
    None
}

pub fn update_crosshair(
    camera_query: Query<&Transform, With<PlayerCamera>>,
    world: Res<WorldManager>,
    mut crosshair_query: Query<&mut BackgroundColor, With<CrosshairLine>>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let origin = camera_transform.translation;
    let direction = *camera_transform.forward();

    let pointed_block = raycast_voxels(origin, direction, 15.0, &world);

    let block_color = match pointed_block {
        Some(crate::world::block::BlockKind::Grass) => Color::srgb(0.2, 0.8, 0.2),
        Some(crate::world::block::BlockKind::Dirt) => Color::srgb(0.6, 0.4, 0.2),
        Some(crate::world::block::BlockKind::Stone) => Color::srgb(0.5, 0.5, 0.5),
        _ => Color::srgb(0.5, 0.7, 1.0), // sky color
    };

    let r = block_color.to_linear().red;
    let g = block_color.to_linear().green;
    let b = block_color.to_linear().blue;
    let luminance = 0.299 * r + 0.587 * g + 0.114 * b;

    // Use dark crosshair on bright targets, and light crosshair on dark targets
    let crosshair_color = if luminance > 0.45 {
        Color::srgb(0.1, 0.1, 0.1) // near black
    } else {
        Color::srgb(0.9, 0.9, 0.9) // near white
    };

    for mut bg in crosshair_query.iter_mut() {
        *bg = BackgroundColor(crosshair_color);
    }
}
