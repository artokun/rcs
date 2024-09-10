use std::f32::consts::PI;

#[cfg(debug_assertions)]
use avian3d::debug_render::PhysicsDebugPlugin;
use avian3d::{
    prelude::{
        AngularDamping, Collider, ColliderDensity, ExternalAngularImpulse, ExternalImpulse,
        Gravity, Inertia, LinearVelocity, Mass, RigidBody, SleepingDisabled,
    },
    PhysicsPlugins,
};
#[cfg(debug_assertions)]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{
    asset::LoadState,
    color::palettes::css::GOLD,
    core_pipeline::Skybox,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

#[derive(Component)]
pub struct Ship;
#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct AttitudeText;

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    image_handle: Handle<Image>,
}
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
            #[cfg(debug_assertions)]
            PhysicsDebugPlugin::default(),
            #[cfg(debug_assertions)]
            FrameTimeDiagnosticsPlugin,
            #[cfg(debug_assertions)]
            WorldInspectorPlugin::new(),
        ))
        .insert_resource(Gravity(Vec3::ZERO))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 10.0,
        })
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_light,
                setup_ship,
                setup_target,
                setup_ui,
            ),
        )
        .add_systems(PreUpdate, asset_loaded)
        .add_systems(FixedUpdate, update_controls)
        .add_systems(FixedPostUpdate, (cam_follow, update_ui))
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
        ExternalImpulse::new(Vec3::new(0.0, 0.0, 0.0)),
        ExternalAngularImpulse::new(Vec3::new(0.0, 0.0, 0.0)),
        AngularDamping(0.0),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(dimensions.x, dimensions.y, dimensions.z)),
            material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
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
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.5, 0.0),
                emissive: LinearRgba::rgb(5.0, 15.0, 13.0),
                ..default()
            }),
            transform: Transform::from_xyz(5.0, 5.0, -20.0),
            ..default()
        },
    ));
}

fn setup_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: light_consts::lux::FULL_DAYLIGHT,
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

fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    let skybox_handle = asset_server.load("cubemap/starmap.ktx2");

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
        Skybox {
            image: skybox_handle.clone(),
            brightness: 500.0,
        },
    ));

    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: Handle::<Image>::default(),
    })
}

fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle) == LoadState::Loaded {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.image = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}

fn cam_follow(mut pan_orbit_q: Query<&mut PanOrbitCamera>, ship_q: Query<&Transform, With<Ship>>) {
    if let Ok(mut pan_orbit) = pan_orbit_q.get_single_mut() {
        if let Ok(ship_tfm) = ship_q.get_single() {
            pan_orbit.target_focus = ship_tfm.translation;
            pan_orbit.force_update = true;
        }
    }
}

pub fn setup_ui(mut commands: Commands) {
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

pub fn update_ui(
    ship_q: Query<(&Transform, &LinearVelocity), With<Ship>>,
    target_q: Query<(&Transform, &LinearVelocity), With<Target>>,
    mut text_query: Query<&mut Text, With<AttitudeText>>,
) {
    if let (Ok((ship_transform, ship_linvel)), Ok((target_transform, target_linvel))) =
        (ship_q.get_single(), target_q.get_single())
    {
        if let Ok(mut text) = text_query.get_single_mut() {
            let relative_pos = target_transform.translation - ship_transform.translation;
            let range = relative_pos.length();

            // Calculate azimuth (horizontal angle)
            let local_relative_pos = ship_transform.rotation.inverse() * relative_pos;
            let mut az = local_relative_pos
                .x
                .atan2(-local_relative_pos.z)
                .to_degrees();
            az = if az > 180.0 {
                az - 360.0
            } else if az < -180.0 {
                az + 360.0
            } else {
                az
            };

            // Calculate elevation (vertical angle)
            let el = (-local_relative_pos.y)
                .atan2((-local_relative_pos.z).hypot(local_relative_pos.x))
                .to_degrees();

            let v_rel = target_linvel.0 - ship_linvel.0;
            let local_v_rel = ship_transform.rotation.inverse() * v_rel;

            // Calculate cross-track velocities in ship's local frame
            let v_crs_y = local_v_rel.y;
            let v_crs_x = local_v_rel.x;

            // Improved ETA calculation
            let closing_velocity = -v_rel.dot(relative_pos.normalize());
            let eta = if closing_velocity.abs() > 0.1 {
                range / closing_velocity
            } else {
                f32::INFINITY
            };

            text.sections[0].value = format!(
                "RNG: {:.2} m\nAZ: {:.2}deg\nEL: {:.2}deg\nVREL: {:.2} m/s\nVCRS_Y: {:.2} m/s\nVCRS_X: {:.2} m/s\nETA: {:.2} s",
                range,
                az,
                el,
                v_rel.length(),
                v_crs_y,
                v_crs_x,
                eta
            );
        }
    }
}

fn update_controls(
    mut ship_q: Query<
        (
            &Transform,
            &mut ExternalImpulse,
            &mut ExternalAngularImpulse,
            &mut AngularDamping,
        ),
        With<Ship>,
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let strength = 0.01;
    let mut force_vec = Vec3::ZERO;
    let mut torque_vec = Vec3::ZERO;
    if let Ok((transform, mut lin_impulse, mut ang_impulse, mut damping)) = ship_q.get_single_mut()
    {
        if keyboard_input.pressed(KeyCode::KeyW) {
            force_vec.z -= strength;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            force_vec.z += strength;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            force_vec.x -= strength;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            force_vec.x += strength;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            force_vec.y += strength;
        }
        if keyboard_input.pressed(KeyCode::ControlLeft) {
            force_vec.y -= strength;
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            torque_vec.z -= strength;
        }
        if keyboard_input.pressed(KeyCode::KeyQ) {
            torque_vec.z += strength;
        }
        if keyboard_input.pressed(KeyCode::KeyR) {
            damping.0 = 5.0;
        } else if keyboard_input.just_released(KeyCode::KeyR) {
            damping.0 = 0.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            torque_vec.x -= strength;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            torque_vec.x += strength;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            torque_vec.y += strength;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            torque_vec.y -= strength;
        }
        let rotated_force = transform.rotation * force_vec;
        let rotated_torque = transform.rotation * torque_vec;
        lin_impulse.apply_impulse(rotated_force.clamp_length_max(strength));
        ang_impulse.apply_impulse(rotated_torque.clamp_length_max(strength));
    }
}
