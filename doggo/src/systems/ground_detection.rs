use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::player_sprite::PlayerSprite;

pub fn ground_detection(
    mut sprite_query: Query<(&mut PlayerSprite, &Transform)>,
    ground_query: Query<(&Collider, &Transform)>,
) {    
    for (mut player_sprite, transform) in sprite_query.iter_mut() {
        let mut is_grounded = false;
        
        // Check for ground collision
        for (_ground_collider, ground_transform) in ground_query.iter() {
            let player_bottom_y = transform.translation.y - 15.0;
            let ground_top_y = -180.7549;
            
            // Add a small threshold for ground detection
            let distance_threshold = 2.0;
            
            // Check if player is within the horizontal bounds of the ground
            let player_left = transform.translation.x - 35.0;
            let player_right = transform.translation.x + 35.0;
            let ground_left = ground_transform.translation.x - ground_transform.scale.x/2.0;
            let ground_right = ground_transform.translation.x + ground_transform.scale.x/2.0;
            
            if player_bottom_y >= ground_top_y - distance_threshold 
                && player_bottom_y <= ground_top_y + distance_threshold
                && player_right >= ground_left
                && player_left <= ground_right {
                is_grounded = true;
                break;
            }
        }
        
        // Only update on_ground at the end of all checks
        player_sprite.on_ground = is_grounded;
    }
}
