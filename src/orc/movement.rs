use bevy::prelude::* ;
use bevy_rapier2d::prelude::* ;
use crate::movement::Player;
use crate::orc::OrcEnemy;
use crate::orc::OrcState;

const ORC_SPEED: f32 = 60.0;
const ATTACK_RANGE: f32 = 40.0;

pub struct OrcMovementPlugin;
impl Plugin for OrcMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, orc_movement_system);
    }
}

fn orc_movement_system(
    mut query: Query<(&Transform, &mut Velocity, &mut OrcEnemy)>,
    player_q: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    // get the player position (bail out if there's no single player)
    let player_pos = match player_q.get_single() {
        Ok(tf) => tf.translation.truncate(),
        Err(_) => return,
    };

    // move each orc toward the player (or switch to attacking)
    for (transform, mut vel, mut orc) in query.iter_mut() {
        let orc_pos = transform.translation.truncate();
        let dir = (player_pos - orc_pos).normalize_or_zero();
        let dist = player_pos.distance(orc_pos);

        if dist > ATTACK_RANGE {
            // Move toward player
            vel.linvel = dir * ORC_SPEED;

            // Only change state if we're not already walking
            if orc.state != OrcState::Walking {
                orc.state = OrcState::Walking;
            }
        } else {
            // Stop and attack
            vel.linvel = Vec2::ZERO;

            // Only change state if we're not already attacking
            if orc.state != OrcState::Attacking {
                orc.state = OrcState::Attacking;
            }
        }
    }
}