use bevy::prelude::*;
use bevy::time::Time;
use bevy::time::Real;
use bevy_rapier2d::prelude::*;


use crate::spell::{SpellType, SpellCastEvent};
use crate::player_code::Player;
use crate::player_movement::FacingDirection;
use crate::animation::AnimationConfig;
use crate::orc::{OrcEnemy, OrcState};
use crate::orc::collision::HurtHitbox;
use crate::shield::DamageEvent;
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

// Component for fireball spell entities
#[derive(Component)]
pub struct Fireball {
    piercing: bool,
    disabled: bool,
    pub damage: f32,
    pub lifetime: Timer,
    pub direction: Vec2,
}

impl Default for Fireball {
    fn default() -> Self {
        Self {
            piercing: false,
            disabled: false,
            damage: FIREBALL_DAMAGE,
            lifetime: Timer::from_seconds(FIREBALL_LIFETIME, TimerMode::Once),
            direction: Vec2::new(1.0, 0.0), 
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
}

// Plugin for fireball spell systems
pub struct FireballPlugin;

impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_fireball_casting,
            update_fireballs,
            handle_fireball_collisions,
            despawn_expired_fireballs,
            handle_death_timers,
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

                // Spawn fireball entity
                let fireball_entity = commands.spawn((
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
                )).id();
                // Print debug info
                info!("Fireball cast in direction: {:?}", direction);
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
        if !fireball.is_disabled() {
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
    hurtbox_query: Query<&HurtHitbox>,
    mut orc_query: Query<(&mut OrcEnemy, &mut Health)>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = event {
            // Check if one entity is a fireball
            let (fireball_entity, other_entity) = 
                if fireball_query.contains(*entity1) { (*entity1, *entity2) }
                else if fireball_query.contains(*entity2) { (*entity2, *entity1) }
                else { continue; }; // Neither entity is a fireball
            
            // Get the fireball component
            if let Ok((_, mut fireball)) = fireball_query.get_mut(fireball_entity) {
                // Skip if fireball is already disabled
                if fireball.is_disabled() {
                    continue;
                }
                
                // Check if the other entity has a hurtbox component
                if let Ok(hurtbox) = hurtbox_query.get(other_entity) {
                    let orc_entity = hurtbox.owner;
                    
                    // Get the orc component and health
                    if let Ok((mut orc, mut health)) = orc_query.get_mut(orc_entity) {
                        // Apply damage to health
                        health.health -= fireball.damage;
                        
                        info!("Orc hit for {} damage! Remaining health: {}", 
                              fireball.damage, health.health);
                              
                        // Set orc to hurt state if still alive
                        if health.health <= 0.0 {
                            orc.state = OrcState::Dying;
                            info!("Orc defeated! Adding death timer");
                            
                            commands.entity(orc_entity).insert(DeathTimer {
                                timer: Timer::from_seconds(0.5, TimerMode::Once),
                            });

                            commands.entity(orc_entity).insert(LockedAxes::TRANSLATION_LOCKED_X | LockedAxes::TRANSLATION_LOCKED_Y);
                        } else {
                            // Only transition to hurt state if not dying
                            orc.state = OrcState::Hurt;
                        }
                        
                        // Disable the fireball
                        fireball.disable();
                        
                        // Despawn if non-piercing
                        if fireball.is_disabled() {
                            commands.entity(fireball_entity).despawn_recursive();
                            info!("Despawned fireball after hitting orc");
                        }
                    }
                }
            }
        }
    }
}

fn despawn_expired_fireballs(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Fireball)>,
    time: Res<Time>,
) {
    for (entity, mut fireball) in query.iter_mut() {
        fireball.lifetime.tick(time.delta());

        if fireball.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn handle_death_timers(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DeathTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.timer.tick(time.delta());
        
        if timer.timer.finished() {
            info!("Death timer finished, despawning entity {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }
    }
}