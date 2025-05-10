use bevy::prelude::*;

#[derive(Resource)]
pub struct OrcAssets {
    pub texture: Handle<Image>,
    pub atlas: Handle<TextureAtlasLayout>,
}

pub struct OrcAssetPlugin;
impl Plugin for OrcAssetPlugin {
    fn build(&self, app: &mut App) {
        // Updated to Bevy 0.15.3 syntax
        app.add_systems(Startup, load_orc_assets);
    }
}

fn load_orc_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("orc/Orc.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(100, 100), 8, 6, None, None);
    let atlas = layouts.add(layout);
    commands.insert_resource(OrcAssets { texture, atlas });
}