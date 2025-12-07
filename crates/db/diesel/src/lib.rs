pub mod dto;
pub mod models;

pub use emixcore::{Error, Result};

pub mod prelude {
    pub use diesel::prelude::*;
    pub use diesel_async::{AsyncConnection, RunQueryDsl};
}

pub struct ClosureFilter<F> {
    filter: F,
}

impl<F> ClosureFilter<F> {
    pub fn new(filter: F) -> Self {
        Self { filter }
    }
}

pub trait TFilterQuery<T> {
    fn apply(&self, query: T) -> T;
}

impl<T, F> TFilterQuery<T> for F
where
    F: Fn(T) -> T,
{
    fn apply(&self, query: T) -> T {
        self(query)
    }
}

impl<F, T> TFilterQuery<T> for ClosureFilter<F>
where
    F: Fn(T) -> T,
{
    fn apply(&self, query: T) -> T {
        (self.filter)(query)
    }
}
