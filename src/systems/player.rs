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
