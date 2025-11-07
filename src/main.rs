use emixcore::{Error, Result};

// Note: This is a workspace project. Tests are in crates/*/tests/ directories
// Run 'cargo test --workspace' to execute all tests across the workspace
// Run 'cargo test -- --ignored' to run manual/interactive tests

#[tokio::main]
async fn main() -> Result<()> {
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
