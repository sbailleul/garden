#![allow(dead_code)]

use actix_web::{web, App};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use garden::adapters::inbound::http::routes::configure;
use garden::adapters::outbound::postgres::vegetable_repository::PostgresVegetableRepository;
use garden::application::ports::vegetable_repository::VegetableRepository;
use tokio_postgres::NoTls;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

// ---------------------------------------------------------------------------
// Database URL resolution (.env.test → DATABASE_URL)
// ---------------------------------------------------------------------------

pub fn database_url() -> Option<String> {
    dotenvy::from_filename(".env.test").ok();
    std::env::var("DATABASE_URL").ok()
}

async fn build_pool(url: &str) -> Pool {
    let pg_config: tokio_postgres::Config = url.parse().expect("invalid database URL");
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    Pool::builder(mgr)
        .max_size(2)
        .build()
        .expect("failed to build pool")
}

pub async fn test_pool() -> Pool {
    let url = database_url().expect(".env.test must define DATABASE_URL");
    let pool = build_pool(&url).await;
    {
        let mut client = pool.get().await.expect("failed to get DB client");
        embedded::migrations::runner()
            .run_async(&mut **client)
            .await
            .expect("migrations failed");
    }
    pool
}

pub async fn build_app_postgres() -> actix_web::App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let url = database_url().expect(".env.test must define DATABASE_URL");
    let pool = build_pool(&url).await;
    {
        let mut client = pool.get().await.expect("failed to get DB client");
        embedded::migrations::runner()
            .run_async(&mut **client)
            .await
            .expect("migrations failed");
    }
    let repo: Box<dyn VegetableRepository> = Box::new(PostgresVegetableRepository::new(pool));
    App::new()
        .app_data(web::Data::new(repo))
        .configure(configure)
        .app_data(web::JsonConfig::default().error_handler(|err, _req| {
            let message = format!("{err}");
            actix_web::error::InternalError::from_response(
                err,
                actix_web::HttpResponse::BadRequest().json(serde_json::json!({ "error": message })),
            )
            .into()
        }))
}

// ---------------------------------------------------------------------------
// Helpers shared by test files
// ---------------------------------------------------------------------------

pub fn null_layout(rows: usize, cols: usize) -> serde_json::Value {
    let empty_cell = serde_json::json!({"type": "Empty"});
    let row: Vec<serde_json::Value> = vec![empty_cell; cols];
    let layout: Vec<serde_json::Value> = (0..rows)
        .map(|_| serde_json::Value::Array(row.clone()))
        .collect();
    serde_json::Value::Array(layout)
}
