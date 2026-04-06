use crate::domain::models::vegetable::Vegetable;

/// Outbound port: provides access to the vegetable catalogue.
/// The application layer defines this trait; adapters implement it.
pub trait VegetableRepository: Send + Sync {
    fn get_all(&self) -> Vec<Vegetable>;
    fn get_by_id(&self, id: &str) -> Option<Vegetable>;
}
