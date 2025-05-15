use bevy::prelude::*;

// Resource to track the number of orcs killed
#[derive(Resource)]
pub struct OrcDeathCounter {
    pub count: u32,
}
impl Default for OrcDeathCounter {
    fn default() -> Self {
        Self {
            count: 0
        }
    }
}

// Event emitted when an orc dies
#[derive(Event)]
pub struct OrcDeathEvent;

// Component for the counter UI
#[derive(Component)]
pub struct KillCountText;

pub struct OrcDeathCounterPlugin;

impl Plugin for OrcDeathCounterPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<OrcDeathCounter>()
            .add_event::<OrcDeathEvent>()
            .add_systems(Startup, setup_kill_counter_ui)
            .add_systems(Update, (increment_death_counter, update_kill_counter_display));

        println!("OrcDeathCounterPlugin initialized");
    }
}

// System to set up the kill counter UI
fn setup_kill_counter_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    death_counter: Res<OrcDeathCounter>,
) {
    // Try to load the font, if it fails, use the default font
    let font_handle = asset_server.load("fonts/NicoPaint-Monospaced.ttf");

    commands
        .spawn(Node {

                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::FlexStart,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
        })
        .with_children(|parent| {
            // Create a text bundle for displaying kill count
            parent.spawn((
                //Text2d::new("Orcs Defeated: "),
                Text::new(death_counter.count.to_string()),
                TextFont {
                    font: font_handle,
                    font_size: 64.0,
                    ..default()
                },
                KillCountText,

            ));
        });
}

// System to increment the death counter when orcs die
fn increment_death_counter(
    mut death_events: EventReader<OrcDeathEvent>,
    mut death_counter: ResMut<OrcDeathCounter>,
) {
    for _ in death_events.read() {
        death_counter.count += 1;
        println!("Orc killed! Total orcs defeated: {}", death_counter.count);
    }
}

// System to update the kill counter display
fn update_kill_counter_display(
    death_counter: Res<OrcDeathCounter>,
    mut query: Query<&mut Text, With<KillCountText>>,
) {
    if death_counter.is_changed() || death_counter.is_added() {
        if let Ok(mut text) = query.get_single_mut() {
            // Update the second section of text with the current count
            **text = death_counter.count.to_string();
        }
    }
}