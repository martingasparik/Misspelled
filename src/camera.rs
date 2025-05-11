use bevy::prelude::*;
use crate::player_code::Player;

/// How quickly the camera should snap to the desired player location
const CAMERA_DECAY_RATE: f32 = 2.0;

#[derive(Component)]
pub struct SmoothCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        SmoothCamera,
    ));
}

/// Update the camera position by smoothly tracking the player
pub fn update_camera(
    mut camera_query: Query<&mut Transform, (With<SmoothCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<SmoothCamera>)>,
    time: Res<Time>,
) {
    // Get single camera transform or return early if none exists
    let mut camera_transform = match camera_query.get_single_mut() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    // Get single player transform or return early if none exists
    let player_transform = match player_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    let Vec3 { x, y, .. } = player_transform.translation;
    let target_position = Vec3::new(x, y, camera_transform.translation.z);

    // Apply smooth tracking using lerp
    camera_transform.translation = camera_transform.translation.lerp(
        target_position,
        CAMERA_DECAY_RATE * time.delta_secs()
    );
}