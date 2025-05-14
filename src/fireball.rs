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

#[derive(Component)]
struct FireballHit;

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
    // skip any fireball that's already been hit
    mut fireball_query: Query<(Entity, &mut Fireball), Without<FireballHit>>,
    hurtbox_query: Query<&HurtHitbox>,
    mut orc_query: Query<(&mut OrcEnemy, &mut Health)>,
) {
    // iterate all new collision events
    for event in collision_events.read() {
        // destructure the reference so we get owned Entities
        if let &CollisionEvent::Started(entity1, entity2, _flags) = event {
            // figure out which one is the fireball
            let (fb_ent, other_ent) = if fireball_query.contains(entity1) {
                (entity1, entity2)
            } else if fireball_query.contains(entity2) {
                (entity2, entity1)
            } else {
                continue;
            };

            // pull out the fireball component
            if let Ok((_, mut fireball)) = fireball_query.get_mut(fb_ent) {
                // skip if it was already disabled/hit
                if fireball.is_disabled() {
                    continue;
                }

                // only proceed if the other entity really is an orc hurtbox
                if let Ok(hb) = hurtbox_query.get(other_ent) {
                    let orc_ent = hb.owner;

                    // apply damage or start death on the orc
                    if let Ok((mut orc, mut health)) = orc_query.get_mut(orc_ent) {
                        health.health -= fireball.damage;
                        if health.health <= 0.0 {
                            orc.state = OrcState::Dying;
                            commands.entity(orc_ent)
                                .insert(LockedAxes::TRANSLATION_LOCKED_X | LockedAxes::TRANSLATION_LOCKED_Y)
                                .insert(DeathTimer {
                                    timer: Timer::from_seconds(1.5, TimerMode::Once),
                                });
                        }
                    }

                    // tag & despawn the fireball exactly once
                    commands.entity(fb_ent)
                        .insert(FireballHit)
                        .despawn();
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

// Handle death timers for dead entities
fn handle_death_timers(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DeathTimer, Option<&Sprite>)>,
    death_fade_query: Query<&DeathFade>,
    time: Res<Time>,
) {
    for (entity, mut timer, _) in query.iter_mut() {
        timer.timer.tick(time.delta());
        
        // Only despawn when the sprite has fully faded out (handled by DeathFade)
        // This timer just ensures the entity will eventually be cleaned up
        if timer.timer.finished() {
            // Check if the entity already has a DeathFade component
            let has_death_fade = death_fade_query.get(entity).is_ok();
            
            if !has_death_fade {
                info!("Death timer finished, entity will be despawned after fade completes");
                // Add DeathFade component to start fading if it doesn't exist yet
                commands.entity(entity).insert(DeathFade {
                    fade_timer: Timer::from_seconds(0.0, TimerMode::Once), // Start fading immediately
                    initial_alpha: 1.0,
                });
            }
        }
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

        // ← here’s the only line you need to change:
        sprite.color.set_alpha(new_alpha);
        //  (or: sprite.color = sprite.color.with_alpha(new_alpha);)

        if fade.fade_timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

