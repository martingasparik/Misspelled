mod animation;
mod movement;
mod camera;
mod spell;
mod orc;
mod fireball;
use bevy::prelude::*;
use bevy::prelude::TextureAtlasLayout;
use bevy::math::UVec2;
use bevy_rapier2d::prelude::*;
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
                })
        )

        // ——— Rapier 2D physics ———
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())


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
                movement::character_movement,
                movement::update_sprite_direction,
                animation::update_animation_state,
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
    // 1) spawn our camera
    camera::setup_camera(commands.reborrow());

    // 2) load the player atlas
    let texture = asset_server.load("characters_atlas.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 32), 9, 10, None, None);
    let texture_atlas_layout = atlas_layouts.add(layout);

    // 3) spawn the player
    movement::setup_player(commands, texture, texture_atlas_layout);
}
