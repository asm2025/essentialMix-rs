use kalosm::*;

use super::{ModelSource, SourceSize};
use crate::{Error, Result};

impl From<SourceSize> for language::LlamaSource {
    fn from(size: SourceSize) -> Self {
        match size {
            SourceSize::Tiny => language::LlamaSource::llama_3_2_1b_chat(),
            SourceSize::Small => language::LlamaSource::llama_7b_chat(),
            SourceSize::Base => language::LlamaSource::llama_8b_chat(),
            SourceSize::Medium => language::LlamaSource::llama_13b_chat(),
            SourceSize::Large => language::LlamaSource::llama_70b_chat(),
        }
    }
}

#[derive(Clone)]
pub struct LlamaSource;

impl ModelSource for LlamaSource {
    type Model = language::Llama;
    type Builder = language::LlamaBuilder;

    fn default_size() -> SourceSize {
        SourceSize::Base
    }

    fn builder() -> Self::Builder {
        Self::Builder::default()
    }

    async fn new() -> Result<Self::Model> {
        Self::from_size(Self::default_size()).await
    }

    async fn from_size(size: SourceSize) -> Result<Self::Model> {
        let source = size.into();
        let model = language::Llama::builder()
            .with_source(source)
            .build()
            .await
            .map_err(|e| Error::Llama(e.to_string()))?;
        Ok(model)
    }
}
