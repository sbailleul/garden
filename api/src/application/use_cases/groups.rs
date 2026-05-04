use crate::application::ports::{group_repository::GroupRepository, Page, RepositoryError};
use crate::domain::models::group::Group;

/// Use case: list all groups from the catalogue.
pub struct ListGroupsUseCase<'a> {
    repo: &'a dyn GroupRepository,
}

impl<'a> ListGroupsUseCase<'a> {
    pub fn new(repo: &'a dyn GroupRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        locale: &str,
        page: usize,
        size: usize,
    ) -> Result<Page<Group>, RepositoryError> {
        self.repo.list_page(locale, page, size).await
    }
}

/// Use case: retrieve a single group by identifier.
pub struct GetGroupUseCase<'a> {
    repo: &'a dyn GroupRepository,
}

impl<'a> GetGroupUseCase<'a> {
    pub fn new(repo: &'a dyn GroupRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: &str, locale: &str) -> Result<Option<Group>, RepositoryError> {
        self.repo.get_by_id(id, locale).await
    }
}
