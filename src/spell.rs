use bevy::{
    prelude::*,
    input::keyboard::{
        Key,
        KeyboardInput},
};
use std::process::exit; // For the "exit" spell
use crate::player_code::Player;

// Constants for display positioning
const SPELL_TEXT_OFFSET_Y: f32 = 40.0; // Offset from player

// Stack resource to hold spell characters
#[derive(Resource, Default)]
pub struct SpellStack {
    characters: Vec<char>,
    active: bool,
}
impl SpellStack {
    pub fn push(&mut self, c: char) {
        self.characters.push(c);
    }

    pub fn pop(&mut self) {
        self.characters.pop();
    }

    pub fn clear(&mut self) {
        self.characters.clear();
    }

    pub fn as_string(&self) -> String {
        self.characters.iter().collect()
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
        if self.active {
            self.clear();
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

// Components for the text display
#[derive(Component)]
pub struct SpellText;

#[derive(Component)]
pub struct SpellTextBackground;

// Enum for spell types
#[derive(Debug, Clone, Copy)]
#[derive(PartialEq)]
pub enum SpellType {
    Fireball,
    Blink,
    Shield,
    Exit,
    Spellbook,
    Unknown,
}

// Event for when a spell is cast
#[derive(Event)]
pub struct SpellCastEvent {
    pub spell_type: SpellType,
    pub spell_name: String,
}

// Plugin to organize all spell related systems
pub struct StackSpellSystemPlugin;

impl Plugin for StackSpellSystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpellStack>()
            .add_event::<SpellCastEvent>()
            .add_systems(Startup, setup_spell_system)
            .add_systems(Update, (
                handle_spell_input,
                update_spell_text,
                update_text_position,
                execute_spells,
            ));
    }
}

// Set up the visual elements for the spell system
pub fn setup_spell_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Background for the spell text
    commands.spawn((
        Sprite {
            color: Color::srgba(0.1, 0.1, 0.1, 0.7),
            custom_size: Some(Vec2::new(200.0, 35.0)), //todo: resize with text
            ..default()
        },
        Transform::from_xyz(0.0, SPELL_TEXT_OFFSET_Y, 1.0),
        Visibility::Hidden,
        SpellTextBackground,
    ));

    // Text entity for spell display
    commands.spawn((
        // Empty text that will be filled when typing
        Text2d::new(""),
        TextFont {
            font: asset_server.load("fonts/NicoClean-Monospaced.ttf"),
            font_size: 20.0,
            ..default()
        },
        // Positioning will be handled in update_text_position system
        Transform::from_xyz(0.0, SPELL_TEXT_OFFSET_Y, 2.0),
        GlobalTransform::default(),
        Visibility::Hidden,
        InheritedVisibility::default(),
        ViewVisibility::default(),

        SpellText,
    ));
}

// Handle keyboard input for spell casting
pub fn handle_spell_input(
    mut spell_stack: ResMut<SpellStack>,
    mut spell_cast_events: EventWriter<SpellCastEvent>,
    mut key_events: EventReader<KeyboardInput>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    // Toggle spell input with Space
    if kbd.just_pressed(KeyCode::Space) {
        spell_stack.toggle();
    }
    // Toggle on spell input with Enter
    if !spell_stack.is_active() && kbd.just_pressed(KeyCode::Enter) {
        spell_stack.toggle();
    }

    // Only process inputs if spell system is active
    if !spell_stack.is_active() {
        return;
    }

    // Toggle off spell input with Escape
    if kbd.just_pressed(KeyCode::Escape) {
        spell_stack.toggle();
    }

    // Process backspace - remove last character
    if kbd.just_pressed(KeyCode::Backspace) && !spell_stack.characters.is_empty() {
        spell_stack.pop();
    }

    // Process Enter - cast spell
    if kbd.just_pressed(KeyCode::Enter) {
        let spell_name = spell_stack.as_string();
        if !spell_name.is_empty() {
            let spell_type = identify_spell(&spell_name);

            // Emit spell cast event
            spell_cast_events.send(SpellCastEvent {
                spell_type,
                spell_name,
            });

            // Deactivate input
            spell_stack.toggle();
        }
    }

    // Add characters to the stack
    for key_event in key_events.read() {
        if key_event.state.is_pressed() {
            if let Key::Character(ref c) = key_event.logical_key {
                // Only add the first character of the string
                if let Some(first_char) = c.chars().next() {
                    // Only add printable characters
                    if !first_char.is_control() {
                        spell_stack.push(first_char);
                    }
                }
            }
        }
    }
}

// Update the spell text display
pub fn update_spell_text(
    spell_stack: Res<SpellStack>,
    mut text_query: Query<(&mut Text2d, &mut Visibility), With<SpellText>>,
    mut bg_query: Query<&mut Visibility, (With<SpellTextBackground>, Without<SpellText>)>,
) {
    if spell_stack.is_changed() {
        // Update text content
        if let Ok((mut text, mut text_visibility)) = text_query.get_single_mut() {
            // Update the text content directly
            **text = spell_stack.as_string();

            // Update visibility based on active state
            *text_visibility = if spell_stack.is_active() {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }

        // Update background visibility
        for mut visibility in bg_query.iter_mut() {
            *visibility = if spell_stack.is_active() {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

// Keep text centered on player
pub fn update_text_position(
    player_query: Query<&Transform, With<Player>>,
    mut text_query: Query<&mut Transform, (With<SpellText>, Without<Player>, Without<SpellTextBackground>)>,
    mut bg_query: Query<&mut Transform, (With<SpellTextBackground>, Without<Player>, Without<SpellText>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_position = player_transform.translation;

        // Update text position
        for mut transform in text_query.iter_mut() {
            transform.translation.x = player_position.x;
            transform.translation.y = player_position.y + SPELL_TEXT_OFFSET_Y;
        }

        // Update background position
        for mut transform in bg_query.iter_mut() {
            transform.translation.x = player_position.x;
            transform.translation.y = player_position.y + SPELL_TEXT_OFFSET_Y;
        }
    }
}

// Identify what spell is being cast based on input
fn identify_spell(input: &str) -> SpellType {
    match input.to_lowercase().as_str() {
        "fireball" => SpellType::Fireball,
        "blink" => SpellType::Blink,
        "shield" => SpellType::Shield,
        "exit" => SpellType::Exit,
        "spellbook" => SpellType::Spellbook,
        _ => SpellType::Unknown,
    }
}

// Execute spells based on type - now properly connected to fireball.rs
pub fn execute_spells(
    mut spell_events: EventReader<SpellCastEvent>,
) {
    for event in spell_events.read() {
        match event.spell_type {
            SpellType::Fireball => {
                // The actual fireball spawning is handled in handle_fireball_casting 
                // in fireball.rs, which responds to this event
                println!("Casting Fireball spell!");
            },
            SpellType::Blink => {
                println!("Casting Blink spell!");
            },
            SpellType::Shield => {
                println!("Casting Shield spell!");
                // TODO: Implement shield spell logic
            },
            SpellType::Exit => {
                println!("Casting Exit");
                exit(0);
            },
            SpellType::Spellbook => {
                println!("Casting Spellbook spell!");
            }
            SpellType::Unknown => {
                println!("Unknown spell: {}", event.spell_name);
            },
        }
    }
}