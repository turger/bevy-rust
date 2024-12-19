use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::constants::{PLAYER_START_X, PLAYER_START_Y, SNOOPY_SIZE, SPRITESHEET_COLS, SPRITESHEET_ROWS, SPRITE_TILE_HEIGHT, SPRITE_TILE_WIDTH};

#[derive(Component)]
pub struct PlayerSprite {
    pub vertical_velocity: f32,
    pub on_ground: bool,
    pub facing_right: bool,
    pub index: usize,
    pub jump_timer: Timer,
    pub death_timer: Timer,
}

impl Default for PlayerSprite {
    fn default() -> Self {
        PlayerSprite {
            vertical_velocity: 0.0,
            on_ground: false,
            facing_right: true,
            index: 0,
            jump_timer: Timer::from_seconds(0.4, TimerMode::Once),
            death_timer: Timer::from_seconds(0.4, TimerMode::Once),
        }
    }
}

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
}

impl AnimationConfig {
    fn new(first: usize) -> Self {
        Self {
            first_sprite_index: first
        }
    }
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/snoopy.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(SPRITE_TILE_WIDTH as u32, SPRITE_TILE_HEIGHT as u32),
        SPRITESHEET_COLS,
        SPRITESHEET_ROWS,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_config = AnimationConfig::new(0);

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(PLAYER_START_X, PLAYER_START_Y, 1.0),
                scale: Vec3::new(SNOOPY_SIZE, SNOOPY_SIZE, 1.0),
                ..Default::default()
            },
            texture: texture.clone(),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: animation_config.first_sprite_index,
        },
        animation_config,
        PlayerSprite::default(),
        Collider::cuboid(35.0, 52.0),
        Player,
        ))
        .insert(RigidBody::Dynamic)
        .insert(KinematicCharacterController::default())
        .insert(GravityScale(1.0));
}

pub fn update_sprite_index(
    mut query: Query<(&PlayerSprite, &mut TextureAtlas)>,
) {
    for (player, mut atlas) in query.iter_mut() {
        atlas.index = player.index;
    }
}