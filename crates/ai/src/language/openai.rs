use async_openai::*;
use config::Config;
use futures::StreamExt;
use kalosm::*;
use reqwest::Client as ReqwestClient;
use std::{
    cmp,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::{Receiver, Sender, channel};
use types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};

use super::SourceSize;
// Note: ModelSource import is commented out until OpenAiSourceModel implementation is completed
// use super::ModelSource;
use crate::{Error, Result};
use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OpenAiSource {
    gpt_3_5_turbo,
    #[default]
    gpt_4o_mini,
    gpt_4o,
    gpt_4o_2024_08_06,
    gpt_4,
    gpt_4_turbo,
    gpt_4_turbo_preview,
    o1_mini,
    o1_preview,
    o3_mini,
}

impl fmt::Display for OpenAiSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpenAiSource::gpt_3_5_turbo => write!(f, "gpt-3.5-turbo"),
            OpenAiSource::gpt_4o_mini => write!(f, "gpt-4o-mini"),
            OpenAiSource::gpt_4o => write!(f, "gpt-4o"),
            OpenAiSource::gpt_4o_2024_08_06 => write!(f, "gpt-4o-2024-08-06"),
            OpenAiSource::gpt_4 => write!(f, "gpt-4"),
            OpenAiSource::gpt_4_turbo => write!(f, "gpt-4-turbo"),
            OpenAiSource::gpt_4_turbo_preview => write!(f, "gpt-4-turbo-preview"),
            OpenAiSource::o1_mini => write!(f, "o1-mini"),
            OpenAiSource::o1_preview => write!(f, "o1-preview"),
            OpenAiSource::o3_mini => write!(f, "o3-mini"),
        }
    }
}

impl From<SourceSize> for OpenAiSource {
    fn from(size: SourceSize) -> Self {
        match size {
            SourceSize::Tiny => OpenAiSource::gpt_4o_mini,
            SourceSize::Small | SourceSize::Base => OpenAiSource::gpt_4o,
            SourceSize::Medium => OpenAiSource::gpt_4o_2024_08_06,
            SourceSize::Large => OpenAiSource::gpt_4_turbo,
        }
    }
}

// ModelSource implementation for OpenAI
//
// TODO: COMPLETE THIS IMPLEMENTATION
//
// You need to verify kalosm's actual OpenAI API structure.
// To find the correct types:
// 1. Run `cargo doc --open --features language` to browse kalosm documentation
// 2. Check kalosm's examples or tests for OpenAI usage
// 3. Look for types in kalosm::language module that implement ChatModel
//
// Common possibilities:
// - kalosm might have language::OpenAi / language::OpenAiBuilder / language::OpenAiSource (similar to Llama)
// - kalosm might use language::Remote for API-based models
// - kalosm might have a different naming convention (e.g., Gpt, OpenAI, etc.)
//
// Once you find the correct types, uncomment and complete the implementation below.
//
// #[derive(Clone)]
// pub struct OpenAiSourceModel;
//
// impl ModelSource for OpenAiSourceModel {
//     // Replace with actual kalosm OpenAI model type that implements ChatModel
//     type Model = /* kalosm::language::OpenAi or similar */;
//
//     // Replace with actual kalosm OpenAI builder type
//     type Builder = /* kalosm::language::OpenAiBuilder or similar */;
//
//     // Use String if kalosm accepts model names as strings,
//     // or use kalosm's source enum type if it has one (like LlamaSource)
//     type Source = String; // or kalosm::language::OpenAiSource
//
//     fn default_size() -> SourceSize {
//         SourceSize::Base
//     }
//
//     fn builder() -> Self::Builder {
//         Self::Builder::default()
//     }
//
//     async fn new() -> Result<Self::Model> {
//         Self::from_size(Self::default_size()).await
//     }
//
//     async fn from_size(size: SourceSize) -> Result<Self::Model> {
//         let our_source: OpenAiSource = size.into();
//         let model_name = our_source.to_string();
//         Self::from_source(model_name).await
//         // OR if kalosm has a source enum:
//         // let source: kalosm::language::OpenAiSource = size.into();
//         // Self::from_source(source).await
//     }
//
//     async fn from_source(source: Self::Source) -> Result<Self::Model> {
//         // Adapt based on kalosm's API. Examples:
//         //
//         // If kalosm has OpenAi similar to Llama:
//         // let model = kalosm::language::OpenAi::builder()
//         //     .with_source(source)  // if source is an enum
//         //     .build()
//         //     .await?;
//         //
//         // If kalosm uses model names as strings:
//         // let model = kalosm::language::OpenAi::builder()
//         //     .with_model_name(&source)  // if source is String
//         //     .build()
//         //     .await?;
//
//         // TODO: Implement based on kalosm's actual API
//         todo!("Implement OpenAI ModelSource once kalosm types are identified")
//     }
// }

#[derive(Clone)]
#[must_use]
pub struct ChatGpt<C: Config> {
    client: Arc<Client<C>>,
    source: OpenAiSource,
    max_tokens: u32,
    sender: Sender<String>,
    receiver: Arc<Mutex<Receiver<String>>>,
}

impl<C: Config> ChatGpt<C> {
    pub fn new(config: C) -> Self {
        Self::from(
            config,
            Some(OpenAiSource::default()),
            ReqwestClient::new(),
            Default::default(),
            None,
        )
    }

    pub fn from_size(config: C, size: SourceSize, capacity: Option<usize>) -> Self {
        let source = size.into();
        Self::from(
            config,
            Some(source),
            ReqwestClient::new(),
            Default::default(),
            capacity,
        )
    }

    pub fn from_client(
        config: C,
        client: ReqwestClient,
        source: Option<OpenAiSource>,
        capacity: Option<usize>,
    ) -> Self {
        Self::from(config, source, client, Default::default(), capacity)
    }

    pub fn from(
        config: C,
        source: Option<OpenAiSource>,
        client: ReqwestClient,
        backoff: backoff::ExponentialBackoff,
        capacity: Option<usize>,
    ) -> Self {
        let capacity = cmp::max(capacity.unwrap_or(128), 4);
        let (sender, receiver) = channel(capacity);
        Self {
            client: Arc::new(Client::build(client, config, backoff)),
            source: source.unwrap_or_default(),
            max_tokens: 1024u32,
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub fn subscribe(&self) -> Arc<Mutex<Receiver<String>>> {
        Arc::clone(&self.receiver)
    }

    pub async fn prompt<T: AsRef<str>>(&self, prompt: T) -> Result<()> {
        let prompt = prompt.as_ref();
        let prompt = if prompt.is_empty() { "\n>" } else { prompt };
        let prompt = language::prompt_input(prompt)?;
        if prompt.is_empty() {
            return Err(Error::NoInput);
        }

        let messages = [ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
            .build()
            .map_err(|e| Error::OpenAI(e.to_string()))?
            .into()];
        let request = CreateChatCompletionRequestArgs::default()
            .model(self.source.to_string())
            .max_tokens(self.max_tokens)
            .messages(messages)
            .build()
            .map_err(|e| Error::OpenAI(e.to_string()))?;
        let mut stream = self
            .client
            .chat()
            .create_stream(request)
            .await
            .map_err(|e| Error::OpenAI(e.to_string()))?;

        while let Some(result) = stream.next().await {
            let response = result.map_err(|e| Error::OpenAI(e.to_string()))?;

            for choice in &response.choices {
                if let Some(ref content) = choice.delta.content {
                    if let Err(e) = self.sender.send(content.clone()).await {
                        return Err(Error::OpenAI(e.to_string()));
                    }
                }
            }
        }

        Ok(())
    }
}
