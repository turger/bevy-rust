use bevy::prelude::*;

// Window constants
pub const WINDOW_WIDTH: f32 = 1024.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;

// Game world constants
pub const SKY_HEIGHT_PERCENT: f32 = 80.0;
pub const GRASS_HEIGHT_PERCENT: f32 = 100.0 - SKY_HEIGHT_PERCENT;
pub const FLOOR_THICKNESS: f32 = 1.0;
pub const PLAYER_VELOCITY_X: f32 = 400.0;
pub const JUMP_VELOCITY: f32 = 5000.0;
pub const GRAVITY_REDUCTION_FACTOR: f32 = 0.5;
pub const GRAVITY: f32 = -9.81;
pub const GRAVITY_REDUCED: f32 = GRAVITY * GRAVITY_REDUCTION_FACTOR;
pub const LEFT_BOUNDARY: f32 = -WINDOW_WIDTH / 2.0 + SNOOPY_SIZE;
pub const RIGHT_BOUNDARY: f32 = WINDOW_WIDTH / 2.0 - SNOOPY_SIZE;
// Position calculations
pub const GRASS_TOP_Y: f32 =
    WINDOW_BOTTOM_Y + ((WINDOW_HEIGHT * GRASS_HEIGHT_PERCENT / 100.0) * 1.0) - 0.1 * WINDOW_HEIGHT;

// Player constants
pub const PLAYER_START_X: f32 = -200.0; // Starting X position of the player
pub const PLAYER_START_Y: f32 = 0.0;     // Starting Y position of the player
pub const SNOOPY_SIZE: f32 = 1.0;       // Size of the snoopy

// Colors
// black
pub const COLOR_FLOOR: Color = Color::srgb(0.45, 0.55, 0.66);
// black
pub const COLOR_PLATFORM: Color = Color::srgba(0.0, 0.0, 0.0, 255.0);
// blue
pub const COLOR_SKY: Color = Color::srgba(0.0, 0.0, 255.0, 255.0);
// lime green
pub const COLOR_GRASS: Color = Color::srgba(0.0, 255.0, 0.0, 255.0);

// Spritesheet constants
pub const SPRITE_TILE_WIDTH: f32 = 130.0;
pub const SPRITE_TILE_HEIGHT: f32 = 140.0;
pub const SPRITESHEET_COLS: u32 = 15;
pub const SPRITESHEET_ROWS: u32 = 16;
