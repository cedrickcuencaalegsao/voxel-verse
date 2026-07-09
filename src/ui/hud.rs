use crate::inventory::inventory::Inventory;
use crate::player::camera::PlayerCamera;
use crate::world::world_manager::WorldManager;
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;

#[derive(Component)]
pub struct HotbarSlot(pub usize);

#[derive(Component)]
pub enum ArmSlot {
    Left,
    Right,
}

#[derive(Component)]
pub enum ArmorSlot {
    Helmet,
    Chestplate,
    Leggings,
    Boots,
}

#[derive(Component)]
pub struct CrosshairLine;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct MinimapContainer {
    pub expanded: bool,
}

#[derive(Component)]
pub struct MinimapImage(pub Handle<Image>);

pub fn setup_hud(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // ── HUD STATS DISPLAY (FPS Meter) ───────────────────────────────────────
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            GlobalZIndex(100),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("FPS: 0.0"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.0)),
                FpsText,
            ));
        });

    // ── INTERACTIVE MINIMAP ──────────────────────────────────────────────────
    let image = Image::new_fill(
        bevy::render::render_resource::Extent3d {
            width: 32,
            height: 32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[50, 100, 150, 255], // default ocean color
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    let minimap_handle = images.add(image);

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                width: Val::Px(120.0),
                height: Val::Px(120.0),
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BorderColor::all(Color::srgb(0.2, 0.2, 0.2)),
            MinimapContainer { expanded: false },
            GlobalZIndex(100),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ImageNode::new(minimap_handle.clone()),
                MinimapImage(minimap_handle),
            ));
        });

    // ── ADVANCED HOTBAR (Arm, Hotbar, and Armor Slots) ───────────────────────
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            GlobalZIndex(100),
        ))
        .with_children(|parent| {
            // ── LEFT SIDE: ARM SLOTS ──
            parent
                .spawn(Node {
                    margin: UiRect::right(Val::Px(15.0)), // Separation margin
                    ..default()
                })
                .with_children(|arms| {
                    // Left Arm (e.g. Shield/Offhand)
                    arms.spawn((
                        Node {
                            width: Val::Px(48.0),
                            height: Val::Px(48.0),
                            border: UiRect::all(Val::Px(2.0)),
                            margin: UiRect::horizontal(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.4, 0.2, 0.2)),
                        BackgroundColor(Color::srgb(0.2, 0.1, 0.1)),
                        ArmSlot::Left,
                    ))
                    .with_children(|slot| {
                        slot.spawn((
                            Text::new("L"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.6, 0.4, 0.4)),
                        ));
                    });

                    // Right Arm (e.g. Weapon/Mainhand)
                    arms.spawn((
                        Node {
                            width: Val::Px(48.0),
                            height: Val::Px(48.0),
                            border: UiRect::all(Val::Px(2.0)),
                            margin: UiRect::horizontal(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.2, 0.4, 0.2)),
                        BackgroundColor(Color::srgb(0.1, 0.2, 0.1)),
                        ArmSlot::Right,
                    ))
                    .with_children(|slot| {
                        slot.spawn((
                            Text::new("R"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.4, 0.6, 0.4)),
                        ));
                    });
                });

            // ── CENTER: HOTBAR (9 Slots) ──
            parent.spawn(Node::default()).with_children(|hotbar| {
                for i in 0..9 {
                    hotbar.spawn((
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

            // ── RIGHT SIDE: ARMOR SLOTS ──
            parent
                .spawn(Node {
                    margin: UiRect::left(Val::Px(15.0)), // Separation margin
                    ..default()
                })
                .with_children(|armor| {
                    let slots = [
                        (ArmorSlot::Helmet, "H"),
                        (ArmorSlot::Chestplate, "C"),
                        (ArmorSlot::Leggings, "L"),
                        (ArmorSlot::Boots, "B"),
                    ];

                    for (slot_type, label) in slots {
                        armor
                            .spawn((
                                Node {
                                    width: Val::Px(48.0),
                                    height: Val::Px(48.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    margin: UiRect::horizontal(Val::Px(2.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor::all(Color::srgb(0.3, 0.3, 0.4)),
                                BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
                                slot_type,
                            ))
                            .with_children(|slot| {
                                slot.spawn((
                                    Text::new(label),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.5, 0.5, 0.7)),
                                ));
                            });
                    }
                });
        });

    // ── CENTER CROSSHAIR NODE ────────────────────────────────────────────────
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
            // Vertical Line
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

// ── FPS METER SYSTEM ─────────────────────────────────────────────────────────

pub fn update_fps(
    time: Res<Time>,
    mut last_update: Local<f32>,
    mut text_query: Query<&mut Text, With<FpsText>>,
) {
    let dt = time.delta_secs();
    if dt == 0.0 {
        return;
    }

    *last_update += dt;
    if *last_update >= 0.25 {
        *last_update = 0.0;
        let fps = 1.0 / dt;
        for mut text in text_query.iter_mut() {
            text.0 = format!("FPS: {:.1}", fps);
        }
    }
}

// ── MINIMAP SYSTEMS ──────────────────────────────────────────────────────────

/// Toggles the minimap's dimensions from small (120px) to expanded (300px) when pressing M.
pub fn toggle_minimap(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut MinimapContainer, &mut Node)>,
) {
    if keys.just_pressed(KeyCode::KeyM) {
        if let Ok((mut container, mut node)) = query.single_mut() {
            container.expanded = !container.expanded;
            if container.expanded {
                node.width = Val::Px(300.0);
                node.height = Val::Px(300.0);
            } else {
                node.width = Val::Px(120.0);
                node.height = Val::Px(120.0);
            }
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

pub fn update_minimap(
    player_query: Query<&Transform, With<crate::world::world_manager::Player>>,
    world: Res<WorldManager>,
    minimap_query: Query<&MinimapImage>,
    mut images: ResMut<Assets<Image>>,
) {
    let Ok(player_tf) = player_query.single() else {
        return;
    };
    let Ok(minimap) = minimap_query.single() else {
        return;
    };
    let Some(image) = images.get_mut(&minimap.0) else {
        return;
    };

    let px = player_tf.translation.x;
    let pz = player_tf.translation.z;
    let py = player_tf.translation.y;

    let Some(ref mut data) = image.data else {
        return;
    };

    for pixel_z in 0..32u32 {
        for pixel_x in 0..32u32 {
            let world_x = px + (pixel_x as f32 - 16.0);
            let world_z = pz + (pixel_z as f32 - 16.0);

            let bx = world_x.floor() as i32;
            let bz = world_z.floor() as i32;

            let ground_y = get_block_ground_y(&world, world_x, world_z);

            // Height shading: higher terrain is slightly brighter
            let height_diff = (ground_y - py).clamp(-8.0, 8.0) / 8.0; // -1.0 to 1.0
            let shade = (height_diff + 1.0) * 0.5 * 0.35 + 0.65; // 0.65–1.0

            // Trees only grow on grass surfaces (above SEA_LEVEL + 2 = 64)
            let is_grass_surface = ground_y > 65.0;
            let is_tree = is_grass_surface && world.generator.should_spawn_tree(bx, bz);

            let color: [u8; 4] = if is_tree {
                // Dark forest green
                [
                    (18.0 * shade) as u8,
                    (90.0 * shade) as u8,
                    (18.0 * shade) as u8,
                    255,
                ]
            } else if ground_y <= 65.0 {
                // Sand / Beach
                [
                    (160.0 * shade) as u8,
                    (130.0 * shade) as u8,
                    (80.0 * shade) as u8,
                    255,
                ]
            } else if ground_y < 90.0 {
                // Grass / Plains
                [
                    (40.0 * shade) as u8,
                    (155.0 * shade) as u8,
                    (45.0 * shade) as u8,
                    255,
                ]
            } else if ground_y < 130.0 {
                // Hills — olive green fading toward grey
                [
                    (80.0 * shade) as u8,
                    (120.0 * shade) as u8,
                    (60.0 * shade) as u8,
                    255,
                ]
            } else {
                // Mountains / Stone
                let v = (150.0 * shade) as u8;
                [v, v, v, 255]
            };

            let idx = ((pixel_z * 32 + pixel_x) * 4) as usize;
            data[idx] = color[0];
            data[idx + 1] = color[1];
            data[idx + 2] = color[2];
            data[idx + 3] = color[3];
        }
    }

    // Red player dot at center (16, 16)
    let center = (16 * 32 + 16) * 4;
    data[center] = 255;
    data[center + 1] = 0;
    data[center + 2] = 0;
    data[center + 3] = 255;
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
        (origin.z - x as f32) * t_delta_z
    };

    for _ in 0..64 {
        let ground_y = get_block_ground_y(world, x as f32, z as f32) as i32;
        if y < ground_y {
            return Some(crate::world::block::BlockKind::Grass);
        }

        if t_max_x < t_max_y {
            if t_max_x < t_max_z {
                x += step_x;
                t_max_x += t_delta_x;
            } else {
                z += step_z;
                t_max_z += t_delta_z;
            }
        } else {
            if t_max_y < t_max_z {
                y += step_y;
                t_max_y += t_delta_y;
            } else {
                z += step_z;
                t_max_z += t_delta_z;
            }
        }

        let dist_sq = (Vec3::new(x as f32, y as f32, z as f32) - origin).length_squared();
        if dist_sq > max_distance * max_distance {
            break;
        }
    }
    None
}

pub fn update_crosshair(
    camera_query: Query<&Transform, With<PlayerCamera>>,
    mut crosshair_query: Query<&mut BackgroundColor, With<CrosshairLine>>,
    world: Res<WorldManager>,
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
        _ => Color::srgb(0.5, 0.7, 1.0),
    };

    let r = block_color.to_linear().red;
    let g = block_color.to_linear().green;
    let b = block_color.to_linear().blue;
    let luminance = 0.299 * r + 0.587 * g + 0.114 * b;

    let crosshair_color = if luminance > 0.45 {
        Color::srgb(0.1, 0.1, 0.1)
    } else {
        Color::srgb(0.9, 0.9, 0.9)
    };

    for mut bg in crosshair_query.iter_mut() {
        *bg = BackgroundColor(crosshair_color);
    }
}
