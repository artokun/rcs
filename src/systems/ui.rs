use avian3d::prelude::LinearVelocity;
use bevy::{color::palettes::css::GOLD, prelude::*};

use crate::{AttitudeText, Ship, Target};

pub fn setup_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "No target",
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
