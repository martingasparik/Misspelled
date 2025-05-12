use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::orc::{OrcEnemy, OrcState};
use crate::player_code::{Health, Player};
use crate::shield::DamageEvent;

/// Marker component for the attack‐hitbox sensor attached to each Orc
#[derive(Component)]
pub struct AttackHitbox {
    pub owner: Entity,
}

#[derive(Component)]
pub struct HurtHitbox {
    pub owner: Entity, // Keeping this field as it might be used elsewhere
}

pub struct OrcCollisionPlugin;

impl Plugin for OrcCollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_hurt_hitboxes)
            .add_systems(Update, spawn_attack_hitboxes)
            .add_systems(Update, update_attack_hitboxes)  // New system to update hitbox position
            .add_systems(Update, orc_player_collision);
    }
}

fn spawn_hurt_hitboxes(
    mut commands: Commands,
    new_orcs: Query<Entity, Added<OrcEnemy>>,
) {
    for orc in new_orcs.iter() {
        commands.entity(orc).with_children(|parent| {
            parent.spawn((
                Collider::capsule(  
                    Vec2::new(0.0, -5.0), // Center of the collider
                    Vec2::new(0.0, 5.0),
                    4.0,
                ),
                ActiveEvents::COLLISION_EVENTS,
                HurtHitbox { owner: orc },
                Transform::from_xyz(0.0, 0.0, 10.0), // Z-offset for visibility
                
            ));
        });
    }
}

/// Spawn a sector-shaped sensor child under each Orc
fn spawn_attack_hitboxes(
    mut commands: Commands,
    new_orcs: Query<Entity, Added<OrcEnemy>>,
) {
    for orc in new_orcs.iter() {
        // Creating a fan/sector shaped collider with points
        // We'll approximate a 90-degree sector with a convex polygon
        let radius = 18.0;
        let mut points = vec![Vec2::ZERO]; // Center point
        
        // Add points to form a 90-degree sector
        let segments = 8; // Number of segments to approximate the arc
        let half_angle = std::f32::consts::FRAC_PI_4; // 45 degrees (half of 90)
        
        for i in 0..=segments {
            let angle = -half_angle + (2.0 * half_angle * i as f32 / segments as f32);
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            points.push(Vec2::new(x, y));
        }
        
        commands.entity(orc).with_children(|parent| {
            parent.spawn((
                Name::new(format!("AttackHitbox-{orc:?}")),
                Collider::convex_hull(&points).unwrap_or(Collider::ball(radius)),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                AttackHitbox { owner: orc },
                // Make the hitbox start at the center of the orc
                Transform::from_xyz(0.0, 0.0, 10.0),
            ));
        });
    }
}


/// System to update the attack hitbox rotation based on orc's facing direction
fn update_attack_hitboxes(
    mut param_set: ParamSet<(
        Query<(&Transform, &Children, &OrcEnemy)>,
        Query<&mut Transform, With<AttackHitbox>>,
        Query<&Transform, With<Player>>
    )>,
) {
    // Get player position if available
    let player_pos = param_set.p2().get_single().ok().map(|t| t.translation);
    
    // Collect necessary data from first query
    let mut updates = Vec::new();
    
    {
        let orc_query = param_set.p0();
        for (orc_transform, children, _orc) in orc_query.iter() {
            // Determine rotation based on player position instead of just the orc's facing
            let facing_angle = if let Some(player_pos) = player_pos {
                let direction = player_pos - orc_transform.translation;
                direction.y.atan2(direction.x)
            } else {
                // Default to facing the direction the orc is scaled
                if orc_transform.scale.x > 0.0 { 0.0 } else { std::f32::consts::PI }
            };
            
            // Store the children and rotation for later use
            for &child in children {
                updates.push((child, facing_angle));
            }
        }
    }
    
    {
        let mut hitbox_query = param_set.p1();
        for (child, angle) in updates {
            if let Ok(mut hitbox_transform) = hitbox_query.get_mut(child) {
                // Update the rotation to point toward the player
                hitbox_transform.rotation = Quat::from_rotation_z(angle);
                // Keep hitbox at the center with just rotation
                hitbox_transform.translation.x = 0.0;
                hitbox_transform.translation.y = 0.0;
            }
        }
    }
}

fn orc_player_collision(
    mut damage_events: EventWriter<DamageEvent>, 
    mut health_q: Query<&mut Health, With<Player>>,
    mut events: EventReader<CollisionEvent>,
    hitbox_q: Query<(&AttackHitbox, &GlobalTransform)>,
    orc_state_q: Query<(&OrcEnemy, &Transform)>,
    _player_tf_q: Query<&GlobalTransform, With<Player>>,
    player_ent_q: Query<Entity, With<Player>>,
) {
    let player = match player_ent_q.get_single() {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut health = match health_q.get_single_mut() {
        Ok(h) => h,
        Err(_) => return,
    };

    for collision_event in events.read() {
        let (a, b) = match collision_event {
            CollisionEvent::Started(a, b, _) => (*a, *b),
            _ => continue,
        };
        
        // Identify which is hitbox & which is player
        let (hitbox_entity, _player_entity) = if a == player {
            (b, a)
        } else if b == player {
            (a, b)
        } else {
            continue;
        };

        // Pull out the hitbox component & its world position
        let (hitbox, _) = match hitbox_q.get(hitbox_entity) {
            Ok(pair) => pair,
            Err(_) => continue,
        };

        // Only when Orc is currently Attacking
        let (orc, _) = match orc_state_q.get(hitbox.owner) {
            Ok(pair) if pair.0.state == OrcState::Attacking => pair,
            _ => continue,
        };

        // We've already checked the sector with our collider shape,
        // so if there's a collision, the player is in the attack zone
        //health.health -= orc.damage;
        damage_events.send(DamageEvent{amount: 5.0});
        info!(
            "Player hit by Orc {:?}! New health: {:.1}",
            hitbox.owner, health.health
        );
    }
}