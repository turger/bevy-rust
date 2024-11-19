use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::constants::PLAYER_VELOCITY_X;

pub fn movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    let mut player = query.single_mut();
    let mut translation = Vec2::new(0.0, 0.0);

    if input.pressed(KeyCode::ArrowRight) {
        translation.x += time.delta_seconds() * PLAYER_VELOCITY_X;
    }

    if input.pressed(KeyCode::ArrowLeft) {
        translation.x += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
    }

    player.translation = Some(translation);
}
