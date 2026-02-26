use actix_web::{middleware, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_addr = "0.0.0.0:8080";
    println!("🌱 Garden API started at http://{bind_addr}");
    println!("   GET  /api/vegetables");
    println!("   GET  /api/vegetables/{{id}}/companions");
    println!("   POST /api/plan");
    println!("   ");
    println!("   📖 Swagger UI → http://{bind_addr}/swagger-ui/");
    println!("   📌 OpenAPI spec → http://{bind_addr}/api-docs/openapi.json");
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
