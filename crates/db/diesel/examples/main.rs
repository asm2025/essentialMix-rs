// Example usage of emix-diesel
// This file demonstrates how to use the repository implementations

// Note: This is a placeholder example file.
// In a real application, you would set up your database connection,
// initialize repositories, and use them for CRUD operations.

// Example structure:
#[allow(dead_code)]
mod repositories;
#[allow(dead_code)]
mod schema;

fn main() {
    println!("emix-diesel example");
    println!("See the README.md and individual example files for usage patterns.");
    println!("");
    println!("Key files:");
    println!("  - examples/schema/image.rs - Image entity definition");
    println!("  - examples/schema/tag.rs - Tag entity definition");
    println!("  - examples/schema/image_tag.rs - Junction table definition");
    println!("  - examples/repositories/image_repository.rs - Image repository implementation");
    println!("  - examples/repositories/tag_repository.rs - Tag repository implementation");
}

// Example usage (commented out as it requires a database connection):
/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use diesel_async::AsyncSqliteConnection;
    use emixdiesel::prelude::*;

    // Connect to database
    let conn = AsyncSqliteConnection::establish("database.db").await?;

    // Create repository
    let image_repo = ImageRepository::new(conn);

    // List images with pagination
    let images = image_repo.list(
        None,
        Some(Pagination { page: 1, page_size: 10 })
    ).await?;

    println!("Found {} images", images.total);
    for image in images.data {
        println!("  - {} ({})", image.title, image.id);
    }

    // Get image with tags
    if let Some(image_with_tags) = image_repo.get_with_related(1).await? {
        println!("\nImage: {}", image_with_tags.item.title);
        println!("Tags:");
        for tag in image_with_tags.related {
            println!("  - {}", tag.name);
        }
    }

    Ok(())
}
*/
