use bevy::prelude::*;
use bevy::asset::Handle;
use bevy::image::Image;
use bevy::math::Vec3;
use bevy::math::Vec2;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use crate::animation::{AnimationConfig, SpriteState};
use crate::player_animation::{FIRST_IDLE, FPS_IDLE, LAST_IDLE};
use crate::player_movement::{FacingDirection, MovementState};
use crate::orc::collision::AttackHitbox;

#[derive(Component)]
pub struct Player;

/// Health component for entities
#[derive(Component)]
pub struct PlayerHealthPlugin;

#[derive(Component)]
pub struct Health {
    pub health: f32,
}

impl Health {
    pub fn new(initial_health: f32) -> Self {
        Health { health: initial_health }
    }
}

impl Plugin for PlayerHealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerDamageEvent>()
            .add_systems(Update, (
                check_orc_attack_collisions,
                handle_player_damage,
                handle_invulnerability,
            ));
    }
}

#[derive(Event)]
pub struct PlayerDamageEvent {
    pub damage: f32,
}

#[derive(Component)]
pub struct Invulnerable {
    pub timer: Timer,
}

pub struct PlayerPhysicsPlugin;

#[derive(Component)]
pub struct Shield {
    pub shield: f32,
    pub max_shield: f32,
}

impl Shield {
    pub fn new(amount: f32) -> Self {
        Shield { 
            shield: amount,
            max_shield: amount 
        }
    }
}
// Set up the player entity with all necessary components
pub fn setup_player(
    mut commands: Commands,
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
) {
    // Create the initial idle animation configuration
    let idle_animation_config = AnimationConfig::new(FIRST_IDLE, LAST_IDLE, FPS_IDLE);

    // Spawn the player with all required components
    let player_entity = commands.spawn((
        RigidBody::Dynamic,
        Velocity::default(),
        Collider::capsule(
            Vec2::new(1.0, 0.0), 
            Vec2::new(1.0, -6.0),
            4.0,
        ),
        ActiveEvents::COLLISION_EVENTS,
        LockedAxes::ROTATION_LOCKED,      
        
        // Visual components
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: FIRST_IDLE,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(5.0)),

        // Game logic components
        Player,
        Health::new(10.0),
        Shield::new(0.0),
        FacingDirection {facing_right: true},
        MovementState::Idle,
        SpriteState::Idle,
        idle_animation_config,
    )).id();
}

fn check_orc_attack_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    attack_hitbox_query: Query<&AttackHitbox>,
    mut damage_events: EventWriter<PlayerDamageEvent>,
) {
    // Get the player entity
    if let Ok(player_entity) = player_query.get_single() {
        for event in collision_events.read() {
            if let CollisionEvent::Started(e1, e2, _) = event {
                // Check if this collision involves the player and an attack hitbox
                let (player_collided, hitbox_entity) = if *e1 == player_entity {
                    (true, *e2)
                } else if *e2 == player_entity {
                    (true, *e1)
                } else {
                    (false, *e1) // Not relevant
                };

                // If the player is involved in the collision
                if player_collided {
                    // Check if the other entity is an attack hitbox
                    if let Ok(_attack_hitbox) = attack_hitbox_query.get(hitbox_entity) {
                        // Send damage event - 1.0 damage per heart
                        damage_events.send(PlayerDamageEvent { damage: 1.0 });
                        info!("Player hit by orc attack!");
                    }
                }
            }
        }
    }
}

// System to handle damage to the player
fn handle_player_damage(
    mut commands: Commands,
    mut damage_events: EventReader<PlayerDamageEvent>,
    mut player_query: Query<(Entity, &mut Health, &mut Shield), (With<Player>, Without<Invulnerable>)>,
) {
    // Only process if player exists and isn't invulnerable
    if let Ok((player_entity, mut health, mut shield)) = player_query.get_single_mut() {
        for event in damage_events.read() {
            let damage_amount = event.damage;
            
            // Try to absorb damage with shield first
            if shield.shield > 0.0 {
                // Shield can absorb some or all damage
                let shield_absorption = shield.shield.min(damage_amount);
                shield.shield -= shield_absorption;
                
                let remaining_damage = damage_amount - shield_absorption;
                
                if remaining_damage > 0.0 {
                    // Shield wasn't enough, apply remaining damage to health
                    health.health -= remaining_damage;
                    info!("Shield absorbed {:.1} damage, player health reduced to {:.1}", 
                          shield_absorption, health.health);
                }
            } else {
                // No shield, damage goes directly to health
                health.health -= damage_amount;
                info!("No shield! Player health reduced to {:.1}", health.health);
            }

            // Add invulnerability period
            commands.entity(player_entity).insert(Invulnerable {
                timer: Timer::new(Duration::from_secs_f32(1.5), TimerMode::Once),
            });
        }
    }
}

// System to handle player invulnerability period
fn handle_invulnerability(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Invulnerable, &mut Sprite)>,
) {
    for (entity, mut invulnerable, mut sprite) in query.iter_mut() {
        // Tick the timer
        invulnerable.timer.tick(time.delta());
        
        // Calculate a pulse effect for the alpha (0.3 to 0.9)
        let pulse_rate = 10.0; // Higher = faster pulse
        let elapsed = invulnerable.timer.elapsed().as_secs_f32();
        
        // Sine wave oscillation for smooth pulsing effect
        let oscillation = (elapsed * pulse_rate).sin() * 0.5 + 0.5; // 0.0 to 1.0
        let alpha = 0.3 + (oscillation * 0.6); // 0.3 to 0.9 range
        
        // Set the alpha using the same method that works in your death fade system
        sprite.color.set_alpha(alpha);
        
        // Remove invulnerability when timer is finished
        if invulnerable.timer.finished() {
            commands.entity(entity).remove::<Invulnerable>();
            
            // Ensure sprite is fully visible when invulnerability ends
            sprite.color.set_alpha(1.0);
            
            info!("Player invulnerability ended");
        }
    }
}
// todo: 
/* fn check_player_in_attack_hitboxes(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    attack_hitbox_query: Query<(Entity, &AttackHitbox, &GlobalTransform)>,
) {
    for (player_entity, player_transform) in player_query.iter() {
        for (hitbox_entity, _attack_hitbox, hitbox_transform) in attack_hitbox_query.iter() {
            // Check if the player is within the hitbox
            if player_transform.translation.distance(hitbox_transform.translation) < 1.0 {
                // Player is in the hitbox
                commands.entity(player_entity).insert(Invulnerable {
                    timer: Timer::new(Duration::from_secs_f32(1.5), TimerMode::Once),
                });
                info!("Player is in attack hitbox!");
            }
        }
    }
} */

/* fn update_player_collider(
    mut query: Query<(&mut Collider, &FacingDirection), With<Player>>,
) {
    for (mut collider, facing) in query.iter_mut() {
        if facing.facing_right {
            *collider = Collider::capsule(
                Vec2::new(1.0, 0.0), 
                Vec2::new(1.0, -6.0),
                4.0,
            );
        } else {
            *collider = Collider::capsule(
                Vec2::new(-1.0, 0.0),
                Vec2::new(-1.0, -6.0),
                4.0,
            );
        }
    }
} */

/* impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_player_collider);
    }
} */ //maybe we need a damage sensor for the player, maybe it can be done with the hurtbox
// knockback?? so far it works in invulnerability