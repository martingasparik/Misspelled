//TODO: 
// - fix positioning of text 
// - toggle also checks if not empty ??

use std::process::exit;
use bevy::{
    prelude::*,
    input::keyboard::{
        Key,
        KeyboardInput},
};
use crate::movement::Player;

// Constants for display positioning
const SPELL_TEXT_OFFSET_Y: f32 = 40.0;

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

// Enum for spell types, identification and functionality at the bottom of this file
#[derive(Debug)]
pub enum SpellType {
    Fireball,
    Blink,
    Shield,
    Exit,
    Unknown,
}

// Event for future - when a spell is cast
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
            ));
    }
}

// Setup the visual elements for the spell system
pub fn setup_spell_system(mut commands: Commands) {
    // Background for the spell text
    commands.spawn((
        Sprite {
                color: Color::srgba(0.1, 0.1, 0.1, 0.7),
                custom_size: Some(Vec2::new(200.0, 30.0)),
                ..default()
        },
        Transform::from_xyz(0.0, SPELL_TEXT_OFFSET_Y, 1.0),
        Visibility::Hidden,
        SpellTextBackground,
    ));

    // Text entity for spell display - using new Bevy 0.15 text system
    commands.spawn((
        // Empty text that will be filled when typing
        Text::new(""),
        TextFont {
            // Could use asset_server.load("fonts/your-font.ttf") if you have a custom font
            font_size: 18.0,
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
    // Toggle spell input with Tab
    if kbd.just_pressed(KeyCode::Tab) {
        spell_stack.toggle();
    }
    // Also start spell input with enter
    if !spell_stack.is_active() && kbd.just_pressed(KeyCode::Enter) {
        spell_stack.toggle();
    }
    
    // Only process inputs if spell system is active
    if !spell_stack.is_active() {
        return;
    }

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
            spell_cast_events.write(SpellCastEvent {
                spell_type,
                spell_name,
            });

            // Clear the stack and deactivate input
            //spell_stack.clear();
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
            } else if key_event.logical_key == Key::Space {
                // For multi-word spells
                spell_stack.push(' ');
            }
        }
    }
}

// Update the spell text display
pub fn update_spell_text(
    spell_stack: Res<SpellStack>,
    mut text_query: Query<(&mut Text, &mut Visibility), With<SpellText>>,
    mut bg_query: Query<&mut Visibility, (With<SpellTextBackground>, Without<SpellText>)>,
) {
    if spell_stack.is_changed() {
        // Update text content with the new Bevy 0.15 text approach
        if let Ok((mut text, mut text_visibility)) = text_query.single_mut() {
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

// Keep text centered on player - or not
pub fn update_text_position(
    player_query: Query<&Transform, With<Player>>,
    mut text_query: Query<&mut Transform, (With<SpellText>, Without<Player>, Without<SpellTextBackground>)>,
    mut bg_query: Query<&mut Transform, (With<SpellTextBackground>, Without<Player>, Without<SpellText>)>,
) {
    if let Ok(player_transform) = player_query.single() {
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
        _ => SpellType::Unknown,
    }
}

// Function to execute spell effects based on type - to be implemented later
pub fn execute_spells(
    mut spell_events: EventReader<SpellCastEvent>,
    // Add other parameters as needed for spell effects
) {
    for event in spell_events.read() {
        match event.spell_type {
            SpellType::Fireball => {
                println!("Casting Fireball spell!");
                //todo!()
            }
            SpellType::Blink => {
                println!("Casting Blink spell!");
                //todo!()
            }
            SpellType::Shield => {
                println!("Casting Shield spell!");
                //todo!()
            }
            SpellType::Exit => {
                println!("Casting Exit");
                exit(0);
            }
            SpellType::Unknown => {
                println!("Unknown spell: {}", event.spell_name);
            }
        }
    }
}