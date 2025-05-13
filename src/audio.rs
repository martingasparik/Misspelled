use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use std::time::Duration;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_kira_audio::AudioPlugin)
            .add_systems(Startup, setup_audio);
    }
}

fn setup_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("audio/ARTHUROS01.ogg"))
        // Loop the audio from 0.5 seconds skipping the intro
        .loop_from(0.5)
        // Fade-in with a dynamic easing over 2 seconds
        .fade_in(AudioTween::new(Duration::from_secs(2), AudioEasing::OutPowi(2)))
        .with_volume(0.25);

    info!("Background music started");
}