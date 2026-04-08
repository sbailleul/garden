use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use garden::adapters::outbound::memory::vegetable_repository::InMemoryVegetableRepository;
use garden::application::ports::vegetable_repository::VegetableRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_addr = "0.0.0.0:8080";
    let repo: Box<dyn VegetableRepository> = Box::new(InMemoryVegetableRepository);
    let repo_data = web::Data::new(repo);
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
