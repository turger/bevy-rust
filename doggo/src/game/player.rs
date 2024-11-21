use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use super::constants::{PLAYER_START_X, PLAYER_START_Y, PLAYER_WIDTH, COLOR_PLAYER};
use super::constants::{SPRITESHEET_COLS, SPRITESHEET_ROWS, SPRITE_TILE_HEIGHT, SPRITE_TILE_WIDTH};

#[derive(Component)]
pub struct Player {
    pub vertical_velocity: f32,
    pub on_ground: bool,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            vertical_velocity: 0.0,
            on_ground: true,
        }
    }
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

#[derive(Component)]
struct LeftSprite;

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(ColorMaterial::from(COLOR_PLAYER)),
            transform: Transform {
                translation: Vec3::new(PLAYER_START_X, PLAYER_START_Y, 1.0),
                scale: Vec3::new(PLAYER_WIDTH, PLAYER_WIDTH, 1.0),
                ..Default::default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(KinematicCharacterController::default())
        .insert(Player::default());
}

pub fn spawn_player_sprite(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/horses.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(SPRITE_TILE_WIDTH as u32, SPRITE_TILE_HEIGHT as u32),
        SPRITESHEET_COLS,
        SPRITESHEET_ROWS,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // the first (left-hand) sprite runs at 10 FPS
    let animation_config_standing = AnimationConfig::new(1, 3, 10);

    // create the first (left-hand) sprite
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(1.0))
                .with_translation(Vec3::new(-50.0, 0.0, 1.0)),
            texture: texture.clone(),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: animation_config_standing.first_sprite_index,
        },
        LeftSprite,
        animation_config_standing,
    ));
}
