use crate::application::ports::{vegetable_repository::VegetableRepository, Page, RepositoryError};
use crate::domain::models::{response::CompanionInfo, vegetable::Vegetable};

/// Use case: list all vegetables from the catalogue.
pub struct ListVegetablesUseCase<'a> {
    repo: &'a dyn VegetableRepository,
}

impl<'a> ListVegetablesUseCase<'a> {
    pub fn new(repo: &'a dyn VegetableRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        locale: &str,
        page: usize,
        size: usize,
    ) -> Result<Page<Vegetable>, RepositoryError> {
        self.repo.list_page(locale, page, size).await
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

/// Resolved companion data returned by [`GetVegetableCompanionsUseCase`].
pub struct CompanionData {
    pub vegetable: Vegetable,
    pub good: Vec<CompanionInfo>,
    pub bad: Vec<CompanionInfo>,
}

/// Use case: resolve good and bad companion information for a given vegetable.
pub struct GetVegetableCompanionsUseCase<'a> {
    repo: &'a dyn VegetableRepository,
}

impl<'a> GetVegetableCompanionsUseCase<'a> {
    pub fn new(repo: &'a dyn VegetableRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        id: &str,
        locale: &str,
    ) -> Result<Option<CompanionData>, RepositoryError> {
        let vegetable = match self.repo.get_by_id(id, locale).await? {
            None => return Ok(None),
            Some(v) => v,
        };
        let all = self.repo.get_all(locale).await?;

        let good = vegetable
            .good_companions
            .iter()
            .filter_map(|cid| {
                all.iter().find(|v| &v.id == cid).map(|v| CompanionInfo {
                    id: v.id.clone(),
                    name: v.name.clone(),
                })
            })
            .collect();

        let bad = vegetable
            .bad_companions
            .iter()
            .filter_map(|cid| {
                all.iter().find(|v| &v.id == cid).map(|v| CompanionInfo {
                    id: v.id.clone(),
                    name: v.name.clone(),
                })
            })
            .collect();

        Ok(Some(CompanionData {
            vegetable,
            good,
            bad,
        }))
    }
}
