use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod game;
mod systems;

use game::constants::*;
use game::setup;
use systems::movement::movement;
use systems::ground_detection::ground_detection;
use game::player_sprite::update_sprite_index;

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
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            ground_detection,
            movement,
            update_sprite_index,
        ).chain())
        .run();
}
