mod assets;
mod spawn;
mod movement;
mod sprite;
mod collision;

pub use assets::OrcAssetPlugin;
pub use spawn::OrcSpawnPlugin;
pub use movement::OrcMovementPlugin;
pub use sprite::OrcSpritePlugin;
pub use collision::OrcCollisionPlugin;

use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct OrcEnemy {
    pub health: f32,
    pub damage: f32,
    pub state: OrcState,
}

impl OrcEnemy {
    pub fn new(health: f32, damage: f32) -> Self {
        Self {
            health,
            damage,
            state: OrcState::Idle,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum OrcState {
    Idle,
    Walking,
    Attacking,
    Hurt,
    Dying,
}

// Main Orc plugin that bundles everything together
pub struct OrcPlugin;
impl Plugin for OrcPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(OrcAssetPlugin)
            .add_plugins(OrcSpawnPlugin)
            .add_plugins(OrcMovementPlugin)
            .add_plugins(OrcSpritePlugin)
            .add_plugins(OrcCollisionPlugin);
    }
}