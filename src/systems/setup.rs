use core::f32;
use std::collections::HashMap;
use std::f32::EPSILON;

use crate::components::*;
use crate::resources::*;
use avian2d::prelude::*;
use bevy::color::palettes::css::GOLD;
use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub fn setup_resources(mut commands: Commands) {
    commands.insert_resource(ChunkManager {
        loaded_chunks: HashMap::new(),
    });
    commands.insert_resource(Gravity(Vec2::ZERO));
}

pub fn setup_graphics(mut commands: Commands) {
    // Camera setup
    let camera_2d_bundle = Camera2dBundle {
        projection: OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            scaling_mode: ScalingMode::FixedVertical(600.0),
            ..default()
        },
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        ..default()
    };
    commands.spawn(camera_2d_bundle);

    // FPS Text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 20.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 20.0,
                color: GOLD.into(),
                ..default()
            }),
        ]),
        FPSText,
    ));

    // Attitude Text
    commands.spawn((
        TextBundle::from_section(
            "Loading",
            TextStyle {
                font_size: 20.0,
                color: GOLD.into(),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        AttitudeText,
    ));
}

pub fn setup_controls(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Controls:\nW,A,S,D to strafe\nQ and E to rotate\nR to stop rotation",
            TextStyle {
                font_size: 20.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(15.0),
            left: Val::Px(15.0),
            ..default()
        }),
        ControlsText,
    ));
}

pub fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let transform = Transform::default();
    let ship_dimensions = Vec2::new(20.0, 30.0);
    /* Create the player's cargo tug. */
    commands
        .spawn((
            Name::new("Player"),
            Player {
                thrust: 500.0,
                torque: 500.0,
            },
            RigidBody::Dynamic,
            Collider::rectangle(ship_dimensions.x, ship_dimensions.y),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(Rectangle::new(ship_dimensions.x, ship_dimensions.y)),
                ),
                material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
                transform,
                ..default()
            },
        ))
        .with_children(|parent| {
            // Draw Arrow
            parent.spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Triangle2d::new(
                    Vec2::ZERO,
                    Vec2::new(-ship_dimensions.x / 4.0, -ship_dimensions.y / 4.0),
                    Vec2::new(ship_dimensions.x / 4.0, -ship_dimensions.y / 4.0),
                ))),
                material: materials.add(ColorMaterial::from(Color::srgba(0.0, 0.0, 0.0, 0.7))),
                transform: Transform::from_xyz(0.0, ship_dimensions.y / 2.0, 0.1),
                ..default()
            });

            // Top Left
            spawn_thruster(
                parent,
                &mut meshes,
                &mut materials,
                ship_dimensions,
                Vec2::new(4.0, 4.0),
                RCSThrusterMount {
                    position: RCSThrusterMountPosition::Left,
                    orientation: RCSThrusterMountOrientation::Horizontal,
                    alignment: RCSThrusterMountAlignment::Start,
                },
            );

            // Top Right
            spawn_thruster(
                parent,
                &mut meshes,
                &mut materials,
                ship_dimensions,
                Vec2::new(4.0, 4.0),
                RCSThrusterMount {
                    position: RCSThrusterMountPosition::Right,
                    orientation: RCSThrusterMountOrientation::Horizontal,
                    alignment: RCSThrusterMountAlignment::Start,
                },
            );

            // Bottom Left
            spawn_thruster(
                parent,
                &mut meshes,
                &mut materials,
                ship_dimensions,
                Vec2::new(4.0, 4.0),
                RCSThrusterMount {
                    position: RCSThrusterMountPosition::Left,
                    orientation: RCSThrusterMountOrientation::Horizontal,
                    alignment: RCSThrusterMountAlignment::End,
                },
            );

            // Bottom Right
            spawn_thruster(
                parent,
                &mut meshes,
                &mut materials,
                ship_dimensions,
                Vec2::new(4.0, 4.0),
                RCSThrusterMount {
                    position: RCSThrusterMountPosition::Right,
                    orientation: RCSThrusterMountOrientation::Horizontal,
                    alignment: RCSThrusterMountAlignment::End,
                },
            );
        });
}

