// Bevy examples - 2D top-down camera
use bevy::prelude::*;
use crate::animation::{AnimationConfig, PlayerState, FIRST_IDLE, LAST_IDLE, FPS_IDLE};

// Player movement speed factor
const PLAYER_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct FacingDirection {
    pub facing_right: bool,
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum MovementState {
    Idle,
    Moving,
}
impl Default for MovementState {
    fn default() -> Self {
        MovementState::Idle
    }
}

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
        FacingDirection {facing_right: true},
        MovementState::Idle,
        PlayerState::Idle,
        idle_animation_config,
    ));
}

// Handle player movement with keyboard input
pub fn character_movement(
    mut query: Query<(
        &mut Transform,
        &mut MovementState, 
        &mut FacingDirection
    ), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (
        mut transform, 
        mut movement_state, 
        mut facing
    ) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        // Where am I going?
        if input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        // If there is any direction there is a movement
        let is_moving = direction != Vec2::ZERO;

        // Update movement state for animation purposes
        *movement_state = if is_moving {
            MovementState::Moving
        } else {
            MovementState::Idle
        };

        // Update facing direction if need be updated
        if direction.x != 0.0 {
            facing.facing_right = direction.x > 0.0;
        }

        // Apply sprite movement
        transform.translation += (direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs()).extend(0.0);

    }
}

// System to flip sprites based on facing direction
pub fn update_sprite_direction(
    mut query: Query<(&FacingDirection, &mut Sprite)>,
) {
    for (facing, mut sprite) in query.iter_mut() {
        sprite.flip_x = !facing.facing_right;
    }
}