use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::orc::OrcEnemy;
use crate::player_code::{Health, Player};

pub struct OrcCollisionPlugin;
impl Plugin for OrcCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, orc_player_collision);
    }
}

fn orc_player_collision(
    mut health_q: Query<&mut Health, With<Player>>,
    orc_q: Query<(&OrcEnemy, Entity)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    // only one player, so bail if it's not there
    let Ok(mut health) = health_q.get_single_mut() else {
        return;
    };

    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            // Check if either entity is an orc
            let is_orc_collision = 
                orc_q.iter()
                     .any(|(_, entity)| entity == *e1 || entity == *e2);
            
            // If it's an orc collision, damage the player
            if is_orc_collision {
                health.health -= 5.0;
                println!("Player hit! Health: {}", health.health);
            }
        }
    }
}