use crate::application::ports::{vegetable_repository::VegetableRepository, RepositoryError};
use crate::domain::models::vegetable::Vegetable;

/// Use case: list all vegetables from the catalogue.
pub struct ListVegetablesUseCase<'a> {
    repo: &'a dyn VegetableRepository,
}

impl<'a> ListVegetablesUseCase<'a> {
    pub fn new(repo: &'a dyn VegetableRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, locale: &str) -> Result<Vec<Vegetable>, RepositoryError> {
        self.repo.get_all(locale).await
    }
}

/// Use case: retrieve a single vegetable by identifier.
pub struct GetVegetableUseCase<'a> {
    repo: &'a dyn VegetableRepository,
}

impl<'a> GetVegetableUseCase<'a> {
    pub fn new(repo: &'a dyn VegetableRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        id: &str,
        locale: &str,
    ) -> Result<Option<Vegetable>, RepositoryError> {
        self.repo.get_by_id(id, locale).await
    }
}
