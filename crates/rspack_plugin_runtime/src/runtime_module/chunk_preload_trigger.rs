use std::hash::BuildHasherDefault;

use cow_utils::CowUtils;
use indexmap::IndexMap;
use rspack_collections::Identifier;
use rspack_core::{
  impl_runtime_module,
  rspack_sources::{BoxSource, RawStringSource, SourceExt},
  Compilation, RuntimeModule, RuntimeModuleStage,
};
use rustc_hash::FxHasher;

#[impl_runtime_module]
#[derive(Debug)]
pub struct ChunkPreloadTriggerRuntimeModule {
  id: Identifier,
  chunk_map: IndexMap<String, Vec<String>, BuildHasherDefault<FxHasher>>,
}

impl ChunkPreloadTriggerRuntimeModule {
  pub fn new(chunk_map: IndexMap<String, Vec<String>, BuildHasherDefault<FxHasher>>) -> Self {
    Self::with_default(
      Identifier::from("webpack/runtime/chunk_preload_trigger"),
      chunk_map,
    )
  }
}

impl RuntimeModule for ChunkPreloadTriggerRuntimeModule {
  fn name(&self) -> Identifier {
    self.id
  }

  fn generate(&self, _: &Compilation) -> rspack_error::Result<BoxSource> {
    Ok(
      RawStringSource::from(
        include_str!("runtime/chunk_preload_trigger.js")
          .cow_replace(
            "$CHUNK_MAP$",
            &serde_json::to_string(&self.chunk_map).expect("invalid json tostring"),
          )
          .into_owned(),
      )
      .boxed(),
    )
  }

  fn stage(&self) -> RuntimeModuleStage {
    RuntimeModuleStage::Trigger
  }
}
