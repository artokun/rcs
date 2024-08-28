use crate::components::{AttitudeText, FPSText, Player};
use avian2d::prelude::LinearVelocity;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub fn update_ui(
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

pub fn update_fps(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FPSText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
