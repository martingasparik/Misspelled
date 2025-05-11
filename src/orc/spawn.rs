use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::animation::AnimationConfig;
use crate::player_code::Health;
use crate::orc::assets::OrcAssets;
use crate::orc::OrcEnemy;

pub struct OrcSpawnPlugin;
impl Plugin for OrcSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_orc_on_click);
    }
}

fn spawn_orc_on_click(
    buttons: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    orc_assets: Res<OrcAssets>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = window_query.single();
        if let Some(screen_pos) = window.cursor_position() {
            if let Ok((camera, cam_tf)) = camera_q.get_single() {
                if let Ok(world_ray) = camera.viewport_to_world(cam_tf, screen_pos) {
                    let spawn_pos = world_ray.origin.truncate().extend(0.0);
                    spawn_orc(&mut commands, &orc_assets, spawn_pos);
                }
            }
        }
    }
}

fn spawn_orc(
    commands: &mut Commands,
    assets: &OrcAssets,
    spawn_pos: Vec3,
) {
    commands.spawn((
        // Visual components
        Sprite {
            image: assets.texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.atlas.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_translation(spawn_pos).with_scale(Vec3::splat(5.0)),
        
        // Game logic components
        OrcEnemy::new(20.0, 5.0),
        Health::new(20.0),
        
        // Animation components
        AnimationConfig::new(0, 7, 10), // Idle animation 
        
        // Physics components
        Collider::ball(40.0),
        RigidBody::Dynamic,
        Velocity::default(),
        ActiveEvents::COLLISION_EVENTS,
    ));
}