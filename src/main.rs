use bevy::prelude::*;
use bevy::sprite::TextureAtlasLayout;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::render::{RapierDebugRenderPlugin, DebugRenderContext};

mod audio;
mod animation;
mod camera;
mod world;
mod ui_hp_display;
mod ui_orc_counter;
mod player_movement;
mod player_code;
mod player_animation;
mod orc;
mod spell;
mod spellbook;
mod blink;
mod fireball;
mod shield;

use audio::AudioPlugin;
use ui_hp_display::{HealthDisplayPlugin};
use ui_orc_counter::OrcDeathCounterPlugin;
use crate::player_code::{ PlayerHealthPlugin};
use orc::OrcPlugin;
use shield::ShieldPlugin;

fn main() {
    App::new()
        // ——— Bevy built-in plugins ———
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Misspelled".into(),
                        resolution: (1200.0, 900.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        )

        // ——— Rapier 2D physics ———
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())   // Core physics
        .insert_resource(DebugRenderContext {
            enabled: true,     // Turn on all colliders/hurtboxes
            ..default()
        })
        .add_event::<CollisionEvent>()
        //.add_plugins(PlayerPhysicsPlugin)
        .add_plugins(PlayerHealthPlugin)

        // ——— Audio system ———
        .add_plugins(AudioPlugin) // Add the audio plugin

        // ——— Health display system ———
        .add_plugins(HealthDisplayPlugin) // Add the health display plugin

        // ——— Orc enemy bundle ———
        .add_plugins(OrcPlugin)

        // ——— Spell-casting systems ———
        .add_event::<spell::SpellCastEvent>()
        .add_plugins(spell::StackSpellSystemPlugin)
        .add_plugins(fireball::FireballPlugin)
        .add_plugins(blink::BlinkPlugin)
        .add_plugins(ShieldPlugin)
        .add_plugins(spellbook::SpellbookPlugin)

        .add_plugins(OrcDeathCounterPlugin)

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


    // Load and spawn the library background
    let library = asset_server.load("library.png");
    world::setup_world(commands.reborrow(), library);
    
    
    // Create the texture atlas for character sprite
    // Layout: 16x32 sprites, 9 columns, 10 rows
    let texture = asset_server.load("characters_atlas.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 32), 9, 10, None, None);
    let texture_atlas_layout = atlas_layouts.add(layout);
    // Spawn the player
    player_code::setup_player(commands, texture, texture_atlas_layout);

    println!("Game setup complete");
}