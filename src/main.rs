use avian2d::prelude::*;
use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    render::camera::ScalingMode,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_inspector_egui::quick::{FilterQueryInspectorPlugin, WorldInspectorPlugin};
use rand::Rng;
use std::collections::HashMap;

const CHUNK_SIZE: f32 = 1200.0;
const PARTICLES_PER_CHUNK: usize = 500;
const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 2.5;

#[derive(Component)]
struct Player {
    thrust: f32,
    torque: f32,
}

#[allow(dead_code)]
#[derive(Component)]
struct SpaceDustChunk {
    chunk_coords: IVec2,
}

#[derive(Component)]
struct FPSText;

#[derive(Component)]
struct AttitudeText;

#[derive(Resource)]
struct ChunkManager {
    loaded_chunks: HashMap<IVec2, Entity>,
}

fn setup_resources(mut commands: Commands) {
    commands.insert_resource(ChunkManager {
        loaded_chunks: HashMap::new(),
    });
    commands.insert_resource(Gravity(Vec2::ZERO));
}

fn setup_graphics(mut commands: Commands) {
    let camera_2d_bundle = Camera2dBundle {
        projection: OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            scaling_mode: ScalingMode::FixedVertical(600.0),
            ..default()
        },
        camera: Camera {
            // hdr: true, // HDR is required for the bloom effect
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        ..default()
    };
    commands.spawn((
        camera_2d_bundle,
        // BloomSettings::NATURAL,
    ));

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

fn zoom_scale(
    mut query_camera: Query<&mut OrthographicProjection, With<Camera2d>>,
    mut wheel_event: EventReader<MouseWheel>,
) {
    let mut projection = query_camera.single_mut();
    let mut zoom_level = 0.0;
    for ev in wheel_event.read() {
        match ev.unit {
            MouseScrollUnit::Line => zoom_level += ev.y,
            MouseScrollUnit::Pixel => zoom_level += ev.y / 120.0,
        }
    }
    projection.scale = (projection.scale / 1.1f32.powf(zoom_level)).clamp(MIN_ZOOM, MAX_ZOOM);
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let transform = Transform::default();
    /* Create the player's cargo tug. */
    commands.spawn((
        Player {
            thrust: 100.0,
            torque: 100.0,
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

fn update_ui(
    positions: Query<(&Transform, &LinearVelocity), With<Player>>,
    mut text_query: Query<&mut Text, With<AttitudeText>>,
) {
    for (transform, linvel) in positions.iter() {
        if let Ok(mut text) = text_query.get_single_mut() {
            let angle = transform.rotation.to_euler(EulerRot::XYZ).2.to_degrees();
            let angle_360 = (angle + 360.0) % 360.0;
            let velocity_magnitude = linvel.length();

            text.sections[0].value = format!(
                "BRG: {:.2} deg\nVREL: {:.2} m/s\nXVEL: {:.2} m/s\nYVEL: {:.2} m/s",
                angle_360, velocity_magnitude, linvel.x, linvel.y
            );
        }
    }
}

fn update_fps(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FPSText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

fn apply_force_to_player(
    mut player_query: Query<
        (
            &Player,
            &mut ExternalImpulse,
            &mut ExternalAngularImpulse,
            &Transform,
        ),
        With<Player>,
    >,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    for (player, mut impulse, mut angular_impulse, transform) in player_query.iter_mut() {
        let mut force = Vec2::ZERO;
        let mut torque = 0.0;
        if kb_input.pressed(KeyCode::KeyW) {
            force.y += player.thrust;
        }
        if kb_input.pressed(KeyCode::KeyS) {
            force.y -= player.thrust;
        }
        if kb_input.pressed(KeyCode::KeyA) {
            force.x -= player.thrust;
        }
        if kb_input.pressed(KeyCode::KeyD) {
            force.x += player.thrust;
        }
        if kb_input.pressed(KeyCode::KeyQ) {
            torque += player.torque;
        }
        if kb_input.pressed(KeyCode::KeyE) {
            torque -= player.torque;
        }
        if kb_input.pressed(KeyCode::KeyR) {
            println!("R pressed");
        }

        let rotated_force = transform.rotation * force.extend(0.0);
        impulse.apply_impulse(rotated_force.truncate());
        angular_impulse.apply_impulse(torque * 2.0);
    }
}

fn update_camera(
    mut camera_transform: Query<&mut Transform, With<Camera>>,
    player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_trans = camera_transform.single_mut();
    let playertrans = player_transform.single().translation.truncate();
    let camtrans = camera_trans.translation.truncate();
    camera_trans.translation = camtrans.lerp(playertrans, 0.1).extend(999.0);
}

fn manage_chunks(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let player_transform = player_query.single();
    let current_chunk = IVec2::new(
        (player_transform.translation.x / CHUNK_SIZE).floor() as i32,
        (player_transform.translation.y / CHUNK_SIZE).floor() as i32,
    );

    // Spawn new chunks
    for dy in -1..=1 {
        for dx in -1..=1 {
            let chunk_coords = current_chunk + IVec2::new(dx, dy);
            if !chunk_manager.loaded_chunks.contains_key(&chunk_coords) {
                spawn_chunk(&mut commands, chunk_coords, &mut chunk_manager);
            }
        }
    }

    // Despawn distant chunks
    let mut to_remove = Vec::new();
    for (chunk_coords, &entity) in chunk_manager.loaded_chunks.iter() {
        if (*chunk_coords - current_chunk).abs().max_element() > 1 {
            commands.entity(entity).despawn_recursive();
            to_remove.push(*chunk_coords);
        }
    }
    for coords in to_remove {
        chunk_manager.loaded_chunks.remove(&coords);
    }
}

fn spawn_chunk(commands: &mut Commands, chunk_coords: IVec2, chunk_manager: &mut ChunkManager) {
    let mut rng = rand::thread_rng();
    let chunk_world_position = Vec2::new(
        chunk_coords.x as f32 * CHUNK_SIZE,
        chunk_coords.y as f32 * CHUNK_SIZE,
    );

    let chunk_entity = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(chunk_world_position.extend(0.0)),
                ..default()
            },
            SpaceDustChunk { chunk_coords },
        ))
        .with_children(|parent| {
            for _ in 0..PARTICLES_PER_CHUNK {
                let x = rng.gen_range(0.0..CHUNK_SIZE);
                let y = rng.gen_range(0.0..CHUNK_SIZE);
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(1.0, 1.0, 1.0, rng.gen_range(0.01..0.1)),
                        custom_size: Some(Vec2::new(
                            rng.gen_range(1.0..3.0),
                            rng.gen_range(1.0..3.0),
                        )),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                });
            }
        })
        .id();

    chunk_manager
        .loaded_chunks
        .insert(chunk_coords, chunk_entity);
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "RCS".to_string(),
                    resolution: (400., 600.).into(),
                    position: WindowPosition::At(IVec2::new(0, 0)),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default().with_length_unit(20.0),
            PhysicsDebugPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            WorldInspectorPlugin::default(),
        ))
        .add_systems(Startup, (setup_resources, setup_graphics, setup_physics))
        .add_systems(FixedUpdate, apply_force_to_player)
        .add_systems(
            FixedPostUpdate,
            (
                update_camera,
                update_ui,
                update_fps,
                manage_chunks,
                zoom_scale,
            ),
        )
        .run();
}
