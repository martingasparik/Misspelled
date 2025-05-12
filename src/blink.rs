use bevy::prelude::*;

use bevy::time::{Time, Real};

use crate::spell::{SpellType, SpellCastEvent};
use crate::player_code::Player;
use crate::player_movement::FacingDirection;
use crate::animation::AnimationConfig;

pub const BLINK_DISTANCE: f32 = 150.0;
pub const BLINK_ANIMATION_FIRST_INDEX: usize = 0;
pub const BLINK_ANIMATION_LAST_INDEX: usize = 5;
pub const BLINK_ANIMATION_FPS: u8 = 15;
pub const BLINK_PHASE_DURATION: f32 = 0.3; // Duration for each phase in seconds

// Component to mark when a blink animation is in progress
#[derive(Component)]
pub struct BlinkingEffect {
    pub phase: BlinkPhase,
    pub target_position: Vec3,
    pub timer: Timer,
}

// Enum to track the current phase of the blink
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlinkPhase {
    Disappearing,
    Moving,
    Reappearing,
    Complete,
}

// Plugin for blink spell systems
pub struct BlinkPlugin;

impl Plugin for BlinkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_blink_casting,
            update_blink_animation,
        ));
    }
}

// System to handle the blink spell casting event
fn handle_blink_casting(
    mut commands: Commands,
    mut spell_events: EventReader<SpellCastEvent>,
    mut player_query: Query<(Entity, &Transform, &FacingDirection, &mut Sprite), With<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for event in spell_events.read() {
        if let SpellType::Blink = event.spell_type {
            if let Ok((player_entity, player_transform, facing, mut sprite)) = player_query.get_single_mut() {
                let direction = if facing.facing_right {
                    Vec3::new(1.0, 0.0, 0.0)
                } else {
                    Vec3::new(-1.0, 0.0, 0.0)
                };

                // Calculate target position for the blink
                let target_position = player_transform.translation + direction * BLINK_DISTANCE;

                // Set up the blink effect component
                let blink_effect = BlinkingEffect {
                    phase: BlinkPhase::Disappearing,
                    target_position,
                    timer: Timer::from_seconds(BLINK_PHASE_DURATION, TimerMode::Once),
                };

                // Load blink animation texture atlas
                let blink_texture = asset_server.load("spells/10.png"); // Adjust path to your blink texture

                // Create a texture atlas layout for the 32x32 blink sprite
                let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 3, 2, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                // Save previous texture information to restore later
                let prev_texture = sprite.image.clone();
                let prev_atlas = sprite.texture_atlas.clone();

                // Update player sprite to use blink animation
                sprite.image = blink_texture;
                sprite.texture_atlas = Some(TextureAtlas {
                    layout: texture_atlas_layout,
                    index: BLINK_ANIMATION_FIRST_INDEX,
                });

                // Adjust sprite scale to match original 16x16 size
                sprite.custom_size = Some(Vec2::new(16.0, 16.0));

                // Create animation configuration for the blink
                let blink_animation = AnimationConfig::new(
                    BLINK_ANIMATION_FIRST_INDEX,
                    BLINK_ANIMATION_LAST_INDEX,
                    BLINK_ANIMATION_FPS,
                );

                // Apply blink effect and animation to player
                commands.entity(player_entity)
                    .insert(blink_effect)
                    .insert(blink_animation)
                    .insert(PreviousSprite {
                        texture: prev_texture,
                        atlas: prev_atlas,
                    });

                println!("Blink spell cast! Target position: {:?}", target_position);
            }
        }
    }
}

// Component to store the previous sprite to restore after blinking
#[derive(Component)]
struct PreviousSprite {
    texture: Handle<Image>,
    atlas: Option<TextureAtlas>,
}

// System to update the blink animation and movement
fn update_blink_animation(
    mut commands: Commands,
    mut player_query: Query<(
        Entity,
        &mut Transform,
        &mut BlinkingEffect,
        &mut Sprite,
        &PreviousSprite
    )>,
    time: Res<Time<Real>>,
) {
    for (entity, mut transform, mut blink_effect, mut sprite, prev_sprite) in player_query.iter_mut() {
        // Update the timer
        blink_effect.timer.tick(time.delta());

        match blink_effect.phase {
            BlinkPhase::Disappearing => {
                // First phase: player disappears (handled by animation)
                if blink_effect.timer.finished() {
                    blink_effect.phase = BlinkPhase::Moving;
                    blink_effect.timer.reset();
                }
            }
            BlinkPhase::Moving => {
                // Second phase: instantly move player to new position
                transform.translation = blink_effect.target_position;
                blink_effect.phase = BlinkPhase::Reappearing;
                blink_effect.timer.reset();
            }
            BlinkPhase::Reappearing => {
                // Third phase: player reappears (handled by animation)
                if blink_effect.timer.finished() {
                    blink_effect.phase = BlinkPhase::Complete;
                }
            }
            BlinkPhase::Complete => {
                // Restore the player's original texture/sprite
                sprite.image = prev_sprite.texture.clone();
                sprite.texture_atlas = prev_sprite.atlas.clone();

                // Remove custom size to ensure original scaling
                sprite.custom_size = None;

                // Clean up - remove the blink components
                commands.entity(entity)
                    .remove::<BlinkingEffect>()
                    .remove::<AnimationConfig>()
                    .remove::<PreviousSprite>();
            }
        }
    }
}