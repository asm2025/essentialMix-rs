use async_trait::async_trait;
use diesel_async::AsyncConnection;

use crate::{
    Result,
    dto::{ModelWithRelated, Pagination, ResultSet},
};

pub struct ClosureFilter<F> {
    filter: F,
}

impl<F> ClosureFilter<F> {
    pub fn new(filter: F) -> Self {
        Self { filter }
    }
}

pub trait FilterQuery<T> {
    fn apply(&self, query: T) -> T;
}

impl<T, F> FilterQuery<T> for F
where
    F: Fn(T) -> T,
{
    fn apply(&self, query: T) -> T {
        self(query)
    }
}

impl<F, T> FilterQuery<T> for ClosureFilter<F>
where
    F: Fn(T) -> T,
{
    fn apply(&self, query: T) -> T {
        (self.filter)(query)
    }
}

/// Trait for types that provide database connection access
///
/// This trait abstracts over different connection types and provides
/// methods for obtaining connections and starting transactions.
#[async_trait]
pub trait HasConnection<Conn>
where
    Conn: AsyncConnection + 'static,
{
    async fn connection(&self) -> Result<&mut Conn>;
    async fn begin_transaction(&self) -> Result<&mut Conn>;
}

/// Base repository trait for CRUD operations
///
/// Type parameters:
/// - `M`: The model type (e.g., `User`)
/// - `U`: The update DTO type
/// - `I`: The ID type (e.g., `i64`)
/// - `Conn`: The connection type (e.g., `AsyncSqliteConnection`)
///
/// # Performance Note
/// Avoid deriving `Debug` for types that hold `Arc<dyn Repository<...>>` trait objects,
/// as this can cause excessive stack allocations when formatting. If debugging is required,
/// consider using a custom `Debug` implementation or increasing stack size via `RUST_MIN_STACK`.
#[async_trait]
pub trait Repository<M, U, I, Conn>: HasConnection<Conn>
where
    M: Send + Sync,
    U: Send + Sync,
    I: Send + Sync,
    Conn: AsyncConnection + 'static,
{
    /// List all entities with optional pagination
    /// Filtering should be implemented in the repository implementation
    async fn list(&self, pagination: Option<Pagination>) -> Result<ResultSet<M>>;

    /// Count all entities
    /// Filtering should be implemented in the repository implementation
    async fn count(&self) -> Result<u64>;

    /// Get a single entity by ID
    async fn get(&self, id: I) -> Result<Option<M>>;

    /// Create a new entity
    async fn create(&self, model: M) -> Result<M>;

    /// Update an entity by ID
    async fn update(&self, id: I, model: U) -> Result<M>;

    /// Delete an entity by ID
    async fn delete(&self, id: I) -> Result<()>;
}

/// Extended repository trait for entities with relationships
///
/// Type parameters:
/// - `M`: The model type (main entity)
/// - `U`: The update DTO type (main entity)
/// - `R`: The related model type
/// - `I`: The ID type
/// - `Conn`: The connection type
///
/// # Performance Note
/// Avoid deriving `Debug` for types that hold `Arc<dyn RepositoryWithRelated<...>>` trait objects,
/// as this can cause excessive stack allocations when formatting. If debugging is required,
/// consider using a custom `Debug` implementation or increasing stack size via `RUST_MIN_STACK`.
#[async_trait]
pub trait RepositoryWithRelated<M, U, R, I, Conn>: Repository<M, U, I, Conn>
where
    M: Send + Sync,
    U: Send + Sync,
    R: Send + Sync,
    I: Send + Sync,
    Conn: AsyncConnection + 'static,
{
    /// List entities with their related entities
    /// Filtering should be implemented in the repository implementation
    async fn list_with_related(
        &self,
        pagination: Option<Pagination>,
    ) -> Result<ResultSet<ModelWithRelated<M, R>>>;

    /// Get a single entity with its related entities
    async fn get_with_related(&self, id: I) -> Result<Option<ModelWithRelated<M, R>>>;

    /// Delete all related entities for a given entity ID
    async fn delete_related(&self, id: I) -> Result<()>;
}
