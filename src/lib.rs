//! Implements a http asset io loader.

use bevy::{
    asset::io::{AssetReader, AssetReaderError, AssetSource, AssetSourceId, PathStream, Reader},
    prelude::*,
    utils::BoxedFuture,
};
use std::path::{Path, PathBuf};

use std::pin::Pin;
use std::task::Poll;

/// A custom asset reader implementation that wraps a given asset reader implementation
pub struct HttpAssetReader {
    client: surf::Client,
}

impl HttpAssetReader {
    /// Creates a new `HttpAssetReader`. The path provided will be used to build URLs to query for assets.
    pub fn new(base_url: Option<&str>) -> Self {
        let base_url = base_url.and_then(|s| surf::Url::parse(s).ok());

        let client = surf::Config::new().set_timeout(Some(std::time::Duration::from_secs(5)));

        let client = if let Some(base_url) = base_url {
            client.set_base_url(base_url)
        } else {
            client
        };

        let client = client.try_into().expect("could not create http client");

        Self { client }
    }

    async fn fetch_bytes<'a>(&self, path: &str) -> Result<Box<Reader<'a>>, AssetReaderError> {
        let resp = self.client.get(path).await;

        trace!("fetched {resp:?} ... ");
        let mut resp = resp.map_err(|e| {
            AssetReaderError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("error fetching {path}: {e}"),
            ))
        })?;

        let status = resp.status();

        if !status.is_success() {
            let err = match status {
                surf::StatusCode::NotFound => AssetReaderError::NotFound(path.into()),
                _ => AssetReaderError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("bad status code: {status}"),
                )),
            };
            return Err(err);
        };

        let bytes = resp.body_bytes().await.map_err(|e| {
            AssetReaderError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("error getting bytes for {path}: {e}"),
            ))
        })?;
        let reader = bevy::asset::io::VecReader::new(bytes);
        Ok(Box::new(reader))
    }
}

struct EmptyPathStream;

impl futures_core::Stream for EmptyPathStream {
    type Item = PathBuf;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        Poll::Ready(None)
    }
}

impl AssetReader for HttpAssetReader {
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(async move { self.fetch_bytes(&path.to_string_lossy()).await })
    }

    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(async move {
            let meta_path = path.to_string_lossy() + ".meta";
            Ok(self.fetch_bytes(&meta_path).await?)
        })
    }

    fn read_directory<'a>(
        &'a self,
        _path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<PathStream>, AssetReaderError>> {
        let stream: Box<PathStream> = Box::new(EmptyPathStream);
        error!("Reading directories is not supported with the HttpAssetReader");
        Box::pin(async move { Ok(stream) })
    }

    fn is_directory<'a>(
        &'a self,
        _path: &'a Path,
    ) -> BoxedFuture<'a, std::result::Result<bool, AssetReaderError>> {
        error!("Reading directories is not supported with the HttpAssetReader");
        Box::pin(async move { Ok(false) })
    }
}

/// A plugins that registers the `HttpAssetReader` as an asset source.
pub struct HttpAssetReaderPlugin;

impl Plugin for HttpAssetReaderPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_source(
            AssetSourceId::Name("remote".into()),
            AssetSource::build().with_reader(|| {
                Box::new(HttpAssetReader::new(Some("https://bevyengine.org/assets/")))
            }),
        );
    }
}
