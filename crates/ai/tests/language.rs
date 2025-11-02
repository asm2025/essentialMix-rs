#[cfg(all(test, feature = "language"))]
mod tests {
    use emixai::{language::{LlamaSource, ModelSource}, Result, SourceSize};

    #[tokio::test]
    #[ignore] // Requires large model download - run manually
    async fn test_llama_from_size() -> Result<()> {
        let _llama = <LlamaSource as ModelSource>::from_size(SourceSize::Small).await?;
        
        // Test that model was loaded
        assert!(true, "Llama model loaded successfully");
        
        Ok(())
    }

    #[tokio::test]
    #[ignore] // Requires large model download - run manually
    async fn test_llama_from_default() -> Result<()> {
        let _llama = <LlamaSource as ModelSource>::new().await?;
        
        // Test that model was loaded
        assert!(true, "Llama model loaded successfully");
        
        Ok(())
    }
}

