use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::player_code::Player;
use crate::orc::{OrcEnemy, OrcState};

const ORC_SPEED: f32 = 80.0;
const ATTACK_RANGE: f32 = 80.0;
const ATTACK_ANIM_DURATION: f32 = 0.5; // 5 frames at 10 FPS
const ATTACK_COOLDOWN: f32 = 1.0;      // 1 second idle after attack

pub struct OrcMovementPlugin;
impl Plugin for OrcMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, orc_movement_system)
           .add_systems(Update, orc_init_system);
    }
}

// New system to ensure orcs start with proper initial state
fn orc_init_system(
    mut query: Query<(&mut OrcEnemy, &mut Sprite), Added<OrcEnemy>>,
) {
    for (mut orc, _sprite) in query.iter_mut() {
        // Newly spawned orcs start in idle state
        orc.state = OrcState::Idle;
        orc.attack_cooldown_timer = 0.0;
    }
}

fn orc_movement_system(
    mut query: Query<(&Transform, &mut Velocity, &mut OrcEnemy, &mut Sprite)>,
    player_q: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    // Get player position
    let player_pos = if let Ok(tf) = player_q.get_single() {
        tf.translation.truncate()
    } else {
        return;
    };

    let dt = time.delta_secs();
    let max_timer = ATTACK_ANIM_DURATION + ATTACK_COOLDOWN;

    for (transform, mut vel, mut orc, mut sprite) in query.iter_mut() {
        let orc_pos = transform.translation.truncate();
        let to_player = player_pos - orc_pos;
        let dist = to_player.length();

        // Flip sprite based on horizontal direction threshold
        if to_player.x.abs() > 5.0 {
            sprite.flip_x = to_player.x < 0.0;
        }

        // Clamp and decrement timer
        orc.attack_cooldown_timer = orc.attack_cooldown_timer.min(max_timer);
        orc.attack_cooldown_timer = (orc.attack_cooldown_timer - dt).max(0.0);
        let timer = orc.attack_cooldown_timer;

        // 1) If in attack animation phase (timer > cooldown)
        if timer > ATTACK_COOLDOWN {
            vel.linvel = Vec2::ZERO;
            orc.state = OrcState::Attacking;
            continue;
        }

        // 2) If in cooldown phase (0 < timer <= cooldown)
        if timer > 0.0 {
            vel.linvel = Vec2::ZERO;
            orc.state = OrcState::Idle;
            continue;
        }

        // 3) Timer == 0: decide new action based on distance
        if dist <= ATTACK_RANGE {
            // Start new attack cycle
            orc.attack_cooldown_timer = max_timer;
            vel.linvel = Vec2::ZERO;
            orc.state = OrcState::Attacking;
        } else {
            // Chase player
            let dir = to_player.normalize_or_zero();
            vel.linvel = dir * ORC_SPEED;
            orc.state = OrcState::Walking;
        }
    }
}