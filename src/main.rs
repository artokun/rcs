use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

#[derive(Component)]
struct Ship;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "RCS".to_string(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: true,
                    ..default()
                }),
                ..default()
            }),
            PanOrbitCameraPlugin,
            PhysicsPlugins::default(),
            #[cfg(not(feature = "production"))]
            PhysicsDebugPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            #[cfg(not(feature = "production"))]
            WorldInspectorPlugin::new(),
        ))
        .insert_resource(Gravity(Vec3::ZERO))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 15.0,
        })
        .add_systems(Startup, (setup_camera, setup_light, setup_ship))
        .add_systems(
            FixedPostUpdate,
            cam_follow, // .after(PhysicsSet::Sync)
                        // .before(TransformSystem::TransformPropagate),
        )
        .run();
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
        RigidBody::Dynamic,
        Collider::cuboid(dimensions.x, dimensions.y, dimensions.z),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(dimensions.x, dimensions.y, dimensions.z)),
            material: materials.add(Color::WHITE),
            ..default()
        },
        AngularVelocity(Vec3::ONE),
        LinearVelocity(Vec3::ONE),
        SleepingDisabled,
    ));
}

fn setup_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            pan_sensitivity: 0.0,
            pan_smoothness: 0.0,
            ..default()
        },
    ));
}

fn cam_follow(mut pan_orbit_q: Query<&mut PanOrbitCamera>, ship_q: Query<&Transform, With<Ship>>) {
    if let Ok(mut pan_orbit) = pan_orbit_q.get_single_mut() {
        if let Ok(ship_tfm) = ship_q.get_single() {
            pan_orbit.target_focus = ship_tfm.translation;
            pan_orbit.force_update = true;
        }
    }
}