fn spawn_thruster(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    parent_dimensions: Vec2,
    thruster_dimensions: Vec2,
    mount: RCSThrusterMount,
) {
    let thruster_mount = match mount.position {
        RCSThrusterMountPosition::Left => Vec2::new(thruster_dimensions.x / 2.0, 0.0),
        RCSThrusterMountPosition::Right => Vec2::new(-thruster_dimensions.x / 2.0, 0.0),
        RCSThrusterMountPosition::Top => Vec2::new(0.0, thruster_dimensions.y / 2.0),
        RCSThrusterMountPosition::Bottom => Vec2::new(0.0, -thruster_dimensions.y / 2.0),
    };
    let ship_mount = match mount.position {
        RCSThrusterMountPosition::Left => match mount.alignment {
            RCSThrusterMountAlignment::Center => Vec2::new(-parent_dimensions.x / 2.0, 0.0),
            RCSThrusterMountAlignment::Start => match mount.orientation {
                RCSThrusterMountOrientation::Horizontal => Vec2::new(
                    -parent_dimensions.x / 2.0,
                    parent_dimensions.y / 2.0 - thruster_dimensions.y / 2.0,
                ),
                RCSThrusterMountOrientation::Vertical => Vec2::new(
                    -parent_dimensions.x / 2.0 + thruster_dimensions.x,
                    parent_dimensions.y / 2.0 + thruster_dimensions.y / 2.0,
                ),
            },
            RCSThrusterMountAlignment::End => match mount.orientation {
                RCSThrusterMountOrientation::Horizontal => Vec2::new(
                    -parent_dimensions.x / 2.0,
                    -parent_dimensions.y / 2.0 + thruster_dimensions.y / 2.0,
                ),
                RCSThrusterMountOrientation::Vertical => Vec2::new(
                    -parent_dimensions.x / 2.0 + thruster_dimensions.x,
                    -parent_dimensions.y / 2.0 - thruster_dimensions.y / 2.0,
                ),
            },
        },
        RCSThrusterMountPosition::Right => match mount.alignment {
            RCSThrusterMountAlignment::Center => Vec2::new(parent_dimensions.x / 2.0, 0.0),
            RCSThrusterMountAlignment::Start => match mount.orientation {
                RCSThrusterMountOrientation::Horizontal => Vec2::new(
                    parent_dimensions.x / 2.0,
                    parent_dimensions.y / 2.0 - thruster_dimensions.y / 2.0,
                ),
                RCSThrusterMountOrientation::Vertical => Vec2::new(
                    parent_dimensions.x / 2.0 - thruster_dimensions.x,
                    parent_dimensions.y / 2.0 + thruster_dimensions.y / 2.0,
                ),
            },
            RCSThrusterMountAlignment::End => match mount.orientation {
                RCSThrusterMountOrientation::Horizontal => Vec2::new(
                    parent_dimensions.x / 2.0,
                    -parent_dimensions.y / 2.0 + thruster_dimensions.y / 2.0,
                ),
                RCSThrusterMountOrientation::Vertical => Vec2::new(
                    parent_dimensions.x / 2.0 - thruster_dimensions.x,
                    -parent_dimensions.y / 2.0 - thruster_dimensions.y / 2.0,
                ),
            },
        },
        RCSThrusterMountPosition::Top => match mount.alignment {
            RCSThrusterMountAlignment::Center => Vec2::new(0.0, parent_dimensions.y / 2.0),
            RCSThrusterMountAlignment::Start => match mount.orientation {
                RCSThrusterMountOrientation::Horizontal => Vec2::new(
                    -parent_dimensions.x / 2.0 + thruster_dimensions.x / 2.0,
                    parent_dimensions.y / 2.0,
                ),
                RCSThrusterMountOrientation::Vertical => Vec2::new(
                    -parent_dimensions.x / 2.0,
                    parent_dimensions.y / 2.0 - thruster_dimensions.y / 2.0,
                ),
            },
            RCSThrusterMountAlignment::End => match mount.orientation {
                RCSThrusterMountOrientation::Horizontal => Vec2::new(
                    parent_dimensions.x / 2.0 - thruster_dimensions.x / 2.0,
                    parent_dimensions.y / 2.0,
                ),
                RCSThrusterMountOrientation::Vertical => Vec2::new(
                    parent_dimensions.x / 2.0,
                    parent_dimensions.y / 2.0 - thruster_dimensions.y / 2.0,
                ),
            },
        },
        RCSThrusterMountPosition::Bottom => match mount.alignment {
            RCSThrusterMountAlignment::Center => Vec2::new(0.0, -parent_dimensions.y / 2.0),
            RCSThrusterMountAlignment::Start => match mount.orientation {
                RCSThrusterMountOrientation::Horizontal => Vec2::new(
                    -parent_dimensions.x / 2.0 + thruster_dimensions.x / 2.0,
                    -parent_dimensions.y / 2.0,
                ),
                RCSThrusterMountOrientation::Vertical => Vec2::new(
                    -parent_dimensions.x / 2.0,
                    -parent_dimensions.y / 2.0 + thruster_dimensions.y / 2.0,
                ),
            },
            RCSThrusterMountAlignment::End => match mount.orientation {
                RCSThrusterMountOrientation::Horizontal => Vec2::new(
                    parent_dimensions.x / 2.0 - thruster_dimensions.x / 2.0,
                    -parent_dimensions.y / 2.0,
                ),
                RCSThrusterMountOrientation::Vertical => Vec2::new(
                    parent_dimensions.x / 2.0,
                    -parent_dimensions.y / 2.0 + thruster_dimensions.y / 2.0,
                ),
            },
        },
    };

    let thruster = parent.spawn((
        Name::new(format!(
            "{:?}_{:?}_{:?}",
            mount.alignment, mount.orientation, mount.position
        )),
        RCSThruster {
            mount,
            active: false,
        },
        RigidBody::Dynamic,
        Collider::rectangle(thruster_dimensions.x * 0.9, thruster_dimensions.y * 0.9),
        ColliderDensity(0.1),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(Rectangle::new(thruster_dimensions.x, thruster_dimensions.y)),
            ),
            material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
            transform: Transform::from_translation(
                ship_mount.extend(0.0) - thruster_mount.extend(0.0),
            ),
            ..default()
        },
    ));
    let thruster_entity = thruster.id();
    let ship_entity = parent.parent_entity();
    parent.spawn(
        FixedJoint::new(ship_entity, thruster_entity)
            .with_local_anchor_1(ship_mount)
            .with_local_anchor_2(thruster_mount),
    );
}
