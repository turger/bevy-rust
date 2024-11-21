use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

use super::constants::{COLOR_PLAYER, PLAYER_START_X, PLAYER_START_Y, PLAYER_WIDTH};

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
