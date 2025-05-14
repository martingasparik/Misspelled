use bevy::prelude::*;
use crate::player_code::Player;
use crate::player_code::Health;
use crate::player_code::Shield;

pub struct HealthDisplayPlugin;

// Components for our health display UI elements
#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct HealthBarFill;

#[derive(Component)]
struct ShieldBarFill;

#[derive(Component)]
struct HeartContainer;

#[derive(Resource)]
struct MaxHealth(f32);

#[derive(Resource)]
struct MaxShield(f32);


impl Plugin for HealthDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            setup_health_display,
            setup_max_values.after(setup_health_display)
        ))
            .add_systems(Update, (update_health_display, update_shield_display));
    }
}


// System to initialize max values based on the player's initial health/shield
fn setup_max_values(
    player_query: Query<(&Health, Option<&Shield>), With<Player>>,
    mut commands: Commands,
) {
    if let Ok((health, shield_opt)) = player_query.get_single() {
        // Set max health to player's initial health
        commands.insert_resource(MaxHealth(health.health));

        // Set max shield based on player's shield, or 0 if not present
        let max_shield = shield_opt.map_or(0.0, |shield| shield.shield);
        commands.insert_resource(MaxShield(max_shield));
    } else {
        // Fallback defaults if player isn't found
        commands.insert_resource(MaxHealth(20.0));
        commands.insert_resource(MaxShield(20.0));
    }
}

fn setup_health_display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Define health bar dimensions
    let health_bar_width = 349.0 * 1.5;
    let health_bar_height = 48.0 * 1.5;

    let hearts_image = asset_server.load("UI/hp_containers.png");
    let red = asset_server.load("UI/red.png");
    let blue = asset_server.load("UI/blue.png");

    // Container node
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        })
        .with_children(|parent| {
            // Health bar container (positioned at top-left)
            parent
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(20.0),
                        top: Val::Px(20.0),
                        width: Val::Px(health_bar_width-(15. + 19.0*1.5)), 
                        height: Val::Px(health_bar_height),
                        ..default()
                    },
                    HealthBar,
                ))
                .with_children(|parent| {
                    // Health bar fill (red rectangle)
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(15.0),
                            top: Val::Px(0.0),
                            width: Val::Percent(100.0), // Start at 100% width
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        ImageNode {
                            image: red.into(),
                            ..default()
                        },
                        HealthBarFill,
                    ));

                    // Shield bar fill (blue rectangle) - positioned absolutely on top of red
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(15.0),
                            top: Val::Px(0.0),
                            width: Val::Percent(0.0), // Start at 0% width
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        ImageNode {
                            image: blue.into(),
                            ..default()
                        },
                        ShieldBarFill,
                    ));
                });

            // Heart container overlay - with parent node for positioning
            parent
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(20.0),
                    top: Val::Px(20.0),
                    width: Val::Px(health_bar_width),
                    height: Val::Px(health_bar_height),
                    ..default()
                })
                .with_children(|parent| {
                    // The actual hearts image
                    parent.spawn((
                        ImageNode {
                            image: hearts_image.into(),
                            ..default()
                        },
                        HeartContainer,
                    ));
                });
        });
}

fn update_health_display(
    player_query: Query<&Health, With<Player>>,
    max_health: Res<MaxHealth>,
    mut health_fill_query: Query<&mut Node, With<HealthBarFill>>,
) {
    if let Ok(health) = player_query.get_single() {
        if let Ok(mut style) = health_fill_query.get_single_mut() {
            // Calculate health percentage
            let health_percent = (health.health / max_health.0) * 100.0;
            // Clamp between 0% and 100%
            let health_percent = health_percent.clamp(0.0, 100.0);
            // Update the width of the health bar fill
            style.width = Val::Percent(health_percent);
        }
    }
}

fn update_shield_display(
    player_query: Query<&Shield, With<Player>>,
    max_shield: Res<MaxHealth>,
    mut shield_query: Query<&mut Node, With<ShieldBarFill>>,
) {
    if let Ok(shield) = player_query.get_single() {
        if let Ok(mut style) = shield_query.get_single_mut() {
            let shield_percent = (shield.shield / max_shield.0) * 100.0;
            let shield_percent = shield_percent.clamp(0.0, 100.0);
            style.width = Val::Percent(shield_percent);
        }
    }
}