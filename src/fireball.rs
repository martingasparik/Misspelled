use bevy::prelude::*;
use crate::spell::{SpellType, SpellCastEvent};
use crate::movement::Player;
use std::time::Duration;
use crate::animation::AnimationConfig;
use std::f32::consts::{PI,TAU};

pub const FIRST_INDEX: usize = 0;
pub const LAST_INDEX: usize = 11;




#[derive(Component)]
pub struct Fireball{
    piercing: bool,
    pub damage: f32,
    timer: Timer,
}

impl Default for Fireball {
    fn default() -> Self {
        Self {
            piercing: false,
            damage: 10.0,
            timer: Timer::from_seconds(5.0,TimerMode::Once),
        }
    }
}
#[derive(Resource, Default)]
pub struct FireballAssets {
    pub flying: Handle<Image>,
    pub impact: Handle<Image>,
}



fn spawn_fireball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("spells/fireball_1.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32,32),6,2,None,None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mid_air_fireball = AnimationConfig::new(FIRST_INDEX,LAST_INDEX, 12);

    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: FIRST_INDEX,
            }),
            ..default()
        },
        Transform::from_translation(Vec3::splat(5.0)),


    ));
}
// === Move Fireballs Forward and Trigger Impact ===

/*fn move_fireballs(
    mut commands: Commands,
    mut fireballs: Query<(Entity, &mut Transform), With<Fireball>>,
    time: Res<Time>,
    fireball_assets: Res<FireballAssets>,
) {
    let speed = 300.0;

    for (entity, mut transform) in fireballs.iter_mut() {
        transform.translation.x += speed * time.delta_seconds();

        if transform.translation.x > 600.0 {
            // Despawn fireball
            commands.entity(entity).despawn();

            // Spawn impact with timer
            commands.spawn((
                SpriteBundle {
                    texture: fireball_assets.impact.clone(),
                    transform: *transform,
                    ..default()
                },
                FireballImpact,
                Timer::from_seconds(0.4, TimerMode::Once),
            ));
        }
    }
}

// === Despawn Impact After Short Delay ===

fn handle_fireball_impacts()
    mut commands: Commands,
    time: Res<Time>,
    mut impacts: Query<(Entity, &mut Timer), With<FireballImpact>>,
) {
    for (entity, mut timer) in impacts.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}*/
