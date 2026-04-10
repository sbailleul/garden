use std::fmt;

use async_trait::async_trait;

use crate::domain::models::vegetable::Vegetable;

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

/// Outbound port: provides access to the vegetable catalogue.
/// The application layer defines this trait; adapters implement it.
/// The `locale` parameter is a BCP-47 language tag (e.g. `"en"`, `"fr"`); the
/// implementation falls back to `"en"` when no translation is available.
#[async_trait]
pub trait VegetableRepository: Send + Sync {
    async fn get_all(&self, locale: &str) -> Result<Vec<Vegetable>, RepositoryError>;
    async fn get_by_id(&self, id: &str, locale: &str)
        -> Result<Option<Vegetable>, RepositoryError>;
}
