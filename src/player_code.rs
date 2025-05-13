use bevy::prelude::{default, Commands, Component, Sprite, TextureAtlas, TextureAtlasLayout, Transform};
use bevy::asset::Handle;
use bevy::image::Image;
use bevy::math::Vec3;
use bevy::math::Vec2;
use bevy_rapier2d::prelude::Collider;
use crate::animation::{AnimationConfig, SpriteState};
use crate::player_animation::{FIRST_IDLE, FPS_IDLE, LAST_IDLE};
use crate::player_movement::{FacingDirection, MovementState};
//use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::orc::collision::HurtHitbox;

#[derive(Component)]
pub struct Player;

/// Health component for entities
#[derive(Component)]
/// Tracks current health; used by player and enemies
pub struct Health {
    pub health: f32,
}
impl Health {
    /// Creates a new Health with the given amount
    pub fn new(amount: f32) -> Self {
        Health { health: amount }
    }
}

#[derive(Component)]
pub struct Shield{
    pub shield: f32,
}
impl Shield {
    pub fn new(amount: f32) -> Self {Shield {shield: amount}}
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
    commands.spawn((
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
    ));
}

