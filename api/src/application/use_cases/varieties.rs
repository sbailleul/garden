use crate::application::ports::{
    variety_response_repository::{VarietyListFilter, VarietyResponse, VarietyResponseRepository},
    Page, RepositoryError,
};

/// Use case: list all varieties from the catalogue.
pub struct ListVarietiesUseCase<'a> {
    repo: &'a dyn VarietyResponseRepository,
}

impl<'a> ListVarietiesUseCase<'a> {
    pub fn new(repo: &'a dyn VarietyResponseRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        locale: &str,
        page: usize,
        size: usize,
        filter: &VarietyListFilter,
    ) -> Result<Page<VarietyResponse>, RepositoryError> {
        self.repo.list_page(locale, page, size, filter).await
    }
}

/// Use case: list all varieties that belong to a given vegetable.
pub struct ListVarietiesByVegetableUseCase<'a> {
    repo: &'a dyn VarietyResponseRepository,
}

impl<'a> ListVarietiesByVegetableUseCase<'a> {
    pub fn new(repo: &'a dyn VarietyResponseRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        vegetable_id: &str,
        locale: &str,
        page: usize,
        size: usize,
        filter: &VarietyListFilter,
    ) -> Result<Page<VarietyResponse>, RepositoryError> {
        self.repo
            .list_page_by_vegetable_id(vegetable_id, locale, page, size, filter)
            .await
    }
}

/// Use case: retrieve a single variety by identifier.
pub struct GetVarietyUseCase<'a> {
    repo: &'a dyn VarietyResponseRepository,
}

impl<'a> GetVarietyUseCase<'a> {
    pub fn new(repo: &'a dyn VarietyResponseRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        id: &str,
        locale: &str,
    ) -> Result<Option<VarietyResponse>, RepositoryError> {
        self.repo.get_by_id(id, locale).await
    }
}
