use bevy::prelude::*;
use std::time::Duration;
use crate::fireball::DeathFade;

#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub enum SpriteState {
    Idle,
    Running,
}
impl Default for SpriteState {
    fn default() -> Self {
        SpriteState::Idle
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
    mut query: Query<(&mut AnimationConfig, &mut Sprite), Without<DeathFade>>
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