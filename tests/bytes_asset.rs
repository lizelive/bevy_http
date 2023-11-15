use std::fmt::Debug;

use bevy::prelude::*;

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    reflect::TypePath,
    utils::BoxedFuture,
};


/// A custom asset type that holds bytes.
#[derive(Asset, TypePath, Debug)]
pub struct BytesAsset {
    pub bytes: Vec<u8>,
}


/// A custom asset loader implementation that loads bytes from a URL.
#[derive(Default)]
struct BytesAssetLoader;

impl AssetLoader for BytesAssetLoader {
    type Asset = BytesAsset;
    type Settings = ();
    type Error = std::io::Error;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            Ok(BytesAsset { bytes })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["bytes"]
    }
}

/// A plugins that registers the `BytesAssetLoader` as an asset loader.
pub struct BytesAssetPlugin;

impl Plugin for BytesAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<BytesAsset>()
            .init_asset_loader::<BytesAssetLoader>();
    }
}
