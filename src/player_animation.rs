use bevy::prelude::{Query, Sprite};
use crate::animation::{AnimationConfig, SpriteState};
use crate::player_movement::FacingDirection;

// Animation constants
pub const CHARACTER_OFFSET: usize = 5;
pub const FIRST_IDLE: usize = 9*CHARACTER_OFFSET;
pub const LAST_IDLE: usize = FIRST_IDLE+3;
pub const FIRST_RUNNING: usize = LAST_IDLE+1;
pub const LAST_RUNNING: usize = FIRST_RUNNING+3;
pub const FPS_IDLE: u8 = 8;
pub const FPS_RUNNING: u8 = 12;

// Animation state management system
pub fn update_animation_state(
    mut query: Query<(&mut SpriteState, &mut AnimationConfig, &crate::player_movement::MovementState)>,
) {
    for (
        mut player_state, 
        mut config, 
        movement_state
    ) in query.iter_mut() {
        let current_state = *player_state;
        let is_moving = *movement_state == crate::player_movement::MovementState::Moving;

        match (current_state, is_moving) {
            (SpriteState::Idle, true) => {
                // Change to running animation
                *player_state = SpriteState::Running;
                *config = AnimationConfig::new(FIRST_RUNNING, LAST_RUNNING, FPS_RUNNING);
            },
            (SpriteState::Running, false) => {
                // Change to idle animation
                *player_state = SpriteState::Idle;
                *config = AnimationConfig::new(FIRST_IDLE, LAST_IDLE, FPS_IDLE);
            },
            _ => {} // No state change needed
        }
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