use bevy::prelude::*;
use crate::player_code::Health;
use crate::player_code::Player;

pub struct HealthDisplayPlugin;

// Components for our health display UI elements
#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct HealthBarFill;

#[derive(Component)]
struct HeartContainer;

// Resource to store the maximum health value for scaling
#[derive(Resource)]
struct MaxHealth(f32);

impl Plugin for HealthDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MaxHealth>()
            .add_systems(Startup, setup_health_display)
            .add_systems(Update, update_health_display);
    }
}

// Default implementation for MaxHealth resource
impl Default for MaxHealth {
    fn default() -> Self {
        MaxHealth(20.0) // Default max health, adjust as needed
    }
}

fn setup_health_display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Define health bar dimensions 48 285
    let health_bar_width = 285.0;
    let health_bar_height = 48.0; // Smaller height for the bar

    let hearts_image = asset_server.load("hp_containers.png");
    let red = asset_server.load("red.png");

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
                        width: Val::Px(health_bar_width),
                        height: Val::Px(health_bar_height),
                        ..default()
                    },
                    HealthBar,
                ))
                .with_children(|parent| {
                    // Health bar fill (red rectangle)
                    parent.spawn((
                        ImageNode {
                            image: red.into(),
                            rect: Some( Rect {
                                min: Vec2::new(0.0, 0.0),
                                max: Vec2::new(health_bar_width,health_bar_height).into()
                            }),
                            ..default()
                        },
                        HealthBarFill,
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
                            rect: Some( Rect { 
                                min: Vec2::new(0.0, 0.0), 
                                max: Vec2::new(health_bar_width,health_bar_height).into() 
                            }),
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
