use crate::components::Player;
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn apply_force_to_player(
    mut player_query: Query<
        (
            &Player,
            &mut ExternalImpulse,
            &mut ExternalAngularImpulse,
            &Transform,
            &AngularVelocity,
        ),
        With<Player>,
    >,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    for (player, mut impulse, mut angular_impulse, transform, angvel) in player_query.iter_mut() {
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
            // Calculate the angular velocity and apply a counter-torque
            let base_torque = -angvel.0 * player.torque;
            let boost_factor = 1.0 + (1.0 - angvel.0.abs().min(1.0));
            torque = (base_torque * boost_factor).clamp(-player.torque, player.torque);
        }

        let rotated_force = transform.rotation * force.extend(0.0);
        let clamped_force = rotated_force.truncate().clamp_length_max(player.thrust);
        impulse.apply_impulse(clamped_force);

        let clamped_torque = torque.clamp(-player.torque * 2.0, player.torque * 2.0);
        angular_impulse.apply_impulse(clamped_torque);
    }
}
