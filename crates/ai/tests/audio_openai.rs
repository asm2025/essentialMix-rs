#[cfg(all(test, feature = "audio", feature = "language"))]
mod tests {
    use async_openai::config::OpenAIConfig;
    use emixai::{Result, SourceSize, audio::OpenAIWhisper};
    use std::cell::RefCell;
    use std::path::PathBuf;

    fn get_audio_file(name: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("audio_files");
        path.push(name);
        path
    }

    #[test]
    fn test_openai_whisper_new() {
        let config = OpenAIConfig::default();
        let _whisper = OpenAIWhisper::new(config);
        // If this compiles and doesn't panic, the test passes
        assert!(true, "OpenAI Whisper created successfully");
    }

    #[test]
    fn test_openai_whisper_from_size() {
        let config = OpenAIConfig::default();
        let _whisper = OpenAIWhisper::from_size(config, SourceSize::Tiny);
        // Test that from_size works
        assert!(true, "OpenAI Whisper created from size successfully");
    }

    #[tokio::test]
    #[ignore] // Requires OpenAI API key - run manually with: cargo test -p emixai --features "audio,language" -- --ignored
    async fn test_openai_whisper_transcribe_async_mp3() -> Result<()> {
        let config = OpenAIConfig::default();
        let whisper = OpenAIWhisper::new(config);
        let audio_path = get_audio_file("awz1.mp3");

        if !audio_path.exists() {
            panic!("Audio file should exist: {:?}", audio_path);
        }

        let transcribed = RefCell::new(false);
        let text_callback = RefCell::new(String::new());

        whisper
            .transcribe_async(&audio_path, |text| {
                *transcribed.borrow_mut() = true;
                text_callback.borrow_mut().push_str(text);
            })
            .await?;

        assert!(*transcribed.borrow(), "Should have transcribed some text");
        assert!(
            !text_callback.borrow().is_empty(),
            "Transcribed text should not be empty"
        );

        Ok(())
    }

    #[tokio::test]
    #[ignore] // Requires OpenAI API key - run manually with: cargo test -p emixai --features "audio,language" -- --ignored
    async fn test_openai_whisper_transcribe_async_wav() -> Result<()> {
        let config = OpenAIConfig::default();
        let whisper = OpenAIWhisper::new(config);
        let audio_path = get_audio_file("pinless.wav");

        if !audio_path.exists() {
            panic!("Audio file should exist: {:?}", audio_path);
        }

        let transcribed = RefCell::new(false);
        let text_callback = RefCell::new(String::new());

        whisper
            .transcribe_async(&audio_path, |text| {
                *transcribed.borrow_mut() = true;
                text_callback.borrow_mut().push_str(text);
            })
            .await?;

        assert!(*transcribed.borrow(), "Should have transcribed some text");
        assert!(
            !text_callback.borrow().is_empty(),
            "Transcribed text should not be empty"
        );

        Ok(())
    }

    #[test]
    #[ignore] // Requires OpenAI API key - run manually with: cargo test -p emixai --features "audio,language" -- --ignored
    fn test_openai_whisper_sync_transcribe() -> Result<()> {
        let config = OpenAIConfig::default();
        let whisper = OpenAIWhisper::new(config);
        let audio_path = get_audio_file("pinless.wav");

        if !audio_path.exists() {
            panic!("Audio file should exist: {:?}", audio_path);
        }

        let transcribed = RefCell::new(false);
        let text_callback = RefCell::new(String::new());

        whisper.transcribe(&audio_path, |text| {
            *transcribed.borrow_mut() = true;
            text_callback.borrow_mut().push_str(text);
        })?;

        assert!(*transcribed.borrow(), "Should have transcribed some text");
        assert!(
            !text_callback.borrow().is_empty(),
            "Transcribed text should not be empty"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_openai_whisper_transcribe_file_not_found() -> Result<()> {
        let config = OpenAIConfig::default();
        let whisper = OpenAIWhisper::new(config);
        let nonexistent = PathBuf::from("definitely_does_not_exist.mp3");

        // This test doesn't require an API key - just test that file not found returns an error
        let result = whisper.transcribe_async(&nonexistent, |_| {}).await;
        assert!(result.is_err(), "Should fail on non-existent file");
        
        Ok(())
    }

    #[test]
    fn test_source_size_to_openai_audio_source_conversion() {
        use emixai::audio::OpenAiAudioSource;

        // Test the From<SourceSize> for OpenAiAudioSource conversion
        let tiny_source: OpenAiAudioSource = SourceSize::Tiny.into();
        assert!(matches!(tiny_source, OpenAiAudioSource::whisper_1));

        let small_source: OpenAiAudioSource = SourceSize::Small.into();
        assert!(matches!(small_source, OpenAiAudioSource::whisper_1));

        let base_source: OpenAiAudioSource = SourceSize::Base.into();
        assert!(matches!(base_source, OpenAiAudioSource::whisper_1));

        let medium_source: OpenAiAudioSource = SourceSize::Medium.into();
        assert!(matches!(medium_source, OpenAiAudioSource::whisper_1));

        let large_source: OpenAiAudioSource = SourceSize::Large.into();
        assert!(matches!(large_source, OpenAiAudioSource::whisper_1));
    }

    #[test]
    fn test_openai_audio_source_display() {
        use emixai::audio::OpenAiAudioSource;
        
        assert_eq!(OpenAiAudioSource::whisper_1.to_string(), "whisper-1");
        assert_eq!(OpenAiAudioSource::default().to_string(), "whisper-1");
    }

    #[test]
    fn test_openai_audio_source_default() {
        use emixai::audio::OpenAiAudioSource;
        
        assert_eq!(OpenAiAudioSource::default(), OpenAiAudioSource::whisper_1);
    }
}
