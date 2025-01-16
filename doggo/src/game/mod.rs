pub mod constants;
pub mod player_sprite;
pub mod world;
pub mod level_1;
pub mod level_2;

use bevy::prelude::*;

use self::{level_1::spawn_level_1, level_2::spawn_level_2, player_sprite::spawn_player};

// for meshes and materials
pub fn setup(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());
    spawn_player(&mut commands, asset_server, texture_atlas_layouts);
    spawn_level_1(commands);
    // spawn_level_2(commands);
}
