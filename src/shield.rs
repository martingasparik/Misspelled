use bevy::prelude::*;
use crate::player_code::{Player, Health, Shield};
use crate::spell::{SpellCastEvent, SpellType};

pub struct ShieldPlugin;

impl Plugin for ShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShieldEvent>()
            .add_event::<DamageEvent>()
            .add_systems(Update, (handle_shield_spell, process_damage));
    }
}

// System to handle shield spell activation
fn handle_shield_spell(
    mut spell_events: EventReader<SpellCastEvent>,
    
    mut player_query: Query<(&mut Shield, &Health), With<Player>>,
    mut event_writer: EventWriter<ShieldEvent>,
) {
    for event in spell_events.read() {
        if let SpellType::Shield = event.spell_type {
            if let Ok((mut shield, health)) = player_query.get_single_mut() {
                // Only allow shield activation if there are red hearts available to convert
                // and we don't already have max shields
                if shield.shield < health.health {
                    // Add one shield point
                    shield.shield = (shield.shield + 1.0).min(health.health);

                    // Emit shield activation event
                    event_writer.send(ShieldEvent::Activated);

                    println!("Shield activated! Current shield: {}", shield.shield);
                } else {
                    println!("Cannot add more shields - already at maximum!");
                }
            }
        }
    }
}

// System to process damage, prioritizing shield (blue hearts) over health
fn process_damage(
    mut player_query: Query<(&mut Health, &mut Shield), With<Player>>,
    mut damage_events: EventReader<DamageEvent>,
    mut shield_events: EventWriter<ShieldEvent>,
) {
    for damage_event in damage_events.read() {
        if let Ok((mut health, mut shield)) = player_query.get_single_mut() {
            let damage_amount = damage_event.amount;

            // If there's shield available, use it first
            if shield.shield > 0.0 {
                let shield_damage = damage_amount.min(shield.shield);
                shield.shield -= shield_damage;

                println!("Shield absorbed {} damage! Remaining shield: {}", shield_damage, shield.shield);

                // If there's still damage left after depleting shield
                let remaining_damage = damage_amount - shield_damage;
                if remaining_damage > 0.0 {
                    health.health -= remaining_damage;
                    println!("Remaining {} damage applied to health! Health now: {}", remaining_damage, health.health);
                }

                // Emit shield depleted event
                shield_events.send(ShieldEvent::Depleted);
            } else {
                // No shield available, damage health directly
                health.health -= damage_amount;
                println!("No shield! Taking {} damage directly to health. Health now: {}", damage_amount, health.health);
            }

            // Ensure health doesn't go below zero
            health.health = health.health.max(0.0);

            // Ensure shield doesn't exceed health - this is crucial
            // If health is reduced, we need to cap shields at current health
            shield.shield = shield.shield.min(health.health);
        }
    }
}

// Event types
#[derive(Event)]
pub enum ShieldEvent {
    Activated,
    Depleted,
}

#[derive(Event)]
pub struct DamageEvent {
    pub amount: f32,
}

// A helper function that can be called from other parts of your code to apply damage
pub fn apply_damage(event_writer: &mut EventWriter<DamageEvent>, amount: f32) {
    event_writer.send(DamageEvent { amount });
}