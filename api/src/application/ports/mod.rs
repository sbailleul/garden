pub mod variety_repository;
pub mod vegetable_repository;

use std::fmt;

/// Shared error type for all outbound repository ports.
#[derive(Debug)]
pub enum RepositoryError {
    Database(tokio_postgres::Error),
    Pool(deadpool_postgres::PoolError),
    Json(serde_json::Error),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(e) => write!(f, "Database error: {e}"),
            Self::Pool(e) => write!(f, "Pool error: {e}"),
            Self::Json(e) => write!(f, "JSON error: {e}"),
        }
    }
}

impl std::error::Error for RepositoryError {}

impl From<tokio_postgres::Error> for RepositoryError {
    fn from(e: tokio_postgres::Error) -> Self {
        Self::Database(e)
    }
}

impl From<deadpool_postgres::PoolError> for RepositoryError {
    fn from(e: deadpool_postgres::PoolError) -> Self {
        Self::Pool(e)
    }
}

impl From<serde_json::Error> for RepositoryError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}
