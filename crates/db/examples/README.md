# Example Implementations

This directory contains example implementations demonstrating how to use the `emixdb` crate.

## Schema Examples

The `schema/` directory contains example SeaORM entity implementations:
- `image.rs` - Example Image entity with timestamps
- `tag.rs` - Example Tag entity
- `image_tag.rs` - Example junction table entity for many-to-many relationships

These examples show how to:
- Use the `Merge` trait from `emixdb::schema` for update DTOs
- Implement SeaORM entities with relationships
- Use DTOs for creating and updating entities

## Repository Examples

The `repositories/` directory contains example repository implementations:
- `image_repository.rs` - Example repository for Image entity with related Tag operations
- `tag_repository.rs` - Example repository for Tag entity with related Image operations

These examples show how to:
- Implement the `IRepository` trait for basic CRUD operations
- Implement the `IRepositoryWithRelated` trait for operations with related entities
- Create custom repository traits that extend the base repository traits
- Handle pagination, filtering, and related entity operations

## Usage

These files are provided as reference implementations. You can copy and adapt them for your own use case. The key patterns demonstrated are:

1. **Schema Implementation**: Define your entities using SeaORM and implement the `Merge` trait for update DTOs
2. **Repository Implementation**: Implement the repository traits provided by `emixdb` to get standard CRUD operations
3. **Custom Extensions**: Extend the base repository traits with domain-specific methods

Note: These examples reference each other (e.g., `crate::image::Entity`). In a real implementation, you would organize your schema and repositories in your own crate structure.

