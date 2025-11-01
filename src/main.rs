#![allow(unused_imports)]
#![allow(dead_code)]

// Note: All tests (automated and manual) are now unified in the tests/ directory
// Run 'cargo test' for automated tests
// Run 'cargo test -- --ignored' for manual/interactive tests

use dotenvy::dotenv;
use emix::{Error, Result, set_debug};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    set_debug(true);

    println!("EssentialMix Test Application");
    println!("==============================");
    println!();
    println!("Automated tests are now in the tests/ directory.");
    println!("Run 'cargo test' to execute all automated tests.");
    println!();
    println!("Manual/interactive tests are in tests/manual_tests.rs");
    println!("Run them with: cargo test -- --ignored");
    println!();

    // Example: Display error type
    println!("Error::Canceled = {}", Error::Canceled);

    Ok(())
}
