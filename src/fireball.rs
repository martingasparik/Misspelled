use bevy::prelude::*;

use bevy::time::Time;
use bevy::time::Real;


use crate::spell::{SpellType, SpellCastEvent};
use crate::player_code::Player;
use crate::player_movement::FacingDirection;
use crate::animation::AnimationConfig;

pub const FIREBALL_SPEED: f32 = 200.0;
pub const FIREBALL_LIFETIME: f32 = 5.0; 
pub const FIREBALL_DAMAGE: f32 = 10.0;
pub const FIREBALL_FIRST_INDEX: usize = 0;
pub const FIREBALL_LAST_INDEX: usize = 11;
pub const FIREBALL_FPS: u8 = 12;

// Component to mark entities as enemies
#[derive(Component)]
pub struct Enemy;

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
                    Name::new("Fireball"),
                ));

                // Print debug info
                println!("Fireball cast in direction: {:?}", direction);
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
    mut fireball_query: Query<(Entity, &Transform, &mut Fireball)>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (fireball_entity, fireball_transform, mut fireball) in fireball_query.iter_mut() {
        if fireball.is_disabled() {
            continue;
        }

        // Check collisions with enemies
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            // Simple circle collision detection
            let distance = fireball_transform.translation.distance(enemy_transform.translation);

            // Assuming the combined radius of fireball and enemy is 25.0 units
            if distance < 25.0 {
                // Apply damage to enemy (you'd typically modify a Health component)
                println!("Enemy hit for {} damage!", fireball.damage);


                fireball.disable();
                
                if fireball.is_disabled() {
                    commands.entity(fireball_entity).despawn();
                }
                if !fireball.piercing {
                    break;
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
/*
// TO DO Implement the execution
pub fn execute_fireball(
    entity: Entity,
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    player_query: &Query<(&Transform, &FacingDirection), With<Player>>,
) {
    // This is a utility function if you need to programmatically cast a fireball
    // It could be called from other systems or events
    if let Ok((player_transform, facing)) = player_query.get_single() {
        let direction = if facing.facing_right {
            Vec2::new(1.0, 0.0)
        } else {
            Vec2::new(-1.0, 0.0)
        };

        // Rest of fireball spawning logic...
        println!("Programmatically casting fireball");
    }
}
*/