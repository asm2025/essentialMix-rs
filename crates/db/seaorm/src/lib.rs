pub mod dto;
pub mod models;
pub mod repositories;

pub use emixcore::{Error, Result};

pub mod prelude {
    pub use sea_orm::prelude::*;
}
