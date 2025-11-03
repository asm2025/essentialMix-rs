#[cfg(all(test, feature = "language"))]
mod tests {
    use emixai::{
        Result, SourceSize,
        language::{ModelSource, llama::LlamaSource},
    };

    #[test]
    fn test_llama_source_default_size() {
        assert_eq!(LlamaSource::default_size(), SourceSize::Base);
    }

    #[test]
    fn test_source_size_to_llama_source_conversion() {
        use kalosm::language::LlamaSource as KalosmLlamaSource;

        // Test the From<SourceSize> for LlamaSource conversion
        let tiny_source: KalosmLlamaSource = SourceSize::Tiny.into();
        // We can't easily test the exact variant without pattern matching on internal types,
        // but we can verify the conversion doesn't panic
        let _ = tiny_source;

        let small_source: KalosmLlamaSource = SourceSize::Small.into();
        let _ = small_source;

        let base_source: KalosmLlamaSource = SourceSize::Base.into();
        let _ = base_source;

        let medium_source: KalosmLlamaSource = SourceSize::Medium.into();
        let _ = medium_source;

        let large_source: KalosmLlamaSource = SourceSize::Large.into();
        let _ = large_source;
    }

    #[tokio::test]
    #[ignore] // Requires large model download - run manually with: cargo test -p emixai --features language -- --ignored
    async fn test_llama_from_size() -> Result<()> {
        let _llama = <LlamaSource as ModelSource>::from_size(SourceSize::Small).await?;

        // Test that model was loaded
        assert!(true, "Llama model loaded successfully");

        Ok(())
    }

    #[tokio::test]
    #[ignore] // Requires large model download - run manually with: cargo test -p emixai --features language -- --ignored
    async fn test_llama_from_default() -> Result<()> {
        let _llama = <LlamaSource as ModelSource>::new().await?;

        // Test that model was loaded
        assert!(true, "Llama model loaded successfully");

        Ok(())
    }

    #[tokio::test]
    #[ignore] // Requires large model download - run manually with: cargo test -p emixai --features language -- --ignored
    async fn test_llama_from_source() -> Result<()> {
        let source: kalosm::language::LlamaSource = SourceSize::Small.into();
        let _llama = <LlamaSource as ModelSource>::from_source(source).await?;

        assert!(true, "Llama model loaded from source successfully");
        Ok(())
    }
}
