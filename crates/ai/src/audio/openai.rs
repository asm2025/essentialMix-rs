use async_openai::*;
use config::Config;
use futures::executor::block_on;
use reqwest::Client as ReqwestClient;
use std::{fmt, path::Path, sync::Arc};
use tokio::sync::mpsc::UnboundedSender;
use types::CreateTranscriptionRequestArgs;

use crate::{Error, Result, SourceSize};

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OpenAiAudioSource {
    #[default]
    whisper_1,
}

impl fmt::Display for OpenAiAudioSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpenAiAudioSource::whisper_1 => write!(f, "whisper-1"),
        }
    }
}

impl From<SourceSize> for OpenAiAudioSource {
    fn from(_size: SourceSize) -> Self {
        // OpenAI currently only has whisper-1, but this allows for future models
        // that can be mapped based on size
        OpenAiAudioSource::whisper_1
    }
}

#[derive(Clone)]
#[must_use]
pub struct OpenAIWhisper<C: Config> {
    client: Arc<Client<C>>,
    source: OpenAiAudioSource,
}

impl<C: Config> OpenAIWhisper<C> {
    pub fn new(config: C) -> Self {
        Self::from(config, None, ReqwestClient::new(), Default::default())
    }

    pub fn from_size(config: C, size: SourceSize) -> Self {
        let source = size.into();
        Self::from(
            config,
            Some(source),
            ReqwestClient::new(),
            Default::default(),
        )
    }

    pub fn from_client(
        config: C,
        client: ReqwestClient,
        source: Option<OpenAiAudioSource>,
    ) -> Self {
        Self::from(config, source, client, Default::default())
    }

    fn from(
        config: C,
        source: Option<OpenAiAudioSource>,
        client: ReqwestClient,
        backoff: backoff::ExponentialBackoff,
    ) -> Self {
        Self {
            client: Arc::new(Client::build(client, config, backoff)),
            source: source.unwrap_or_default(),
        }
    }

    pub fn transcribe<T: AsRef<Path>>(
        &self,
        file_name: T,
        callback: impl Fn(&str) -> (),
    ) -> Result<()> {
        block_on(self.transcribe_async(file_name, callback))
    }

    pub async fn transcribe_async<T: AsRef<Path>>(
        &self,
        file_name: T,
        callback: impl Fn(&str) -> (),
    ) -> Result<()> {
        let file_path = file_name.as_ref();

        // Convert path to string for async-openai
        let file_path_str = file_path
            .to_str()
            .ok_or_else(|| Error::InvalidInput("Invalid file path".to_string()))?;

        // Build the transcription request
        // async-openai reads the file from the path
        let request = CreateTranscriptionRequestArgs::default()
            .file(file_path_str)
            .model(self.source.to_string())
            .build()
            .map_err(|e| Error::OpenAI(e.to_string()))?;

        // Create transcription
        let response = self
            .client
            .audio()
            .transcribe(request)
            .await
            .map_err(|e| Error::OpenAI(e.to_string()))?;

        // Call callback with the transcribed text
        callback(&response.text);

        Ok(())
    }

    pub fn stream<T: AsRef<Path>>(
        &self,
        file_name: T,
        sender: UnboundedSender<String>,
    ) -> Result<()> {
        block_on(async {
            let file_path = file_name.as_ref();

            // Convert path to string for async-openai
            let file_path_str = file_path
                .to_str()
                .ok_or_else(|| Error::OpenAI("Invalid file path".to_string()))?;

            // Build the transcription request
            let request = CreateTranscriptionRequestArgs::default()
                .file(file_path_str)
                .model(self.source.to_string())
                .build()
                .map_err(|e| Error::OpenAI(e.to_string()))?;

            // Create transcription
            let response = self
                .client
                .audio()
                .transcribe(request)
                .await
                .map_err(|e| Error::OpenAI(e.to_string()))?;

            // Send the transcribed text through the channel
            sender
                .send(response.text)
                .map_err(|e| Error::OpenAI(format!("Failed to send transcription: {}", e)))?;

            Ok(())
        })
    }
}
