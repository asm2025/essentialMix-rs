// Manual/Interactive tests - these require user input or external resources
// These are NOT run automatically but can be called manually
// Use #[ignore] to skip them in automated test runs

use emix::Result;
// Note: TempMail and TempMailProvider are available if needed for manual testing
// use emixnet::web::mail::{TempMail, TempMailProvider};

#[allow(dead_code)]
#[tokio::test]
#[ignore] // Manual test - requires user input
async fn test_tempmail() -> Result<()> {
    println!("\n=== Manual Test: TempMail ===");
    println!("This test requires user input and external email services.");
    
    // Example implementation - uncomment and modify as needed:
    /*
    let input = emix::io::terminal::get(Some("Enter the email [ENTER to generate]: "))?;
    let email = if input.is_empty() {
        TempMail::random().await?
    } else {
        TempMail::parse(TempMailProvider::Tempmail, &input)
    };
    println!("Email: {}", email.address());
    */
    
    Ok(())
}

#[allow(dead_code)]
#[tokio::test]
#[ignore] // Manual test - requires model downloads and audio files
async fn test_rwhisper() -> Result<()> {
    println!("\n=== Manual Test: Whisper Audio Transcription ===");
    println!("This test requires audio files and model downloads.");
    println!("See src/tests/audio.rs for the original implementation.");
    Ok(())
}

#[allow(dead_code)]
#[tokio::test]
#[ignore] // Manual test - requires model downloads and is interactive
async fn test_llma() -> Result<()> {
    println!("\n=== Manual Test: LLM Chat ===");
    println!("This test requires model downloads and is interactive.");
    println!("See src/tests/language.rs for the original implementation.");
    Ok(())
}

#[allow(dead_code)]
#[tokio::test]
#[ignore] // Manual test - requires image models
async fn test_image() -> Result<()> {
    println!("\n=== Manual Test: Image Processing ===");
    println!("This test requires image models and processing capabilities.");
    println!("See src/tests/vision.rs for the original implementation.");
    Ok(())
}

#[allow(dead_code)]
#[tokio::test]
#[ignore] // Manual test - requires ExpressVPN installation
async fn test_expressvpn() -> Result<()> {
    println!("\n=== Manual Test: ExpressVPN ===");
    println!("This test requires ExpressVPN to be installed and configured.");
    println!("See src/tests/vpn.rs for the original implementation.");
    Ok(())
}

