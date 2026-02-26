use actix_web::{middleware, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let bind_addr = "0.0.0.0:8080";
    log::info!("🌱 Garden API started at http://{bind_addr}");
    log::info!("   GET  /api/vegetables");
    log::info!("   GET  /api/vegetables/{{id}}/companions");
    log::info!("   POST /api/plan");
    log::info!("   📖 Swagger UI  → http://{bind_addr}/swagger-ui/");
    log::info!("   📌 OpenAPI spec → http://{bind_addr}/api-docs/openapi.json");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(garden::api::routes::configure)
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
