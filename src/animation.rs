use bevy::prelude::*;
use std::time::Duration;

// Animation constants
pub const CHARACTER_OFFSET: usize = 5;
pub const FIRST_IDLE: usize = 9*CHARACTER_OFFSET;
pub const LAST_IDLE: usize = FIRST_IDLE+3;
pub const FIRST_RUNNING: usize = LAST_IDLE+1;
pub const LAST_RUNNING: usize = FIRST_RUNNING+3;
pub const FPS_IDLE: u8 = 8;
pub const FPS_RUNNING: u8 = 12;

#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub enum PlayerState {
    Idle,
    Running,
}
impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idle
    }
}

#[derive(Component, Clone)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub frame_timer: Timer,
    pub current_frame: usize,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            frame_timer: Self::timer_from_fps(fps),
            current_frame: first,
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Repeating)
    }
}

// Handle animations
pub fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite)>
) {
    for (mut config, mut sprite) in &mut query {
        // Tick the animation timer
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the correct amount of time (calculated from fps in anim. config)
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                // If the animation needs to start again
                if atlas.index >= config.last_sprite_index {
                    atlas.index = config.first_sprite_index;
                } else {
                    atlas.index += 1;
                }

                // Update the current frame tracker
                config.current_frame = atlas.index;
            }
        }
    }
}

// Animation state management system
pub fn update_animation_state(
    mut query: Query<(&mut PlayerState, &mut AnimationConfig, &crate::movement::MovementState)>,
) {
    for (
        mut player_state, 
        mut config, 
        movement_state
    ) in query.iter_mut() {
        let current_state = *player_state;
        let is_moving = *movement_state == crate::movement::MovementState::Moving;

        match (current_state, is_moving) {
            (PlayerState::Idle, true) => {
                // Change to running animation
                *player_state = PlayerState::Running;
                *config = AnimationConfig::new(FIRST_RUNNING, LAST_RUNNING, FPS_RUNNING);
            },
            (PlayerState::Running, false) => {
                // Change to idle animation
                *player_state = PlayerState::Idle;
                *config = AnimationConfig::new(FIRST_IDLE, LAST_IDLE, FPS_IDLE);
            },
            _ => {} // No state change needed
        }
    }
}