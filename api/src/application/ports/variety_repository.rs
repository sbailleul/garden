use std::fmt;

use async_trait::async_trait;

use crate::domain::models::variety::Variety;

#[derive(Debug)]
pub enum VarietyRepositoryError {
    Database(tokio_postgres::Error),
    Pool(deadpool_postgres::PoolError),
}

impl fmt::Display for VarietyRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(e) => write!(f, "Database error: {e}"),
            Self::Pool(e) => write!(f, "Pool error: {e}"),
        }
    }
}

impl std::error::Error for VarietyRepositoryError {}

impl From<tokio_postgres::Error> for VarietyRepositoryError {
    fn from(e: tokio_postgres::Error) -> Self {
        Self::Database(e)
    }
}

impl From<deadpool_postgres::PoolError> for VarietyRepositoryError {
    fn from(e: deadpool_postgres::PoolError) -> Self {
        Self::Pool(e)
    }
}

/// Outbound port: provides access to the variety catalogue.
/// The `locale` parameter is a BCP-47 language tag (e.g. `"en"`, `"fr"`); the
/// implementation falls back to `"en"` when no translation is available.
#[async_trait]
pub trait VarietyRepository: Send + Sync {
    async fn get_all(&self, locale: &str) -> Result<Vec<Variety>, VarietyRepositoryError>;
    async fn get_by_id(
        &self,
        id: &str,
        locale: &str,
    ) -> Result<Option<Variety>, VarietyRepositoryError>;
}
