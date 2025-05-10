use bevy::prelude::*;
use crate::orc::OrcEnemy;
use crate::orc::OrcState;
use crate::animation::AnimationConfig;

pub struct OrcSpritePlugin;
impl Plugin for OrcSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_orc_animation);
    }
}

fn update_orc_animation(
    mut query: Query<(&mut AnimationConfig, &mut Sprite, &OrcEnemy)>,
) {
    for (mut config, mut sprite, orc) in query.iter_mut() {
        // Update animation indices based on state
        match orc.state {
            OrcState::Idle => {
                if config.first_sprite_index != 0 {
                    *config = AnimationConfig::new(0, 5, 8);
                    // Reset the current frame to the first frame of this animation
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = 0;
                    }
                }
            },
            OrcState::Walking => {
                if config.first_sprite_index != 8 {
                    *config = AnimationConfig::new(8, 15, 12);
                    // Reset the current frame to the first frame of this animation
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = 8;
                    }
                }
            },
            OrcState::Attacking => {
                if config.first_sprite_index != 16 {
                    *config = AnimationConfig::new(16, 21, 15);
                    // Reset the current frame to the first frame of this animation
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = 16;
                    }
                }
            },
            OrcState::Hurt => {
                if config.first_sprite_index != 32 {
                    *config = AnimationConfig::new(32, 39, 10);
                    // Reset the current frame to the first frame of this animation
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = 32;
                    }
                }
            },
            OrcState::Dying => {
                if config.first_sprite_index != 40 {
                    *config = AnimationConfig::new(40, 47, 8);
                    // Reset the current frame to the first frame of this animation
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = 40;
                    }
                }
            },
        }
    }
}