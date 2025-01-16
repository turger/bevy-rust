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
    for (mut sprite_controller, mut player_sprite, mut transform) in sprite_query.iter_mut() {
        let mut translation = Vec2::new(0.0, 0.0);
        
        // Apply gravity when not on ground
        if !player_sprite.on_ground {
            player_sprite.vertical_velocity += GRAVITY_REDUCED * time.delta_seconds();
            // Clamp falling speed
            player_sprite.vertical_velocity = player_sprite.vertical_velocity.max(-1000.0);
        } else {
            // Reset vertical velocity when on ground
            player_sprite.vertical_velocity = 0.0;
        }

        // Handle jump
        if input.just_pressed(KeyCode::Space) && player_sprite.on_ground {
            player_sprite.vertical_velocity = JUMP_VELOCITY;
            player_sprite.on_ground = false;
            player_sprite.index = 13;
            
            // Start the timer
            player_sprite.jump_timer.reset();
        }

        // Check timer and update index
        if player_sprite.jump_timer.tick(time.delta()).just_finished() {
            player_sprite.index = 15;
        }

        if player_sprite.on_ground && player_sprite.death_timer.finished() {
            player_sprite.index = 3;
        }

        // Horizontal movement
        if input.pressed(KeyCode::ArrowRight) {
            translation.x += time.delta_seconds() * PLAYER_VELOCITY_X;
            player_sprite.facing_right = true;
        }

        if input.pressed(KeyCode::ArrowLeft) {
            translation.x += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
            player_sprite.facing_right = false;
        }

        // Apply vertical movement
        translation.y += player_sprite.vertical_velocity * time.delta_seconds();

        // Boundary checks
        let new_position = sprite_controller.translation.unwrap_or_default() + translation;
        
        if new_position.x < LEFT_BOUNDARY {
            translation.x = LEFT_BOUNDARY - new_position.x;
        }
        if new_position.x > RIGHT_BOUNDARY {
            translation.x = RIGHT_BOUNDARY - new_position.x;
        }

        sprite_controller.translation = Some(translation);
        transform.rotation = Quat::IDENTITY;
    }
}
