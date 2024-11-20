pub mod constants;
pub mod player;
pub mod world;

use self::constants::{SPRITESHEET_COLS, SPRITESHEET_ROWS, SPRITE_TILE_WIDTH};
use bevy::prelude::*;

use self::{player::spawn_player, world::spawn_world};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    spawn_player(&mut commands, &mut meshes, &mut materials);
    spawn_world(commands);
}

/*
// TODO does not work yet, sprite sheet is not loading
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // let texture = asset_server.load("textures/horses.png");
    // the sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(SPRITE_TILE_WIDTH as u32), SPRITESHEET_COLS, SPRITESHEET_ROWS, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(Camera2dBundle::default());
    // spawn_player(&mut commands, &mut meshes, &mut materials);
    spawn_world(commands);
  }
*/
