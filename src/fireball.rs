use bevy::prelude::*;
use bevy::time::Time;
use bevy::time::Real;
use bevy_rapier2d::prelude::*;


use crate::spell::{SpellType, SpellCastEvent};
use crate::player_code::Player;
use crate::player_movement::FacingDirection;
use crate::animation::AnimationConfig;
use crate::orc::{OrcEnemy, OrcState};
use crate::orc::collision::{HurtHitbox, AttackHitbox};
use crate::player_code::Health;

pub const FIREBALL_SPEED: f32 = 200.0;
pub const FIREBALL_LIFETIME: f32 = 5.0;
pub const FIREBALL_DAMAGE: f32 = 10.0;
pub const FIREBALL_FIRST_INDEX: usize = 0;
pub const FIREBALL_LAST_INDEX: usize = 11;
pub const FIREBALL_FPS: u8 = 12;

//Death timer
#[derive(Component)]
pub struct DeathTimer {
    pub timer: Timer,
}


// Add this component to track the fading state
#[derive(Component)]
pub struct DeathFade {
    pub fade_timer: Timer,
    pub initial_alpha: f32,
}

// Component for fireball spell entities
#[derive(Component)]
pub struct Fireball {
    piercing: bool,
    disabled: bool,
    pub damage: f32,
    pub lifetime: Timer,
    pub direction: Vec2,
    pub marked_for_despawn: bool, // New field to track despawn status
}

impl Default for Fireball {
    fn default() -> Self {
        Self {
            piercing: false,
            disabled: false,
            damage: FIREBALL_DAMAGE,
            lifetime: Timer::from_seconds(FIREBALL_LIFETIME, TimerMode::Once),
            direction: Vec2::new(1.0, 0.0),
            marked_for_despawn: false, // Initialize as false
        }
    }
}

impl Fireball {
    pub fn new(direction: Vec2, damage: f32) -> Self {
        Self {
            direction,
            damage,
            ..Default::default()
        }
    }

    pub fn disable(&mut self) {
        if !self.piercing {
            self.disabled = true;
        }
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    // New method to mark for despawn
    pub fn mark_for_despawn(&mut self) {
        self.marked_for_despawn = true;
    }
}

// Add event to signal fireball despawn
#[derive(Event)]
pub struct FireballDespawnEvent(pub Entity);

// Plugin for fireball spell systems
pub struct FireballPlugin;

impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<FireballDespawnEvent>()
            .add_systems(Update, (
                handle_fireball_casting,
                update_fireballs,
                handle_fireball_collisions,
                process_fireball_despawn_events.after(handle_fireball_collisions),
                despawn_expired_fireballs.after(process_fireball_despawn_events),
                handle_death_timers,
                handle_death_fade,
            ));
    }
}

// System to handle the fireball spell casting event
fn handle_fireball_casting(
    mut commands: Commands,
    mut spell_events: EventReader<SpellCastEvent>,
    player_query: Query<(&Transform, &FacingDirection), With<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for event in spell_events.read() {
        if let SpellType::Fireball = event.spell_type {
            // Get player position and facing direction
            if let Ok((player_transform, facing)) = player_query.get_single() {
                // Determine fireball direction based on player facing
                let direction = if facing.facing_right {
                    Vec2::new(1.0, 0.0)
                } else {
                    Vec2::new(-1.0, 0.0)
                };

                // Position the fireball slightly in front of the player
                let offset = direction * 30.0; // Offset to place fireball in front of player
                let spawn_position = player_transform.translation + Vec3::new(offset.x, offset.y, 0.0);

                // Load texture and create texture atlas
                let fireball_texture = asset_server.load("spells/03.png");
                let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 6, 2, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                // Create animation configuration
                let fireball_animation = AnimationConfig::new(
                    FIREBALL_FIRST_INDEX,
                    FIREBALL_LAST_INDEX,
                    FIREBALL_FPS,
                );

                // Spawn fireball entity - removed the .id() call since we don't use the return value
                commands.spawn((
                    Sprite {
                        image: fireball_texture.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlas_layout,
                            index: FIREBALL_FIRST_INDEX,
                        }),
                        flip_x: !facing.facing_right,
                        ..default()
                    },
                    Transform::from_translation(spawn_position)
                        .with_scale(Vec3::splat(2.0)), // Size of the fireball
                    Fireball::new(direction, FIREBALL_DAMAGE),
                    fireball_animation,
                    
                    // Add physics components for collision detection
                    Collider::ball(8.0),
                    Sensor, // Make it a sensor so it doesn't push things
                    ActiveEvents::COLLISION_EVENTS,
                    
                    Name::new("Fireball"),
                ));
            }
        }
    }
}

