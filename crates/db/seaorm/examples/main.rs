// Example usage of emixseaorm
// This file demonstrates how to use the repository implementations
// with proper connection management for SQLite, PostgreSQL, and MySQL/MariaDB

mod models;
mod repositories;

use emixdb::dto::Pagination;
use emixseaorm::prelude::*;
use emixseaorm::repositories::{TRepository, TRepositoryWithRelated};
use sea_orm::Database;
use models::*;
use repositories::*;

// Connection type based on feature
#[cfg(feature = "sqlite")]
const DATABASE_BACKEND: &str = "SQLite";

#[cfg(feature = "postgres")]
const DATABASE_BACKEND: &str = "PostgreSQL";

#[cfg(feature = "mysql")]
const DATABASE_BACKEND: &str = "MySQL/MariaDB";

/// Create a database connection
async fn create_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    #[cfg(feature = "sqlite")]
    let db = Database::connect(database_url).await?;

    #[cfg(feature = "postgres")]
    let db = Database::connect(database_url).await?;

    #[cfg(feature = "mysql")]
    let db = Database::connect(database_url).await?;

    Ok(db)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== emixseaorm Example ===\n");
    println!("Database backend: {}\n", DATABASE_BACKEND);

    // Set up database URL
    let database_url = get_database_url();
    println!("Database URL: {}\n", database_url);

    // Create database connection
    let db = create_connection(&database_url).await?;

    // Create repositories
    let image_repo = ImageRepository::new(db.clone());
    let tag_repo = TagRepository::new(db.clone());

    // Example 1: Create some tags
    println!("--- Creating Tags ---");
    let tag1 = tag_repo
        .create(
            CreateTagDto {
                name: "Nature".to_string(),
            }
            .into(),
        )
        .await?;
    println!("Created tag: {} (ID: {})", tag1.name, tag1.id);

    let tag2 = tag_repo
        .create(
            CreateTagDto {
                name: "Landscape".to_string(),
            }
            .into(),
        )
        .await?;
    println!("Created tag: {} (ID: {})", tag2.name, tag2.id);

    // Example 2: Create an image with tags
    println!("\n--- Creating Image with Tags ---");
    let image = image_repo
        .create_with_tags(CreateImageDto {
            title: "Mountain Sunset".to_string(),
            description: Some("A beautiful sunset over the mountains".to_string()),
            extension: "jpg".to_string(),
            file_size: 2048576,
            mime_type: "image/jpeg".to_string(),
            width: Some(1920),
            height: Some(1080),
            alt_text: Some("Sunset view".to_string()),
            tags: Some("Nature,Landscape".to_string()),
        })
        .await?;
    println!("Created image: {} (ID: {})", image.title, image.id);

    // Example 3: Get image with tags
    println!("\n--- Getting Image with Tags ---");
    if let Some(image_with_tags) = image_repo.get_with_related(image.id).await? {
        println!("Image: {}", image_with_tags.item.title);
        println!("Tags:");
        for tag in &image_with_tags.related {
            println!("  - {}", tag.name);
        }
    }

    // Example 4: List all images with pagination
    println!("\n--- Listing Images (Paginated) ---");
    let images = image_repo
        .list(
            None,
            Some(Pagination {
                page: 1,
                page_size: 10,
            }),
        )
        .await?;
    println!("Found {} total images", images.total);
    for img in images.data {
        println!("  - {} ({})", img.title, img.id);
    }

    // Example 5: List all tags with their images
    println!("\n--- Listing Tags with Images ---");
    let tags_with_images = tag_repo
        .list_with_related(
            None,
            None,
            Some(Pagination {
                page: 1,
                page_size: 10,
            }),
        )
        .await?;
    println!("Found {} total tags", tags_with_images.total);
    for tag_with_images in tags_with_images.data {
        println!("Tag: {}", tag_with_images.item.name);
        if !tag_with_images.related.is_empty() {
            println!("  Images:");
            for img in &tag_with_images.related {
                println!("    - {}", img.title);
            }
        }
    }

    // Example 6: Update an image
    println!("\n--- Updating Image ---");
    let updated_image = image_repo
        .update(
            image.id,
            UpdateImageDto {
                title: Some("Beautiful Mountain Sunset".to_string()),
                description: None,
                extension: None,
                file_size: None,
                mime_type: None,
                width: None,
                height: None,
                alt_text: Some("Beautiful sunset over mountain peaks".to_string()),
            },
        )
        .await?;
    println!("Updated image title to: {}", updated_image.title);

    // Example 7: Add another tag to the image
    println!("\n--- Adding Tag to Image ---");
    let tag3 = tag_repo
        .create(
            CreateTagDto {
                name: "Photography".to_string(),
            }
            .into(),
        )
        .await?;
    image_repo.add_tag(image.id, tag3.id).await?;
    println!(
        "Added tag '{}' to image '{}'",
        tag3.name, updated_image.title
    );

    // Example 8: Count records
    println!("\n--- Counting Records ---");
    let image_count = image_repo.count(None).await?;
    let tag_count = tag_repo.count(None).await?;
    println!("Total images: {}", image_count);
    println!("Total tags: {}", tag_count);

    // Example 9: List tags for an image
    println!("\n--- Listing Tags for Image ---");
    let image_tags = image_repo
        .list_tags(
            image.id,
            None,
            Some(Pagination {
                page: 1,
                page_size: 10,
            }),
        )
        .await?;
    println!("Image '{}' has {} tags:", updated_image.title, image_tags.total);
    for tag in image_tags.data {
        println!("  - {}", tag.name);
    }

    // Example 10: Clean up - delete the created data
    println!("\n--- Cleanup ---");
    image_repo.delete(image.id).await?;
    println!("Deleted image: {}", updated_image.title);

    tag_repo.delete(tag1.id).await?;
    tag_repo.delete(tag2.id).await?;
    tag_repo.delete(tag3.id).await?;
    println!("Deleted all tags");

    println!("\n=== Example completed successfully! ===");
    Ok(())
}

fn get_database_url() -> String {
    #[cfg(feature = "sqlite")]
    return "sqlite::memory:".to_string();

    #[cfg(feature = "postgres")]
    return std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost/seaorm_demo".to_string());

    #[cfg(feature = "mysql")]
    return std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:password@localhost/seaorm_demo".to_string());
}

