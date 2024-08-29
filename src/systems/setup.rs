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
    /* Create the player's cargo tug. */
    commands.spawn((
        Player {
            thrust: 500.0,
            torque: 500.0,
        },
        RigidBody::Dynamic,
        Collider::rectangle(20.0, 30.0),
        Mass(20.0 * 30.0),
        Inertia(1.0),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(20.0, 30.0))),
            material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
            transform,
            ..default()
        },
    ));
}
