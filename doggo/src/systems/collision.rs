use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game::world::DeathPlatform;
use crate::game::player_sprite::Player;

pub fn check_death_collision(
    mut collision_events: EventReader<CollisionEvent>,
    death_platform_query: Query<Entity, With<DeathPlatform>>,
    player_query: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    let player_entity = player_query.single();
    let death_platform_entity = death_platform_query.single();

    for collision_event in collision_events.read() {
        println!("Collision event detected: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                println!("Entities involved: {:?} and {:?}", e1, e2);
                if (*e1 == player_entity && *e2 == death_platform_entity) 
                    || (*e2 == player_entity && *e1 == death_platform_entity) {
                    println!("Player hit the death platform!");
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
} 