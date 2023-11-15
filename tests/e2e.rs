

use bevy::{
    app::{AppExit},
    prelude::*,
};

use bevy_http::HttpAssetReaderPlugin;

mod bytes_asset;
use bytes_asset::*;

#[derive(Resource, Default)]
struct State {
    handle: Handle<BytesAsset>,
}

#[test]
fn main() {
    App::new()
        .add_plugins((
            HttpAssetReaderPlugin,
            DefaultPlugins,
            // ScheduleRunnerPlugin::run_loop(Duration::from_secs(1)),
            BytesAssetPlugin,
        ))
        .init_resource::<State>()
        .add_systems(Startup, setup)
        .add_systems(Update, print_on_load)
        .run();
}

fn setup(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    state.handle = asset_server.load("remote://icon.png");
}

fn print_on_load(
    state: Res<State>,
    custom_assets: ResMut<Assets<BytesAsset>>,
    mut exit: EventWriter<AppExit>,
) {
    let custom_asset = custom_assets.get(&state.handle);

    if let Some(custom_asset) = custom_asset {
        let _length = custom_asset.bytes.len();
        exit.send(AppExit);
    }
}
