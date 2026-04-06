use crate::application::ports::vegetable_repository::VegetableRepository;
use crate::domain::models::{response::CompanionInfo, vegetable::Vegetable};

/// Use case: list all vegetables from the catalogue.
pub struct ListVegetablesUseCase<'a> {
    repo: &'a dyn VegetableRepository,
}

impl<'a> ListVegetablesUseCase<'a> {
    pub fn new(repo: &'a dyn VegetableRepository) -> Self {
        Self { repo }
    }

    pub fn execute(&self) -> Vec<Vegetable> {
        self.repo.get_all()
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

    pub fn execute(&self, id: &str) -> Option<Vegetable> {
        self.repo.get_by_id(id)
    }
}

/// Resolved companion data returned by [`GetCompanionsUseCase`].
pub struct CompanionData {
    pub vegetable: Vegetable,
    pub good: Vec<CompanionInfo>,
    pub bad: Vec<CompanionInfo>,
}

/// Use case: resolve good and bad companion information for a given vegetable.
pub struct GetCompanionsUseCase<'a> {
    repo: &'a dyn VegetableRepository,
}

impl<'a> GetCompanionsUseCase<'a> {
    pub fn new(repo: &'a dyn VegetableRepository) -> Self {
        Self { repo }
    }

    pub fn execute(&self, id: &str) -> Option<CompanionData> {
        let vegetable = self.repo.get_by_id(id)?;
        let all = self.repo.get_all();

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

        Some(CompanionData {
            vegetable,
            good,
            bad,
        })
    }
}
