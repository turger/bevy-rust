use bevy::prelude::*;

use super::constants::*;

use super::world::spawn_world;

pub fn spawn_level_1(commands: Commands) { 
    let platform_y = GRASS_TOP_Y + 90.0;
    
    let stone_platform_positions = vec![
        [0.0, platform_y, 0.0],
        [150.0, platform_y, 0.0]
    ];
    
    let death_platform_positions = vec![
        [-100.0, platform_y, 0.0],
        [100.0, platform_y, 0.0]
    ];

    spawn_world(commands, stone_platform_positions, death_platform_positions);
}
