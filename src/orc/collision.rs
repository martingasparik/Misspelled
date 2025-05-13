use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::orc::OrcEnemy;
use crate::player_code::{Health, Player};
use crate::shield::DamageEvent;

pub struct OrcCollisionPlugin;
impl Plugin for OrcCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, orc_player_collision);
    }
}

fn orc_player_collision(
    mut damage_events: EventWriter<DamageEvent>,
    orc_q: Query<(&OrcEnemy, Entity)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            // Check if either entity is an orc
            let is_orc_collision =
                orc_q.iter()
                    .any(|(_, entity)| entity == *e1 || entity == *e2);

            // If it's an orc collision, damage the player
            if is_orc_collision {
                // Instead of directly modifying health, send a damage event
                damage_events.send(DamageEvent { amount: 5.0 });
                println!("Player hit by orc! Damage: 5.0");
            }
        }
    }
}