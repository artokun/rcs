use std::collections::HashMap;

use crate::components::*;
use crate::resources::*;
use avian2d::prelude::*;
use bevy::color::palettes::css::GOLD;
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
            Mass(ship_dimensions.x * ship_dimensions.y),
            Inertia(1.0),
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
            ); // Left thruster
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
            RCSThrusterMountAlignment::Center => match mount.orientation {
                _ => Vec2::new(-parent_dimensions.x / 2.0, 0.0),
            },
            RCSThrusterMountAlignment::Start => match mount.orientation {
                RCSThrusterMountOrientation::Horizontal => Vec2::new(
                    -parent_dimensions.x / 2.0,
                    -parent_dimensions.y / 2.0 + thruster_dimensions.y / 2.0,
                ),
                RCSThrusterMountOrientation::Vertical => Vec2::new(
                    -parent_dimensions.x / 2.0 + thruster_dimensions.x / 2.0,
                    -parent_dimensions.y / 2.0,
                ),
            },
            RCSThrusterMountAlignment::End => match mount.orientation {
                RCSThrusterMountOrientation::Horizontal => Vec2::new(
                    -parent_dimensions.x / 2.0,
                    parent_dimensions.y / 2.0 - thruster_dimensions.y / 2.0,
                ),
                RCSThrusterMountOrientation::Vertical => Vec2::new(
                    -parent_dimensions.x / 2.0 + thruster_dimensions.x / 2.0,
                    parent_dimensions.y / 2.0,
                ),
            },
        },
        RCSThrusterMountPosition::Right => match mount.alignment {
            RCSThrusterMountAlignment::Center => match mount.orientation {
                _ => Vec2::new(parent_dimensions.x / 2.0, 0.0),
            },
            RCSThrusterMountAlignment::Start => match mount.orientation {
                RCSThrusterMountOrientation::Horizontal => Vec2::new(
                    parent_dimensions.x / 2.0,
                    -parent_dimensions.y / 2.0 + thruster_dimensions.y / 2.0,
                ),
                RCSThrusterMountOrientation::Vertical => Vec2::new(
                    parent_dimensions.x / 2.0 - thruster_dimensions.x / 2.0,
                    -parent_dimensions.y / 2.0,
                ),
            },
            RCSThrusterMountAlignment::End => match mount.orientation {
                RCSThrusterMountOrientation::Horizontal => Vec2::new(
                    parent_dimensions.x / 2.0,
                    parent_dimensions.y / 2.0 - thruster_dimensions.y / 2.0,
                ),
                RCSThrusterMountOrientation::Vertical => Vec2::new(
                    parent_dimensions.x / 2.0 - thruster_dimensions.x / 2.0,
                    parent_dimensions.y / 2.0,
                ),
            },
        },
        RCSThrusterMountPosition::Top => match mount.alignment {
            RCSThrusterMountAlignment::Center => match mount.orientation {
                _ => Vec2::new(0.0, parent_dimensions.y / 2.0),
            },
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
            RCSThrusterMountAlignment::Center => match mount.orientation {
                _ => Vec2::new(0.0, -parent_dimensions.y / 2.0),
            },
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
        Collider::rectangle(thruster_dimensions.x, thruster_dimensions.y),
        Mass(thruster_dimensions.x * thruster_dimensions.y / 5.0),
        Inertia(1.0 / 5.0),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(Rectangle::new(thruster_dimensions.x, thruster_dimensions.y)),
            ),
            material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
            transform: Transform::from_translation(ship_mount.extend(0.0)),
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
