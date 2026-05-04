//! PostgreSQL repository integration tests.
//!
//! Requires a running PostgreSQL database. These tests run automatically with
//! `cargo test`. The database URL is read from `api/.env.test` (`DATABASE_URL`).

mod common;

use common::test_pool;
use garden::adapters::outbound::postgres::group_repository::PostgresGroupRepository;
use garden::adapters::outbound::postgres::variety_repository::PostgresVarietyRepository;
use garden::adapters::outbound::postgres::vegetable_repository::PostgresVegetableRepository;
use garden::application::ports::group_repository::GroupRepository;
use garden::application::ports::variety_repository::VarietyRepository;
use garden::application::ports::vegetable_repository::VegetableRepository;

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_get_all_returns_varieties() {
    let pool = test_pool().await;
    let repo = PostgresVarietyRepository::new(pool);
    let varieties = repo.get_all("en").await.expect("get_all failed");
    assert!(!varieties.is_empty(), "expected at least one variety");
    assert!(
        varieties.iter().any(|v| v.id == "tomato"),
        "tomato should be present"
    );
}

#[tokio::test]
async fn test_get_all_returns_french_names() {
    let pool = test_pool().await;
    let repo = PostgresVarietyRepository::new(pool);
    let varieties = repo.get_all("fr").await.expect("get_all failed");
    let tomato = varieties
        .iter()
        .find(|v| v.id == "tomato")
        .expect("tomato not found");
    assert_eq!(tomato.name, "Tomate", "expected French name for tomato");
}

#[tokio::test]
async fn test_get_by_id_returns_variety() {
    let pool = test_pool().await;
    let repo = PostgresVarietyRepository::new(pool);
    let result = repo
        .get_by_id("carrot", "en")
        .await
        .expect("get_by_id failed");
    let carrot = result.expect("carrot not found");
    assert_eq!(carrot.id, "carrot");
    assert_eq!(carrot.name, "Carrot");
}

#[tokio::test]
async fn test_get_by_id_unknown_returns_none() {
    let pool = test_pool().await;
    let repo = PostgresVarietyRepository::new(pool);
    let result = repo
        .get_by_id("does-not-exist", "en")
        .await
        .expect("get_by_id failed");
    assert!(result.is_none());
}

#[tokio::test]
async fn test_locale_fallback_to_en() {
    let pool = test_pool().await;
    let repo = PostgresVarietyRepository::new(pool);
    // "de" has no translations; should fall back to English name
    let varieties = repo.get_all("de").await.expect("get_all failed");
    let tomato = varieties
        .iter()
        .find(|v| v.id == "tomato")
        .expect("tomato not found");
    assert_eq!(
        tomato.name, "Tomato",
        "expected English fallback name for tomato"
    );
}

// ---------------------------------------------------------------------------
// GroupRepository tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_group_get_all_returns_groups() {
    let pool = test_pool().await;
    let repo = PostgresGroupRepository::new(pool);
    let groups = repo.get_all("en").await.expect("get_all failed");
    assert!(!groups.is_empty(), "expected at least one group");
    assert!(
        groups.iter().any(|g| g.id == "bulbes"),
        "bulbes group should be present"
    );
}

#[tokio::test]
async fn test_group_get_all_returns_french_names() {
    let pool = test_pool().await;
    let repo = PostgresGroupRepository::new(pool);
    let groups = repo.get_all("fr").await.expect("get_all failed");
    let bulbes = groups
        .iter()
        .find(|g| g.id == "bulbes")
        .expect("bulbes not found");
    assert_eq!(bulbes.name, "Bulbes");
}

#[tokio::test]
async fn test_group_get_all_returns_english_names() {
    let pool = test_pool().await;
    let repo = PostgresGroupRepository::new(pool);
    let groups = repo.get_all("en").await.expect("get_all failed");
    let bulbes = groups
        .iter()
        .find(|g| g.id == "bulbes")
        .expect("bulbes not found");
    assert_eq!(bulbes.name, "Bulbs");
}

#[tokio::test]
async fn test_group_get_by_id_returns_group() {
    let pool = test_pool().await;
    let repo = PostgresGroupRepository::new(pool);
    let group = repo
        .get_by_id("legumes-fruits", "en")
        .await
        .expect("get_by_id failed")
        .expect("legumes-fruits not found");
    assert_eq!(group.id, "legumes-fruits");
    assert_eq!(group.name, "Fruiting Vegetables");
}

