mod animation;
mod camera;
mod spell;
mod orc;

mod player_movement;
mod player_code;
mod player_animation;
mod fireball;

use bevy::prelude::*;
use bevy::math::UVec2;
use bevy::prelude::TextureAtlasLayout;
use bevy_rapier2d::prelude::*;

// Import the debug‐renderer types from the `render` module:
use bevy_rapier2d::render::{RapierDebugRenderPlugin, DebugRenderContext};

use orc::OrcPlugin;

fn main() {
    App::new()
        // ——— Bevy built-in plugins ———
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Misspelled".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        )

        // ——— Rapier 2D physics ———
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())   // Core physontentReference[oaicite:5]{index=5}
        .add_plugins(RapierDebugRenderPlugin::default())             // Debug overlay :contentReference[oaicite:6]{index=6}
        .insert_resource(DebugRenderContext {
            enabled: true,     // Turn on all colliders/hurtboxes :contentReference[oaicite:7]{index=7}
            ..default()
        })
        .add_event::<CollisionEvent>()

        // ——— Orc enemy bundle ———
        .add_plugins(OrcPlugin)

        // ——— Spell-casting systems ———
        .add_event::<spell::SpellCastEvent>()
        .add_plugins(spell::StackSpellSystemPlugin)
        .add_plugins(fireball::FireballPlugin)

        // ——— Startup & Update loops ———
        .add_systems(Startup, setup_game)
        .add_systems(
            Update,
            (
                player_movement::character_movement,
                player_animation::update_sprite_direction,
                player_animation::update_animation_state,
                animation::execute_animations,
                camera::update_camera,
            ),
        )
        .run();
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Setup camera
    camera::setup_camera(commands.reborrow());

    // Create the texture atlas
    let texture = asset_server.load("characters_atlas.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 32), 9, 10, None, None);
    let texture_atlas_layout = atlas_layouts.add(layout);

    // Spawn the player
    player_code::setup_player(commands, texture, texture_atlas_layout);

    println!("Game setup complete");
}
