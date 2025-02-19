use std::collections::HashMap;

use rspack_collections::DatabaseItem;
use rspack_core::{
  ApplyContext, Chunk, CompilationChunkIds, CompilerOptions, Plugin, PluginContext,
};
use rspack_hook::{plugin, plugin_hook};

use crate::id_helpers::{
  assign_ascending_chunk_ids, assign_names_par, compare_chunks_natural, get_long_chunk_name,
  get_short_chunk_name, get_used_chunk_ids,
};

#[plugin]
#[derive(Debug)]
pub struct NamedChunkIdsPlugin {
  pub delimiter: String,
  pub context: Option<String>,
}

impl NamedChunkIdsPlugin {
  pub fn new(delimiter: Option<String>, context: Option<String>) -> Self {
    Self::new_inner(delimiter.unwrap_or_else(|| "-".to_string()), context)
  }
}

#[plugin_hook(CompilationChunkIds for NamedChunkIdsPlugin)]
fn chunk_ids(&self, compilation: &mut rspack_core::Compilation) -> rspack_error::Result<()> {
  // set default value
  for chunk in compilation.chunk_by_ukey.values_mut() {
    if let Some(name) = chunk.name() {
      chunk.set_id(Some(name.to_owned()));
    }
  }

  let mut used_ids = get_used_chunk_ids(compilation);
  let chunk_graph = &compilation.chunk_graph;
  let module_graph = compilation.get_module_graph();
  let context = self
    .context
    .clone()
    .unwrap_or_else(|| compilation.options.context.to_string());
  let chunks = compilation
    .chunk_by_ukey
    .values()
    .filter(|chunk| chunk.id().is_none())
    .map(|chunk| chunk as &Chunk)
    .collect::<Vec<_>>();
  let mut chunk_id_to_name = HashMap::with_capacity(chunks.len());
  let unnamed_chunks = assign_names_par(
    chunks,
    |chunk| get_short_chunk_name(chunk, chunk_graph, &context, &self.delimiter, &module_graph),
    |chunk, _| get_long_chunk_name(chunk, chunk_graph, &context, &self.delimiter, &module_graph),
    |a, b| compare_chunks_natural(chunk_graph, &module_graph, &compilation.module_ids, a, b),
    &mut used_ids,
    |chunk, name| {
      chunk_id_to_name.insert(chunk.ukey(), name);
    },
  );

  let unnamed_chunks = unnamed_chunks
    .iter()
    .map(|chunk| chunk.ukey())
    .collect::<Vec<_>>();

  chunk_id_to_name.into_iter().for_each(|(chunk_ukey, name)| {
    let chunk = compilation.chunk_by_ukey.expect_get_mut(&chunk_ukey);
    chunk.set_id(Some(name.clone()));
  });

  if !unnamed_chunks.is_empty() {
    assign_ascending_chunk_ids(&unnamed_chunks, compilation)
  }

  Ok(())
}

impl Plugin for NamedChunkIdsPlugin {
  fn name(&self) -> &'static str {
    "rspack.NamedChunkIdsPlugin"
  }

  fn apply(
    &self,
    ctx: PluginContext<&mut ApplyContext>,
    _options: &CompilerOptions,
  ) -> rspack_error::Result<()> {
    ctx
      .context
      .compilation_hooks
      .chunk_ids
      .tap(chunk_ids::new(self));
    Ok(())
  }
}
