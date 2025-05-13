use bevy::prelude::Commands;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const WALL_WIDTH: f32 = 3880.0;
const WALL_HEIGHT: f32 = 120.0;

#[derive(Component)]
struct LibraryBackground;
pub fn setup_world(
    mut commands: Commands,
    texture: Handle<Image>,

) {
    commands
        .spawn((
        Sprite {
            image: texture,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 32.0, -1.0), // Position at z=-1 so it's behind other entities
            scale: Vec3::splat(2.0),
            ..default()
        },
        LibraryBackground,
    ));

    //Top Wall collider
    commands.spawn((
        Collider::cuboid(WALL_WIDTH, WALL_HEIGHT),
        Sprite {
            color: Color::srgba(1.0, 0.0, 0.0, 0.2),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, WALL_HEIGHT+32.0, 0.0),
            ..Default::default()
        }
    ));
    // Bottom
    commands.spawn((
        Collider::cuboid(WALL_WIDTH, WALL_HEIGHT),
        Sprite {
            color: Color::srgba(1.0, 0.0, 0.0, 0.2),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, -(2.*WALL_HEIGHT + 32.0), 0.0),
            ..Default::default()
        }
    ));
    //Left
    commands.spawn((
        Collider::cuboid(WALL_HEIGHT, WALL_HEIGHT),
        Sprite {
            color: Color::srgba(1.0, 0.0, 0.0, 0.2),
            ..default()
        },    
        Transform {
            translation: Vec3::new(-WALL_WIDTH, -32.0, 0.0),
            ..Default::default()
        }
    ));
    //Right
    commands.spawn((
        Collider::cuboid(WALL_HEIGHT, WALL_HEIGHT),
        Sprite {
            color: Color::srgba(1.0, 0.0, 0.0, 0.2),
            ..default()
        },
        Transform {
            translation: Vec3::new(WALL_WIDTH, -32.0, 0.0),
            ..Default::default()
        }
    ));
}