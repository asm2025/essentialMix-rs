use async_trait::async_trait;
use sea_orm::{
    Condition, DatabaseConnection, DatabaseTransaction, EntityTrait, PrimaryKeyTrait, QueryFilter,
    Select, SelectTwoMany,
};

use crate::{
    Result,
    dto::{ModelWithRelated, Pagination, ResultSet},
    models::TMerge,
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

pub trait TFilterCondition<E: EntityTrait> {
    fn apply(&self, query: Select<E>) -> Select<E>;
}

impl<E: EntityTrait> TFilterCondition<E> for Condition {
    fn apply(&self, query: Select<E>) -> Select<E> {
        query.filter(self.clone())
    }
}

impl<E: EntityTrait, F> TFilterCondition<E> for F
where
    F: Fn(Select<E>) -> Select<E>,
{
    fn apply(&self, query: Select<E>) -> Select<E> {
        self(query)
    }
}

impl<E: EntityTrait, F> TFilterCondition<E> for ClosureFilter<F>
where
    F: Fn() -> Condition,
{
    fn apply(&self, query: Select<E>) -> Select<E> {
        query.filter((self.condition)())
    }
}

impl<E: EntityTrait> TFilterCondition<E> for DirectCondition {
    fn apply(&self, query: Select<E>) -> Select<E> {
        query.filter(self.0.clone())
    }
}

pub trait TFilterRelatedCondition<E: EntityTrait, R: EntityTrait> {
    fn apply(&self, query: SelectTwoMany<E, R>) -> SelectTwoMany<E, R>;
}

impl<E: EntityTrait, R: EntityTrait> TFilterRelatedCondition<E, R> for Condition {
    fn apply(&self, query: SelectTwoMany<E, R>) -> SelectTwoMany<E, R> {
        query.filter(self.clone())
    }
}

impl<E: EntityTrait, R: EntityTrait, F> TFilterRelatedCondition<E, R> for F
where
    F: Fn(SelectTwoMany<E, R>) -> SelectTwoMany<E, R>,
{
    fn apply(&self, query: SelectTwoMany<E, R>) -> SelectTwoMany<E, R> {
        self(query)
    }
}

impl<E: EntityTrait, R: EntityTrait, F> TFilterRelatedCondition<E, R> for ClosureFilter<F>
where
    F: Fn() -> Condition,
{
    fn apply(&self, query: SelectTwoMany<E, R>) -> SelectTwoMany<E, R> {
        query.filter((self.condition)())
    }
}

impl<E: EntityTrait, R: EntityTrait> TFilterRelatedCondition<E, R> for DirectCondition {
    fn apply(&self, query: SelectTwoMany<E, R>) -> SelectTwoMany<E, R> {
        query.filter(self.0.clone())
    }
}

/// Trait for types that provide database connection access
///
/// This trait abstracts over database connections and provides
/// methods for obtaining connections and starting transactions.
#[async_trait]
pub trait THasDatabase {
    fn database(&self) -> &DatabaseConnection;
    async fn begin_transaction(&self) -> Result<DatabaseTransaction>;
}

/// Base repository trait for CRUD operations.
///
/// # Performance Note
/// Avoid deriving `Debug` for types that hold `Arc<dyn TRepository<...>>` trait objects,
/// as this can cause excessive stack allocations when formatting. If debugging is required,
/// consider using a custom `Debug` implementation or increasing stack size via `RUST_MIN_STACK`.
#[async_trait]
pub trait TRepository<E, U>: THasDatabase
where
    E: EntityTrait + Send + Sync,
    U: TMerge<<E as EntityTrait>::ActiveModel> + Send + Sync,
{
    async fn list(
        &self,
        filter: Option<Box<dyn TFilterCondition<E> + Send + Sync>>,
        pagination: Option<Pagination>,
    ) -> Result<ResultSet<<E as EntityTrait>::Model>>;
    async fn count(&self, filter: Option<Box<dyn TFilterCondition<E> + Send + Sync>>)
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
/// Avoid deriving `Debug` for types that hold `Arc<dyn TRepositoryWithRelated<...>>` trait objects,
/// as this can cause excessive stack allocations when formatting. If debugging is required,
/// consider using a custom `Debug` implementation or increasing stack size via `RUST_MIN_STACK`.
#[async_trait]
pub trait TRepositoryWithRelated<E, U, R>: TRepository<E, U>
where
    E: EntityTrait + Send + Sync,
    U: TMerge<<E as EntityTrait>::ActiveModel> + Send + Sync,
    R: EntityTrait + Send + Sync,
{
    async fn list_with_related(
        &self,
        filter: Option<Box<dyn TFilterCondition<E> + Send + Sync>>,
        filter_related: Option<Box<dyn TFilterRelatedCondition<E, R> + Send + Sync>>,
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
