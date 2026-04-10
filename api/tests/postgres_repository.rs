//! PostgreSQL repository integration tests.
//!
//! Requires a running PostgreSQL database. Set `TEST_DATABASE_URL` to opt in:
//!
//! ```sh
//! TEST_DATABASE_URL=postgres://garden:garden@localhost/garden_test cargo test -- --include-ignored
//! ```

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use garden::adapters::outbound::postgres::vegetable_repository::PostgresVegetableRepository;
use garden::application::ports::vegetable_repository::VegetableRepository;
use tokio_postgres::NoTls;

async fn setup_pool(url: &str) -> Pool {
    let pg_config: tokio_postgres::Config = url.parse().expect("invalid DATABASE_URL");
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    Pool::builder(mgr)
        .max_size(2)
        .build()
        .expect("failed to build pool")
}

async fn run_migrations(pool: &Pool) {
    let mut client = pool
        .get()
        .await
        .expect("could not get client for migrations");
    embedded::migrations::runner()
        .run_async(&mut **client)
        .await
        .expect("migrations failed");
}

fn database_url() -> Option<String> {
    std::env::var("TEST_DATABASE_URL").ok()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn test_get_all_returns_vegetables() {
    let Some(url) = database_url() else {
        return;
    };
    let pool = setup_pool(&url).await;
    run_migrations(&pool).await;
    let repo = PostgresVegetableRepository::new(pool);
    let vegetables = repo.get_all("en").await.expect("get_all failed");
    assert!(!vegetables.is_empty(), "expected at least one vegetable");
    assert!(
        vegetables.iter().any(|v| v.id == "tomato"),
        "tomato should be present"
    );
}

#[tokio::test]
#[ignore]
async fn test_get_all_returns_french_names() {
    let Some(url) = database_url() else {
        return;
    };
    let pool = setup_pool(&url).await;
    run_migrations(&pool).await;
    let repo = PostgresVegetableRepository::new(pool);
    let vegetables = repo.get_all("fr").await.expect("get_all failed");
    let tomato = vegetables
        .iter()
        .find(|v| v.id == "tomato")
        .expect("tomato not found");
    assert_eq!(tomato.name, "Tomate", "expected French name for tomato");
}

#[tokio::test]
#[ignore]
async fn test_get_by_id_returns_vegetable() {
    let Some(url) = database_url() else {
        return;
    };
    let pool = setup_pool(&url).await;
    run_migrations(&pool).await;
    let repo = PostgresVegetableRepository::new(pool);
    let result = repo
        .get_by_id("carrot", "en")
        .await
        .expect("get_by_id failed");
    let carrot = result.expect("carrot not found");
    assert_eq!(carrot.id, "carrot");
    assert_eq!(carrot.name, "Carrot");
}

#[tokio::test]
#[ignore]
async fn test_get_by_id_unknown_returns_none() {
    let Some(url) = database_url() else {
        return;
    };
    let pool = setup_pool(&url).await;
    run_migrations(&pool).await;
    let repo = PostgresVegetableRepository::new(pool);
    let result = repo
        .get_by_id("does-not-exist", "en")
        .await
        .expect("get_by_id failed");
    assert!(result.is_none());
}

#[tokio::test]
#[ignore]
async fn test_locale_fallback_to_en() {
    let Some(url) = database_url() else {
        return;
    };
    let pool = setup_pool(&url).await;
    run_migrations(&pool).await;
    let repo = PostgresVegetableRepository::new(pool);
    // "de" has no translations; should fall back to English name
    let vegetables = repo.get_all("de").await.expect("get_all failed");
    let tomato = vegetables
        .iter()
        .find(|v| v.id == "tomato")
        .expect("tomato not found");
    assert_eq!(
        tomato.name, "Tomato",
        "expected English fallback name for tomato"
    );
}
