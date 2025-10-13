use async_openai::*;
use config::Config;
use futures::StreamExt;
use kalosm::*;
use reqwest::Client as ReqwestClient;
use std::{
    cmp, fmt,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::{Receiver, Sender, channel};
use types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};

use crate::{Error, Result, SourceSize};

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OpenAiSource {
    gpt_3_5_turbo,
    #[default]
    gpt_4o_mini,
    gpt_4o,
    gpt_4,
    gpt_4_turbo,
    o1_mini,
}

impl fmt::Display for OpenAiSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpenAiSource::gpt_3_5_turbo => write!(f, "gpt-3.5-turbo"),
            OpenAiSource::gpt_4o_mini => write!(f, "gpt-4o-mini"),
            OpenAiSource::gpt_4o => write!(f, "gpt-4o"),
            OpenAiSource::gpt_4 => write!(f, "gpt-4"),
            OpenAiSource::gpt_4_turbo => write!(f, "gpt-4-turbo"),
            OpenAiSource::o1_mini => write!(f, "o1-mini"),
        }
    }
}

impl From<SourceSize> for OpenAiSource {
    fn from(size: SourceSize) -> Self {
        match size {
            SourceSize::Tiny => OpenAiSource::gpt_4o_mini,
            SourceSize::Small | SourceSize::Base => OpenAiSource::gpt_4o,
            SourceSize::Medium => OpenAiSource::gpt_4,
            SourceSize::Large => OpenAiSource::gpt_4_turbo,
        }
    }
}

#[derive(Clone)]
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
