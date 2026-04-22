use crate::application::ports::{variety_repository::VarietyRepository, RepositoryError};
use crate::domain::models::{response::CompanionInfo, variety::Variety};

/// Use case: list all varieties from the catalogue.
pub struct ListVarietiesUseCase<'a> {
    repo: &'a dyn VarietyRepository,
}

impl<'a> ListVarietiesUseCase<'a> {
    pub fn new(repo: &'a dyn VarietyRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, locale: &str) -> Result<Vec<Variety>, RepositoryError> {
        self.repo.get_all(locale).await
    }
}

/// Use case: list all varieties that belong to a given vegetable.
pub struct ListVarietiesByVegetableUseCase<'a> {
    repo: &'a dyn VarietyRepository,
}

impl<'a> ListVarietiesByVegetableUseCase<'a> {
    pub fn new(repo: &'a dyn VarietyRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        vegetable_id: &str,
        locale: &str,
    ) -> Result<Vec<Variety>, RepositoryError> {
        self.repo.get_by_vegetable_id(vegetable_id, locale).await
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
    ) -> Result<Option<Variety>, RepositoryError> {
        self.repo.get_by_id(id, locale).await
    }
}

/// Resolved companion data returned by [`GetCompanionsUseCase`].
pub struct CompanionData {
    pub variety: Variety,
    pub good: Vec<CompanionInfo>,
    pub bad: Vec<CompanionInfo>,
}

/// Use case: resolve good and bad companion information for a given variety.
pub struct GetCompanionsUseCase<'a> {
    repo: &'a dyn VarietyRepository,
}

impl<'a> GetCompanionsUseCase<'a> {
    pub fn new(repo: &'a dyn VarietyRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        id: &str,
        locale: &str,
    ) -> Result<Option<CompanionData>, RepositoryError> {
        let variety = match self.repo.get_by_id(id, locale).await? {
            None => return Ok(None),
            Some(v) => v,
        };
        let all = self.repo.get_all(locale).await?;

        let good = variety
            .good_companions
            .iter()
            .filter_map(|cid| {
                all.iter().find(|v| &v.id == cid).map(|v| CompanionInfo {
                    id: v.id.clone(),
                    name: v.name.clone(),
                })
            })
            .collect();

        let bad = variety
            .bad_companions
            .iter()
            .filter_map(|cid| {
                all.iter().find(|v| &v.id == cid).map(|v| CompanionInfo {
                    id: v.id.clone(),
                    name: v.name.clone(),
                })
            })
            .collect();

        Ok(Some(CompanionData { variety, good, bad }))
    }
}
