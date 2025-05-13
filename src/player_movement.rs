// Bevy examples - 2D top-down camera
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::player_code::Player;

// Player movement speed factor
const PLAYER_SPEED: f32 = 275.0;

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

// Handle player movement with keyboard input - updated for physics
pub fn character_movement(
    mut query: Query<(
        &mut Velocity,
        &mut MovementState,
        &mut FacingDirection
    ), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (
        mut velocity,
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

        // Apply movement through physics velocity instead of transform
        if is_moving {
            velocity.linvel = direction.normalize_or_zero() * PLAYER_SPEED;
        } else {
            // Stop the player when no keys are pressed
            velocity.linvel = Vec2::ZERO;
        }
    }
}