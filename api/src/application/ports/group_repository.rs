use async_trait::async_trait;

use crate::application::ports::{Page, RepositoryError};
use crate::domain::models::group::Group;

/// Outbound port: provides access to the group catalogue.
/// The `locale` parameter is a BCP-47 language tag (e.g. `"en"`, `"fr"`); the
/// implementation falls back to `"en"` when no translation is available.
#[async_trait]
pub trait GroupRepository: Send + Sync {
    async fn get_all(&self, locale: &str) -> Result<Vec<Group>, RepositoryError>;
    async fn get_by_id(&self, id: &str, locale: &str) -> Result<Option<Group>, RepositoryError>;
    async fn list_page(
        &self,
        locale: &str,
        page: usize,
        size: usize,
    ) -> Result<Page<Group>, RepositoryError>;
}
