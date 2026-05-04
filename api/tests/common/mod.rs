#![allow(dead_code)]

use actix_web::{web, App};
use garden::adapters::inbound::http::routes::configure;
use garden::adapters::outbound::postgres::group_repository::PostgresGroupRepository;
use garden::adapters::outbound::postgres::variety_repository::PostgresVarietyRepository;
use garden::adapters::outbound::postgres::variety_response_repository::PostgresVarietyResponseRepository;
use garden::adapters::outbound::postgres::vegetable_repository::PostgresVegetableRepository;
use garden::application::ports::group_repository::GroupRepository;
use garden::application::ports::variety_repository::VarietyRepository;
use garden::application::ports::variety_response_repository::VarietyResponseRepository;
use garden::application::ports::vegetable_repository::VegetableRepository;

pub mod db;
pub use db::migrated_pool;

pub async fn test_pool() -> deadpool_postgres::Pool {
    db::migrated_pool().await
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
    let pool = migrated_pool().await;
    let repo: Box<dyn VarietyRepository> = Box::new(PostgresVarietyRepository::new(pool.clone()));
    let variety_response_repo: Box<dyn VarietyResponseRepository> =
        Box::new(PostgresVarietyResponseRepository::new(pool.clone()));
    let vegetable_repo: Box<dyn VegetableRepository> =
        Box::new(PostgresVegetableRepository::new(pool.clone()));
    let group_repo: Box<dyn GroupRepository> = Box::new(PostgresGroupRepository::new(pool));
    App::new()
        .app_data(web::Data::new(repo))
        .app_data(web::Data::new(variety_response_repo))
        .app_data(web::Data::new(vegetable_repo))
        .app_data(web::Data::new(group_repo))
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
