mod animation;
mod movement;
mod camera;
mod spell;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())// pixelated
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Misspelled".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .build(),
        )
        .add_systems(Startup, setup_game)
        .add_systems(
            Update,
            (
                // Movement and control systems
                movement::character_movement,
                movement::update_sprite_direction,

                // Animation systems
                animation::update_animation_state,
                animation::execute_animations,

                // Camera systems
                camera::update_camera,

                spell::execute_spells,
            )
        )
        // Spell casting systems
        .add_event::<spell::SpellCastEvent>() // Register spell event
        .add_plugins(spell::StackSpellSystemPlugin)
        .run();
}

// Main setup function that initializes all game entities
fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Setup camera
    camera::setup_camera(commands.reborrow());
    

    // Create the texture atlas for character sprite 
    // Layout: 16x32 sprites, 9 columns, 10 rows
    let texture = asset_server.load("characters_atlas.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 32), 9, 10, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Setup player entity
    movement::setup_player(commands, texture, texture_atlas_layout);
}