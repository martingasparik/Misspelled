use bevy::{
    prelude::*,
    time::Time,
    time::Real,
};
use crate::spell::{SpellCastEvent, SpellType};
use crate::player_code::Player;

// Constants for the spellbook display
const SPELLBOOK_DISPLAY_TIME: f32 = 7.5; // How long the spellbook stays visible
const SPELLBOOK_OFFSET_Y: f32 = 80.0; // Offset from player position
const SPELLBOOK_Z_LAYER: f32 = 10.0; // Make sure spellbook displays above other elements

// Plugin for managing the spellbook display
pub struct SpellbookPlugin;

impl Plugin for SpellbookPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpellbookState>()
            .add_systems(Update, (
                handle_spellbook_events,
                update_spellbook_display,
                update_spellbook_position,
            ));
    }
}

// Resource to track spellbook visibility state
#[derive(Resource)]
pub struct SpellbookState {
    visible: bool,
    timer: Timer,
    entity: Option<Entity>,
}

impl Default for SpellbookState {
    fn default() -> Self {
        Self {
            visible: false,
            timer: Timer::from_seconds(SPELLBOOK_DISPLAY_TIME, TimerMode::Once),
            entity: None,
        }
    }
}

// Component to mark the spellbook entity
#[derive(Component)]
pub struct Spellbook;

// Handle spellbook cast events
fn handle_spellbook_events(
    mut commands: Commands,
    mut spell_events: EventReader<SpellCastEvent>,
    mut spellbook_state: ResMut<SpellbookState>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    for event in spell_events.read() {
        if event.spell_type == SpellType::Spellbook {
            // If there's already a spellbook, despawn it
            if let Some(entity) = spellbook_state.entity {
                commands.entity(entity).despawn();
            }

            // Get player position for spawning the spellbook
            if let Ok(player_transform) = player_query.get_single() {
                // Load spellbook texture
                let spellbook_texture = asset_server.load("UI/Spellbook.png");

                // Calculate spawn position above player
                let spawn_position = player_transform.translation +
                    Vec3::new(0.0, SPELLBOOK_OFFSET_Y, SPELLBOOK_Z_LAYER);

                // Spawn the spellbook entity
                let spellbook_entity = commands.spawn((
                    Sprite {
                        image: spellbook_texture,
                        ..default()
                    },
                    Transform::from_translation(spawn_position)
                        .with_scale(Vec3::splat(2.0)), // Scale appropriately for your game
                    GlobalTransform::default(),
                    Name::new("Spellbook"),
                    Spellbook,
                )).id();

                // Store the entity and reset the timer
                spellbook_state.entity = Some(spellbook_entity);
                spellbook_state.visible = true;
                spellbook_state.timer.reset();

                println!("Spellbook opened!");
            }
        }
    }
}

// Update the spellbook display based on timer
fn update_spellbook_display(
    mut commands: Commands,
    time: Res<Time<Real>>,
    mut spellbook_state: ResMut<SpellbookState>,
) {
    // If the spellbook is visible, update the timer
    if spellbook_state.visible {
        spellbook_state.timer.tick(time.delta());

        // If timer is finished, hide the spellbook
        if spellbook_state.timer.finished() {
            spellbook_state.visible = false;

            // Despawn the entity if it exists
            if let Some(entity) = spellbook_state.entity.take() {
                commands.entity(entity).despawn();
                println!("Spellbook closed!");
            }
        }
    }
}

// Update the spellbook position to follow the player
// Fix: Using ParamSet to avoid Transform conflicts
fn update_spellbook_position(
    mut param_set: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Spellbook>>
    )>,
) {
    // First, get the player position
    let player_pos = {
        let player_query = param_set.p0();
        if let Ok(player_transform) = player_query.get_single() {
            player_transform.translation
        } else {
            return; // No player found
        }
    };

    // Then update the spellbook position
    let mut spellbook_query = param_set.p1();
    for mut spellbook_transform in spellbook_query.iter_mut() {
        // Position the spellbook above the player
        spellbook_transform.translation.x = player_pos.x;
        spellbook_transform.translation.y = player_pos.y + SPELLBOOK_OFFSET_Y;
    }
}