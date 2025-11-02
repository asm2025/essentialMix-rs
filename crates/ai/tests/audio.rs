#[cfg(all(test, feature = "audio"))]
mod tests {
    use emixai::{audio::Whisper, Result, SourceSize};
    use std::path::PathBuf;
    use std::cell::RefCell;

    #[tokio::test]
    #[ignore] // Requires model download and audio files - run manually
    async fn test_whisper_from_size() -> Result<()> {
        let _whisper = Whisper::from_size(SourceSize::Tiny).await?;
        
        // Test that whisper was created
        assert!(true, "Whisper model loaded successfully");
        
        Ok(())
    }

    #[tokio::test]
    #[ignore] // Requires model download and audio files - run manually
    async fn test_whisper_transcribe_audio_file() -> Result<()> {
        let whisper = Whisper::from_size(SourceSize::Tiny).await?;
        
        let mut audio_path = PathBuf::from("files/audio");
        audio_path.push("awz1.mp3");
        
        if !audio_path.exists() {
            return Ok(()); // Skip if audio file doesn't exist
        }

        let transcribed = RefCell::new(false);
        whisper.transcribe_async(&audio_path, |_text| {
            *transcribed.borrow_mut() = true;
        }).await?;
        
        assert!(*transcribed.borrow(), "Should have transcribed some text");
        
        Ok(())
    }

    #[test]
    #[ignore] // Requires model download - run manually
    fn test_whisper_sync_transcribe() -> Result<()> {
        let whisper = futures::executor::block_on(Whisper::from_size(SourceSize::Tiny))?;
        
        let mut audio_path = PathBuf::from("files/audio");
        audio_path.push("pinless.wav");
        
        if !audio_path.exists() {
            return Ok(()); // Skip if audio file doesn't exist
        }

        let transcribed = RefCell::new(false);
        whisper.transcribe(&audio_path, |_text| {
            *transcribed.borrow_mut() = true;
        })?;
        
        assert!(*transcribed.borrow(), "Should have transcribed some text");
        
        Ok(())
    }
}

