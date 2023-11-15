use std::fmt::Debug;

use bevy::prelude::*;

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    reflect::TypePath,
    utils::BoxedFuture,
};

use bevy_http::HttpAssetReaderPlugin;


mod bytes_asset;
use bytes_asset::*;

#[derive(Resource, Default)]
struct State {
    handle: Handle<BytesAsset>,
    printed: bool,
}

#[test]
fn main() {
    App::new()
        .add_plugins((HttpAssetReaderPlugin, DefaultPlugins, BytesAssetPlugin))
        .init_resource::<State>()
        .add_systems(Startup, setup)
        .add_systems(Update, print_on_load)
        .run();
}


fn setup(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    state.handle = asset_server.load("remote://icon.png");
}

fn print_on_load(mut state: ResMut<State>, custom_assets: ResMut<Assets<BytesAsset>>) {
    let custom_asset = custom_assets.get(&state.handle);
    if state.printed || custom_asset.is_none() {
        return;
    }

    info!("Custom asset loaded: {:?}", custom_asset.unwrap());
    state.printed = true;
}
