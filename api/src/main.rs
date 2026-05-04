use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

use garden::adapters::outbound::postgres::group_repository::PostgresGroupRepository;
use garden::adapters::outbound::postgres::variety_repository::PostgresVarietyRepository;
use garden::adapters::outbound::postgres::variety_response_repository::PostgresVarietyResponseRepository;
use garden::adapters::outbound::postgres::vegetable_repository::PostgresVegetableRepository;
use garden::application::ports::group_repository::GroupRepository;
use garden::application::ports::variety_repository::VarietyRepository;
use garden::application::ports::variety_response_repository::VarietyResponseRepository;
use garden::application::ports::vegetable_repository::VegetableRepository;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

fn build_pool(database_url: &str) -> Pool {
    let pg_config: tokio_postgres::Config = database_url.parse().expect("Invalid DATABASE_URL");
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    Pool::builder(mgr)
        .max_size(16)
        .build()
        .expect("Failed to build connection pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");

    let pool = build_pool(&database_url);

    // Run pending migrations at startup.
    {
        let mut client = pool
            .get()
            .await
            .expect("Failed to acquire DB connection for migrations");
        embedded::migrations::runner()
            .run_async(&mut **client)
            .await
            .expect("Database migration failed");
    }

    let repo: Box<dyn VarietyRepository> = Box::new(PostgresVarietyRepository::new(pool.clone()));
    let repo_data = web::Data::new(repo);
    let variety_response_repo: Box<dyn VarietyResponseRepository> =
        Box::new(PostgresVarietyResponseRepository::new(pool.clone()));
    let variety_response_repo_data = web::Data::new(variety_response_repo);
    let vegetable_repo: Box<dyn VegetableRepository> =
        Box::new(PostgresVegetableRepository::new(pool.clone()));
    let vegetable_repo_data = web::Data::new(vegetable_repo);
    let group_repo: Box<dyn GroupRepository> = Box::new(PostgresGroupRepository::new(pool));
    let group_repo_data = web::Data::new(group_repo);

    let bind_addr = "0.0.0.0:8080";
    log::info!("Starting server on {bind_addr}");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://127.0.0.1:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(repo_data.clone())
            .app_data(variety_response_repo_data.clone())
            .app_data(vegetable_repo_data.clone())
            .app_data(group_repo_data.clone())
            .configure(garden::adapters::inbound::http::routes::configure)
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                let message = format!("JSON deserialization error: {err}");
                actix_web::error::InternalError::from_response(
                    err,
                    actix_web::HttpResponse::BadRequest()
                        .json(serde_json::json!({ "error": message })),
                )
                .into()
            }))
    })
    .bind(bind_addr)?
    .run()
    .await
}
