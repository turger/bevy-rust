pub mod constants;
pub mod player_sprite;
pub mod world;

use bevy::prelude::*;

use self::{world::spawn_world, player_sprite::spawn_player};

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
    spawn_world(commands);
}
