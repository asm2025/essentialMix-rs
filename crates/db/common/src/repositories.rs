use serde::{Deserialize, Serialize};

/// A model with its related entities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelWithRelated<M, R> {
    pub item: M,
    pub related: Vec<R>,
}

/// Pagination parameters for query results
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u64,
    pub page_size: u64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 10,
        }
    }
}

/// Result set with pagination information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultSet<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub pagination: Option<Pagination>,
}

impl Default for ResultSet<()> {
    fn default() -> Self {
        Self {
            data: vec![],
            total: 0,
            pagination: None,
        }
    }
}

