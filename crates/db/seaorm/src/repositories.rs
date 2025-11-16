use async_trait::async_trait;
use sea_orm::{
    Condition, DatabaseConnection, DatabaseTransaction, EntityTrait, PrimaryKeyTrait, QueryFilter,
    Select, SelectTwoMany,
};

use crate::{
    Result,
    dto::{ModelWithRelated, Pagination, ResultSet},
    models::Merge,
};

pub struct ClosureFilter<F>
where
    F: Fn() -> Condition,
{
    condition: F,
}

impl<F> ClosureFilter<F>
where
    F: Fn() -> Condition,
{
    pub fn new(condition: F) -> Self {
        Self { condition }
    }
}

pub struct DirectCondition(pub Condition);

pub trait FilterCondition<E: EntityTrait> {
    fn apply(&self, query: Select<E>) -> Select<E>;
}

impl<E: EntityTrait> FilterCondition<E> for Condition {
    fn apply(&self, query: Select<E>) -> Select<E> {
        query.filter(self.clone())
    }
}

impl<E: EntityTrait, F> FilterCondition<E> for F
where
    F: Fn(Select<E>) -> Select<E>,
{
    fn apply(&self, query: Select<E>) -> Select<E> {
        self(query)
    }
}

impl<E: EntityTrait, F> FilterCondition<E> for ClosureFilter<F>
where
    F: Fn() -> Condition,
{
    fn apply(&self, query: Select<E>) -> Select<E> {
        query.filter((self.condition)())
    }
}

impl<E: EntityTrait> FilterCondition<E> for DirectCondition {
    fn apply(&self, query: Select<E>) -> Select<E> {
        query.filter(self.0.clone())
    }
}

pub trait FilterRelatedCondition<E: EntityTrait, R: EntityTrait> {
    fn apply(&self, query: SelectTwoMany<E, R>) -> SelectTwoMany<E, R>;
}

impl<E: EntityTrait, R: EntityTrait> FilterRelatedCondition<E, R> for Condition {
    fn apply(&self, query: SelectTwoMany<E, R>) -> SelectTwoMany<E, R> {
        query.filter(self.clone())
    }
}

impl<E: EntityTrait, R: EntityTrait, F> FilterRelatedCondition<E, R> for F
where
    F: Fn(SelectTwoMany<E, R>) -> SelectTwoMany<E, R>,
{
    fn apply(&self, query: SelectTwoMany<E, R>) -> SelectTwoMany<E, R> {
        self(query)
    }
}

impl<E: EntityTrait, R: EntityTrait, F> FilterRelatedCondition<E, R> for ClosureFilter<F>
where
    F: Fn() -> Condition,
{
    fn apply(&self, query: SelectTwoMany<E, R>) -> SelectTwoMany<E, R> {
        query.filter((self.condition)())
    }
}

impl<E: EntityTrait, R: EntityTrait> FilterRelatedCondition<E, R> for DirectCondition {
    fn apply(&self, query: SelectTwoMany<E, R>) -> SelectTwoMany<E, R> {
        query.filter(self.0.clone())
    }
}

/// Trait for types that provide database connection access
///
/// This trait abstracts over database connections and provides
/// methods for obtaining connections and starting transactions.
#[async_trait]
pub trait HasDatabase {
    fn database(&self) -> &DatabaseConnection;
    async fn begin_transaction(&self) -> Result<DatabaseTransaction>;
}

/// Base repository trait for CRUD operations.
///
/// # Performance Note
/// Avoid deriving `Debug` for types that hold `Arc<dyn Repository<...>>` trait objects,
/// as this can cause excessive stack allocations when formatting. If debugging is required,
/// consider using a custom `Debug` implementation or increasing stack size via `RUST_MIN_STACK`.
#[async_trait]
pub trait Repository<E, U>: HasDatabase
where
    E: EntityTrait + Send + Sync,
    U: Merge<<E as EntityTrait>::ActiveModel> + Send + Sync,
{
    async fn list(
        &self,
        filter: Option<Box<dyn FilterCondition<E> + Send + Sync>>,
        pagination: Option<Pagination>,
    ) -> Result<ResultSet<<E as EntityTrait>::Model>>;
    async fn count(&self, filter: Option<Box<dyn FilterCondition<E> + Send + Sync>>)
    -> Result<u64>;
    async fn get(
        &self,
        id: <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<Option<<E as EntityTrait>::Model>>;
    async fn create(&self, model: <E as EntityTrait>::Model) -> Result<<E as EntityTrait>::Model>;
    async fn update(
        &self,
        id: <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
        model: U,
    ) -> Result<<E as EntityTrait>::Model>;
    async fn delete(
        &self,
        id: <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<()>;
}

/// Extended repository trait for entities with relationships.
///
/// # Performance Note
/// Avoid deriving `Debug` for types that hold `Arc<dyn RepositoryWithRelated<...>>` trait objects,
/// as this can cause excessive stack allocations when formatting. If debugging is required,
/// consider using a custom `Debug` implementation or increasing stack size via `RUST_MIN_STACK`.
#[async_trait]
pub trait RepositoryWithRelated<E, U, R>: Repository<E, U>
where
    E: EntityTrait + Send + Sync,
    U: Merge<<E as EntityTrait>::ActiveModel> + Send + Sync,
    R: EntityTrait + Send + Sync,
{
    async fn list_with_related(
        &self,
        filter: Option<Box<dyn FilterCondition<E> + Send + Sync>>,
        filter_related: Option<Box<dyn FilterRelatedCondition<E, R> + Send + Sync>>,
        pagination: Option<Pagination>,
    ) -> Result<ResultSet<ModelWithRelated<<E as EntityTrait>::Model, <R as EntityTrait>::Model>>>;
    async fn get_with_related(
        &self,
        id: <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<Option<ModelWithRelated<<E as EntityTrait>::Model, <R as EntityTrait>::Model>>>;
    async fn delete_related(
        &self,
        id: <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<()>;
}
