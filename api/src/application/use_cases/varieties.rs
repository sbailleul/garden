use crate::application::ports::variety_repository::{VarietyRepository, VarietyRepositoryError};
use crate::domain::models::variety::Variety;

/// Use case: list all varieties from the catalogue.
pub struct ListVarietiesUseCase<'a> {
    repo: &'a dyn VarietyRepository,
}

impl<'a> ListVarietiesUseCase<'a> {
    pub fn new(repo: &'a dyn VarietyRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, locale: &str) -> Result<Vec<Variety>, VarietyRepositoryError> {
        self.repo.get_all(locale).await
    }
}

/// Use case: retrieve a single variety by identifier.
pub struct GetVarietyUseCase<'a> {
    repo: &'a dyn VarietyRepository,
}

impl<'a> GetVarietyUseCase<'a> {
    pub fn new(repo: &'a dyn VarietyRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        id: &str,
        locale: &str,
    ) -> Result<Option<Variety>, VarietyRepositoryError> {
        self.repo.get_by_id(id, locale).await
    }
}
