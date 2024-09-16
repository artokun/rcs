#[cfg(debug_assertions)]
use avian3d::debug_render::PhysicsDebugPlugin;
use avian3d::{prelude::Gravity, PhysicsPlugins};
#[cfg(debug_assertions)]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{core_pipeline::Skybox, pbr::DirectionalLightShadowMap, prelude::*};
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

mod components;
mod levels;
mod loading;
mod pipelines_ready;
mod resources;
mod systems;

use pipelines_ready::*;
use resources::{LoadingData, LoadingState};

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
            PipelinesReadyPlugin,
            PanOrbitCameraPlugin,
            PhysicsPlugins::default(),
            #[cfg(debug_assertions)]
            PhysicsDebugPlugin::default(),
            #[cfg(debug_assertions)]
            FrameTimeDiagnosticsPlugin,
            #[cfg(debug_assertions)]
            WorldInspectorPlugin::new(),
        ))
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .insert_resource(Gravity(Vec3::ZERO))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 10.0,
        })
        .insert_resource(LoadingData::new(3))
        .insert_resource(LoadingState::default())
        .add_systems(
            Startup,
            (
                systems::light::setup_light,
                systems::camera::setup_camera,
                systems::ui::setup_ui,
                loading::setup_levels,
                loading::load_loading_screen,
            ),
        )
        .add_systems(PreUpdate, systems::cubemap::asset_loaded)
        .add_systems(
            Update,
            (
                loading::update_loading_data,
                loading::level_selection,
                loading::display_loading_screen,
            ),
        )
        .add_systems(FixedUpdate, systems::controller::update_controls)
        .add_systems(
            FixedPostUpdate,
            (systems::camera::cam_follow, systems::ui::update_ui),
        )
        .run();
}
