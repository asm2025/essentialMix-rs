use futures::{executor::block_on, stream::StreamExt};
use image::{ImageBuffer, Rgb};
use kalosm::vision::{Wuerstchen, WuerstchenInferenceSettings};
use std::sync::Arc;

use crate::{Error, Result};

pub struct Image {
    model: Arc<Wuerstchen>,
}

impl Image {
    pub fn new(model: Wuerstchen) -> Self {
        Self {
            model: Arc::new(model),
        }
    }

    pub async fn from(
        flash_attn: bool,
        decoder_weights: impl Into<String>,
        clip_weights: impl Into<String>,
        prior_clip_weights: impl Into<String>,
        prior_weights: impl Into<String>,
        vqgan_weights: impl Into<String>,
        tokenizer: impl Into<String>,
        prior_tokenizer: impl Into<String>,
    ) -> Result<Self> {
        let model = Wuerstchen::builder()
            .with_flash_attn(flash_attn)
            .with_decoder_weights(decoder_weights)
            .with_clip_weights(clip_weights)
            .with_prior_clip_weights(prior_clip_weights)
            .with_prior_weights(prior_weights)
            .with_vqgan_weights(vqgan_weights)
            .with_tokenizer(tokenizer)
            .with_prior_tokenizer(prior_tokenizer)
            .build()
            .await
            .map_err(|e| Error::from_std_error(e))?;
        Ok(Self::new(model))
    }

    pub fn generate(
        &self,
        prompt: &str,
        callback: impl Fn(ImageBuffer<Rgb<u8>, Vec<u8>>) -> (),
    ) -> Result<()> {
        let settings = WuerstchenInferenceSettings::new(prompt);
        let stream = self.model.run(settings);
        block_on(async move {
            let mut stream = stream;
            while let Some(img) = stream.next().await {
                if let Some(buffer) = img.generated_image() {
                    callback(buffer);
                }
            }
        });
        Ok(())
    }

    pub async fn generate_async(
        &self,
        prompt: &str,
        callback: impl Fn(ImageBuffer<Rgb<u8>, Vec<u8>>) -> (),
    ) -> Result<()> {
        let settings = WuerstchenInferenceSettings::new(prompt);
        let mut stream = self.model.run(settings);

        while let Some(img) = stream.next().await {
            if let Some(buffer) = img.generated_image() {
                callback(buffer);
            }
        }

        Ok(())
    }
}
