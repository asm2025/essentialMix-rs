#![allow(unused_imports)]
#![allow(dead_code)]

// Note: This is a workspace project. Tests are in crates/*/tests/ directories
// Run 'cargo test --workspace' to execute all tests across the workspace
// Run 'cargo test -- --ignored' to run manual/interactive tests

use dotenvy::dotenv;
use emix::{Error, Result, set_debug};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    set_debug(true);

    println!("EssentialMix Test Application");
    println!("==============================");
    println!();
    println!("This is a workspace project. Tests are in crates/*/tests/ directories.");
    println!("Run 'cargo test --workspace' to execute all tests across the workspace.");
    println!();
    println!("Manual/interactive tests (marked with #[ignore]) can be run with:");
    println!("  cargo test --workspace -- --ignored");
    println!();

    // Example: Display error type
    println!("Error::Canceled = {}", Error::Canceled);

    Ok(())
}
