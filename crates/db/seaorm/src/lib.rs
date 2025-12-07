pub mod dto;
pub mod models;

pub use emixcore::{Error, Result};

use sea_orm::{Condition, EntityTrait, QueryFilter, Select, SelectTwoMany};

pub mod prelude {
    pub use sea_orm::prelude::*;
}

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