// System to update fireball positions
fn update_fireballs(
    mut fireball_query: Query<(&mut Transform, &Fireball)>,
    time: Res<Time<Real>>,
) {
    for (mut transform, fireball) in fireball_query.iter_mut() {
        if !fireball.is_disabled() && !fireball.marked_for_despawn {
            let movement = fireball.direction * FIREBALL_SPEED * time.delta_secs();
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
}

// System to handle fireball collision with enemies
fn handle_fireball_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut fireball_query: Query<(Entity, &mut Fireball)>,
    hurtbox_query: Query<(Entity, &HurtHitbox)>,
    attack_hitbox_query: Query<(Entity, &crate::orc::collision::AttackHitbox)>,
    mut orc_query: Query<(&mut OrcEnemy, &mut Health)>,
    mut despawn_events: EventWriter<FireballDespawnEvent>,
) {
    // iterate all new collision events
    for event in collision_events.read() {
        // destructure the &CollisionEvent
        if let CollisionEvent::Started(e1, e2, _flags) = *event {
            // figure out which one is the fireball
            let (fb_ent, other_ent) = if fireball_query.contains(e1) {
                (e1, e2)
            } else if fireball_query.contains(e2) {
                (e2, e1)
            } else {
                continue;
            };

            // Check if the fireball entity still exists in the world
            if !fireball_query.get(fb_ent).is_ok() {
                continue;
            }

            // grab &mut Fireball
            if let Ok((_, mut fb)) = fireball_query.get_mut(fb_ent) {
                // skip if already disabled by some other logic
                if fb.is_disabled() || fb.marked_for_despawn {
                    continue;
                }

                // only proceed if we really hit an orc hurtbox
                if let Ok((_, hurtbox)) = hurtbox_query.get(other_ent) {
                    let orc_ent = hurtbox.owner;

                    // damage the orc
                    if let Ok((mut orc, mut health)) = orc_query.get_mut(orc_ent) {
                        health.health -= fb.damage;

                        if health.health <= 0.0 {
                            // → enter dying state
                            orc.state = OrcState::Dying;

                            // lock its position and start your death timer
                            commands.entity(orc_ent)
                                .insert(LockedAxes::TRANSLATION_LOCKED_X | LockedAxes::TRANSLATION_LOCKED_Y)
                                .insert(Sensor)
                                .insert(ActiveCollisionTypes::empty())
                                .insert(CollisionGroups::new(
                                    Group::NONE, // Remove from all collision groups
                                    Group::NONE  // Don't collide with anything
                                ))
                                .insert(DeathTimer {
                                    timer: Timer::from_seconds(1.5, TimerMode::Once),
                                });
                         info!("Orc dying, disabling all collisions!");
                            // immediately tear down all hurtboxes
                            for (hb_ent, hurtbox) in hurtbox_query.iter() {
                                if hurtbox.owner == orc_ent {
                                    // Safely despawn if the entity exists
                                    commands.entity(hb_ent).despawn_recursive();
                                }
                            }
                            
                            // immediately tear down all attack hitboxes for this orc
                            for (attack_ent, attack_hitbox) in attack_hitbox_query.iter() {
                                if attack_hitbox.owner == orc_ent {
                                    // Safely despawn if the entity exists
                                    commands.entity(attack_ent).despawn_recursive();
                                }
                            }
                        }
                    }

                    // Mark the fireball for despawn and send an event
                    fb.mark_for_despawn();
                    fb.disable();
                    despawn_events.send(FireballDespawnEvent(fb_ent));
                }
            }
        }
    }
}

// New system to process fireball despawn events
fn process_fireball_despawn_events(
    mut commands: Commands,
    mut despawn_events: EventReader<FireballDespawnEvent>,
    fireball_query: Query<Entity, With<Fireball>>,
) {
    for event in despawn_events.read() {
        let entity = event.0;
        
        // Only despawn if the entity still exists and is a fireball
        if fireball_query.get(entity).is_ok() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_expired_fireballs(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Fireball)>,
    mut despawn_events: EventWriter<FireballDespawnEvent>,
) {
    for (entity, mut fireball) in query.iter_mut() {
        // Skip if already marked for despawn from collision
        if fireball.marked_for_despawn {
            continue; // The process_fireball_despawn_events system will handle this
        }

        // Tick the lifetime timer directly
        fireball.lifetime.tick(time.delta());
        
        // Check if lifetime has expired
        if fireball.lifetime.finished() {
            info!("Fireball lifetime expired, marking for despawn");
            fireball.mark_for_despawn();
            despawn_events.send(FireballDespawnEvent(entity));
        }
    }
}

// Handle death timers for dead entities
fn handle_death_timers(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DeathTimer)>,
    orc_query: Query<&OrcEnemy>,
    hurt_hitboxes: Query<(Entity, &HurtHitbox)>,
    attack_hitboxes: Query<(Entity, &AttackHitbox)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.timer.tick(time.delta());
        
        // Double-check for any remaining hitboxes
        if orc_query.get(entity).is_ok() {
            // This is an orc with a death timer
            // Check for any lingering hurt hitboxes
            for (hitbox_entity, hitbox) in hurt_hitboxes.iter() {
                if hitbox.owner == entity {
                    commands.entity(hitbox_entity).despawn_recursive();
                }
            }
            
            // Check for any lingering attack hitboxes
            for (hitbox_entity, hitbox) in attack_hitboxes.iter() {
                if hitbox.owner == entity {
                    commands.entity(hitbox_entity).despawn_recursive();
                }
            }
        }
        
        // Rest of your code stays the same
    }
}

// Handle fading out of dying entities
fn handle_death_fade(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Sprite, &mut DeathFade)>,
) {
    for (entity, mut sprite, mut fade) in query.iter_mut() {
        fade.fade_timer.tick(time.delta());

        let elapsed  = fade.fade_timer.elapsed().as_secs_f32();
        let total    = fade.fade_timer.duration().as_secs_f32();
        let progress = (elapsed / total).clamp(0.0, 1.0);

        let new_alpha = fade.initial_alpha * (1.0 - progress);

        sprite.color.set_alpha(new_alpha);

        if fade.fade_timer.finished() {
            // Use despawn_recursive to ensure all child entities are also despawned
            commands.entity(entity).despawn_recursive();
        }
    }
}