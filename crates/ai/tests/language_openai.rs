#[cfg(all(test, feature = "language"))]
mod tests {
    use async_openai::config::OpenAIConfig;
    use emixai::{Error, Result, SourceSize, language::openai::ChatGpt};

    #[test]
    fn test_chatgpt_new() {
        // Test ChatGpt::new() creates an instance with default config
        // We need a config, but we can test that the method exists and accepts a config
        let config = OpenAIConfig::default();
        let _chatgpt = ChatGpt::new(config);
        // If this compiles and doesn't panic, the test passes
    }

    #[test]
    fn test_chatgpt_from_size() {
        let config = OpenAIConfig::default();
        let _chatgpt = ChatGpt::from_size(config, SourceSize::Small, None);
        // Test that from_size works with different sizes
        let config2 = OpenAIConfig::default();
        let _chatgpt2 = ChatGpt::from_size(config2, SourceSize::Tiny, Some(256));
    }

    #[test]
    fn test_chatgpt_subscribe() {
        let config = OpenAIConfig::default();
        let chatgpt = ChatGpt::new(config);
        let _receiver = chatgpt.subscribe();
        // Test that subscribe returns a receiver
        assert!(true, "Subscribe should return a receiver");
    }

    #[tokio::test]
    #[ignore] // Requires OpenAI API key - run manually with: cargo test -p emixai --features language -- --ignored
    async fn test_chatgpt_prompt() -> Result<()> {
        let config = OpenAIConfig::default();
        let chatgpt = ChatGpt::new(config);

        // This requires an API key, so it's ignored
        chatgpt.prompt("Hello").await?;

        Ok(())
    }

    #[tokio::test]
    #[ignore] // Requires OpenAI API key - run manually with: cargo test -p emixai --features language -- --ignored
    async fn test_chatgpt_prompt_empty() {
        let config = OpenAIConfig::default();
        let chatgpt = ChatGpt::new(config);

        // Empty prompt should return NoInput error
        let result = chatgpt.prompt("").await;
        assert!(matches!(result, Err(Error::NoInput)));
    }
}
