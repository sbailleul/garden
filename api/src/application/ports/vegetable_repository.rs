use async_trait::async_trait;

use crate::application::ports::{Page, RepositoryError};
use crate::domain::models::vegetable::Vegetable;

/// Outbound port: provides access to the vegetable catalogue.
/// The `locale` parameter is a BCP-47 language tag (e.g. `"en"`, `"fr"`); the
/// implementation falls back to `"en"` when no translation is available.
#[async_trait]
pub trait VegetableRepository: Send + Sync {
    async fn get_all(&self, locale: &str) -> Result<Vec<Vegetable>, RepositoryError>;
    async fn get_by_id(&self, id: &str, locale: &str)
        -> Result<Option<Vegetable>, RepositoryError>;
    async fn list_page(
        &self,
        locale: &str,
        page: usize,
        size: usize,
    ) -> Result<Page<Vegetable>, RepositoryError>;
    async fn list_page_by_group_id(
        &self,
        group_id: &str,
        locale: &str,
        page: usize,
        size: usize,
    ) -> Result<Page<Vegetable>, RepositoryError>;
}
