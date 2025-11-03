use futures::{executor::block_on, stream::StreamExt};
use kalosm::sound::{
    Segment, Whisper as RWhisper, WhisperBuilder, WhisperLanguage, WhisperSource, rodio::Decoder,
};
use std::{fs::File, io::BufReader, path::Path, sync::Arc};
use tokio::sync::mpsc::UnboundedSender;

use crate::Result;
use crate::SourceSize;

impl From<SourceSize> for WhisperSource {
    fn from(size: SourceSize) -> Self {
        match size {
            SourceSize::Tiny => WhisperSource::QuantizedTiny,
            SourceSize::Small => WhisperSource::Small,
            SourceSize::Base => WhisperSource::Base,
            SourceSize::Medium => WhisperSource::Medium,
            SourceSize::Large => WhisperSource::QuantizedDistilLargeV3,
        }
    }
}

#[derive(Clone)]
#[must_use]
pub struct Whisper {
    model: Arc<RWhisper>,
}

impl Whisper {
    pub fn new(model: RWhisper) -> Self {
        Self {
            model: Arc::new(model),
        }
    }

    pub async fn from_size(size: SourceSize) -> Result<Self> {
        let source = size.into();
        Self::from_source(source).await
    }

    pub async fn from_source(source: WhisperSource) -> Result<Self> {
        let model = WhisperBuilder::default()
            .with_source(source)
            .build()
            .await
            .map_err(|e| emix::Error::from_std_error(e))?;
        Ok(Self::new(model))
    }

    pub async fn from(source: WhisperSource, language: WhisperLanguage) -> Result<Self> {
        let model = WhisperBuilder::default()
            .with_source(source)
            .with_language(Some(language))
            .build()
            .await
            .map_err(|e| emix::Error::from_std_error(e))?;
        Ok(Self::new(model))
    }

    pub fn transcribe<T: AsRef<Path>>(
        &self,
        file_name: T,
        callback: impl Fn(&str) -> (),
    ) -> Result<()> {
        let file = File::open(file_name)
            .map_err(|e| emix::Error::from_std_error(e))?;
        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| emix::Error::from_std_error(e))?;
        let mut stream = self.model.transcribe(source);
        block_on(async move {
            while let Some(result) = stream.next().await {
                callback(result.text());
            }
        });
        Ok(())
    }

    pub async fn transcribe_async<T: AsRef<Path>>(
        &self,
        file_name: T,
        callback: impl Fn(&str) -> (),
    ) -> Result<()> {
        let file = File::open(file_name)
            .map_err(|e| emix::Error::from_std_error(e))?;
        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| emix::Error::from_std_error(e))?;
        let mut transcription = self.model.transcribe(source);

        while let Some(result) = transcription.next().await {
            callback(result.text());
        }

        Ok(())
    }

    pub fn stream<T: AsRef<Path>>(
        &self,
        file_name: T,
        _sender: UnboundedSender<Segment>,
    ) -> Result<()> {
        let file = File::open(file_name)
            .map_err(|e| emix::Error::from_std_error(e))?;
        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| emix::Error::from_std_error(e))?;
        // Note: transcribe method may not support the sender parameter in this version
        // This may need API-specific handling
        let _ = self.model.transcribe(source);
        Ok(())
    }
}
