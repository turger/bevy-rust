pub mod constants;
pub mod player;
pub mod world;

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
