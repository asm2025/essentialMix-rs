mod error;
#[cfg(feature = "vpn")]
pub mod vpn;
pub mod web;

pub use emixcore::{Error, Result};
pub use error::{NetError, Result as NetResult};
