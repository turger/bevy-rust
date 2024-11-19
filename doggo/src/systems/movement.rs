use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::{
    constants::{GRAVITY_REDUCED, JUMP_VELOCITY, LEFT_BOUNDARY, PLAYER_VELOCITY_X, RIGHT_BOUNDARY},
    player::Player,
};

pub fn movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut KinematicCharacterController, &mut Player, &Transform)>,
) {
    let (mut player_controller, mut player, transform) = query.single_mut();
    let mut translation = Vec2::new(0.0, 0.0);

    if !player.on_ground {
        player.vertical_velocity += GRAVITY_REDUCED * time.delta_seconds();
    } else {
        player.vertical_velocity = 0.0;
    }

    if input.pressed(KeyCode::ArrowRight) {
        translation.x += time.delta_seconds() * PLAYER_VELOCITY_X;
    }

    if input.pressed(KeyCode::ArrowLeft) {
        translation.x += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
    }

    if input.just_pressed(KeyCode::Space) && player.on_ground {
        player.vertical_velocity += JUMP_VELOCITY;
        player.on_ground = false;
    }

    translation.y += player.vertical_velocity * time.delta_seconds();

    let new_position =
        player_controller.translation.unwrap_or_default() + transform.translation.xy();
    // println!("New Position: {}, Left Boundary: {}", new_position.x, LEFT_BOUNDARY);
    // prevent player from moving left of the left boundary
    if new_position.x < LEFT_BOUNDARY {
        translation.x = LEFT_BOUNDARY - new_position.x;
    }
    // prevent player from moving right of the right boundary
    if new_position.x > RIGHT_BOUNDARY {
        translation.x = RIGHT_BOUNDARY - new_position.x;
    }
    player_controller.translation = Some(translation);
}