#[tokio::test]
async fn test_group_get_by_id_returns_none_for_unknown() {
    let pool = test_pool().await;
    let repo = PostgresGroupRepository::new(pool);
    let result = repo
        .get_by_id("does-not-exist", "en")
        .await
        .expect("get_by_id failed");
    assert!(result.is_none());
}

#[tokio::test]
async fn test_group_list_page() {
    let pool = test_pool().await;
    let repo = PostgresGroupRepository::new(pool);
    let page = repo.list_page("en", 1, 10).await.expect("list_page failed");
    assert!(page.total >= 4, "expected at least 4 groups");
    assert!(!page.items.is_empty());
}

#[tokio::test]
async fn test_group_locale_fallback_to_en() {
    let pool = test_pool().await;
    let repo = PostgresGroupRepository::new(pool);
    let groups = repo.get_all("de").await.expect("get_all failed");
    let bulbes = groups
        .iter()
        .find(|g| g.id == "bulbes")
        .expect("bulbes not found");
    assert_eq!(bulbes.name, "Bulbs", "expected English fallback");
}

// ---------------------------------------------------------------------------
// VegetableRepository tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_vegetable_get_all_returns_vegetables() {
    let pool = test_pool().await;
    let repo = PostgresVegetableRepository::new(pool);
    let vegetables = repo.get_all("en").await.expect("get_all failed");
    assert!(!vegetables.is_empty(), "expected at least one vegetable");
    assert!(
        vegetables.iter().any(|v| v.id == "tomato"),
        "tomato should be present"
    );
}

#[tokio::test]
async fn test_vegetable_get_all_has_group_id() {
    let pool = test_pool().await;
    let repo = PostgresVegetableRepository::new(pool);
    let vegetables = repo.get_all("en").await.expect("get_all failed");
    for v in &vegetables {
        assert!(
            !v.group_id.is_empty(),
            "vegetable '{}' must have a non-empty group_id",
            v.id
        );
    }
    let tomato = vegetables
        .iter()
        .find(|v| v.id == "tomato")
        .expect("tomato not found");
    assert_eq!(
        tomato.group_id, "legumes-fruits",
        "tomato must belong to legumes-fruits"
    );
}

#[tokio::test]
async fn test_vegetable_get_by_id_returns_vegetable() {
    let pool = test_pool().await;
    let repo = PostgresVegetableRepository::new(pool);
    let result = repo
        .get_by_id("onion", "en")
        .await
        .expect("get_by_id failed");
    let onion = result.expect("onion not found");
    assert_eq!(onion.id, "onion");
    assert_eq!(onion.group_id, "bulbes", "onion must belong to bulbes");
}

#[tokio::test]
async fn test_vegetable_get_by_id_unknown_returns_none() {
    let pool = test_pool().await;
    let repo = PostgresVegetableRepository::new(pool);
    let result = repo
        .get_by_id("does-not-exist", "en")
        .await
        .expect("get_by_id failed");
    assert!(result.is_none());
}

#[tokio::test]
async fn test_vegetable_list_page() {
    let pool = test_pool().await;
    let repo = PostgresVegetableRepository::new(pool);
    let page = repo.list_page("en", 1, 5).await.expect("list_page failed");
    assert!(page.total >= 10, "expected at least 10 vegetables total");
    assert_eq!(page.items.len(), 5, "page size must be 5");
    for v in &page.items {
        assert!(!v.group_id.is_empty(), "paged vegetable must have group_id");
    }
}

#[tokio::test]
async fn test_vegetable_list_page_by_group_id() {
    let pool = test_pool().await;
    let repo = PostgresVegetableRepository::new(pool);
    let page = repo
        .list_page_by_group_id("bulbes", "en", 1, 20)
        .await
        .expect("list_page_by_group_id failed");
    assert!(page.total >= 4, "bulbes must have at least 4 vegetables");
    for v in &page.items {
        assert_eq!(
            v.group_id, "bulbes",
            "all vegetables in page must have group_id=bulbes"
        );
    }
    let ids: Vec<&str> = page.items.iter().map(|v| v.id.as_str()).collect();
    for expected in ["onion", "garlic", "leek", "chive"] {
        assert!(
            ids.contains(&expected),
            "bulbes group must contain '{expected}'"
        );
    }
}

#[tokio::test]
async fn test_vegetable_french_locale() {
    let pool = test_pool().await;
    let repo = PostgresVegetableRepository::new(pool);
    let vegetables = repo.get_all("fr").await.expect("get_all failed");
    let tomato = vegetables
        .iter()
        .find(|v| v.id == "tomato")
        .expect("tomato not found");
    assert_eq!(tomato.name, "Tomate", "expected French name for tomato");
}
