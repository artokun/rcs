mod components;
mod constants;
mod resources;
mod systems;

use avian2d::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use constants::DEBUG;

use crate::systems::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "RCS".to_string(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default().with_length_unit(20.0),
            PhysicsDebugPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            WorldInspectorPlugin::default().run_if(|| DEBUG),
        ))
        .add_systems(
            Startup,
            (
                setup::setup_resources,
                setup::setup_graphics,
                setup::setup_physics,
            ),
        )
        .add_systems(FixedUpdate, player::apply_force_to_player)
        .add_systems(
            FixedPostUpdate,
            (
                camera::update_camera,
                ui::update_ui,
                ui::update_fps,
                space_dust::space_dust,
                camera::zoom_scale,
            ),
        )
        .run();
}
