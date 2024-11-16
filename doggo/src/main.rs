//! Animates a sprite in response to a keyboard event.
//!
//! See `sprite_sheet.rs` for an example where the sprite animation loops indefinitely.

use std::time::Duration;

use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, execute_animations)
        .add_systems(
            Update,
            (
                trigger_animation::<RightSprite>.run_if(input_just_pressed(KeyCode::ArrowRight)),
                trigger_animation::<LeftSprite>.run_if(input_just_pressed(KeyCode::ArrowLeft)),
            ),
        )
        .run();
}

fn trigger_animation<S: Component>(mut query: Query<&mut AnimationConfig, With<S>>) {
    let mut animation = query.single_mut();
    animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
}

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
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
) {
    for (mut config, mut atlas) in &mut query {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            if atlas.index == config.last_sprite_index {
                atlas.index = config.first_sprite_index;
            } else {
                atlas.index += 1;
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}

#[derive(Component)]
struct LeftSprite;

#[derive(Component)]
struct RightSprite;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Load the horses sprite sheet
    const TEXTURE_PATH: &str = "textures/horses.png";
    let texture = asset_server.load(TEXTURE_PATH);

    const SPRITE_SIZE: u32 = 64;
    const SPRITE_ROWS: u32 = 8;
    const SPRITE_COLUMNS: u32 = 12;

    // Update the layout to match the horses sprite sheet
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(SPRITE_SIZE, SPRITE_SIZE),
        SPRITE_COLUMNS,
        SPRITE_ROWS,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Configure animations for the white horse
    let animation_config_stance = AnimationConfig::new(0, 0, 10); // Stance
    const columns: usize = SPRITE_COLUMNS as usize;
    let animation_config_walk_left = AnimationConfig::new(columns, columns + 2, 10); // Walk
    let animation_config_walk_right = AnimationConfig::new(columns * 2, columns*2 + 2, 10); // Walk

    // Spawn the white horse with animations
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(2.0))
                .with_translation(Vec3::new(0.0, 0.0, 0.0)), // Centered position
            texture: texture.clone(),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: animation_config_stance.first_sprite_index,
        },
        LeftSprite,
        animation_config_stance,
    ));

    // Optionally, you can add a right horse if needed
    /*
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(6.0))
                .with_translation(Vec3::new(50.0, 0.0, 0.0)), // Adjust position for right sprite
            texture: texture.clone(),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: animation_config_walk.first_sprite_index,
        },
        RightSprite,
        animation_config_walk,
    ));
    */

    commands.spawn(TextBundle {
        text: Text::from_section(
            "Left Arrow Key: Animate Left Sprite\nRight Arrow Key: Animate Right Sprite",
            TextStyle::default(),
        ),
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        ..default()
    });
}
