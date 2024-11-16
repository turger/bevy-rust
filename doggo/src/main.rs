use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

const HORSE_PATH: &str = "textures/horses.png";

const PLATFORM_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 255.0);

const FLOOR_THICKNESS: f32 = 1.0;

const COLOR_FLOOR: Color = Color::rgb(0.45, 0.55, 0.66);

const SKY_HEIGHT_PERCENT: f32 = 80.0;
const GRASS_HEIGHT_PERCENT: f32 = 100.0 - SKY_HEIGHT_PERCENT;

const GRASS_TOP_Y: f32 = WINDOW_BOTTOM_Y + ((WINDOW_HEIGHT * GRASS_HEIGHT_PERCENT / 100.0) * 1.0) - 0.1 * WINDOW_HEIGHT;
// Add a new struct for the Horse component
#[derive(Component)]
struct Horse;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Platformer".to_string(),
                        resizable: false,
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .add(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
                .add(RapierDebugRenderPlugin::default()),
        )
        .add_systems(Startup, setup)
        // .add_systems(FixedUpdate, update_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Draw the sky
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(0.0, 0.0, 255.0, 255.0),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, WINDOW_HEIGHT * 0.5, 0.0), // Centered vertically
            scale: Vec3::new(
                WINDOW_WIDTH,
                WINDOW_HEIGHT * SKY_HEIGHT_PERCENT / 100.0 * 2.0,
                1.0,
            ), // 70% height
            ..Default::default()
        },
        ..Default::default()
    });

    // Draw the grass
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(0.0, 255.0, 0.0, 255.0),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, GRASS_TOP_Y, 0.0), // Position it at the bottom
            scale: Vec3::new(
                WINDOW_WIDTH,
                WINDOW_HEIGHT * GRASS_HEIGHT_PERCENT / 100.0,
                1.0,
            ),
            ..Default::default()
        },
        ..Default::default()
    });

    // Stone
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: PLATFORM_COLOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 200.0, 0.0),
                scale: Vec3::new(40.0, 40.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5));

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
        .insert(Collider::cuboid(0.5, 0.5));

    commands.spawn(Camera2dBundle::default());
}
