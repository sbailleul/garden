//! PostgreSQL repository integration tests.
//!
//! Requires a running PostgreSQL database. These tests run automatically with
//! `cargo test`. The database URL is read from `api/.env.test` (`DATABASE_URL`).

mod common;

use common::test_pool;
use garden::adapters::outbound::postgres::variety_repository::PostgresVarietyRepository;
use garden::application::ports::variety_repository::VarietyRepository;

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
