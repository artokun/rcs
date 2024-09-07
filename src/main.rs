use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{
    asset::LoadState,
    core_pipeline::Skybox,
    prelude::*,
    render::{
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        renderer::RenderDevice,
        texture::CompressedImageFormats,
    },
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

#[derive(Component)]
struct Ship;

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    index: usize,
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
            brightness: 1.0,
        })
        .add_systems(Startup, (setup_camera, setup_light, setup_ship))
        .add_systems(Update, asset_loaded)
        .add_systems(FixedPostUpdate, cam_follow)
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
        AngularVelocity(Vec3::ZERO),
        LinearVelocity(Vec3::ZERO),
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
            brightness: 1000.0,
        },
    ));

    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
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
