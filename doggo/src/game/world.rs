use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::constants::*;

#[derive(Component)]
pub struct DeathPlatform;

pub fn spawn_world(mut commands: Commands) {
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

    // Stone platform (set to fixed) - Solid platform
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLOR_PLATFORM,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, GRASS_TOP_Y + 90.0, 0.0),
                scale: Vec3::new(40.0, 40.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5));

    // Invisible death trigger
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLOR_PLATFORM,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, GRASS_TOP_Y + 90.0, 0.0),
                scale: Vec3::new(40.0, 40.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Sensor)
        .insert(Collider::cuboid(0.6, 0.6))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(DeathPlatform);
        
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
