use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::player::Player;

pub fn ground_detection(
    mut query: Query<(&mut Player, &Transform)>,
    ground_query: Query<(&Collider, &Transform)>,
) {
    for (mut player, transform) in query.iter_mut() {
        player.on_ground = false; // Reset on_ground status

        // Check for ground collision
        for (ground_collider, ground_transform) in ground_query.iter() {
            // Check if the player's position is close to the ground collider
            let player_bottom_y = transform.translation.y - 15.0; // Adjust based on player size
            let ground_top_y = ground_transform.translation.y + 0.5; // Adjust based on ground collider size

            if player_bottom_y <= ground_top_y {
                player.on_ground = true; // Player is on the ground
                break; // No need to check further
            }
        }
    }
}
