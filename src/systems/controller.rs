use avian3d::prelude::{AngularDamping, ExternalAngularImpulse, ExternalImpulse};
use bevy::prelude::*;

use crate::components::Ship;

pub fn update_controls(
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
