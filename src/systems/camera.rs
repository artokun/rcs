use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping, Skybox},
    prelude::*,
};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::{components::Ship, resources::Cubemap};

pub fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    let skybox_handle = asset_server.load("cubemap/starmap.ktx2");

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(1.0, 1.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings::NATURAL,
        PanOrbitCamera {
            pan_sensitivity: 0.0,
            pan_smoothness: 0.0,
            ..default()
        },
        Skybox {
            image: skybox_handle.clone(),
            brightness: 500.0,
        },
        EnvironmentMapLight {
            diffuse_map: skybox_handle.clone(),
            specular_map: skybox_handle.clone(),
            intensity: 1000.0,
        },
    ));

    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: Handle::<Image>::default(),
    })
}

pub fn cam_follow(
    mut pan_orbit_q: Query<&mut PanOrbitCamera>,
    ship_q: Query<&Transform, With<Ship>>,
) {
    if let Ok(mut pan_orbit) = pan_orbit_q.get_single_mut() {
        if let Ok(ship_tfm) = ship_q.get_single() {
            pan_orbit.target_focus = ship_tfm.translation;
            pan_orbit.force_update = true;
        }
    }
}
