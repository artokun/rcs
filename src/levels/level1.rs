use avian3d::prelude::*;
use bevy::{color::palettes::css::GOLD, prelude::*};

use crate::{components::LevelComponents, resources::LoadingData, Ship, Target};

pub fn load_level_1(mut commands: Commands) {
    commands.register_one_shot_system(setup_ship);
    commands.register_one_shot_system(setup_target);
    commands.register_one_shot_system(setup_planet_scene);
}

fn setup_ship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dimensions = Vec3::new(1.0, 1.0, 2.0);
    commands.spawn((
        Ship,
        Name::new("Ship"),
        LevelComponents,
        RigidBody::Dynamic,
        Collider::cuboid(dimensions.x, dimensions.y, dimensions.z),
        ExternalImpulse::new(Vec3::new(0.0, 0.0, 0.0)),
        ExternalAngularImpulse::new(Vec3::new(0.0, 0.0, 0.0)),
        AngularDamping(0.0),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(dimensions.x, dimensions.y, dimensions.z)),
            material: materials.add(StandardMaterial {
                base_color: GOLD.into(),
                metallic: 1.0,
                reflectance: 0.5,
                perceptual_roughness: 0.2,
                ..default()
            }),
            ..default()
        },
        SleepingDisabled,
    ));
}

fn setup_target(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Target,
        Name::new("Target"),
        LevelComponents,
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.5, 0.0),
                ..default()
            }),
            transform: Transform::from_xyz(5.0, 5.0, -20.0),
            ..default()
        },
    ));
}

fn setup_planet_scene(
    mut commands: Commands,
    mut loading_data: ResMut<LoadingData>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let planet_scene = asset_server
        .load(GltfAssetLabel::Scene(0).from_asset("models/planet_of_phoenix/scene.gltf"));
    loading_data
        .loading_assets
        .push(planet_scene.clone().into());

    commands
        .spawn((
            Name::new("Planet"),
            LevelComponents,
            SceneBundle {
                scene: planet_scene,
                transform: Transform::from_xyz(0.0, 0.0, -6000.0)
                    .with_scale(Vec3::new(1000.0, 1000.0, 1000.0)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Atmosphere"),
                PbrBundle {
                    mesh: meshes.add(Sphere::new(1.75).mesh().ico(15).unwrap()),
                    material: materials.add(StandardMaterial {
                        alpha_mode: AlphaMode::Blend,
                        base_color: Color::srgba(0.1, 0.2, 0.5, 0.3),
                        perceptual_roughness: 1.0,
                        metallic: 0.0,
                        double_sided: true,
                        reflectance: 0.5,
                        ..default()
                    }),
                    ..default()
                },
            ));
        });
}
