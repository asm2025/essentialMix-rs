#[cfg(all(test, feature = "imaging"))]
mod tests {
    use emixai::{Result, imaging::Image};
    use futures::executor::block_on;
    use std::cell::RefCell;

    #[test]
    #[ignore] // Requires large model download - run manually with: cargo test -p emixai --features imaging -- --ignored
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
        let image_count = RefCell::new(0);

        image.generate("a cat", |buffer| {
            *generated.borrow_mut() = true;
            *image_count.borrow_mut() += 1;
            // Verify buffer has content
            assert!(buffer.width() > 0, "Image width should be greater than 0");
            assert!(buffer.height() > 0, "Image height should be greater than 0");
        })?;

        assert!(*generated.borrow(), "Should have generated an image");
        assert!(
            *image_count.borrow() > 0,
            "Should have generated at least one image"
        );

        Ok(())
    }

    #[tokio::test]
    #[ignore] // Requires large model download - run manually with: cargo test -p emixai --features imaging -- --ignored
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
        )
        .await?;

        let generated = RefCell::new(false);
        let image_count = RefCell::new(0);

        image
            .generate_async("a dog", |buffer| {
                *generated.borrow_mut() = true;
                *image_count.borrow_mut() += 1;
                // Verify buffer has content
                assert!(buffer.width() > 0, "Image width should be greater than 0");
                assert!(buffer.height() > 0, "Image height should be greater than 0");
            })
            .await?;

        assert!(*generated.borrow(), "Should have generated an image");
        assert!(
            *image_count.borrow() > 0,
            "Should have generated at least one image"
        );

        Ok(())
    }

    #[test]
    #[ignore] // Requires large model download - run manually with: cargo test -p emixai --features imaging -- --ignored
    fn test_image_generate_different_prompts() -> Result<()> {
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

        let prompts = vec!["a cat", "a dog", "a bird"];

        for prompt in prompts {
            let generated = RefCell::new(false);
            image.generate(prompt, |_buffer| {
                *generated.borrow_mut() = true;
            })?;
            assert!(
                *generated.borrow(),
                "Should have generated an image for prompt: {}",
                prompt
            );
        }

        Ok(())
    }
}
