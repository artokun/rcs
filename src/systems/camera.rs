use crate::components::Player;
use crate::constants::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

pub fn update_camera(
    mut camera_transform: Query<&mut Transform, With<Camera>>,
    player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_trans = camera_transform.single_mut();
    let playertrans = player_transform.single().translation.truncate();
    let camtrans = camera_trans.translation.truncate();
    camera_trans.translation = camtrans.lerp(playertrans, 0.1).extend(999.0);
}

pub fn zoom_scale(
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
