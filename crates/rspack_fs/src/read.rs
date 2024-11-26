use std::fmt::Debug;
use std::sync::Arc;

use futures::future::BoxFuture;
use rspack_paths::Utf8Path;
use rspack_paths::Utf8PathBuf;
use tokio::task::block_in_place;
use tokio::task::spawn_blocking;

use crate::{FileMetadata, Result};
#[async_trait::async_trait]
pub trait ReadableFileSystem: Debug + Send + Sync {
  /// See [std::fs::read]
  fn read(&self, path: &Utf8Path) -> Result<Vec<u8>>;

  /// See [std::fs::metadata]
  fn metadata(&self, path: &Utf8Path) -> Result<FileMetadata>;

  /// See [std::fs::symlink_metadata]
  fn symlink_metadata(&self, path: &Utf8Path) -> Result<FileMetadata>;

  /// See [std::fs::canonicalize]
  fn canonicalize(&self, path: &Utf8Path) -> Result<Utf8PathBuf>;
  /// The following implementation is native implementation for testing
  async fn async_read(&self, file: &Utf8Path) -> Result<Vec<u8>> {
    self.read(file)
  }
  async fn async_metadata(self: Arc<Self>, path: &Utf8Path) -> Result<FileMetadata> {
    self.metadata(path)
  }
  async fn async_symlink_metadata(self: Arc<Self>, path: &Utf8Path) -> Result<FileMetadata> {
    self.symlink_metadata(path)
  }
}
