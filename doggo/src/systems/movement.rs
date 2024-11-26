use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::{
    constants::{GRAVITY_REDUCED, JUMP_VELOCITY, LEFT_BOUNDARY, PLAYER_VELOCITY_X, RIGHT_BOUNDARY},
    player_sprite::PlayerSprite,
};

pub fn movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut sprite_query: Query<(&mut KinematicCharacterController, &mut PlayerSprite, &mut Transform)>,
) {
    // Handle PlayerSprite movement
    for (mut sprite_controller, mut player_sprite, mut transform) in sprite_query.iter_mut() {
        let mut translation = Vec2::new(0.0, 0.0);

        if !player_sprite.on_ground {
            player_sprite.vertical_velocity += GRAVITY_REDUCED * time.delta_seconds();
        } else {
            player_sprite.vertical_velocity = 0.0; // Reset vertical velocity when on ground
        }

        if input.pressed(KeyCode::ArrowRight) {
            translation.x += time.delta_seconds() * PLAYER_VELOCITY_X;
            player_sprite.facing_right = true;
        }

        if input.pressed(KeyCode::ArrowLeft) {
            translation.x += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
            player_sprite.facing_right = false;
        }

        if input.just_pressed(KeyCode::Space) && player_sprite.on_ground {
            player_sprite.vertical_velocity += JUMP_VELOCITY;
            player_sprite.on_ground = false;
        }

        translation.y += player_sprite.vertical_velocity * time.delta_seconds();

        // Boundary checks for PlayerSprite
        let new_position = sprite_controller.translation.unwrap_or_default() + translation;
        // println!("New Position: {}, Left Boundary: {}", new_position.x, LEFT_BOUNDARY);

        // Prevent PlayerSprite from moving left of the left boundary
        if new_position.x < LEFT_BOUNDARY {
            translation.x = LEFT_BOUNDARY - new_position.x;
        }
        // Prevent PlayerSprite from moving right of the right boundary
        if new_position.x > RIGHT_BOUNDARY {
            translation.x = RIGHT_BOUNDARY - new_position.x;
        }

        sprite_controller.translation = Some(translation);
        // Lock rotation to prevent turning
        transform.rotation = Quat::IDENTITY;
    }
}
