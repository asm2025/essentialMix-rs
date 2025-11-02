#[cfg(all(test, feature = "imaging"))]
mod tests {
    use emixai::{imaging::Image, Result};
    use futures::executor::block_on;
    use std::cell::RefCell;

    #[test]
    #[ignore] // Requires large model download - run manually  
    fn test_image_generate() -> Result<()> {
        let image = block_on(Image::from(
            false,
            "decoder_weights_url",
            "clip_weights_url",
            "prior_clip_weights_url",
            "prior_weights_url",
            "vqgan_weights_url",
            "tokenizer_url",
            "prior_tokenizer_url",
        ))?;
        
        let generated = RefCell::new(false);
        image.generate("a cat", |_buffer| {
            *generated.borrow_mut() = true;
        })?;
        
        assert!(*generated.borrow(), "Should have generated an image");
        
        Ok(())
    }

    #[tokio::test]
    #[ignore] // Requires large model download - run manually
    async fn test_image_generate_async() -> Result<()> {
        let image = Image::from(
            false,
            "decoder_weights_url",
            "clip_weights_url",
            "prior_clip_weights_url",
            "prior_weights_url",
            "vqgan_weights_url",
            "tokenizer_url",
            "prior_tokenizer_url",
        ).await?;
        
        let generated = RefCell::new(false);
        image.generate_async("a dog", |_buffer| {
            *generated.borrow_mut() = true;
        }).await?;
        
        assert!(*generated.borrow(), "Should have generated an image");
        
        Ok(())
    }
}

