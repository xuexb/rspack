mod pack;

use std::sync::Arc;

pub use pack::{PackFs, PackMemoryFs, PackNativeFs, PackOptions, PackStorage, PackStorageOptions};
use rspack_error::Result;
use tokio::sync::oneshot::Receiver;

pub type StorageItemKey = Vec<u8>;
pub type StorageItemValue = Vec<u8>;
pub type StorageContent = Vec<(Arc<StorageItemKey>, Arc<StorageItemValue>)>;

#[async_trait::async_trait]
pub trait Storage: std::fmt::Debug + Sync + Send {
  async fn get_all(&self, scope: &'static str) -> Result<Vec<(Arc<Vec<u8>>, Arc<Vec<u8>>)>>;
  fn set(&self, scope: &'static str, key: Vec<u8>, value: Vec<u8>);
  fn remove(&self, scope: &'static str, key: &[u8]);
  fn idle(&self) -> Result<Receiver<Result<()>>>;
}

pub type ArcStorage = Arc<dyn Storage>;
