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
    let orc_entity = commands.spawn((
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
        
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED, // This prevents ALL rotation
        Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.0, // Explicitly set angular velocity to 0
        },
        Damping {
            linear_damping: 2.0,
            angular_damping: 10.0, // High value to kill any rotation quickly
        },

        // Game logic components
        OrcEnemy::new(10.0, 1.0), // This now includes attack timers
        Health::new(10.0),
        
        // Animation components
        AnimationConfig::new(0, 7, 10), // Idle animation 
        
        // Physics components for collision
        Collider::capsule(  
            Vec2::new(0.0, -5.0), // Center of the collider
            Vec2::new(0.0, 5.0),
            4.0,
        ),
        ActiveEvents::COLLISION_EVENTS,
        CollisionGroups::new(
            Group::GROUP_1, // Body is in group 1
            Group::GROUP_2, // Can collide with group 2 (environment)
        ),

        Name::new(format!("Orc-{:?}", spawn_pos)),
    )).id();
    
    info!("Spawned orc {:?} at {:?}", orc_entity, spawn_pos);
}