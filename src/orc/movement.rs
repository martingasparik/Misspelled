use bevy::prelude::* ;
use bevy_rapier2d::prelude::* ;
use crate::movement::Player;
use crate::orc::OrcEnemy;
use crate::orc::OrcState;

const ORC_SPEED: f32 = 80.0;
const ATTACK_RANGE: f32 = 50.0;
const ATTACK_COOLDOWN: f32 = 1.0;

pub struct OrcMovementPlugin;
impl Plugin for OrcMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, orc_movement_system);
    }
}

fn orc_movement_system(
    mut query: Query<(&Transform, &mut Velocity, &mut OrcEnemy, &mut Sprite)>,
    player_q: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let player_pos = match player_q.get_single() {
        Ok(tf) => tf.translation.truncate(),
        Err(_) => return,
    };

    for (transform, mut vel, mut orc, mut sprite) in query.iter_mut() {
        let orc_pos = transform.translation.truncate();
        let to_player = player_pos - orc_pos;
        let dist = to_player.length();
        
        // Update sprite flip based on direction to player
        // Only flip if the x-component is significant enough
        if to_player.x.abs() > 5.0 { // Add a small threshold to prevent flipping when nearly vertical
            // If player is to the right, don't flip (face right)
            // If player is to the left, flip horizontally (face left)
            sprite.flip_x = to_player.x < 0.0;
        }
        // If x is very small (player is nearly directly above/below), keep current orientation
        
        if dist > 0.1 { // Small threshold to prevent jitter
            let dir = to_player.normalize_or_zero();
            
            if dist > ATTACK_RANGE {
                vel.linvel = dir * ORC_SPEED;
                orc.state = OrcState::Walking;
            } else {
                vel.linvel = Vec2::ZERO;
                orc.state = OrcState::Attacking;
            }
        }
    }
}