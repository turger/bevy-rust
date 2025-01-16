use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::constants::*;

#[derive(Component)]
pub struct DeathPlatform;

pub fn create_stone_platform(stone_position: [f32; 3], color: Color) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(stone_position[0], stone_position[1], stone_position[2]),
            scale: Vec3::new(40.0, 40.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn spawn_world(mut commands: Commands, stone_platform_positions: Vec<[f32; 3]>, death_platform_positions: Vec<[f32; 3]>) {
    // Sky
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: COLOR_SKY,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, WINDOW_HEIGHT * 0.5, 0.0),
            scale: Vec3::new(
                WINDOW_WIDTH,
                WINDOW_HEIGHT * SKY_HEIGHT_PERCENT / 100.0 * 2.0,
                1.0,
            ),
            ..Default::default()
        },
        ..Default::default()
    });

    // Grass
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: COLOR_GRASS,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, GRASS_TOP_Y, 0.0),
            scale: Vec3::new(
                -WINDOW_WIDTH,
                WINDOW_HEIGHT * GRASS_HEIGHT_PERCENT / 100.0,
                1.0,
            ),
            ..Default::default()
        },
        ..Default::default()
    });


    for stone_position in stone_platform_positions {
        commands
            .spawn(create_stone_platform(stone_position, COLOR_PLATFORM))
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(0.5, 0.5));
    }
    
    for death_position in death_platform_positions {
        // Solid platform
        commands
            .spawn(create_stone_platform(death_position, COLOR_DEATH_PLATFORM))
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(0.5, 0.5));

        // Invisible death platform
        commands
            .spawn(create_stone_platform(death_position, COLOR_INVISIBLE))
            .insert(RigidBody::Fixed)
            .insert(Sensor)
            .insert(Collider::cuboid(0.6, 0.6))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(DeathPlatform);
    }
        
    // Floor
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLOR_FLOOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, GRASS_TOP_Y + 70.0, 0.0),
                scale: Vec3::new(WINDOW_WIDTH, FLOOR_THICKNESS, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(WINDOW_WIDTH / 2.0, FLOOR_THICKNESS / 2.0));
}
