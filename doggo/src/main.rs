use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use std::time::Duration;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

const HORSE_PATH: &str = "textures/horses.png";

const PLATFORM_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 255.0);

const FLOOR_THICKNESS: f32 = 1.0;

const COLOR_FLOOR: Color = Color::srgb(0.45, 0.55, 0.66);

const SKY_HEIGHT_PERCENT: f32 = 80.0;
const GRASS_HEIGHT_PERCENT: f32 = 100.0 - SKY_HEIGHT_PERCENT;

const GRASS_TOP_Y: f32 =
    WINDOW_BOTTOM_Y + ((WINDOW_HEIGHT * GRASS_HEIGHT_PERCENT / 100.0) * 1.0) - 0.1 * WINDOW_HEIGHT;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once),
        }
    }
}

#[derive(Component)]
struct LeftSprite;

#[derive(Component)]
struct RightSprite;

/*
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        transform.translation.x += direction.x * 5.0; // Adjust speed as necessary
    }
}
    */

// This system runs when the user clicks the left arrow key or right arrow key
fn trigger_animation<S: Component>(mut query: Query<&mut AnimationConfig, With<S>>) {
    let mut animation = query.single_mut();
    // we create a new timer when the animation is triggered
    animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
}

impl AnimationConfig {
    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Load the horse sprite sheet
    let texture = asset_server.load("textures/horses.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 8, 12, None, None); // 8 columns, 12 rows
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Create the player entity (first black horse)
    let animation_config = AnimationConfig::new(0, 2, 10); // Indices for the first black horse

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(6.0))
                .with_translation(Vec3::new(0.0, 0.0, 0.0)), // Start at the center
            texture: texture.clone(),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: animation_config.first_sprite_index,
        },
        Player,
        animation_config,
    ));

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
                -WINDOW_WIDTH,
                WINDOW_HEIGHT * GRASS_HEIGHT_PERCENT / 100.0,
                1.0,
            ), // 30% height
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
}

fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
) {
    for (mut config, mut atlas) in &mut query {
        // Track how long the current sprite has been displayed
        config.frame_timer.tick(time.delta());

        // Check if the timer has finished
        if config.frame_timer.just_finished() {
            if atlas.index == config.last_sprite_index {
                // If it's the last frame, reset to the first frame
                atlas.index = config.first_sprite_index;
            } else {
                // Move to the next frame
                atlas.index += 1;
                // Reset the frame timer for the next frame
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}

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
                .set(ImagePlugin::default_nearest())
                .add(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
                .add(RapierDebugRenderPlugin::default()),
        )
        .add_systems(Startup, setup)
        // .add_systems(Update, player_movement)
        .add_systems(
            Update,
            (
                trigger_animation::<RightSprite>.run_if(input_just_pressed(KeyCode::ArrowRight)),
                trigger_animation::<LeftSprite>.run_if(input_just_pressed(KeyCode::ArrowLeft)),
            ),
        )
        .add_systems(Update, execute_animations)
        .run();
}
