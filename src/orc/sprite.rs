use bevy::prelude::*;
use crate::orc::OrcEnemy;
use crate::orc::OrcState;
use crate::animation::AnimationConfig;
use crate::fireball::DeathFade;
use crate::orc::collision::HurtHitbox;

pub struct OrcSpritePlugin;
impl Plugin for OrcSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_orc_animation)
           .add_systems(Update, handle_death_animation_completion);
    }
}

fn update_orc_animation(
    mut query: Query<(&mut AnimationConfig, &mut Sprite, &OrcEnemy), Without<DeathFade>>,
) {
    for (mut config, mut sprite, orc) in query.iter_mut() {
        // Update animation indices based on state
        match orc.state {
            OrcState::Idle => {
                if config.first_sprite_index != 0 {
                    *config = AnimationConfig::new(0, 5, 8);
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = 0;
                    }
                }
            },
            OrcState::Walking => {
                if config.first_sprite_index != 8 {
                    *config = AnimationConfig::new(8, 15, 12);
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = 8;
                    }
                }
            },
            OrcState::Attacking => {
                if config.first_sprite_index != 16 {
                    *config = AnimationConfig::new(16, 21, 10);
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = 16;
                    }
                }
            },
            OrcState::Hurt => {
                if config.first_sprite_index != 32 {
                    *config = AnimationConfig::new(32, 35, 10);
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = 32;
                    }
                }
            },
            OrcState::Dying => {
                if config.first_sprite_index != 40 {
                    *config = AnimationConfig::new(40, 43, 8);
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = 40;
                    }
                }
            }
        }
    }
}

// This system detects when the death animation has completed
fn handle_death_animation_completion(
    mut commands: Commands,
    hitbox_query: Query<(Entity, &HurtHitbox)>,
    query: Query<(Entity, &Sprite, &AnimationConfig, &OrcEnemy), Without<DeathFade>>,
) {
    for (orc_entity, sprite, config, orc) in query.iter() {
        if orc.state == OrcState::Dying {
            if let Some(atlas) = &sprite.texture_atlas {
                if atlas.index >= config.last_sprite_index {
                    // 1) start the fade on the sprite
                    commands.entity(orc_entity).insert(DeathFade {
                        fade_timer: Timer::from_seconds(0.5, TimerMode::Once),
                        initial_alpha: 1.0,
                    });

                    // 2) immediately despawn its hurtboxes
                    for (hb_entity, hurtbox) in hitbox_query.iter() {
                        if hurtbox.owner == orc_entity {
                            commands.entity(hb_entity).despawn();
                        }
                    }
                }
            }
        }
    }
}