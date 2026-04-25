use async_trait::async_trait;

use crate::application::ports::{Page, RepositoryError};
use crate::domain::models::variety::Variety;

/// Outbound port: provides access to the variety catalogue.
/// The application layer defines this trait; adapters implement it.
/// The `locale` parameter is a BCP-47 language tag (e.g. `"en"`, `"fr"`); the
/// implementation falls back to `"en"` when no translation is available.
#[async_trait]
pub trait VarietyRepository: Send + Sync {
    async fn get_all(&self, locale: &str) -> Result<Vec<Variety>, RepositoryError>;
    async fn get_by_id(&self, id: &str, locale: &str) -> Result<Option<Variety>, RepositoryError>;
    async fn get_by_vegetable_id(
        &self,
        vegetable_id: &str,
        locale: &str,
    ) -> Result<Vec<Variety>, RepositoryError>;
    async fn list_page(
        &self,
        locale: &str,
        page: usize,
        size: usize,
    ) -> Result<Page<Variety>, RepositoryError>;
    async fn list_page_by_vegetable_id(
        &self,
        vegetable_id: &str,
        locale: &str,
        page: usize,
        size: usize,
    ) -> Result<Page<Variety>, RepositoryError>;
}
