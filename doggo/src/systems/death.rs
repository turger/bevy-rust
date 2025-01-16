use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game::world::DeathPlatform;
use crate::game::player_sprite::Player;
use crate::game::{
  constants::{PLAYER_START_X, PLAYER_START_Y},
  player_sprite::PlayerSprite,
};

pub fn handle_death(
    mut collision_events: EventReader<CollisionEvent>,
    death_platform_query: Query<Entity, With<DeathPlatform>>,
    mut player_query: Query<(Entity, &mut Transform, &mut PlayerSprite), With<Player>>,
    time: Res<Time>,
) {
    let death_platform = death_platform_query.single();
    let (player_entity, mut player_transform, mut player_sprite) = player_query.single_mut();

    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if (*e1 == player_entity && *e2 == death_platform) 
                    || (*e2 == player_entity && *e1 == death_platform) {
                    println!("Player hit the death platform!");
                    player_sprite.index = 9;
                    player_sprite.death_timer.reset();
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }

    if player_sprite.death_timer.tick(time.delta()).just_finished() {
        player_transform.translation.x = PLAYER_START_X;
        player_transform.translation.y = PLAYER_START_Y;
        player_sprite.index = 3;
    }
} 